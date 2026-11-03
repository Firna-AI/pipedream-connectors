//! OAuth token requests from the CLI client configuration.

use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use clap::Parser;
use http::{HeaderMap, Method, StatusCode};
use pipedream_connect::PipedreamClient;
use pipedream_connect::runtime::ClockMock;
use pipedream_connect::transport::{HttpRequest, HttpResponse, HttpTransportMock};
use pipedream_connect_types::ListUsageRequest;
use serde_json::Value;
use unimock::{MockFn, Unimock, matching};

use crate::cli::Cli;
use crate::settings::resolve;
use crate::test_support::Env;

#[tokio::test]
async fn cli_client_token_request_uses_connect_scope() {
    let cli =
        Cli::try_parse_from(["pipedream-cli", "usage", "--start", "1", "--end", "2"]).unwrap();
    let settings = resolve(&cli.global, &Unimock::new(Env::CONFIGURED.clauses())).unwrap();
    let transport = Arc::new(Unimock::new((
        HttpTransportMock::send
            .some_call(matching!((request) if request.method == Method::POST))
            .answers(&|_, request: HttpRequest| {
                let body: Value = serde_json::from_slice(request.body.as_deref().unwrap()).unwrap();
                assert_eq!(body["scope"], "connect:*");
                Ok(HttpResponse {
                    status: StatusCode::OK,
                    headers: HeaderMap::new(),
                    body:
                        br#"{"access_token":"test-bearer","token_type":"Bearer","expires_in":3600}"#
                            .to_vec(),
                })
            }),
        HttpTransportMock::send
            .some_call(matching!((request) if request.method == Method::GET))
            .returns(Ok(HttpResponse {
                status: StatusCode::OK,
                headers: HeaderMap::new(),
                body: br#"{"data":[]}"#.to_vec(),
            })),
    )));
    let clock = Arc::new(Unimock::new((
        ClockMock::now.each_call(matching!()).returns(UNIX_EPOCH),
        ClockMock::monotonic
            .each_call(matching!())
            .returns(Duration::ZERO),
    )));
    let client = PipedreamClient::new(
        settings.client_config().unwrap(),
        transport,
        clock,
        Arc::new(Unimock::new(())),
    )
    .unwrap();

    client
        .usage()
        .list(ListUsageRequest {
            start_ts: 1,
            end_ts: 2,
        })
        .await
        .unwrap();
}
