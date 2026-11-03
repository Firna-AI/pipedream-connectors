# xtask command types are wider than needed

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the decisions-round fixes listed it outside its scope on
  2026-10-26.
- **Location:** `xtask/src/command.rs:9-20`, `xtask/src/command.rs:56-72`,
  `xtask/src/main.rs:1-3`.
- **Reason:** The items date from the workspace bootstrap and were outside the
  diff under review, so a separate xtask branch narrows them.

- **Source:** Review.
- **Category:** repository rule.
- **Terms:**
  - **xtask:** the developer task binary under `xtask/`. `cargo xtask check`
    runs the workspace gate.
  - **Binary crate:** a crate that builds a program and has no library
    target. No other crate can use its items.
  - **`unreachable_pub`:** a rustc lint that warns about a `pub` item that no
    other crate can reach.
  - **unimock:** the mock library of the xtask tests. Its attribute on a trait
    generates a mock API, such as `CommandRunnerMock`.
- **Background:**
  - `xtask/src/command.rs` runs external commands, such as `cargo clippy`,
    behind the trait `CommandRunner`. The tests replace the trait with a
    unimock mock.
  - The struct `CommandSpec` and its four fields are `pub`. The struct
    `CommandOutcome` and its three fields are `pub`. The trait
    `CommandRunner` is `pub`.
  - The other items of the module are `pub(crate)`.
  - The shared `rust` skill asks for the narrowest visibility that works:
    private first, then `pub(super)` or `pub(crate)`.
  - The CLI binary and the three library crates enable
    `#![warn(unreachable_pub)]`. `xtask/src/main.rs` does not.
  - On 2026-10-26, commit `eed3df9` narrowed `DirectoryEntry` in
    `xtask/src/file_access.rs` to `pub(crate)`. It also changed the
    `FileAccess` mock to the flattened unimock form,
    `api = [FileAccessReadDirMock, FileAccessReadToStringMock]`, so that the
    mock compiles with that visibility.
- **Problem:** Three items and their seven fields are `pub` in a binary crate,
  where no other crate can use them. The module mixes two visibility levels,
  and no lint reports it.
- **Impact if no action:** The items look like an external API. New xtask code
  can copy the `pub` pattern, as `DirectoryEntry` did.
- **Root question:** How do the xtask command types get the narrowest
  visibility that works?
- **Options:**
  - **A.** Change `CommandSpec`, `CommandOutcome`, `CommandRunner`, and their
    fields to `pub(crate)`. If `CommandRunnerMock` then fails to compile, use
    the flattened unimock form, as commit `eed3df9` did for `FileAccess`.
    Effort: Small.
  - **B.** Add `#![warn(unreachable_pub)]` to `xtask/src/main.rs`, as the CLI
    binary has. Then narrow every item that the lint flags. This adds a lint.
    Effort: Small.
- **Left out:** Change nothing. It fails the goal, because the `rust` skill
  asks for the narrowest visibility. Only two approaches are real: narrow the
  items by hand, or let a lint find them.
- **Wider fix:** B. It catches each new `pub` item in xtask. It is blocked,
  because it adds a lint, and only the user can add a lint.
- **Previous examples:** `DirectoryEntry` in `xtask/src/file_access.rs` was
  `pub`. Commit `eed3df9` narrowed it to `pub(crate)` on 2026-10-26.
- **Related:** `plans/follow-ups/xtask-unreachable-pub-lint.md` holds option B
  as its own item, because the lint needs a user decision. The two items share
  that wider fix. If the user approves the lint, its branch narrows these
  items too and deletes this item.
- **Recommendation:** A. It narrows the items without a new lint. B gives the
  best protection, but the user must approve a new lint.
- **Action:** fix after merge, apply A, because the items predate the reviewed
  change and sit outside its diff.
