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
- **Left out:** Set `license-file` to the root file. Cargo warns when a
  manifest sets both `license` and `license-file`, and without `license`,
  crates.io shows no SPDX name. Options A and B are better on every point.
- **Wider fix:** None. A gate step that fails when a package has no `LICENSE`
  would catch a new library crate. It adds a gate, and only the user can add
  a gate.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/publish-crates-io.md`, work that waits for
  this item. The first crates.io release needs this fix first.
- **Recommendation:** A. One file holds the text, and every package carries
  it.
- **Action:** fix after merge, apply A, because no package is published yet,
  and the fix adds the existing license text without a change of license.
