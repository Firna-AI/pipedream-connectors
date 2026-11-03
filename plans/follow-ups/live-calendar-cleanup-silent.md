# Calendar live test cleanup fails without a message

- **Severity:** Medium. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-03, from branch `calummoore/pipedream-rust-sdk-plan`. The
  check of review finding 89 listed it outside the review scope on
  2026-11-03.
- **Location:**
  `crates/pipedream-connect/tests/_support/live_emitter_dates.rs:142-183`,
  `crates/pipedream-connect/tests/live_pipedream.rs:160-187`,
  `crates/pipedream-connect/README.md:230-231`.
- **Reason:** The reviewed change only moved this cleanup code into the
  support module, so its behaviour was outside the scope of that review.

- **Source:** Review.
- **Category:** test.
- **Terms:**
  - **External user:** a partition of a Pipedream project that owns connected
    accounts and deployed triggers.
  - **Shared user:** the external user from `PIPEDREAM_EXTERNAL_USER_ID`, which
    owns the Google Calendar account from `PIPEDREAM_CONNECTED_ACCOUNT_ID`.
  - **Lifecycle smoke:** the live test
    `live_pipedream_google_calendar_trigger_lifecycle` in
    `crates/pipedream-connect/tests/live_pipedream.rs`, which deploys, lists,
    and deletes one Google Calendar trigger.
  - **Recovery:** the function `cleanup_created_triggers` in
    `crates/pipedream-connect/tests/_support/live_emitter_dates.rs`, which the
    lifecycle smoke calls after a failed deploy call.
  - **Smoke trigger:** a deployed trigger with the component key
    `google_calendar-new-or-updated-event-instant` and a webhook URL that
    contains `pipedream-trigger-smoke`.
- **Background:**
  - The lifecycle smoke calls the real Pipedream API in the development
    environment. It runs only with `PIPEDREAM_LIVE_TESTS=1` and the
    variables `PIPEDREAM_CLIENT_ID`, `PIPEDREAM_CLIENT_SECRET`,
    `PIPEDREAM_PROJECT_ID`, `PIPEDREAM_EXTERNAL_USER_ID`, and
    `PIPEDREAM_CONNECTED_ACCOUNT_ID`. Without them, it prints the missing
    names and skips. This command runs it:

    ```sh
    PIPEDREAM_LIVE_TESTS=1 cargo test -p pipedream-connect \
      --test live_pipedream -- --nocapture
    ```

  - The test deploys one smoke trigger for the shared user, with the webhook
    URL `http://127.0.0.1:50151/pipedream-trigger-smoke`. No other test
    deploys a smoke trigger.
  - After a successful deploy call, the test deletes the trigger by its ID.
    The test fails when that delete call fails.
  - After a failed deploy call, or a deploy call that returns another emitter
    type, the test calls the recovery and then fails. Pipedream can create
    the trigger before its response times out or fails to decode, so the
    test may have no trigger ID.
  - The recovery lists one page of the deployed triggers of the shared user.
    A page holds at most 100 triggers, the limit of `PageRequest::default()`.
    The recovery deletes each smoke trigger on that page.
  - `BoundedPaginator` and `FilteredPageSource` in
    `crates/pipedream-connect/src/pagination.rs` read the pages of a list.
    `ListDeployedTriggersRequest` supports them. The client README shows
    their use under "Adding an operation".
  - The client README, `crates/pipedream-connect/README.md`, says of the
    lifecycle smoke: "It always cleans up a trigger created by the test."
  - Commit `a4c09f1` added the recovery to `live_pipedream.rs` on 2026-10-22.
    Commit `754b2ee` moved it to the support module on 2026-11-03 without a
    change, to keep `live_pipedream.rs` at or below 300 lines.
  - `live_pipedream.rs` also holds the emitter date test
    `live_pipedream_emitter_dates_are_recent`. On 2026-11-03, the user chose
    for it one fixed external user, `pd-sdk-live-emitter-dates`, that the
    test deletes after each run. That choice accepts that two runs at the
    same time can make each other fail.
