# No `unreachable_pub` lint in the xtask binary

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** Yes.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of 2026-10-26 raised it as finding 4. Commit `eed3df9` applied the
  direct fix, and this item holds the blocked wider fix.
- **Location:** `xtask/src/main.rs:1-3`, `xtask/src/command.rs:11`,
  `xtask/src/command.rs:58`, `xtask/src/command.rs:68-69`.
- **Reason:** Option B adds a lint to the gate, and only the user approves a
  new lint.

- **Source:** Review.
- **Category:** repository rule.
- **Terms:**
  - **xtask:** the binary crate in `xtask/` for developer tasks. Its `check`
    command is the gate, `cargo xtask check`, which CI also runs.
  - **Wide item:** an item that is `pub`, although no other crate can reach
    it. In a binary crate, every `pub` item is wide.
  - **`unreachable_pub`:** a rustc lint that flags a wide item. Its help text
    suggests `pub(crate)`.
  - **unimock:** the mock library of the workspace. A
    `#[unimock::unimock(api = ...)]` attribute on a trait generates a mock API
    for tests.
- **Background:**
  - The `rust` skill of the shared `futex-ai/skills` rules asks for the
    narrowest visibility that works. It also asks for
    `#![warn(unreachable_pub)]` in internal crates.
  - `crates/pipedream-cli/src/main.rs` has the lint. The roots of
    `pipedream-connect`, `pipedream-connect-types`, and
    `pipedream-connect-webhooks` have it too. `xtask/src/main.rs` does not.
  - The gate runs clippy with `-D warnings`, so the lint fails the gate on
    each `pub` item that it flags.
  - Commit `95fc0e8` of 2026-10-26 added `DirectoryEntry` to
    `xtask/src/file_access.rs` as `pub`, but its trait and fields were
    `pub(crate)`. A re-review on 2026-10-26 found it, and commit `eed3df9`
    made it `pub(crate)`.
  - On 2026-11-02, `cargo clippy --locked -p xtask --all-targets -- -W
    unreachable_pub` flagged three items in `xtask/src/command.rs`:
    `CommandSpec` at line 11, `CommandOutcome` at line 58, and `CommandRunner`
    at line 69. They have been `pub` since the workspace bootstrap.
  - A scratch copy at commit `eaf1f8d` checked the steps of B on 2026-11-02:
    1. Add `#![warn(unreachable_pub)]` to `xtask/src/main.rs`, after the
       module doc comment.
    2. Make the three flagged items `pub(crate)`.
    3. The test build then fails with error E0446, "crate-private type
       `command::CommandSpec` in public interface", at `command.rs:71`. The
       attribute `api = CommandRunnerMock` generates a `pub` mock API.
    4. Change that attribute to the flattened form
       `api = [CommandRunnerExecuteMock]`, as commit `eed3df9` did for
       `FileAccess`.
    5. Replace `CommandRunnerMock::execute` and the `CommandRunnerMock`
       imports with `CommandRunnerExecuteMock`. They are in `check_tests.rs`,
       `command_tests.rs`, and `file_length_tests.rs` under
       `xtask/src/_tests_/`.
  - After these steps, `cargo fmt --all -- --check`,
    `cargo clippy --locked -p xtask --all-targets -- -D warnings`, and the 17
    xtask tests passed. The lint flagged nothing more, also not the `pub`
    fields of the two structs.
- **Problem:** No lint stops a new wide item in the xtask binary. Three wide
  items are in `xtask/src/command.rs` today.
- **Impact if no action:** Wide items look like external API, and files mix
  two visibility levels. Reviews must find each new wide item by hand. New
  items copy the pattern, as `DirectoryEntry` did, and each copy adds work to
  B.
- **Root question:** How does the xtask crate keep each item at the narrowest
  visibility that works?
- **Options:**
  - **A.** Narrow each wide item by hand when a review finds it, as commit
    `eed3df9` did for `DirectoryEntry`. No new lint. New wide items can land
    again. Effort: Small.
  - **B.** Add `#![warn(unreachable_pub)]` to `xtask/src/main.rs`, as the CLI
    binary has. Then narrow every item that it flags. This adds a lint.
    Effort: Small.
- **Left out:** Not recorded.
- **Wider fix:** B. It catches each new wide item in the gate. It is blocked,
  because it adds a lint.
- **Previous examples:** `DirectoryEntry` was one case, and commit `eed3df9`
  narrowed it. The three `pub` items of `xtask/src/command.rs` are older
  cases. The re-review listed them outside its scope.
- **Related:** `plans/follow-ups/xtask-command-types-visibility.md`, same code.
  That item narrows the three `pub` items of `xtask/src/command.rs` without a
  lint. Today the lint flags only these three items, so B does the same
  narrowing. When B lands first, its branch also deletes that item. When that
  item lands first, B adds only the lint line.
- **Recommendation:** B. It catches each new wide item in the gate, and the
  scratch run shows that it changes only five files.
- **Action:** ask the user, because B adds a new lint to the gate, and only
  the user approves a new lint.
