# Shared date type adds new decode rejections

- **Severity:** Low. **Likelihood:** Rare. **Effort:** Small.
  **Confidence:** High. **Harder later:** No.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  decisions-round review of 2026-10-26 raised it as finding S2.
- **Location:** `crates/pipedream-connect-types/src/shared.rs:196-216`,
  `crates/pipedream-connect-types/src/shared.rs:112-148`,
  `crates/pipedream-connect-types/src/execution/observations.rs:22`,
  `crates/pipedream-connect-types/src/triggers/events.rs:17`.
- **Reason:** Option A accepts date forms that no requirement covers, so it
  waits for a user decision, which the user deferred on 2026-11-02 as not
  urgent.

- **Source:** Review.
- **Category:** product bug.
- **Terms:**
  - **Timestamp:** the shared date type in
    `crates/pipedream-connect-types/src/shared.rs`. It keeps a UTC instant and
    the wire form that it decoded.
  - **Observation:** an execution log record. Action run, configure, and
    reload responses hold a list of them.
  - **Emitted event:** an event that a deployed trigger emitted. The trigger
    event list response holds a list of them.
- **Background:**
  - The `pipedream-connect-types` crate holds the Rust models of this
    Pipedream Connect SDK. The client decodes each API response into them.
  - On 2026-10-26 the user chose one shared date type for every date field.
    Commit `c9b6a87` moved every date field to `Timestamp`.
  - Before that commit, `Observation.ts` was a plain JSON number, and
    `EmittedEvent.ts` was a plain `i64`. Only their accessors failed on an
    out-of-range value.
  - Before that commit, the date fields of `Account`, `ExternalUser`,
    `ConnectTokenResponse`, `OauthApp`, and `AppOverride` were
    `DateTime<Utc>`. They used the chrono serde parser.
  - `Timestamp` now checks each value during decoding. Its string path accepts
    only RFC 3339, through `DateTime::parse_from_rfc3339`.
  - The date rule at `docs/protocol/connect-resources.md:47-55` says that date
    fields MUST accept RFC 3339 strings and epoch integers. It does not say
    what a value in neither form does.
  - A probe on 2026-11-02 with chrono 0.4.45 confirmed each rejection that the
    Problem names. The old `DateTime<Utc>` serde parser and the
    `DateTime<FixedOffset>` parser accept all three string forms.
  - For an emitted event, only an out-of-range integer is new. The old `i64`
    field already rejected `1e30`.
- **Problem:** One out-of-range number, such as `ts: 1e30` in an observation,
  now fails a whole action run, configure, reload, or event list response. A
  date string with the offset `+0000`, or with a space before the offset
  `+00:00` or `UTC`, now fails the whole model. No decision asked for these
  rejections.
- **Impact if no action:** These inputs are rare, and the Pipedream examples do
  not use them. When one occurs, the host loses the whole result instead of
  one date.
- **Root question:** What does a date field do with a value that is not a
  valid RFC 3339 string or an in-range epoch number?
- **Options:**
  - **A.** Parse date strings with the chrono `DateTime<FixedOffset>` parser,
    as the old fields did. Out-of-range numbers still fail. Effort: Small.
  - **B.** Do A. Also keep an out-of-range number as a raw value without an
    instant, so that only the accessor fails. `as_datetime` then returns an
    `Option`, which changes the public API. Effort: Medium.
  - **C.** Keep the strict rules. Record in the date rule of
    `docs/protocol/connect-resources.md` that a date value in neither form
    fails its record. Effort: Small.
- **Left out:** Not recorded.
- **Wider fix:** B. It restores graceful decoding for every date field through
  the one shared type.
- **Previous examples:** A webhook trigger delivery has a `context` object
  with a `ts` date. A numeric `ts` once rejected the whole delivery. Commit
  `c9b6a87` fixed that, and an invalid `ts` now stays in the unknown context
  fields. That graceful rule covers only the trigger context.
- **Related:** A closed decision on the same code. On 2026-10-26 the user
  chose one shared date type for every date field, over a fix for the trigger
  context date only. Commit `c9b6a87` applied it. Each option here changes that
  shared type. The decision chose the shared type, not its strict string rule.
- **Recommendation:** A. It removes the string rejections that nobody asked
  for with one change. B best protects the codebase, but it changes the
  public accessor.
- **Action:** ask the user, because A accepts date forms that no requirement
  covers. The protocol doc states only RFC 3339 strings and epoch numbers.
