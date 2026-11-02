# Emitter dates may decode in the wrong unit

- **Severity:** High. **Likelihood:** Common. **Effort:** Medium.
  **Confidence:** Medium. **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the decisions-round fixes listed `schedule_changed_at` outside
  its scope on 2026-10-26. The check for this item found the public example
  values on 2026-11-02.
- **Location:** `crates/pipedream-connect-types/src/triggers/emitters.rs:29-32`,
  `crates/pipedream-connect-types/src/triggers/emitters.rs:71-74`,
  `crates/pipedream-connect-types/src/triggers/emitters.rs:96-101`,
  `crates/pipedream-connect-types/src/shared.rs:196-216`,
  `docs/protocol/connect-resources.md:47-51`,
  `crates/pipedream-connect-types/README.md:12-16`,
  `crates/pipedream-connect-types/tests/conformance/timestamp_tests.rs:80-121`.
- **Reason:** Only a live Pipedream response or an answer from Pipedream can
  settle the unit, and the review had neither.

- **Source:** Review.
- **Category:** product bug.
- **Terms:**
  - **Emitter:** one entry of the deployed triggers list or of a trigger
    deploy response. Its SDK type is `Emitter` in
    `crates/pipedream-connect-types/src/triggers/emitters.rs`.
  - **Timer interface:** the emitter of a schedule trigger. Its SDK type is
    `TimerInterface`, and its ID starts with `ti_`.
  - **`Timestamp`:** the shared SDK date type in
    `crates/pipedream-connect-types/src/shared.rs`. Its default decoder reads
    a JSON integer as epoch milliseconds.
  - **OpenAPI copy:** the vendored Pipedream OpenAPI document,
    `crates/pipedream-connect-types/tests/fixtures/pipedream_openapi_swagger.json`.
- **Background:**
  - Three emitter kinds have integer dates: `DeployedComponent`,
    `HttpInterface`, and `TimerInterface`. Each has `created_at` and
    `updated_at`. `TimerInterface` also has `schedule_changed_at`.
  - All seven fields use `Timestamp` with its default decoder. So an integer
    in these fields decodes as epoch milliseconds.
  - `docs/protocol/connect-resources.md:47-51` gives the date rule. Integers
    in `usage_start_ts`, `usage_end_ts`, and the `created_at` and `updated_at`
    of project webhooks and deployed trigger webhooks decode as epoch seconds.
    Integers in any other date field decode as epoch milliseconds.
  - In the OpenAPI copy, the descriptions of six of the seven fields say
    "epoch milliseconds". `TimerInterface.schedule_changed_at` is only
    `{"type": "integer"}`. It is the only integer date in the OpenAPI copy
    without a unit.
  - The conformance test `integer_date_fields_follow_the_openapi_epoch_unit`
    checks each integer date against the unit in its OpenAPI description. It
    leaves out `schedule_changed_at`, because that field has no description.
  - The Pipedream Connect page "Deploying Triggers" shows example responses.
    Every emitter date in them has 10 digits. The page states no unit and no
    capture date.
  - In the "Schedule" example, the timer interface `ti_aqGTJ2` has
    `schedule_changed_at`, `created_at`, and `updated_at` all 1744508391. The
    HTTP interface example has `created_at` 1744508049. The deployed component
    example has `created_at` 1734028283.
  - As epoch seconds, 1744508391 is 2025-04-13T01:39:51Z. As epoch
    milliseconds, it is 1970-01-21T04:35:08Z.
  - The same page deploys the timer with the trigger ID
    `schedule-custom-interval` and the `configured_props`
    `{"cron": {"intervalSeconds": 900}}`. It deploys the HTTP interface with
    the trigger ID `http-new-requests` and no props.
- **Problem:** No source confirms the unit of the seven emitter dates. The
  OpenAPI copy says milliseconds for six of them and nothing for
  `schedule_changed_at`. The public example responses show 10-digit values,
  which fit epoch seconds.
- **Impact if no action:** If Pipedream sends epoch seconds, each emitter date
  decodes to a day in January 1970, without an error. Re-encoding keeps the
  original integer, so only callers that read the dates get wrong values.
- **Root question:** How does the SDK decode each emitter date to the instant
  that Pipedream means?
- **Options:**
  - **A.** Change nothing. Keep epoch milliseconds, as the OpenAPI copy says
    for six fields. If the example responses show the real wire, all seven
    dates stay wrong. Effort: Small.
  - **B.** Confirm the unit with live responses, then apply it. Deploy the
    timer and the HTTP interface of the examples for a test external user.
    List the deployed triggers, compare each date with the current time, and
    delete both triggers. The live test
    `live_pipedream_google_calendar_trigger_lifecycle` in
    `crates/pipedream-connect/tests/live_pipedream.rs` can do the same for a
    deployed component. Give each field that uses epoch seconds the decoder
    `deserialize_timestamp_seconds`. The conformance test then needs an
    exception for each field that contradicts its OpenAPI description. Update
    the protocol date rule, the types README, and the tests. Effort: Medium.
  - **C.** Ask Pipedream to state the unit of `schedule_changed_at` and to
    check the other six units in its OpenAPI document. Then refresh the
    OpenAPI copy and add `schedule_changed_at` to the conformance test. The SDK
    keeps milliseconds until Pipedream answers. Effort: Small.
  - **D.** Decode an integer emitter date by its size. Read a value below
    100,000,000,000 as epoch seconds and a larger value as epoch milliseconds.
    The rule is right for every date from March 1973 to the year 5138. It adds
    decode behaviour that no requirement covers. Effort: Small.
  - **E.** Switch the seven fields to epoch seconds now, from the example
    responses alone. This contradicts the OpenAPI copy for six fields.
    Effort: Small.
- **Left out:** Type the seven fields as plain JSON numbers. It fails the goal,
  because the SDK then gives no instant.
- **Wider fix:** None. The conformance test already checks each integer date
  against its OpenAPI unit, but it cannot find an OpenAPI description that is
  wrong.
- **Previous examples:** On 2026-10-26, a review found that eight usage record
  and project webhook dates used epoch seconds, but the SDK read them as
  milliseconds. Commit `62024a1` added `deserialize_timestamp_seconds` in
  `crates/pipedream-connect-types/src/shared.rs` for them.
- **Recommendation:** B. A live response shows which source is right, and the
  fix then follows the real wire. C adds lasting protection, because the
  conformance test can then check every emitter date against a stated unit.
- **Action:** ask the user, because the live check needs Pipedream
  credentials, and a switch to seconds changes the protocol date rule.

The example values come from
<https://pipedream.com/docs/connect/components/triggers>, read on 2026-11-02.
