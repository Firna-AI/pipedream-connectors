# Vendored OpenAPI file is behind the live API

- **Severity:** Medium. **Effort:** Medium. **Confidence:** High.
  **Harder later:** Yes.
- **Kind:** Post-merge task.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`.
  Finding 53 of the final review of 2026-10-22, and a note outside the scope
  of its webhooks review. The user chose outcome A on 2026-10-26.
- **Location:** `crates/pipedream-connect-types/tests/fixtures/pipedream_openapi_swagger.json`,
  `crates/pipedream-connect-types/tests/fixtures/refresh-openapi.sh`, and
  `crates/pipedream-connect-types/src/admin/oauth_apps.rs:16` and `:54`.
- **Reason:** On 2026-10-26 the user chose to refresh the fixture in a
  separate change, so that its diff gets its own review.

- **Terms:**
  - **Fixture:** the copy of the Pipedream OpenAPI document at
    `crates/pipedream-connect-types/tests/fixtures/pipedream_openapi_swagger.json`.
    It is from 2026-10-20.
  - **Live document:** the OpenAPI document at
    `https://pipedream.com/docs/pipedream_openapi_swagger.json`.
  - **Coverage test:** `crates/pipedream-connect/tests/openapi_coverage/`. It
    checks that each fixture operation has a protocol table row, a golden
    wire row, and a client method.
  - **Conformance test:** `crates/pipedream-connect-types/tests/conformance/`.
    It builds samples from the fixture schemas. It checks that the models
    decode and encode each documented field.
  - **JWT client authentication:** an OAuth token request that sends a JSON
    Web Token, signed with the private key of the client, instead of a client
    secret.
- **Background:**
  - The SDK models are hand-written. The SDK does not generate code from the
    OpenAPI document. The coverage and conformance tests compare the client
    and the models with the fixture.
  - `refresh-openapi.sh` downloads the live document over the fixture.
    `crates/pipedream-connect-types/tests/fixture_refresh.rs` tests the script
    without a network call.
  - The final review of 2026-10-22 found the fixture behind the live
    document. The live document had new fields, the credential field
    `client_assertion`, and the operation
    `resetDeployedTriggerWebhookDeliveryState`.
  - On 2026-10-26 the user chose to refresh the fixture in a separate change,
    to implement each new operation, and to review the diff.
  - On 2026-11-02 a structural diff found 26 changes. It did not count
    descriptions and code samples. The live document had 73 operations and
    141 schemas. The fixture had 72 operations and 140 schemas.
  - Both files are formatted JSON, so `git diff` shows the changes. On
    2026-11-02 the text diff had 312 added lines and 24 removed lines.
  - These were the changes on 2026-11-02:
    - The operation `resetDeployedTriggerWebhookDeliveryState` is new. It is
      `POST /v1/connect/{project_id}/deployed-triggers/{trigger_id}/webhooks/{webhook_id}/reset_delivery_state`.
      It reactivates a disabled webhook and clears its delivery failure
      evidence. It requires the `x-pd-environment` header and the
      `external_user_id` query. It returns `GetWebhookWithSigningKeyResponse`.
    - The schema `WebhookDeliveryState` is new. Its `state` is `happy`,
      `penalty`, `life_support`, or `deactivated`. It also has
      `consecutive_failures`, `sheds`, a nullable `since` in epoch seconds,
      and a nullable `last_error`.
    - `TriggerWebhook` and `WebhookWithSigningKey` get `active`,
      `delivery_state`, and `delivery_backend` with the values `jvm` and
      `valkey`.
    - `OauthApp.client_id` and `OauthApp.owner_id` become nullable.
      `client_id` is null for a client that has no provider credentials yet.
      `owner_id` is null for a Pipedream official client.
    - The scope `connect:oauth_apps:read` now also fetches Pipedream official
      clients.
    - `CreateOAuthTokenOpts` gets `client_assertion` and
      `client_assertion_type`, and `client_secret` becomes optional. This
      allows JWT client authentication. Pipedream rejects a request that
      sends both a secret and an assertion.
    - `POST /v1/oauth/token` documents a 400 and a 401 response. Each has the
      body `{ "error": string }`.
    - `CreateOauthAppOpts` makes `client_id` and `client_secret` optional.
    - `CreateTokenOpts` gets the optional `app_id` and `oauth_app_id`.
    - `CreateAccountOpts` gets the optional `app_override_id` with the
      pattern `^ao_[0-9A-Za-z]+$`.
    - `GET /v1/connect/tokens/{ctok}/validate` makes `app_id` optional. It
      gets the optional query `app_override_id`.
    - `ValidateTokenResponse` gets the nullable `app_override_field_names`,
      `app_override_id`, and `oauth_app_connect_base_url`.
    - `App` gets the nullable `oauth_app_id`, the ID of its official OAuth
      client.
  - The SDK model `OauthApp` declares `client_id: String` at line 16 and
    `owner_id: String` at line 54 of
    `crates/pipedream-connect-types/src/admin/oauth_apps.rs`.
  - A probe on 2026-11-02 decoded two `OauthApp` objects. One had
    `"client_id": null`, and the other had `"owner_id": null`. Both failed
    with `invalid type: null, expected a string`.
  - The trait `OauthAppsApi` in
    `crates/pipedream-connect/src/resources/oauth_apps.rs` has `list`,
    `create`, `get`, and `update`. Each call fails when its response holds
    such a client. No other code reads the two fields.
  - `docs/protocol/connect-client.md:41-42` says that the token request MUST
    contain `client_secret`. So JWT client authentication changes a
    requirement, and the user decides it.
  - `docs/protocol/connect-resources.md:112-116` keeps the common grammar for
    app override IDs, although the OpenAPI document shows `^ao_`. It adds no
    prefix rule without a user decision. So `app_override_id` uses the
    existing `AppOverrideId` type.
  - On 2026-11-02 a scratch copy of the repository ran both tests with the
    live document:
    - The coverage test `covered_operations_have_documentation_and_golden_wire_rows`
      failed with `operation table count differs`, left 68 and right 69.
    - In the conformance test,
      `request_schemas_tests::schema_request_samples_encode_the_same_documented_body`
      failed with `CreateAccountOpts.app_override_id: lost field`.
    - `query_schemas_tests::operation_query_samples_match_encoded_models`
      failed with `validateToken.app_override_id: lost field`.
    - `every_response_union_map_null_and_optional_shape_decodes` failed with
      `documented field in extra: oauth_app_id: "oa_sample"`.
    - The other three conformance tests passed. Each failed test stops at
      its first gap, so more gaps show after each fix.
  - A new trigger webhook operation goes beside
    `regenerate_webhook_signing_key` in these files:
    - The trait `DeployedTriggersApi` in
      `crates/pipedream-connect/src/resources/deployed_triggers.rs`.
    - The request code in
      `crates/pipedream-connect/src/resources/trigger_operations.rs`.
    - The table row in `docs/protocol/resources/triggers.md`.
    - The golden wire row in
      `crates/pipedream-connect/tests/openapi_coverage/wire.rs`, and the
      client call in `admin.rs` of the same folder.
