# Library crates are not on crates.io

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Post-merge task.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  plan `plans/pipedream-connect-sdk.md` recorded it as a post-merge task. It
  follows the error runtime decision of 2026-10-20.
- **Location:** `Cargo.toml:28`, the `internal-error` workspace dependency.
  The manifests `crates/pipedream-connect-types/Cargo.toml`,
  `crates/pipedream-connect/Cargo.toml`, and
  `crates/pipedream-connect-webhooks/Cargo.toml`.
- **Reason:** crates.io rejects the library crates until `internal-error` and
  `internal-error-derive` are on crates.io.

- **Terms:**
  - **crates.io:** the public Rust package registry. `cargo publish` uploads a
    crate to it.
  - **docs.rs:** the site that builds and hosts the API docs of each crates.io
    release.
  - **`internal-error`:** the shared futex crate that gives `InternalError` and
    the `ErrorContract` derive. Its source is the repository
    `futex-ai/internal-error`.
  - **Library crates:** `pipedream-connect-types`, `pipedream-connect`, and
    `pipedream-connect-webhooks`. The `pipedream-cli` and `xtask` crates set
    `publish = false`.
- **Background:**
  - The error enums of `pipedream-connect-types` and `pipedream-connect` have
    an `Internal(InternalError)` variant. The shared futex Rust rules require
    that fallback.
  - On 2026-10-20 the user chose to depend on `internal-error` by git
    revision, the same as Firna. Consumers use the SDK by git revision, and
    V1 has no crates.io release.
  - The user rejected the other option. It gave the SDK a small error type of
    its own, so that a release was possible at once.
  - The decision made the crates.io release a follow-up for the day when
    `internal-error` is on crates.io.
  - `Cargo.toml:28` declares `internal-error` with `version = "0.1.0"`, the
    git URL `https://github.com/futex-ai/internal-error.git`, and revision
    `6477922a1c4db8c3e35189af69f0147464004b99`.
  - `cargo publish` drops the git source of a dependency and looks up its
    version on crates.io.
  - `pipedream-connect-types` and `pipedream-connect` depend on
    `internal-error`. `pipedream-connect-webhooks` depends on
    `pipedream-connect-types`. So all three library crates wait.
  - The `internal-error` repository has two crates at that revision:
    `internal-error` and its proc-macro crate `internal-error-derive`.
    `internal-error` depends on `internal-error-derive`.
  - On 2026-11-02 crates.io had neither crate. It also had no crate with the
    names `pipedream-connect`, `pipedream-connect-types`, or
    `pipedream-connect-webhooks`. Unrelated crates use the names `pipedream`
    and `pipedream-rs`.
  - On 2026-11-02 `cargo publish --dry-run --locked -p pipedream-connect-types`
    failed with this text:

    ```text
    error: failed to prepare local package for uploading

    Caused by:
      no matching package named `internal-error` found
      location searched: crates.io index
      required by package `pipedream-connect-types v0.1.0 (...)`
    ```

  - When Cargo packages `pipedream-connect` or `pipedream-connect-webhooks`,
    it replaces their path dependency on `pipedream-connect-types` with a
    crates.io dependency. So `cargo publish --dry-run -p <crate>` for one of
    them fails with "no matching package named `pipedream-connect-types`
    found" until that crate is on crates.io. This holds also after
    `internal-error` is on crates.io.
  - Cargo 1.95.0 is the pinned toolchain in `rust-toolchain.toml`. With
    `--workspace`, `cargo publish` handles the dependency between the library
    crates itself:
    - It packages and verifies all three library packages before the first
      upload. It takes the unpublished `pipedream-connect-types` from a
      temporary local registry under `target/package/tmp-registry`.
    - It skips `pipedream-cli` and `xtask`, because they set
      `publish = false`.
    - It uploads `pipedream-connect-types` first. It then waits up to 60
      seconds until the crates.io index lists that crate, and only then
      uploads `pipedream-connect` and `pipedream-connect-webhooks`.
    - When the index does not list the first crate in time, Cargo stops with
      "unable to publish ... due to a timeout while waiting for published
      dependencies to be available."
    - On 2026-11-02 a scratch workspace with Cargo 1.95.0 showed this. The
      dry run of a dependent crate alone failed. The dry run of the whole
      workspace passed, and it listed first the upload of the crate that the
      other two depend on. The wait and the timeout text come from the
      `cargo publish` source of Cargo 1.95.0.
  - On 2026-11-02 `cargo package --list` showed no license file in any
    library package. Each package held only `.cargo_vcs_info.json`,
    `Cargo.lock`, `Cargo.toml`, `Cargo.toml.orig`, `README.md`, `src/`, and
    `tests/`.
  - The `LICENSE` file is at the workspace root. The manifests set only
    `license = "MIT"`. The MIT license requires its text in each copy of the
    software.
  - Cargo treats a git copy and a crates.io copy of one crate as two
    different crates. A consumer that pins `internal-error` by git then gets
    a second `InternalError` type. Firna pins `internal-error` by git.
