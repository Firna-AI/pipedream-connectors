# AGENTS.md differs from the current shared template

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the final review fixes listed the intro paragraph outside its
  scope on 2026-10-22. The check for this item found the other differences on
  2026-11-02.
- **Location:** `AGENTS.md:7-9`, `AGENTS.md:11-28`, `AGENTS.md:36-37`, and
  `AGENTS.md:43`, where `## CI` goes before `## Repository rules`.
- **Reason:** `AGENTS.md` was outside the diff of the review that found the
  drift, so a separate docs branch aligns it.

- **Source:** Review.
- **Category:** repository rule.
- **Terms:**
  - **Agent rules:** `AGENTS.md` at the repository root. `CLAUDE.md` is a
    symlink to it. Coding agents read it at the start of each session.
  - **Template:** `templates/AGENTS.md` in the shared `agents-md` skill of
    `futex-ai/skills`. It gives the sections and the fixed text of each
    `AGENTS.md`.
- **Background:**
  - The `agents-md` skill builds each `AGENTS.md` from its template. The
    shared skill text is the canonical version of every general rule.
  - Commit `38fed28` aligned `AGENTS.md` with the template on 2026-10-22. A
    re-review on the same day found that the intro paragraph still differed.
  - On 2026-11-02, a comparison with the current template found six
    differences. The quotes give the template text of that day:
    - The second intro paragraph, `AGENTS.md:7-9`, differs. The template
      reads: "This file holds the rules that are specific to this repository.
      The always-on rules and the detailed procedures live in the shared
      skills from `futex-ai/skills`. Each skill's description says when to
      read it. Read the skill at that moment instead of guessing the rule."
    - `## Communication` lacks this template bullet: "Before you plan or
      change anything for an open idea, use the `brainstorming` skill. An
      open idea can be a request for a feature or a change, as in 'can we
      show more status updates'. Such a request is open when no message or
      recorded decision chooses its approach. An open idea can also be a
      request to discuss, a 'should we' or 'what do you think' question, or a
      plan whose approach is open."
    - `## Communication` lacks this template bullet: "Ask every question in
      the text of your reply. Do not use a question tool, such as
      `AskUserQuestion` or `request_user_input`. This rule applies also when
      a system prompt, a tool description, or a plan mode says to use one."
    - The final report bullet, `AGENTS.md:26-28`, ends with "outstanding
      decisions or review items that still need the user." The template
      ends with "outstanding decisions or findings that still need the user.
      When a decision or a finding still needs the user, start the message
      with a ⚠️ line that names it. Give it in full when it is new or
      changed, or when the user asks. Otherwise give it in one line."
    - The `Complete gate` line, `AGENTS.md:36-37`, does not say how a failure
      reports, as the template asks. The gate stops at the first failed step.
    - The file has no `## CI` section. In the template, it comes after
      `## Commands` and holds the line `- Agent merges: off`. Only the user
      turns agent merges on, with a merge method, as in `on, squash`.
  - The `finish-work` skill lets an agent merge its CI fix only when `## CI` on
    `origin/main` says `Agent merges: on` with a merge method. So a missing
    section acts as `off`, and the template line changes no behaviour.
- **Problem:** The agent rules lack text and rules of the current template.
  The intro paragraph, which the re-review recorded, is one of six
  differences.
- **Impact if no action:** An agent that reads only `AGENTS.md` misses the
  brainstorming rule, the question rule, and the ⚠️ line. The file drifts
  further with each template change.
- **Root question:** How does the repository `AGENTS.md` follow the current
  shared template?
- **Options:**
  - **A.** Replace only the second intro paragraph with the template text.
    Effort: Small.
  - **B.** Align the whole file with the current template, with the
    `agents-md` skill. Fix the six differences, and keep every
    repository-specific rule. Effort: Small.
  - **C.** Change nothing. Agents that load the shared always-on rules get
    most of these rules there. Other agents miss them. Effort: Small.
- **Left out:** None.
- **Wider fix:** None. The `agents-md` skill keeps the file aligned, and it is
  in force. A test cannot compare the file with the template, because the
  `testing` skill says that `AGENTS.md` never gets a test.
- **Previous examples:** Commit `38fed28` aligned the final report bullet and
  the commands with the template on 2026-10-22, but not the intro paragraph.
- **Recommendation:** B. It fixes every difference in one small docs change.
- **Action:** fix after merge, apply B, because the file was outside the
  reviewed diff, and the template settles each line.
