# Library packages leave out the license file

- **Severity:** Low. **Likelihood:** Common. **Effort:** Small.
  **Confidence:** High. **Harder later:** Yes.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the final review fixes listed it outside its scope on
  2026-10-22.
- **Location:** `Cargo.toml:14`, `crates/pipedream-connect-types/Cargo.toml:5`,
  `crates/pipedream-connect/Cargo.toml:5`,
  `crates/pipedream-connect-webhooks/Cargo.toml:5`, `LICENSE`.
- **Reason:** No package is published yet, and the manifests were outside the
  diff under review, so a separate branch fixes the packages.

- **Source:** Review.
- **Category:** product bug.
- **Terms:**
  - **Library package:** one of the three publishable crates:
    `pipedream-connect-types`, `pipedream-connect`, and
    `pipedream-connect-webhooks`. The CLI and `xtask` set `publish = false`.
  - **Package:** the `.crate` archive that `cargo package` builds and
    `cargo publish` uploads to crates.io. It holds files of the crate
    directory.
  - **SPDX expression:** the standard license name in the `license` field of
    a manifest, such as `MIT`. It names the license but holds no license text.
- **Background:**
  - The workspace manifest sets `license = "MIT"` at `Cargo.toml:14`. Each
    library package inherits it with `license.workspace = true`.
  - The repository root holds `LICENSE`, the MIT license text with the line
    "Copyright (c) 2026 Futex".
  - `LICENSE:12-13` says: "The above copyright notice and this permission
    notice shall be included in all copies or substantial portions of the
    Software."
  - The library packages do not set `publish = false`, so `cargo publish` can
    upload them. On 2026-11-02, crates.io had no package of the SDK.
  - On 2026-11-02, `cargo package --list --offline --allow-dirty -p <package>`
    listed the files of each library package. Each list holds `Cargo.toml`,
    `Cargo.lock`, `README.md`, and the sources, but no `LICENSE`.
  - Cargo does not copy the root `LICENSE` into a package for a `license`
    field.
- **Problem:** A published library package would not hold the license text,
  but the MIT terms ask for the notice in every copy.
- **Impact if no action:** Each version that is published before the fix stays
  without the license text, because crates.io never changes a published
  version. Users who vendor or redistribute the crate cannot ship the notice
  from the package.
- **Root question:** How does each library package carry the MIT license
  text?
- **Options:**
  - **A.** Add a `LICENSE` symlink to `../../LICENSE` in each library crate
    directory. Cargo then packs the content of the root file. Check that
    `cargo package --list` names `LICENSE` for each package, and that the
    packed file holds the license text. A checkout without symlink support
    turns each link into a small text file. Effort: Small.
  - **B.** Copy `LICENSE` into each library crate directory. The three copies
    must then stay the same as the root file. Effort: Small.
  - **C.** Change nothing until the first publish. The repository holds the
    text, and the `license` field names MIT. A publish before the fix ships
    packages without the text. Effort: Small.
  - **D.** Add a step to the gate `cargo xtask check` that fails when a
    library package leaves out `LICENSE`. It catches a new library crate
    that has no license file.
    - Check the file list with `cargo package --list -p <package>`. A full
      `cargo package` run looks up `internal-error` on crates.io and fails
      with "no matching package named `internal-error` found".
    - After `internal-error` is on crates.io, one gate step can run
      `cargo publish --dry-run --workspace` and check the file list of each
      package that it writes under `target/package/`. That step also
      verifies that the packages build. A dry run only warns when a version
      already exists, so the step keeps working after the first release.
      Each run of that step reads the crates.io index and builds the three
      packages again.
    - The dry run alone does not check for `LICENSE`. On 2026-11-02 a
      scratch workspace with `license = "MIT"` and no `LICENSE` file passed
      `cargo publish --dry-run --workspace` with Cargo 1.95.0.
    - D is blocked, because it adds a gate, and only the user approves a
      gate. Effort: Small.
- **Left out:** Set `license-file` to the root file. Cargo warns when a
  manifest sets both `license` and `license-file`, and without `license`,
  crates.io shows no SPDX name. Options A and B are better on every point.
- **Wider fix:** D. It catches each library package without `LICENSE` in the
  gate. It is blocked, because it adds a gate.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/publish-crates-io.md`, work that waits for
  this item. The first crates.io release needs this fix first.
- **Recommendation:** A. One file holds the text, and every package carries
  it. D gives the best protection, but it waits for the user, because only
  the user approves a gate.
- **Action:** fix after merge, apply A, because no package is published yet,
  and the fix adds the existing license text without a change of license.
