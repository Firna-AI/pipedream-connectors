# Audit read errors name the workspace root

- **Severity:** Low. **Likelihood:** Rare. **Effort:** Small.
  **Confidence:** High. **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the decisions-round fixes listed it outside its scope on
  2026-10-26.
- **Location:** `xtask/src/layout.rs:12-20`, `xtask/src/file_length.rs:19-28`,
  `xtask/src/file_access.rs:28-59`.
- **Reason:** The audit code was outside the diff under review, so a separate
  xtask branch fixes it.

- **Source:** Review.
- **Category:** product bug.
- **Terms:**
  - **xtask:** the developer task binary under `xtask/`. `cargo xtask check`
    runs the workspace gate.
  - **Audit:** one of the two source checks at the end of the gate: the
    300-line file-length audit and the module-layout audit.
  - **Shared walker:** `walk_workspace_roots` in `xtask/src/file_access.rs`.
    It lists every directory under the `crates` and `xtask` roots for both
    audits.
- **Background:**
  - Both audits walk the `crates` and `xtask` directories through the shared
    walker.
  - The walker skips a root that does not exist. It returns every other read
    error as a bare `std::io::Error`, which holds no path.
  - `layout::run` maps that error to `Error::ReadLayout` with
    `path: root.to_path_buf()`. `file_length::run` maps it to
    `Error::ReadDirectory` with the same path.
  - `root` is the workspace root, the directory of the top-level
    `Cargo.toml`.
  - `xtask/src/error.rs` prints both variants as "failed to read directory",
    then the path and the I/O error.
  - Commit `eed3df9` added the shared walker on 2026-10-26. Its message says
    that nested read errors keep the workspace-root path.
  - The tests `reports_a_nested_not_found_with_layout_prefix` and
    `reports_a_nested_not_found_with_file_length_prefix` check only the prefix
    of the message. No test checks the path.
- **Problem:** A read error in a nested directory names the workspace root,
  not the directory that failed. Example: a permission error in
  `crates/demo/src` prints the workspace root path and "Permission denied
  (os error 13)".
- **Impact if no action:** A developer must search the tree by hand to find
  the directory that the gate could not read. The gate still fails, so no
  audit passes in error.
- **Root question:** How does an audit read error tell the developer which
  directory failed?
- **Options:**
  - **A.** Make the shared walker return the failed path with the I/O error,
    for example in a small struct. Put that path in `ReadLayout` in the layout
    audit, and in `ReadDirectory` in the file-length audit. Add one test for
    each audit that a nested read error names the nested directory.
    Effort: Small.
  - **B.** Move the error mapping into the shared walker. The walker then
    returns one xtask error variant that holds the audit name, the failed
    path, and the source. This changes the error contract of both audits.
    Effort: Small.
- **Left out:**
  - Add the failed path to the text of each I/O error in
    `OsFileAccess::read_dir`. Option A is better on every point, because the
    typed `path` field would still name the root.
  - Change nothing. It fails the goal.
- **Wider fix:** A. The shared walker reads every directory for both audits,
  so any later audit that uses it also names the failed directory.
- **Previous examples:** On 2026-10-26, commit `95fc0e8` gave layout read
  errors their own `[xtask/layout]` prefix. Before, they used the
  `[xtask/file_length]` prefix.
- **Related:** `plans/follow-ups/xtask-exit-code-format.md` changes how the
  same read errors in `xtask/src/error.rs` print their paths. One xtask branch
  can do both items.
- **Recommendation:** A. It names the failed directory with the smallest
  change to the error contract.
- **Action:** fix after merge, apply A, because the error is rare, its
  severity is Low, and the code was outside the scope of the review.
