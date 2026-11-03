# CLI change warning fires when no request left

- **Severity:** Low. **Likelihood:** Common. **Effort:** Medium.
  **Confidence:** High. **Harder later:** Yes.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  decisions-round review of the CLI and workspace listed it outside its scope
  on 2026-10-26.
- **Location:** `crates/pipedream-cli/src/report/api.rs:40-54`,
  `crates/pipedream-connect/src/reqwest_transport.rs:88-102`,
  `crates/pipedream-connect/src/error.rs:137-144`,
  `crates/pipedream-cli/tests/cli_smoke.rs:159-176`,
  `crates/pipedream-cli/README.md:262`.
- **Reason:** The fix changes the SDK error type and user-visible CLI text, so
  it needs a user decision and its own branch.

- **Source:** Review.
- **Category:** product bug.
- **Terms:**
  - **Change command:** a CLI command that changes data: `actions run`,
    `tokens create`, `triggers deploy`, `triggers delete`, and `proxy` with
    POST, PUT, PATCH, or DELETE.
  - **Change warning:** the text that a change command prints when its
    request may have reached Pipedream without a readable answer.
  - **`Unavailable`:** the SDK error in `crates/pipedream-connect/src/error.rs`
    for a timeout, a transport failure, a 408, or a 5xx answer.
- **Background:**
  - `pipedream-cli` in `crates/pipedream-cli` is the command-line tool. It
    calls the SDK client crate `pipedream-connect`.
  - Each CLI run first requests an OAuth token from Pipedream. Then it sends
    the command request.
  - `transport_error` in `crates/pipedream-connect/src/reqwest_transport.rs`
    maps every transport failure to
    `Error::Unavailable { status: None, retry_after: None }`.
  - That error covers a refused connection, a DNS failure, and a reset after
    the request was sent. `transport_error` logs
    `reqwest::Error::is_connect()` at debug level, and then drops it.
  - A failed token request gives the same error, but the command request
    never left.
  - The SDK never retries POST, PUT, PATCH, or DELETE, as
    `docs/protocol/connect-client.md` requires. So when the connection does
    not open, Pipedream received nothing.
  - `explain` in `crates/pipedream-cli/src/report/api.rs` gives every
    `Unavailable` error of a change command the change warning.
  - The public SDK `Error` enum is not `#[non_exhaustive]`. A new field or
    variant breaks a caller that matches every field or every variant. No
    version of the SDK is released yet.
  - The CLI README gives exit code 6 the meaning "Pipedream was unavailable
    or did not respond. The request may have run."
  - The smoke test `every_command_with_an_unreachable_origin_exits_six` in
    `crates/pipedream-cli/tests/cli_smoke.rs` uses the origin
    `https://127.0.0.1:9`, a closed local port. It asserts the change warning
    for each change command, and it passes on 2026-11-02.
- **Problem:** When no request reaches Pipedream, a change command still says
  that the change may have happened. For `actions run` with the closed port,
  the CLI prints this error:
  `The CLI could not reach Pipedream at https://127.0.0.1:9. The action may
  have run. Check its effects before you run it again.`
- **Impact if no action:** After a network outage, a DNS failure, or a wrong
  API origin, the user checks for a change that cannot exist. The user can
  also delay a retry that is safe.
- **Root question:** How does the CLI tell the user whether a change can have
  happened when the network fails?
- **Options:**
  - **A.** Change nothing. The warning stays cautious, and the check that it
    asks for costs the user little. Effort: Small.
  - **B.** Reword each change warning so that it is true in both cases.
    Example: "If the request reached Pipedream, the action may have run." The
    user still cannot tell which case happened. Effort: Small.
  - **C.** Let the SDK report whether the request left. A connection that
    never opened, or a failed token request, means that nothing was sent.
    Carry this fact in `Unavailable` or in a new error variant. The CLI then
    prints the network error without the change warning when nothing was
    sent. Update `docs/protocol/connect-client.md`, the CLI README, and the
    tests. Effort: Medium.
- **Left out:** A CLI step that requests the token before the command request.
  It fails the goal for a connection failure of the command request itself.
- **Wider fix:** C. Every change command reads its warning through
  `after_request` in `crates/pipedream-cli/src/report/target.rs`, so one SDK
  fact fixes all five commands. SDK hosts also learn that nothing was sent.
- **Previous examples:** None found.
- **Recommendation:** C. The warning then appears only when the change can
  have happened.
- **Action:** ask the user, because option C changes user-visible CLI text and
  adds an SDK error field that no requirement covers.