- **Task:**
  1. Run `crates/pipedream-connect-types/tests/fixtures/refresh-openapi.sh`.
  2. Diff the old and the new fixture. Review each change against the list
     above. The live document can change again.
  3. Run `cargo test -p pipedream-connect --test openapi_coverage` and
     `cargo test -p pipedream-connect-types --test conformance`. Fix one gap,
     then run them again.
  4. Implement each new operation. Give it a client method, a protocol table
     row, a golden wire row, a coverage call, and a unit test.
  5. Add each new documented field to its model. Make each nullable field an
     `Option`. Give `WebhookDeliveryState.since` the seconds decoder
     `deserialize_timestamp_seconds`, unless the shared date decoder picks the
     unit by size by then. The default decoder reads an integer as epoch
     milliseconds.
  6. Wrap each new credential field in a secret type, such as `SecretString`.
  7. Ask the user before the SDK adds JWT client authentication or another
     new requirement. Record a follow-up item when the user defers it.
  8. Update the protocol docs and the READMEs that the change affects.
  9. Run `cargo xtask check`.
- **Impact if no action:**
  - Each `OauthAppsApi` call fails when its response holds an OAuth client
    without a client ID or without an owner.
  - The SDK cannot reset a disabled trigger webhook, send the new inputs, or
    use JWT client authentication.
  - The tests cannot see these changes. Each host that adopts the current
    `OauthApp` model must change its code when the two fields become
    optional.
- **Needs:** Network access to the live document. No Pipedream credentials.
  The answer of the user before the SDK adds JWT client authentication.
- **Done when:**
  - The fixture is the live document of the refresh day.
  - The coverage and conformance tests pass against it.
  - `OauthApp` decodes a null `client_id` and a null `owner_id`.
  - Each new operation has a client method, a protocol table row, a golden
    wire row, and a unit test.
  - The user answered the question on JWT client authentication, or a
    follow-up item records it.
  - `cargo xtask check` passes.
- **Related:**
  - A closed decision on the same code. Since the user decision of
    2026-11-03, the emitter dates decode as epoch seconds: `created_at` and
    `updated_at` of `DeployedComponent`, `HttpInterface`, and
    `TimerInterface`, and `schedule_changed_at` of `TimerInterface`. The
    reason is that every published Pipedream example since 2020 shows these
    dates in epoch seconds. The live document of 2026-11-02 still says "epoch
    milliseconds" for six of these dates, and it gives `schedule_changed_at`
    no unit. So the conformance test records an exception for each of the
    seven emitter dates. The refresh must keep these exceptions, unless
    Pipedream corrects its text. When the text of a field changes to epoch
    seconds, the refresh removes the exception of that field. That decision
    added no size rule to the shared date decoder, so step 5 still needs the
    seconds decoder.
  - `plans/follow-ups/deployed-trigger-email-prefix.md`, same code. That item
    asks how the SDK accepts the IDs of email triggers, which start with
    `ei_`. The live document of 2026-11-02 has no email interface, so this
    refresh does not settle that item. The new reset call for
    `resetDeployedTriggerWebhookDeliveryState` takes a deployed trigger ID,
    so the trigger ID rule that the other item picks applies to it.
  - `plans/follow-ups/workflows-shutdown.md`, the same cause. Pipedream
    Workflows shut down on 2027-03-31. That item leaves the deployed-trigger
    workflow routes to this refresh: `listDeployedTriggerWorkflows`,
    `updateDeployedTriggerWorkflows`, and the `workflow_id` of a trigger
    deploy. The live document of 2026-11-02 still has them. When a refresh
    finds them gone, it removes their client methods, protocol rows, golden
    wire rows, and tests. Get the approval of the user first, because the
    removal deletes functionality.
