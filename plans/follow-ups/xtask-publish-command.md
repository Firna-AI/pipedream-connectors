# Release resume needs a hand-built Cargo command

- **Severity:** Low. **Effort:** Medium. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Fix after merge.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`.
  Round 3 of its review raised finding 85, and the check of that finding added
  this wider fix as option C. Commit `78088d6` applied the direct fix, option
  A.
- **Location:** `plans/follow-ups/publish-crates-io.md:112-131`,
  `xtask/src/cli.rs:14-25`, `xtask/src/main.rs:5-12`,
  `xtask/src/main.rs:33-38`, `xtask/README.md:20-51`, `AGENTS.md:30-41`.
- **Reason:** C adds a Rust command with tests to `xtask/`, and the branch
  that found the problem changes only docs and plans.

- **Source:** Review.
- **Category:** process.
- **Terms:**
  - **crates.io:** the public Rust package registry, where a version can be
    yanked but never deleted or uploaded again.
  - **crates.io index:** the list of each crate version on crates.io, which
    Cargo reads before an upload.
  - **Library crates:** `pipedream-connect-types`, `pipedream-connect`, and
    `pipedream-connect-webhooks`, the three crates without `publish = false`.
  - **Resume:** a second publish run that uploads the library crates that a
    stopped release did not upload.
  - **xtask:** the binary crate in `xtask/` for developer tasks, which runs as
    `cargo xtask <command>` through the alias in `.cargo/config.toml`.
- **Background:**
  - `plans/follow-ups/publish-crates-io.md` holds the steps of the first
    crates.io release. That release waits until `internal-error` and
    `internal-error-derive` are on crates.io.
  - A person runs each release by hand, after the approval of the user. The
    person needs a crates.io API token of the account that owns the crates.
  - On 2026-10-20 the user chose three separate library crates. So each
    release has three uploads. The three crates take the workspace version,
    `version = "0.1.0"` at `Cargo.toml:13`.
  - The release has a dry-run step and an upload step. Step 7, the dry-run
    step, runs `cargo publish --dry-run --workspace` before the approval.
    Step 9, the upload step, runs `cargo publish --workspace`.
  - Commit `a0b0cc4` chose this workspace form, so that Cargo orders the
    uploads. Before that commit, the release published each crate alone.
  - With `--workspace`, the pinned Cargo 1.95.0 uploads
    `pipedream-connect-types` first. It waits up to 60 seconds until the index
    lists that crate. It then uploads `pipedream-connect` and
    `pipedream-connect-webhooks` in name order.
  - A rejection by crates.io or an index timeout can stop the run after one
    upload or after two.
  - Before it packages a crate, a real `cargo publish` looks up each selected
    version in the index. It stops at the first version that exists, with
    `crate <name>@<version> already exists on crates.io index`.
  - A dry run only warns with the same text, and it exits with status 0. The
    check is `verify_unpublished` in `src/cargo/ops/registry/publish.rs` at
    tag `rust-1.95.0` of the `rust-lang/cargo` repository. Lines 148-159 call
    it for each selected crate.
  - The first resume command of the upload step excluded only
    `pipedream-connect-types`. Suppose `pipedream-connect-types` and
    `pipedream-connect` upload, and then `pipedream-connect-webhooks` fails.
    That command then stops on the existing `pipedream-connect` version, and
    it uploads nothing.
  - On 2026-11-02 a scratch run with Cargo 1.95.0 showed this. Its index
    listed `pipedream-connect-types` and `pipedream-connect`. The old command
    stopped at once with "crate pipedream-connect@0.1.0 already exists".
  - In the same run, two corrected commands passed the check and sent the
    upload. One excluded both uploaded crates. The other used
    `-p pipedream-connect-webhooks`.
  - Commit `78088d6` corrected the upload step. To resume, run the dry run,
    and add one `--exclude` for each crate that it warns about. After an index
    timeout, run the dry run again until it warns that
    `pipedream-connect-types` exists.
  - That resume rule lives only in `plans/follow-ups/publish-crates-io.md`.
    The branch that does the first release deletes that item, because a branch
    deletes each follow-up item that it completes. Later releases then have no
    resume rule.
  - The sparse index serves one file for each crate, with one JSON line for
    each version. The field `vers` holds the version. The URL path is the
    first two letters of the name, the next two letters, and the name.
  - An example is `https://index.crates.io/pi/pe/pipedream-connect`. On
    2026-11-02 it returned HTTP 404, because the crate is not on crates.io.
  - `cargo metadata --no-deps --format-version 1` prints each workspace
    package as JSON, with its `name`, `version`, and `publish` fields. On
    2026-11-02 `publish` was `null` for the library crates and `[]` for the
    other two.
  - The xtask commands are the variants of the `Command` enum in
    `xtask/src/cli.rs:14-25`. `xtask/src/main.rs:33-38` sends each variant to
    the `run` function of its module.
  - xtask starts each external program through the `CommandRunner` trait in
    `xtask/src/command.rs:67-72`. The tests replace it with its unimock mock.
    `CommandSpec::captured` captures only standard output.
  - `command::run` turns each non-zero exit into `Error::CommandFailed`, such
    as status 22 of `curl --fail` for HTTP 404. The error text holds
    `CommandSpec::label`, which joins the program and every argument.
  - `xtask/Cargo.toml` depends only on `clap` and `thiserror`. `serde_json`
    is a workspace dependency at `Cargo.toml:33`.
  - `xtask/README.md` lists the commands under "Quick Start" and the modules
    under "Key Code". `AGENTS.md:30-41` lists the commands. `README.md:124`
    and `docs/architecture.md:39-41` describe `xtask` only as the gate.
