# No gate step builds without default features

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  final review of 2026-10-22 raised it as finding 18.
- **Location:** `xtask/src/check.rs:25-57`,
  `xtask/src/_tests_/check_tests.rs:13-39`, `.github/workflows/ci.yml`.
- **Reason:** Each option adds a gate step, so the item waits for a user
  decision, and the user deferred that decision on 2026-11-02 as not urgent.

- **Source:** Review.
- **Category:** test.
- **Terms:**
  - **Gate:** `cargo xtask check`, the complete local check. CI runs the same
    command as its only check step.
  - **Default features:** Cargo features that stay on unless a consumer sets
    `default-features = false`. For `pipedream-connect`, this is only
    `reqwest`.
  - **`reqwest` feature:** the built-in HTTP transport of `pipedream-connect`.
    It adds the `builder` and `reqwest_transport` modules.
  - **`HttpTransport`:** the trait in `crates/pipedream-connect/src/transport.rs`
    through which a host supplies its own HTTP stack.
- **Background:**
  - `pipedream-connect` is the async client crate of this Pipedream Connect
    SDK. Its manifest declares `default = ["reqwest"]`.
  - A host can turn off `reqwest` and supply its own HTTP stack through the
    `HttpTransport` trait. `crates/pipedream-connect/README.md` documents this
    setup.
  - `#[cfg(feature = "reqwest")]` gates two modules in
    `crates/pipedream-connect/src/lib.rs`. It also gates two variants of the
    internal `WireFailure` error in `crates/pipedream-connect/src/error.rs`.
  - The gate runs clippy, tests, and docs only with `--all-features`, so
    `reqwest` is always on.
  - The test `check_plan_has_the_required_order_and_flags` in
    `xtask/src/_tests_/check_tests.rs` pins the four gate commands.
  - The "Development" section of the client README tells developers to run
    `cargo test -p pipedream-connect --no-default-features` by hand.
  - Reviewers built the client without default features by hand three times.
    Each build was clean. In the final review, clippy and tests took about 25
    seconds with a warm cache.
  - On 2026-11-02, clippy with `-D warnings` and the tests passed without
    default features. 140 unit tests passed. Each step took about 15 seconds
    in a new worktree with a shared compiler cache.
- **Problem:** No automatic check builds the client without `reqwest`. Code
  that uses `reqwest` outside its feature gate passes the gate.
- **Impact if no action:** A change can break every host that supplies its own
  transport. Nobody sees it until such a host upgrades.
- **Root question:** How do we keep the client working for hosts that turn off
  the built-in transport?
- **Options:**
  - **A.** Add two gate steps for `pipedream-connect` with
    `--no-default-features`: clippy and tests. CI runs them too, because CI
    runs the gate. Update the gate test and the gate summary in `AGENTS.md`.
    This adds about 25 seconds to each gate run. Effort: Small.
  - **B.** Run `cargo hack --feature-powerset` on every crate. This adds a
    tool, checks every feature combination, and is slower. Effort: Large.
- **Left out:** Not recorded.
- **Wider fix:** B.
- **Previous examples:** Reviewers ran the build without default features by
  hand in three reviews. These were the reviews of the managed auth and proxy
  resources, of the trigger and administration resources, and the final review.
- **Recommendation:** A. It protects the one documented optional setup for
  about 25 seconds.
- **Action:** ask the user, because both options add a new gate step, and only
  the user approves a new gate.