- **Task:**
  1. Check that crates.io has `internal-error` and `internal-error-derive`.
     Check that the released `internal-error` has the API of the pinned
     revision.
  2. In `Cargo.toml:28`, replace the git source of `internal-error` with the
     crates.io version. Run `cargo update -p internal-error`.
  3. Run `cargo xtask check`.
  4. Add the license text to each library package.
     `plans/follow-ups/library-packages-license-file.md` holds the fix and
     its options. Check the result with `cargo package --list -p <crate>`.
  5. Do the task in `plans/follow-ups/rustdoc-absolute-readme-links.md` first.
  6. Check that the three crate names are still free on crates.io.
  7. Run `cargo publish --dry-run --workspace`. It packages and verifies the
     three library packages together and uploads nothing.
  8. Get the approval of the user. A crates.io release is public. crates.io
     can yank a version, but it cannot delete it.
  9. Run `cargo publish --workspace`. Cargo gets the upload order from the
     workspace, as the background says.
     - Risk: crates.io can reject a crate, or the index wait can time out.
       Cargo uploads `pipedream-connect-types` first. It then uploads
       `pipedream-connect` and `pipedream-connect-webhooks` in name order. So
       a failure can stop the run after one upload or after two.
     - A second `cargo publish --workspace` then stops at once, because a
       version already exists on crates.io. To find the uploaded crates, run
       `cargo publish --dry-run --workspace`. It warns
       `crate <name>@<version> already exists` for each uploaded crate and
       exits with status 0.
     - Publish the rest with one `--exclude` for each uploaded crate, for
       example
       `cargo publish --workspace --exclude pipedream-connect-types --exclude pipedream-connect`.
       After an index timeout, run the dry run again until it warns that
       `pipedream-connect-types` exists, then resume.
  10. Check that docs.rs builds the docs of each crate.
  11. Tell the consumers that pin `internal-error` by git, such as Firna, to
      move to the crates.io release.
- **Impact if no action:** Rust users outside futex can use the SDK only by
  git revision. docs.rs has no pages for the SDK. Another publisher can take
  the free crate names.
- **Needs:** `internal-error` and `internal-error-derive` on crates.io. A
  crates.io API token of the account that will own the crates. The approval
  of the user for the release.
- **Done when:** crates.io lists the three library crates. docs.rs shows the
  docs of each crate. `Cargo.toml` takes `internal-error` from crates.io.
- **Related:**
  - `plans/follow-ups/rustdoc-absolute-readme-links.md`: an order of work. Do
    that task before the first release, because docs.rs builds the docs of
    each version once, from its README.
  - `plans/follow-ups/library-packages-license-file.md`: the same code. It
    records the missing `LICENSE` file in each package and holds the fix for
    step 4 of the task. Apply it before the first release.
  - `plans/follow-ups/workflows-shutdown.md`: work that waits for an answer.
    Two of its options act in or before the first release, so get the answer
    of the user before the release.
