# Webhook protocol names the client, not the decoder

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  re-review of the final review fixes listed the sentence, then at line 115,
  outside its scope on 2026-10-22. The check for this item found the same
  pattern at line 179 on 2026-11-02.
- **Location:** `docs/protocol/webhooks.md:137` and
  `docs/protocol/webhooks.md:179`.
- **Reason:** Both sentences date from the first protocol commit and were
  outside the diff under review, so a separate docs branch fixes them.

- **Source:** Review.
- **Category:** docs or spec.
- **Terms:**
  - **Protocol doc:** a file under `docs/protocol/` that states the SDK
    requirements as MUST and MAY sentences. A human reviews each change to it.
  - **Client:** the Connect API client crate `pipedream-connect`. It sends
    requests to Pipedream and never decodes a webhook.
  - **Decoder:** the name that the webhook protocol gives the payload decoding
    of the webhooks crate `pipedream-connect-webhooks`.
- **Background:**
  - `docs/protocol/webhooks.md` states the rules for trigger deliveries and
    connection events. Most of its rules name the verifier, the decoder, or
    the host.
  - Line 137 says: "The client MUST preserve unknown fields in either
    connection payload."
  - The webhooks crate keeps these fields. `ConnectionEvent` in
    `crates/pipedream-connect-webhooks/src/events.rs` keeps unknown top-level
    fields in `extra`. The shared `Account` model keeps unknown account fields.
  - `docs/architecture.md` says that the webhooks library makes no Connect API
    calls. A host decodes a connection webhook, and it uses the client only to
    confirm the account.
  - Line 179 says: "The consumer MUST preserve the raw event JSON and unknown
    context fields." Line 128 uses "a consumer" for the host that can fetch
    account credentials.
  - `docs/architecture.md` also says that the host decides which app-defined
    fields to log, store, redact, or publish. So line 179 cannot bind the
    host.
  - The trigger delivery model in
    `crates/pipedream-connect-webhooks/src/trigger.rs` keeps the raw `event`
    and the unknown context fields.
  - The "Lossless data" section of the same doc already requires the exact
    JSON text of three parts. They are the trigger `event`, the top-level
    unknown fields, and the unknown `context` fields.
- **Problem:** Line 137 binds the client to a rule that only the decoder can
  meet. Line 179 binds "the consumer", a name that the doc also uses for the
  host.
- **Impact if no action:** A reader can look for unknown-field handling in the
  client crate, or think that a rule binds the host. A review of the client
  against the protocol checks a rule that the client cannot meet.
- **Root question:** How does the webhook protocol name the part of the SDK
  that keeps unknown payload fields?
- **Options:**
  - **A.** Replace "The client" with "The decoder" on line 137. Keep line 179.
    Effort: Small.
  - **B.** Option A, and replace "The consumer" with "The decoder" on line
    179. Every payload rule then names the decoder. Effort: Small.
  - **C.** Delete lines 137 and 179. Extend the "Lossless data" section, so
    that the decoder keeps every unknown field of a webhook payload at every
    level. Effort: Small.
- **Left out:** Change nothing. It fails the goal, because line 137 binds a
  crate that never sees a webhook.
- **Wider fix:** None. The `documentation` skill is the wider fix for docs
  drift, and it is in force.
- **Previous examples:** Commit `3ecdc77` fixed the same wrong subject in the
  Connect token rule on 2026-10-21. It left line 137 unchanged.
- **Recommendation:** B. It keeps both requirements and names the component
  that meets them, with a one-word change on each line.
- **Action:** fix after merge, apply B, because both sentences predate the
  reviewed change, and the architecture doc already says which component
  decodes webhooks.
