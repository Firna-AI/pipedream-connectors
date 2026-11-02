# Emitter dates may decode in the wrong unit

- **Severity:** High. **Likelihood:** Common. **Effort:** Medium.
  **Confidence:** High. **Harder later:** Yes.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the decisions-round fixes listed `schedule_changed_at` outside
  its scope on 2026-10-26. The check for this item found the public example
  values on 2026-11-02. A second check on 2026-11-02 found more Pipedream
  examples and raised the confidence.
- **Location:** `crates/pipedream-connect-types/src/triggers/emitters.rs:29-32`,
  `crates/pipedream-connect-types/src/triggers/emitters.rs:71-74`,
  `crates/pipedream-connect-types/src/triggers/emitters.rs:96-101`,
  `crates/pipedream-connect-types/src/shared.rs:196-216`,
  `docs/protocol/connect-resources.md:47-51`,
  `crates/pipedream-connect-types/README.md:12-16`,
  `crates/pipedream-connect-types/tests/conformance/timestamp_tests.rs:80-121`.
- **Reason:** Each fix changes the date rule of the protocol doc, so the item
  waits for a user decision.

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
  - Older Pipedream examples agree. The Connect API reference of December 2024
    shows 1734028283, 1733512889, and 1733198039. The classic REST API docs of
    February 2024 show 1707418403 and 1707418408. The REST API docs of 2020
    show 1587679599. All have 10 digits.
  - Read as epoch seconds, each example value falls hours to days before the
    commit that added it to the Pipedream docs repository.
  - The same pages use 13 digits for millisecond fields, such as the trigger
    event `ts` 1737155977519 and the field `indexed_at_ms`.
  - The text "epoch milliseconds" first appears in July 2025, in the OpenAPI
    document and the SDKs that Fern generates from it. It has no example
    value. The live OpenAPI document of 2026-11-02 has the same text.
  - No published source shows that Pipedream changed the unit after April
    2025. Only a live response can prove the unit.
  - Firna reads these dates with `DateTime::from_timestamp_millis`, in
    `crates/fna-connectors-pipedream/src/mapping/triggers.rs` of
    `futex-ai/firna`.
- **Problem:** Every published Pipedream example since 2020 shows these dates
  in epoch seconds. The SDK follows the OpenAPI text and reads them as epoch
  milliseconds, so it most likely decodes all seven dates wrong.
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
  - **F.** Do E, and plan the confirmation:
    - Give the published examples as the reason in the protocol decision and
      in the conformance test exceptions. Add `schedule_changed_at` to the
      conformance test.
    - Add a live test that deploys `http-new-requests` and
      `schedule-custom-interval`, checks that each date is within a day of the
      current time, and deletes both. Add the same check to the Google
      Calendar lifecycle test. The live tests need Pipedream credentials.
      They also confirm that each emitter has a `type` field, because the SDK
      decodes an emitter without one as `Other`, with no dates.
    - Ask Pipedream to correct the six descriptions and to state the unit of
      `schedule_changed_at`, as C does.
    - Risk: if Pipedream sends milliseconds, the dates show a year after
      55,000 until the first live run fails. Effort: Medium.
  - **G.** Let the shared date decoder pick the unit by the size of the value,
    for every integer date field. A value below 100,000,000,000 is epoch
    seconds, and a larger value is epoch milliseconds. Remove the decoders
    that set seconds for single fields. State one rule in the protocol doc and
    the crate READMEs. Test values such as `1_000_000` in the deployed trigger
    lifecycle tests must change. It gives the right date from March 1973 to
    the year 5138 in either unit, and it stays right if Pipedream changes a
    unit. It adds decode behaviour that no requirement covers.
    Effort: Medium.
- **Left out:**
  - Type the seven fields as plain JSON numbers. It fails the goal, because
    the SDK then gives no instant.
  - A test that checks the published example values. It gives no date by
    itself, and G prevents the class with a structural fix.
  - A note that tells callers to re-read the value as seconds. Each caller
    that misses the note still gets a date in 1970.
  - A fix in the client crate or a local edit of the OpenAPI copy. The types
    crate owns decoding, and a refresh overwrites the OpenAPI copy.
- **Wider fix:** G. A size rule in the shared decoder prevents this class of
  defect for every integer date, also for fields that a later OpenAPI refresh
  adds.
- **Previous examples:** On 2026-10-26, a review found that eight usage record
  and project webhook dates used epoch seconds, but the SDK read them as
  milliseconds. Commit `62024a1` added `deserialize_timestamp_seconds` in
  `crates/pipedream-connect-types/src/shared.rs` for them. The re-review of
  that fix offered a size rule as an option, and the fix kept the OpenAPI
  units instead.
- **Related:**
  - `plans/follow-ups/date-decode-rejections.md`, same code. Both items change
    the shared date decoder and the date rule of the protocol doc. A new size
    rule must use the out-of-range rule that the other item picks.
  - `plans/follow-ups/openapi-fixture-refresh.md`, same code and a shared
    wider fix. The refresh adds `WebhookDeliveryState.since` in epoch
    seconds. Without G, that field needs its own seconds decoder.
  - `plans/follow-ups/firna-adoption-plan.md`, same cause. Firna must take
    these dates from `Timestamp::as_datetime`, or it keeps wrong dates after
    this fix.
  - `plans/follow-ups/client-readme-live-managed-auth.md`, same code. A new
    live test from F needs a line in the client README Key Code list.
  - `plans/follow-ups/deployed-trigger-email-prefix.md`, same code. That item
    is about the IDs of email triggers, which start with `ei_`. Its option E
    adds a typed email interface with `created_at` and `updated_at` to the
    emitter model in `emitters.rs`. So E needs the date unit that this item
    picks, and its live run can share the live run of B or F here. That item
    also records that no emitter example on the Connect page has a `type`
    field. The live tests of F check that field.
- **Recommendation:** F. It applies the unit that every published example
  shows, without waiting for credentials, and the first live run confirms it.
  The first check recommended B, but B keeps the likely wrong unit until
  credentials exist. G best protects the codebase.
- **Action:** ask the user, because F changes the date rule of the protocol
  doc, and no recorded decision covers the change.

The example values come from
<https://pipedream.com/docs/connect/components/triggers>, read on 2026-11-02.
