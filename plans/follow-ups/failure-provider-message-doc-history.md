# Error message helper comment describes history

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  decisions-round review of the SDK crates listed it outside its scope on
  2026-10-26.
- **Location:** `crates/pipedream-connect/src/failure.rs:111-116`.
- **Reason:** The comment came with the first client dispatch commit and was
  outside the diff under review, so a separate branch rewrites it.

- **Source:** Review.
- **Category:** docs or spec.
- **Terms:**
  - **Error envelope:** the JSON error body of a failed Pipedream request. Its
    SDK type is `ErrorResponse` in
    `crates/pipedream-connect-types/src/wire.rs`.
  - **`ErrorMessage`:** the type of the `error` field of the envelope. It is
    `Text` for a JSON string, or `Json` for any other JSON value.
- **Background:**
  - `crates/pipedream-connect/src/failure.rs` maps a failed HTTP status to an
    SDK error.
  - The private function `provider_message` gives the provider text for the
    `Unauthorized`, `Forbidden`, `NotFound`, and `Rejected` errors.
  - It serializes `envelope.error` to JSON. It returns the result only when
    the result is a JSON string.
  - Its doc comment says: "Isolates the error text representation while model
    review changes it."
  - Commit `8e452ca` added the function and the comment on 2026-10-21, in the
    first build of the client.
  - The shared `documentation` skill gives each kind of content one home. Doc
    comments explain how the code works. The plan and the pull request
    description hold history.
- **Problem:** The comment describes a work state of 2026-10-21, not what the
  function does.
- **Impact if no action:** A reader cannot learn the behaviour from the
  comment. The reader can also think that the function is temporary.
- **Root question:** How does a reader of `failure.rs` learn what
  `provider_message` does?
- **Options:**
  - **A.** Replace the comment with the behaviour. Example: "Returns the
    provider error text when the `error` field of the envelope is a JSON
    string." Effort: Small.
  - **B.** Option A, and match on `ErrorMessage` instead of the JSON round
    trip. Return the text for `Text`, and for `Json` that holds a string. The
    results stay the same. Effort: Small.
- **Left out:** Delete the comment. Option A is better on every point, because
  the JSON round trip is not clear without a comment.
- **Wider fix:** None. On 2026-11-02, a search of the code comments under
  `crates/` and `xtask/` found no other comment that records history. The
  `documentation` skill already gives history its home.
- **Previous examples:** None found.
- **Recommendation:** A. It states the behaviour and changes no code.
- **Action:** fix after merge, apply A, because the comment predates the
  reviewed change and sits outside its diff.
