# Delivery identity changes after a serde round trip

- **Severity:** Low. **Likelihood:** Rare. **Effort:** Small.
  **Confidence:** High. **Harder later:** Yes.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  final review of 2026-10-22 raised it as finding 25.
- **Location:** `crates/pipedream-connect-webhooks/src/trigger.rs:30`,
  `crates/pipedream-connect-webhooks/src/trigger.rs:76-87`,
  `crates/pipedream-connect-webhooks/README.md:20-25`.
- **Reason:** It waits for a user decision between a public serde API and a
  stable identity, which the user deferred on 2026-11-02 as not urgent.

- **Source:** Review.
- **Category:** product bug.
- **Terms:**
  - **Trigger delivery:** a POST that carries an event from a deployed
    trigger. The `pipedream-connect-webhooks` crate decodes it into
    `TriggerDelivery`.
  - **Context ID:** the optional `context.id` field of a trigger delivery.
  - **Identity:** the value of `TriggerDelivery::identity()`. It is the
    context ID, or else the SHA-256 digest of the body text. Hosts use it to
    drop duplicates.
  - **Round trip:** a host serializes a decoded delivery with serde, stores
    the JSON, and later deserializes it with serde.
- **Background:**
  - `TriggerDelivery` derives `Serialize` and implements `Deserialize`.
    `crates/pipedream-connect-webhooks/src/lib.rs` exports it, so both are
    public API.
  - `TriggerDelivery::decode` and `decode_with_limit` hash the exact raw body
    bytes.
  - The serde `Deserialize` path hashes the JSON value text without outer
    whitespace. `crates/pipedream-connect-webhooks/README.md` documents both
    rules.
  - `Serialize` writes an absent context ID as `null` and sorts unknown
    top-level keys. It does not write the identity.
  - So a round trip hashes different text and gives a new identity.
  - This matters only for a delivery without a context ID that the host stores
    through serde.
  - A probe on 2026-11-02 showed it. The body `{"event":{"x":1}}` serialized
    as `{"event":{"x":1},"context":{"id":null}}`, and the reloaded delivery got
    a new identity. A delivery with a context ID kept its identity.
  - Pipedream signs each trigger delivery. The crate verifies the signature
    over the exact raw body, so a host already has the raw body.
  - Only the tests of the webhooks crate use the serde decode path today. The
    library crates are not yet published.
- **Problem:** The identity is not stable across a serde round trip.
- **Impact if no action:** A host that stores decoded deliveries and removes
  duplicates after a reload treats a redelivery as new. After the release,
  hosts can start to read stored deliveries through serde, and options C and D
  then break them.
- **Root question:** How does a host get the same identity for a delivery
  each time it reads the delivery?
- **Options:**
  - **A.** Document it. Tell hosts to store `identity()` or the raw body with
    each delivery. No code changes, but the trap stays. Effort: Small.
  - **B.** Keep the raw JSON text in the struct and write it back on
    serialize. The identity then survives round trips. `Serialize` then
    ignores edited fields, and outer whitespace still differs. Effort: Medium.
  - **C.** Remove serde support from `TriggerDelivery`. Hosts store the raw
    body and call `decode` again. This removes a public API. Effort: Small.
  - **D.** Remove only `Deserialize` and keep `Serialize`. The identity then
    comes only from raw bytes, and hosts can still log deliveries. Stored
    deliveries cannot be read back. Effort: Small.
- **Left out:** Not recorded.
- **Wider fix:** C. With no serde decode path, re-encoded text never feeds the
  hash.
- **Previous examples:** The second review of the typed models and the
  webhooks crate set the serde path hash. Commit `7187664` of 2026-10-21 made
  `decode` hash the exact body and the serde path hash the value text. Commit
  `d339b51` of the same day first gave the serde path an identity.
- **Related:** A closed decision on the same design. The serde path of the
  `ConnectionEvent` webhook model skipped the checks of its `decode` method.
  On 2026-10-26 the user chose one private constructor for both paths,
  instead of a note that documents the gap. Commit `2fc736b` applied it in
  `crates/pipedream-connect-webhooks/src/events.rs`. `TriggerDelivery` has
  used the same design since commit `7187664`. Options B, C, and D change the
  serde support of `TriggerDelivery`, which calls its shared constructor.
- **Recommendation:** D. It removes the trap and keeps output support. Hosts
  already need the raw body for the signature check.
- **Action:** ask the user, because D removes a public API, and the options
  trade that API against a stable identity.
