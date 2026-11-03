# Rustdoc links in crate docs are broken

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Post-merge task.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`.
  Finding 63 of the final review of 2026-10-22, from its re-review. The user
  chose outcome A on 2026-10-26.
- **Location:** `crates/pipedream-connect-types/README.md:123-126`,
  `crates/pipedream-connect/README.md:313-316`, and
  `crates/pipedream-connect-webhooks/README.md:120-122`.
- **Reason:** The new links point to the `main` branch, and `main` holds the
  linked docs only after the merge.

- **Terms:**
  - **Crate docs:** the API docs that rustdoc builds for a crate. `cargo doc`
    builds them locally, and docs.rs builds them for each crates.io release.
  - **Library README:** the `README.md` of `pipedream-connect-types`,
    `pipedream-connect`, or `pipedream-connect-webhooks`.
  - **Related Docs list:** the last section of each library README. It links
    to the protocol docs and to the architecture doc.
- **Background:**
  - Line 1 of `src/lib.rs` in each library crate is
    `#![doc = include_str!("../README.md")]`. So the README is also the front
    page of the crate docs.
  - Each Related Docs list uses relative links, such as
    `../../docs/architecture.md`. Relative links work when GitHub shows the
    README.
  - Rustdoc copies a relative link without a change. The front page is
    `target/doc/<crate>/index.html`, so the link points to
    `target/docs/architecture.md`. That file does not exist.
  - On docs.rs the same link points to an address outside the crate docs,
    which does not exist either.
  - A run on 2026-11-02 showed the problem. `cargo doc --no-deps` for the
    three library crates gave no warning. Each generated `index.html` kept the
    relative links, and no link target existed. The `grep` command of the
    task found 11 relative links.
  - The gate runs rustdoc with `-D warnings`. Rustdoc does not check these
    links, so the gate passes.
  - On 2026-11-02 the Related Docs lists linked to these files:
    - Types README: `docs/protocol/connect-resources.md`,
      `docs/protocol/connect-client.md`, `docs/architecture.md`, and the SDK
      plan `plans/pipedream-connect-sdk.md`.
    - Client README: `docs/protocol/connect-client.md`,
      `docs/protocol/connect-resources.md`,
      `docs/protocol/resources/triggers.md`, and `docs/architecture.md`.
    - Webhooks README: `docs/protocol/webhooks.md`,
      `docs/protocol/connect-resources.md`, and `docs/architecture.md`.
  - The SDK plan is deleted before the merge. That deletion removes each link
    to the plan.
  - The repository `futex-ai/pipedream-connectors` is public, and its default
    branch is `main`. On 2026-11-02 `main` held only `README.md`.
  - `crates/pipedream-cli/README.md` also has relative links. No rustdoc page
    includes it, and the CLI crate is not published. So it keeps them.
  - The final review of 2026-10-22 found the dead links. On 2026-10-26 the
    user chose absolute repository URLs. The other option was a separate
    short crate doc instead of the README.
- **Task:**
  1. In each Related Docs list, replace each relative link with
     `https://github.com/futex-ai/pipedream-connectors/blob/main/<path>`.
     `<path>` is the path from the repository root, such as
     `docs/architecture.md`.
  2. Remove each link whose target file does not exist.
  3. Build the crate docs, and search them for relative links:

     ```sh
     cargo doc --no-deps -p pipedream-connect-types -p pipedream-connect \
       -p pipedream-connect-webhooks
     grep -o 'href="\.\./\.\./[^"]*"' target/doc/pipedream_connect*/index.html
     ```

     The `grep` command must print nothing.
  4. Open each new URL, and check that it loads.
  5. Run `npx --yes markdownlint-cli2` on the three READMEs. Run
     `cargo xtask check`.
- **Impact if no action:** Readers of the crate docs get dead links to the
  protocol docs and to the architecture doc. This includes the docs.rs pages
  of a later crates.io release.
- **Needs:** This SDK merged on `main`, so that the linked files exist there.
- **Done when:** The Related Docs lists of the three library READMEs use only
  absolute URLs. The `grep` command of the task prints nothing. Each URL
  loads.
- **Related:** `plans/follow-ups/publish-crates-io.md`, an order of work.
  Finish this task before the first crates.io release, because docs.rs builds
  the docs of each version once.
