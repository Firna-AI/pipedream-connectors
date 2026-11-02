# No check finds one-way follow-up links

- **Severity:** Low. **Effort:** Medium. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`.
  Round 2 of its review found the class in finding 84. The check of that
  finding named this blocked wider fix.
- **Location:** `skills/plans/references/follow-up-items.md:57-60`,
  `skills/plans/templates/follow-up.md:24-28`, and
  `skills/plans/scripts/follow-ups.sh` in the repository `futex-ai/skills`.
  `xtask/src/check.rs:12-22` in this repository.
- **Reason:** The recommended option adds a new check, and only the user
  approves a new check.

- **Source:** Review.
- **Category:** repository rule.
- **Terms:**
  - **Follow-up item:** one Markdown file under `plans/follow-ups/` that
    records work that waits until after a merge. Each item must stand alone.
  - **Related line:** the **Related** field of a follow-up item. It names
    each linked item by path and says what the link means.
  - **One-way link:** a link from one item to a second item that does not
    name the first item back.
  - **`follow-ups.sh`:** the script of the shared `plans` skill that lists
    the open follow-up items by priority score.
  - **Gate:** `cargo xtask check`, the complete local check of this
    repository. CI runs the same command.
- **Background:**
  - The shared skills live in the repository `futex-ai/skills`. Its file
    `skills/plans/references/follow-up-items.md` holds the rules for
    follow-up items.
  - The rule at lines 57-60 says: "When two linked items stay apart, give
    each a **Related** line. Name the other file, and say what the link
    means."
  - Nothing checks this rule. `follow-ups.sh` reads only the title, the
    grades, and the kind of each item. Reviews find a one-way link by hand.
    `tests/follow_ups_test.sh` in `futex-ai/skills` tests the script.
  - Each branch that fixes an item deletes its file. So a link can also
    point to an item that no longer exists.
  - The template `skills/plans/templates/follow-up.md` gives the body
    layouts. At lines 24-28, the layout of a post-merge task lists
    **Terms**, **Background**, **Task**, **Impact if no action**,
    **Needs**, and **Done when**. It has no **Related** field, although the
    rule applies to every item.
  - Two post-merge tasks of this repository already have a Related line:
    `plans/follow-ups/publish-crates-io.md` and
    `plans/follow-ups/openapi-fixture-refresh.md`.
  - The class occurred twice in branch `calummoore/pipedream-rust-sdk-plan`
    on 2026-11-02:
    - Commit `a36667f` linked `plans/follow-ups/publish-crates-io.md` to
      `plans/follow-ups/library-packages-license-file.md` in one direction.
      Commit `0f8b3ad` added the reverse line.
    - Commit `4c045ce` added `plans/follow-ups/deployed-trigger-email-prefix.md`
      with links to `plans/follow-ups/emitter-date-unit.md` and
      `plans/follow-ups/openapi-fixture-refresh.md`. Neither item named it
      back until a review found it.
  - The gate runs format, clippy, tests, docs, a file-length audit, and a
    module-layout audit. `xtask/src/check.rs:12-22` runs the two audits.
  - The shared rules of `futex-ai/skills` say that only the user approves a
    new gate, lint, or stricter validation. A link check is a lint of the
    follow-up items, so B, C, and D wait for the user.
- **Problem:** The rules require a Related line in each linked item, but
  nothing checks it, and the class occurred twice in one branch. The
  post-merge layout of the template also has no Related field.
- **Impact if no action:** A reader who opens one item of a pair can miss
  the linked work, such as an order of work or a shared rule. Reviews keep
  finding one-way links by hand.
- **Root question:** How does a reader of either linked item find the other
  one?
- **Options:**
  - **A.** Add a **Related** field to the post-merge task layout of
    `skills/plans/templates/follow-up.md`. Writers of post-merge tasks then
    see the field. It does not find a one-way link. Effort: Small.
  - **B.** Add a link check to `follow-ups.sh`.
    - For each item, find the paths of the other items that it names.
      Print a warning for each named item that does not name the item back.
      Print a warning for each named item that does not exist.
    - Add a test case to `tests/follow_ups_test.sh`, and one sentence on the
      warnings to `skills/plans/references/follow-up-items.md`.
    - It covers every repository that uses the shared skills. A warning
      does not fail a gate.
    - It is blocked, because it adds a check. Effort: Medium.
  - **C.** Add the same check as an audit in the gate, next to the
    file-length and module-layout audits in `xtask/src/check.rs`. CI then
    fails a pull request with a one-way link. It covers only this
    repository, and it ties the Rust xtask to the item layout of a shared
    skill. It is blocked, because it adds a gate. Effort: Medium.
  - **D.** Do A and B in one change of `futex-ai/skills`. It is blocked for
    the same reason as B. Effort: Medium.
- **Left out:**
  - Change nothing. It fails the goal, because the class occurred twice in
    one branch while the rule was in force.
  - Add more words to the rule. The rule already states the requirement.
    The `writing-skills` skill puts a rule that a script can enforce in a
    check.
  - Show the reverse links only in a generated index or in the script
    output. It breaks the rule that an item stands alone.
  - A helper that writes both lines. An agent who skips the rule can also
    skip the helper.
  - Delete the forward links. It fails the goal, and it drops facts that the
    items must keep.
- **Wider fix:** B. It finds each one-way link and each link to a deleted
  item in every repository that uses the shared skills. It is blocked,
  because it adds a check.
- **Previous examples:** The two cases in the background: commits `a36667f`
  and `0f8b3ad`, and commit `4c045ce`.
- **Recommendation:** D. B finds each one-way link without a gate failure,
  and A closes the gap in the template in the same change.
- **Action:** ask the user, because D adds a new check to the shared
  listing script, and only the user approves a new check.
