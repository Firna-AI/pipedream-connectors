# xtask prints exit codes as `Some(101)`

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  decisions-round review of the CLI and workspace listed it outside its scope
  on 2026-10-26.
- **Location:** `xtask/src/error.rs:93-109`.
- **Reason:** The message dates from the workspace bootstrap and was outside
  the diff under review, so a separate xtask branch fixes it.

- **Source:** Review.
- **Category:** UX wording.
- **Terms:**
  - **xtask:** the developer task binary under `xtask/`. `cargo xtask check`
    runs the workspace gate: format, clippy, tests, docs, and two source
    audits.
- **Background:**
  - When a gate command fails, `xtask/src/command.rs` returns
    `Error::CommandFailed`. The error holds the command text and
    `code: Option<i32>`.
  - `code` is `None` when the process has no exit code. On Unix, this happens
    when a signal stops the process.
  - `xtask/src/error.rs:99` formats the message with this pattern:
    ``[xtask/command] `{command}` failed with exit code {code:?}``. The `:?`
    prints the Rust debug form of the `Option`.
  - On 2026-10-26, a review probe added a broken doc link and ran the gate.
    The gate printed this line:
    ``[xtask/command] `cargo doc --locked --workspace --no-deps --all-features`
    failed with exit code Some(101)``
  - Four other variants print a path with `:?`: `MissingWorkspaceParent`,
    `ReadDirectory`, `ReadLayout`, and `ReadFile`. The path then shows in
    quotes inside the backticks of the message.
  - The xtask unit tests match `CommandFailed` by its fields. No test checks
    the message text.
- **Problem:** The message shows the Rust form `Some(101)` instead of `101`. A
  process that a signal stops gives "failed with exit code None".
- **Impact if no action:** Developers read Rust internals in the gate output.
  "exit code None" does not say that a signal stopped the command.
- **Root question:** How does a failed gate command report its exit status in
  plain words?
- **Options:**
  - **A.** Print the code with Display, as "failed with exit code 101". Print
    "failed without an exit code" when `code` is `None`. Add a test of both
    texts. Effort: Small.
  - **B.** Option A, and print the four path fields with `Path::display`
    instead of `:?`. Add a test of one path message. Effort: Small.
  - **C.** Store the process `ExitStatus` instead of the code, and print its
    Display form, such as "exit status: 101". The mocked tests then need a
    platform-specific way to build an `ExitStatus`. Effort: Medium.
- **Left out:** Change nothing. It fails the goal, because the gate output
  keeps the Rust form.
- **Wider fix:** B. It removes the debug form from every field of the xtask
  error messages, not only from the exit code.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/xtask-read-error-path.md` changes which path
  the read errors name. Option B here changes how the same paths print. One
  xtask branch can do both items.
- **Recommendation:** B. It fixes every debug-form field of the xtask error
  messages in one small change.
- **Action:** fix after merge, apply B, because the message predates the
  reviewed change, only developers see it, and the new text keeps its meaning.
