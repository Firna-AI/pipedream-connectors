# Client README key code omits a live test

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the final review fixes listed it outside its scope on
  2026-10-22. Commit `e0531a1` added a run command on 2026-10-26, so this item
  now covers only the `### Key Code` list.
- **Location:** `crates/pipedream-connect/README.md:308-309`.
- **Reason:** The README list was outside the diff under review, and the fix
  is a short docs change for a separate branch.

- **Source:** Review.
- **Category:** docs or spec.
- **Terms:**
  - **Key Code:** the `### Key Code` list of a crate README. The shared
    `documentation` skill makes it the home of the entry points that a reader
    opens first.
  - **Live test:** an integration test that calls the real Pipedream API. It
    runs only with `PIPEDREAM_LIVE_TESTS=1` and Pipedream credentials, and it
    skips otherwise.
- **Background:**
  - `crates/pipedream-connect/README.md` documents the Connect client crate.
  - The crate has two live test files. `tests/live_pipedream.rs` holds the
    registry smoke test and the trigger lifecycle smoke test.
    `tests/live_managed_auth.rs` holds the managed-auth and proxy smoke test.
  - On 2026-10-26, commit `e0531a1` added the run command and a description
    of `--test live_managed_auth` to the development section.
  - The Key Code list names `tests/http_transport.rs` and
    `tests/live_pipedream.rs`. Line 309 calls `tests/live_pipedream.rs` "the
    credential-gated registry smoke test".
  - The README never names the file `tests/live_managed_auth.rs`.
  - The `documentation` skill says to update the Key Code list when an entry
    point moves or changes.
- **Problem:** The Key Code list leaves out `tests/live_managed_auth.rs`. It
  also calls `tests/live_pipedream.rs` only the registry smoke test, but the
  file also holds the trigger lifecycle test.
- **Impact if no action:** A reader who uses the Key Code list to find the live
  tests misses the managed-auth test and the trigger test.
- **Root question:** How does the client README point a reader to each live
  test file?
- **Options:**
  - **A.** Add a Key Code line for `tests/live_managed_auth.rs`. Example:
    "`tests/live_managed_auth.rs` provides the credential-gated managed-auth
    and proxy smoke test." Change line 309 to name the registry and trigger
    lifecycle smoke tests. Effort: Small.
  - **B.** Change nothing. The development section already names both live
    test targets and gives their run commands. Effort: Small.
  - **C.** Remove the two test lines from the Key Code list, so that only the
    development section names the tests. This deletes live doc content, so
    it needs the user. Effort: Small.
- **Left out:** None.
- **Wider fix:** None. The `documentation` skill already says to update the
  Key Code list. A test cannot check README prose, because the `testing`
  skill says that doc prose never gets a test.
- **Previous examples:** On 2026-10-23, commit `be03dc0` corrected the
  sentence of the same README that overstated the coverage test.
- **Recommendation:** A. It makes the Key Code list match both live test files
  with a two-line change.
- **Action:** fix after merge, apply A, because the README list was outside
  the diff under review.