- **Problem:** Each resume needs a hand-built command with one `--exclude`
  for each uploaded crate. The first such command missed a crate and failed.
  The corrected rule lives only in an item that the first release deletes.
- **Impact if no action:** A later release that stops part way stays
  incomplete until the operator works out the command again. A wrong
  `--exclude` list stops the resume again.
- **Root question:** How does each release, the first and each later one,
  upload only the crate versions that crates.io does not have yet?
- **Options:**
  - **A.** Keep the resume rule of the upload step as text. Commit `78088d6`
    applied it, so A changes nothing more. The operator still builds each
    resume command by hand, and the rule goes away with the publish item.
    Effort: Small.
  - **B.** Keep the workspace dry run, then publish each crate alone with
    `-p`. Check the index before a resume. It adds manual ordering and
    waiting. It brings back the hand-written upload order that commit
    `a0b0cc4` removed. The rule also lives only in the publish item. Effort:
    Small.
  - **C.** Add one command, `cargo xtask publish`, for the first upload and
    each resume:
    - It reads the index file of each library crate. It finds each workspace
      version that the index lists.
    - It runs `cargo publish --workspace` with one `--exclude` for each listed
      version. It prints each crate that it skips.
    - A `--dry-run` flag adds `--dry-run` to the Cargo command. So the check
      before the approval uses the same selection.
    - It takes no token argument. Cargo reads the token from `cargo login` or
      `CARGO_REGISTRY_TOKEN`. A `--token` argument would show in the error
      text, because the label holds every argument.
    - When the index lists every version, the Cargo command selects no crate.
      Cargo 1.95.0 then warns "nothing to publish" and exits with status 0.
    - After an index timeout, the index can lag behind an upload. C then
      sends that upload again, crates.io rejects it, and nothing else
      uploads. Run the command again when the index lists the crate.
    - When `plans/follow-ups/publish-crates-io.md` still exists, its dry-run
      step and its upload step then name only this command.
    - Files: the work adds `xtask/src/publish.rs` and its tests in
      `xtask/src/_tests_/publish_tests.rs`. It changes `xtask/src/cli.rs`,
      `xtask/src/_tests_/cli_tests.rs`, `xtask/src/main.rs`,
      `xtask/src/error.rs`, `xtask/README.md`, and `AGENTS.md`.
    - A JSON parser such as `serde_json` also changes `xtask/Cargo.toml`.
      Check `README.md:124` and `docs/architecture.md:39-41`, which describe
      `xtask` only as the gate.
    - Cost: network code or a shell-out, such as `curl` through
      `CommandRunner`. It also needs tests with a fake index, and docs. The
      code runs only at release time. No test can run it end to end without a
      token.
    - Effort: Medium.
  - **D.** Use a release tool that skips published versions, for the upload
    and for each resume. An example is the `publish` step of `cargo-release`:
    - For each version that the index lists, that step warns "disabled due to
      previous publish" and skips the crate. It then publishes the rest.
    - Keep the workspace dry run before the approval.
    - On 2026-11-02 a review of the first release left out `release-plz`, a
      tool of the same kind, because plain Cargo met that goal. Plain Cargo
      cannot skip an existing version, so that reason does not hold here.
    - Cost: a new tool that you must install and pin for a public release with
      the owner's token. Its checks and defaults need a scratch run first. It
      replaces the Cargo upload that commit `a0b0cc4` chose.
    - Effort: Medium.
  - **E.** Write the release commands and the resume rule of A in a lasting
    doc, such as a release section in the root `README.md`. This needs no
    code. The operator still builds each `--exclude` list by hand, so a wrong
    list can stop a resume again. Effort: Small.
- **Left out:**
  - Plain Cargo. On 2026-11-02 the `--help` output of the pinned Cargo 1.95.0
    and of stable Cargo 1.99.1 showed no flag that skips an existing version.
  - crates.io. It takes one crate in each upload request, so the three crates
    cannot upload as one unit.
  - A CI release workflow. It runs the same Cargo commands, so a rerun stops
    on the same check. It also needs a crates.io token in CI.
  - Other forms of A: one resume command for each stop point, or `-p` for
    each crate that the dry run does not warn about. They select the same
    crates as A. The first form must also know the upload order.
  - Remove the resume rule and rely on the Cargo errors. This breaks the
    follow-up rule that says to copy each command that the work needs.
  - Merge the library crates into one, so that a release has one upload.
    This breaks the user decision of 2026-10-20 for three crates.
  - More checks before the first upload. This fails the goal, because a
    network error or the index wait can still stop the run part way.
  - A shell script outside `xtask/`. `AGENTS.md` puts developer tasks in
    `xtask/`, and C does the same work behind the tested `CommandRunner`
    trait.
- **Wider fix:** C. One command builds each `--exclude` list from the index,
  so no release needs a hand-built command. The rule of A holds for every
  stop point, but only in the publish item. C keeps it for each later
  release. C is not blocked, because its tests use a fake index, and only the
  real upload needs the token.
- **Previous examples:** Before commit `a0b0cc4`, the dry-run step ran
  `cargo publish --dry-run -p <crate>` for each library crate. That command
  fails for `pipedream-connect` and `pipedream-connect-webhooks` until
  `pipedream-connect-types` is on crates.io. Commit `78088d6` fixed the
  resume command that this item covers.
- **Related:** `plans/follow-ups/publish-crates-io.md`, the same code. Its
  upload step holds the rule of A. When C lands before the first release, the
  dry-run step and the upload step of that item name `cargo xtask publish`.
  When the release comes first, it uses the rule of A. Its branch then deletes
  that item and this link.
- **Recommendation:** C. One tested command does the first upload and each
  resume, also after the publish item is gone.
- **Action:** fix after merge, apply C, because C adds an `xtask` command
  outside the docs-only branch that found it.
