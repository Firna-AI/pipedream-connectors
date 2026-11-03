//! Credential setup, emitter date checks, and cleanup for live trigger tests.

use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use pipedream_connect::PipedreamClient;
use pipedream_connect::config::ClientCredentials;
use pipedream_connect_types::{
    DeleteTriggerRequest, DeployedTriggerId, Emitter, Environment, ExternalUserId,
    ListDeployedTriggersRequest, PageRequest, ProjectId,
};

/// Builds a development client only when the live opt-in and credentials exist.
pub(super) fn client_from_env() -> Option<PipedreamClient> {
    if env::var("PIPEDREAM_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live Pipedream emitter dates: set PIPEDREAM_LIVE_TESTS=1");
        return None;
    }
    let names = [
        "PIPEDREAM_CLIENT_ID",
        "PIPEDREAM_CLIENT_SECRET",
        "PIPEDREAM_PROJECT_ID",
    ];
    let values = names.map(|name| env::var(name).ok().filter(|value| !value.trim().is_empty()));
    let missing: Vec<_> = names
        .iter()
        .zip(&values)
        .filter_map(|(name, value)| value.is_none().then_some(*name))
        .collect();
    if !missing.is_empty() {
        eprintln!(
            "skipping live Pipedream emitter dates: missing {}",
            missing.join(", ")
        );
        return None;
    }
    let [Some(client_id), Some(client_secret), Some(project_id)] = values else {
        unreachable!("checked all required credentials")
    };
    Some(
        PipedreamClient::builder()
            .project_id(ProjectId::new(project_id).expect("valid live project ID"))
            .environment(Environment::Development)
            .client_credentials(
                ClientCredentials::new(client_id, client_secret).expect("valid live credentials"),
            )
            .build()
            .expect("live client configuration"),
    )
}

/// Keeps the deployment ID available for cleanup even if its discriminator is missing.
pub(super) fn emitter_id(emitter: &Emitter) -> Option<&str> {
    match emitter {
        Emitter::DeployedComponent(value) => Some(value.id.as_str()),
        Emitter::HttpInterface(value) => Some(&value.id),
        Emitter::TimerInterface(value) => Some(&value.id),
        Emitter::Other(value) => value.get("id").and_then(serde_json::Value::as_str),
    }
}

/// Requires a typed emitter and dates within one day of the current time.
pub(super) fn check_emitter_dates(emitter: &Emitter) -> Result<(), String> {
    let dates = match emitter {
        Emitter::DeployedComponent(value) => vec![
            ("created_at", &value.created_at),
            ("updated_at", &value.updated_at),
        ],
        Emitter::HttpInterface(value) => vec![
            ("created_at", &value.created_at),
            ("updated_at", &value.updated_at),
        ],
        Emitter::TimerInterface(value) => vec![
            ("created_at", &value.created_at),
            ("updated_at", &value.updated_at),
            ("schedule_changed_at", &value.schedule_changed_at),
        ],
        Emitter::Other(_) => return Err("expected a typed emitter with its type field".into()),
    };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?;
    let now = i64::try_from(now.as_secs()).map_err(|error| error.to_string())?;
    for (field, date) in dates {
        if date.timestamp().abs_diff(now) > 86_400 {
            return Err(format!(
                "emitter {} {field} is not within one day of now: {}",
                emitter_id(emitter).unwrap_or("unknown"),
                date.as_datetime()
            ));
        }
    }
    Ok(())
}

/// Attempts every deletion before reporting cleanup errors.
pub(super) async fn delete_created_triggers(
    client: &PipedreamClient,
    external_user_id: &ExternalUserId,
    trigger_ids: &[DeployedTriggerId],
) -> Vec<String> {
    let mut errors = Vec::new();
    for trigger_id in trigger_ids {
        if let Err(error) = client
            .deployed_triggers()
            .delete(
                trigger_id.clone(),
                DeleteTriggerRequest {
                    external_user_id: external_user_id.clone(),
                    ignore_hook_errors: Some(true),
                    missing_ok: true,
                },
            )
            .await
        {
            errors.push(format!("delete {trigger_id}: {error:?}"));
        }
    }
    errors
}

/// Cleans up Google Calendar smoke deployments after an unreadable deploy response.
pub(super) async fn cleanup_created_triggers(
    client: &PipedreamClient,
    external_user_id: &ExternalUserId,
) {
    let Ok(listed) = client
        .deployed_triggers()
        .list(ListDeployedTriggersRequest {
            external_user_id: external_user_id.clone(),
            page: PageRequest::default(),
            emitter_type: None,
        })
        .await
    else {
        return;
    };
    for emitter in listed.data {
        let Emitter::DeployedComponent(trigger) = emitter else {
            continue;
        };
        let matches_test = trigger
            .component_key
            .as_ref()
            .is_some_and(|key| key.as_str() == "google_calendar-new-or-updated-event-instant")
            && trigger.webhook_urls.as_ref().is_some_and(|urls| {
                urls.iter()
                    .any(|url| url.contains("pipedream-trigger-smoke"))
            });
        if matches_test {
            let _ = client
                .deployed_triggers()
                .delete(
                    trigger.id,
                    DeleteTriggerRequest {
                        external_user_id: external_user_id.clone(),
                        ignore_hook_errors: Some(true),
                        missing_ok: true,
                    },
                )
                .await;
        }
    }
}