- **Problem:** When the list call of the recovery fails, the recovery returns
  at once and prints nothing. It also ignores each delete result and never
  reads a second page. So a failed run can leave a smoke trigger without a
  message, and the README claim is then false.
- **Impact if no action:** A failed run can leave an active Google Calendar
  trigger for the shared user, and the developer does not learn of it. A
  later run removes it only when its own deploy call fails.
- **Root question:** How does the lifecycle smoke make sure that no smoke
  trigger stays deployed without notice, also when a run fails or stops?
- **Options:**
  - **A.** Make the recovery return its list error and each delete error, as
    `delete_created_triggers` in the same module does. Read the pages with
    `BoundedPaginator`, not only the first page. Add the errors to the
    failure message of the test. Correct the README sentence to describe
    this behaviour. Cost: the developer must delete a reported trigger by
    hand, for example with the CLI commands `triggers list` and
    `triggers delete`. A stopped run, such as one ended with Ctrl-C, still
    leaves its trigger without a message. Effort: Small.
  - **B.** Do A. Also run the recovery as a sweep before each deploy call.
    Each run then deletes the smoke triggers that earlier failed or stopped
    runs left. When the sweep cannot list or delete, the run fails before
    its deploy call and names each error. Put the new code in the support
    module, because `live_pipedream.rs` is close to the 300-line limit.
    Cost: one more list call in each run. Two runs at the same time can
    delete each other's trigger. One run then fails its list check, but
    nothing leaks. The user accepted the same cost for the emitter date test.
    Effort: Small.
  - **C.** Give the lifecycle smoke its own external user with its own Google
    Calendar connection. Reserve that user for this test. Before each deploy
    call, delete every deployed trigger of that user, and report each error
    as in A. The cleanup then needs no smoke trigger match and never deletes
    from the shared user. Cost: a person must connect a Google Calendar
    account to the new user once, in a browser. The test needs a new
    variable for that account. The new user also takes one of the 10
    external users that the development environment permits. Effort: Medium.
- **Left out:**
  - Change nothing, or change only the README sentence. Both fail the goal,
    because a failed run still leaves a trigger without notice.
  - Delete the shared user after each run. It breaks a requirement that
    stays. `docs/protocol/resources/accounts.md` says that user deletion
    removes all resources of the user. That includes the Google Calendar
    account that the lifecycle smoke and
    `crates/pipedream-connect/tests/live_managed_auth.rs` need. Only a
    manual Google sign-in restores that account.
  - A retry or a wait before the list call of the recovery. It breaks the
    `testing` skill rule that a test adds a retry or a sleep only with the
    user's approval. A long outage also defeats it.
  - Keep the trigger ID after an unreadable deploy response. A deploy call
    through `RequestDispatcher` can read the ID before the typed decode. It
    breaks the `testing` skill rule to call the entry point that real
    callers use. An ID in the SDK decode error breaks the client README rule
    that a success decode error keeps only the line and column. Both still
    miss a timeout and a stopped run.
  - A manual or a scheduled cleanup command. A manual command fails the goal
    unless someone runs it. A schedule needs stored credentials and new
    infrastructure, which this work cannot add.
  - A guard that fails the run when a smoke trigger already exists. B is
    better on every point, because B also deletes what the guard only
    reports.
- **Wider fix:** B. The next run removes each leftover smoke trigger, so the
  cleanup no longer depends on the run that failed. It still works when the
  failure changes shape: a failed list call, a failed delete call, a missed
  page, or a stopped run.
- **Previous examples:**
  - Before commit `a4c09f1` of 2026-10-22, the lifecycle smoke had no
    recovery. A failed list call or list check panicked before the delete
    call. That commit moved the delete call before each failure and added
    the recovery.
  - On 2026-11-03, a review found that the emitter date test deleted only
    the IDs that it read from deploy responses. An unreadable response left
    an emitter that no run removed. That day, the user chose the fixed
    external user to fix it.
- **Recommendation:** B. It reports each cleanup error and also removes what
  earlier failed or stopped runs left, for one more list call in each run.
- **Action:** fix after merge, apply B, because the code was outside the scope
  of the review.
