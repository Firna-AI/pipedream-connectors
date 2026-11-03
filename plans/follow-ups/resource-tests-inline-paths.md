# Tests use inline types crate paths

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the final review fixes listed `accounts_tests.rs:38` and
  `tokens_tests.rs:71` outside its scope on 2026-10-22. The check for this
  item found the other files on 2026-11-02.
- **Location:**
  `crates/pipedream-connect/src/resources/_tests_/accounts_tests.rs:38-40`,
  `crates/pipedream-connect/src/resources/_tests_/accounts_tests.rs:152-156`,
  `crates/pipedream-connect/src/resources/_tests_/accounts_tests.rs:171`,
  `crates/pipedream-connect/src/resources/_tests_/accounts_tests.rs:210`,
  `crates/pipedream-connect/src/resources/_tests_/tokens_tests.rs:73`,
  `crates/pipedream-connect/src/resources/_tests_/users_tests.rs:102`,
  `crates/pipedream-connect/src/resources/_tests_/rate_limits_tests.rs:50`,
  `crates/pipedream-connect/src/_tests_/pagination_tests.rs:164`,
  `crates/pipedream-connect/tests/openapi_coverage/cases.rs:47`,
  `crates/pipedream-cli/src/_tests_/cli_tests.rs:158`,
  `crates/pipedream-cli/src/_tests_/cli_tests.rs:162`,
  `crates/pipedream-cli/src/_tests_/inputs_tests.rs:125`.
- **Reason:** The test files were outside the diff under review, and the fix
  is a style change for a separate branch.

- **Source:** Review.
- **Category:** repository rule.
- **Terms:**
  - **Types crate:** `pipedream-connect-types`, the crate of the request,
    response, and identifier models. Rust code names it
    `pipedream_connect_types`.
  - **Inline path:** a crate path in the code, such as
    `pipedream_connect_types::PageRequest`, instead of a `use` import at the
    top of the file.
- **Background:**
  - The shared `rust` skill says: "Prefer imports over inline `crate::...`
    paths in code, type aliases, and `dyn` trait objects."
  - On 2026-10-22, commit `2c5bb64` fixed import order and inline paths in
    seven resource test files. It replaced `crate::` paths and types crate
    paths with imports.
  - `accounts_tests.rs` imports seven types crate items. It still writes nine
    inline paths for four other items: `OauthAppId`, `PageRequest`, `Cursor`,
    and `AccountRef`.
  - `users_tests.rs:102` writes `pipedream_connect_types::AppSlug`, although
    the file imports `AppSlug`. Commit `2c5bb64` missed this line.
  - Six more inline paths name items without a clash: `ListProjectsRequest`,
    `ListRegistryRequest`, `DeployTriggerRequest`, two `Environment` values,
    and `ConfiguredProps`. The Location line gives their files.
  - `tokens_tests.rs:73` writes `pipedream_connect_types::Error`. The file
    also imports the client error `crate::error::Error`, so an import of the
    types crate `Error` would clash.
  - On 2026-11-02, 14 files wrote an inline `pipedream_connect_types::Error`
    or `pipedream_connect_types::Result` path. In 12 of them, another `Error`
    is in scope.
  - The other two are `crates/pipedream-cli/src/report/input.rs` and
    `crates/pipedream-cli/src/report/_tests_/input/hint_tests.rs`. The CLI
    crate has its own `Error`, so the full path shows which `Error` they use.
- **Problem:** Sixteen inline types crate paths in seven test files have no
  name clash, so an import can replace each of them.
- **Impact if no action:** The test files mix two styles for the same items,
  and new tests copy the inline form.
- **Root question:** How do the tests refer to types crate items?
- **Options:**
  - **A.** Fix only the two recorded files. Import `AccountRef`, `Cursor`,
    `OauthAppId`, and `PageRequest` in `accounts_tests.rs`, and replace its
    nine inline paths. Keep the clashing `Error` path in `tokens_tests.rs`.
    Effort: Small.
  - **B.** Option A, and replace the seven other inline paths without a clash
    in the six other files. Keep the inline `Error` and `Result` paths of the
    14 files. Effort: Small.
  - **C.** Enable the clippy restriction lint `absolute_paths` in the
    workspace, and fix each path that it flags. By default, it flags paths
    with more than two segments, such as
    `pipedream_connect_types::PageRequest::default`. It also flags the kept
    `Error` paths, such as `pipedream_connect_types::Error::InvalidHttpsUrl`.
    This adds a lint. Effort: Medium.
- **Left out:** Import the clashing types crate `Error` under a new name with
  `as`. It adds a second name for one type, and the 14 files that keep the
  inline path would then differ.
- **Wider fix:** C. It catches new inline paths with three or more segments.
  It is blocked, because it adds a lint, and only the user can add a lint.
- **Previous examples:** On 2026-10-22, commit `2c5bb64` fixed import order
  and inline paths in seven resource test files.
- **Recommendation:** B. It removes every inline types crate path that has no
  name clash in one small change. C gives the best protection, but the user
  must approve a new lint.
- **Action:** fix after merge, apply B, because the test files were outside
  the diff under review, and the change is style only.
