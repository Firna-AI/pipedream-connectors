# Client README calls the dot ID rule open

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`.
  Round 2 of its review listed it outside the scope.
- **Location:** `crates/pipedream-connect/README.md:183-184`.
- **Reason:** The review that found it covered only follow-up items, so the
  README was outside its diff.

- **Source:** Review.
- **Category:** docs or spec.
- **Terms:**
  - **Client README:** `crates/pipedream-connect/README.md`, the README of
    the async client crate `pipedream-connect`.
  - **Typed identifier:** an SDK ID type, such as `ExternalUserId` or
    `AppSlug`, in `crates/pipedream-connect-types/src/ids.rs`. Its
    constructor and its JSON decoder check the value.
  - **`ApiPath`:** the public request path type in
    `crates/pipedream-connect/src/request.rs`. A host uses it for a custom
    operation and passes each path parameter as a raw string segment.
  - **Dot segment:** a URL path segment that is exactly `.` or `..`.
- **Background:**
  - `crates/pipedream-connect/README.md:183-185` says: "The URL library
    skips literal `.` and `..` segments. Their identifier policy is still an
    open plan decision. Slash, percent, question mark, and hash characters
    are encoded inside each segment."
  - The paragraph follows the README text on custom operations with
    `ApiPath::project`. Commit `8e452ca` wrote it while the plan decision on
    dot identifiers was open.
  - The decision had two options. One rejected `.` and `..` identifiers
    before the token call. The other kept them through a URL form that does
    not normalize them. On 2026-10-26 the user chose to reject them.
  - `docs/protocol/connect-resources.md:80` says: "An identifier MUST NOT be
    exactly `.` or `..`." Its decision says: "Dot segments were rejected
    because URL path normalization could route a request to another
    resource."
  - `common_identifier` at `crates/pipedream-connect-types/src/ids.rs:10-17`
    rejects `.` and `..`. Each typed identifier of a path checks it, directly
    or through `prefixed_alphanumeric`. The test
    `common_grammar_checks_all_forbidden_bytes_and_lengths` in
    `crates/pipedream-connect-types/src/_tests_/ids_tests.rs` checks the
    rejection.
  - `crates/pipedream-connect-types/README.md:37` says: "Path identifiers
    reject the dot segments `.` and `..`."
  - A raw `ApiPath` segment can still be `.` or `..`, and the URL library
    skips it. The test
    `path_parameters_cannot_inject_routes_and_dot_segments_are_skipped` in
    `crates/pipedream-connect/src/_tests_/dispatch_tests.rs` shows this. So
    the first sentence of the paragraph is still true.
- **Problem:** The client README calls the dot identifier rule an open
  decision. The user decided it on 2026-10-26, and every typed identifier
  rejects both values.
- **Impact if no action:** A reader of the client README can think that a
  resource call can send `.` or `..` in a path, or that the rule can still
  change. The README contradicts the protocol doc and the types crate
  README.
- **Root question:** How does the client README state the dot identifier
  rule that is in force?
- **Options:**
  - **A.** Replace the sentence "Their identifier policy is still an open plan
    decision." with the rule in force. For example: "Typed identifiers reject
    both values, so only a raw `ApiPath` segment can hold one." Keep the two
    other sentences. Effort: Small.
  - **B.** Delete only the stale sentence. The protocol doc and the types
    crate README then state the rule alone. A reader of the client README
    does not learn that a resource call never sends a dot segment.
    Effort: Small.
- **Left out:**
  - Change nothing. It fails the goal, because the README contradicts the
    code and the decision.
  - Make `ApiPath` reject `.` and `..` segments too. It answers another
    question, because it changes the behaviour of the raw request API. It
    adds a new rejection, so only the user can approve it.
  - Only two approaches are real: state the rule in the client README, or
    leave the rule to the other two docs.
- **Wider fix:** None. The `documentation` skill already requires each README
  to change with the code and the decisions that it describes.
- **Previous examples:** Commit `0f8b3ad` ticked the plan TODO for this
  decision on 2026-11-02, but it did not change the README. On 2026-10-23,
  commit `be03dc0` corrected another sentence of the same README, which
  overstated the coverage test.
- **Related:** `plans/follow-ups/client-readme-live-managed-auth.md`, same
  code. That item fixes the `### Key Code` list of the same README. One
  branch can fix both items.
- **Recommendation:** A. It states the rule where the reader of the segment
  rules looks, and it keeps the first sentence, which is still true.
- **Action:** fix after merge, apply A, because the README was outside the
  diff of the review that found it, and the fix changes only README text.
