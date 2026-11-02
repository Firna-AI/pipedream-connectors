# Firna keeps its own Pipedream adapter

- **Severity:** Medium. **Effort:** Medium. **Confidence:** High.
  **Harder later:** Yes.
- **Kind:** Post-merge task.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  plan `plans/pipedream-connect-sdk.md` recorded it as a post-merge task on
  2026-10-20.
- **Location:** `crates/fna-connectors-pipedream/` and
  `crates/fna-connectors/src/provider_factory.rs` in `futex-ai/firna`, branch
  `calummoore/tel-aviv-v4`.
- **Reason:** The work happens in another repository, and it pins a commit of
  this SDK that is on `main`.

- **Terms:**
  - **Firna:** the private product in the repository `futex-ai/firna`.
  - **Adapter:** the Firna crate `fna-connectors-pipedream`. It calls the
    Pipedream Connect API for Firna.
  - **`ConnectorProvider`:** the provider-neutral Firna trait in
    `crates/fna-connectors-interface/src/provider.rs`. The adapter implements
    it.
  - **SDK:** the library crates of this repository: `pipedream-connect-types`,
    `pipedream-connect`, and `pipedream-connect-webhooks`.
  - **Firna policy:** adapter behaviour that serves the Firna product, not the
    Pipedream contract. The SDK leaves it out.
- **Background:**
  - The SDK generalizes the adapter. It ports these adapter mechanics:
    single-flight token refresh, redacted diagnostics, bounded streaming
    bodies, strict identifier grammars, proxy header admission, and signature
    verification.
  - The adapter has about 2,200 lines of source code plus tests. Its modules
    are `client`, `config`, `http`, `token`, `transport`, `proxy`, `failure`,
    `endpoints`, `wire`, and `mapping`.
  - `client.rs` holds `PipedreamConnectorProvider` and its
    `ConnectorProvider` implementation. `proxy.rs` admits request headers and
    filters response headers. `failure.rs` maps HTTP failures to Firna
    errors.
  - The adapter exists only on the Firna branch `calummoore/tel-aviv-v4`.
    Firna `main` has no connector crates.
  - On 2026-11-02 the head of that branch was still `905976b` of 2026-09-16.
    The branch was 66 commits ahead of Firna `main` and 15 behind. No pull
    request was open for it.
  - The Firna plan `plans/pipedream-connectors.md` on that branch marks every
    milestone completed.
  - `ConnectorProvider` has 21 methods. They cover apps, components, prop
    configuration and reload, action runs, the proxy, Connect tokens,
    accounts, external users, deployed triggers, trigger webhooks, trigger
    events, and usage. The SDK covers every Connect operation, so it covers
    all 21.
  - `crates/fna-connectors/src/provider_factory.rs` builds the provider. It
    uses these adapter types: `PipedreamConnectorProvider`, `PipedreamConfig`,
    `ClientCredentials`, `Environment`, `ProjectId`, and
    `ReqwestPipedreamTransport`.
  - Both repositories pin the shared error crate `internal-error` at revision
    `6477922a1c4db8c3e35189af69f0147464004b99`.
  - The SDK is not on crates.io. On 2026-10-20 the user chose that consumers
    depend on the SDK by git revision.
  - The SDK leaves out four kinds of Firna policy:
    - Closed prop model. The adapter omits a whole component when a prop type
      is unknown. The SDK keeps an unknown prop type as `PropKind::Other`.
    - Product redaction. The adapter wraps app data in `SecretString`, for
      example option labels, proxy bodies, event payloads, and provider error
      text. The SDK wraps only credentials.
    - Proxy response filter. The adapter returns only the upstream headers
      that `safe_response_header` in
      `crates/fna-connectors-interface/src/proxy_policy.rs` admits. The SDK
      returns every upstream header.
    - Retries. Firna retries idempotent reads in its service layer. The SDK
      retries only GET requests, and only when the host turns on its retry
      policy.
  - The transport limits differ. The adapter uses a 10-second connection
    timeout, a 30-second total timeout, and a 1 MiB response limit. The SDK
    defaults are 10 seconds, 60 seconds, and 10 MiB. A host can set each
    limit.
  - The SDK webhooks crate verifies trigger delivery signatures and decodes
    deliveries. Firna decodes deliveries in
    `crates/fna-connectors/src/triggers/normalization.rs`.
- **Task:** Write a plan in `futex-ai/firna` with the `plans` skill. Follow
  the plan rules of the Firna repository too. At commit `905976b`, they
  require the consent of the user and a line in `plans/README.md`. The plan
  makes the adapter depend on the SDK and keeps its `ConnectorProvider`
  implementation. It covers these points:
  1. Choose the Firna branch for the plan. The connector crates are not on
     Firna `main`, so the plan can target their branch before it merges.
  2. Add the SDK crates as git dependencies at a commit on `main` of
     `futex-ai/pipedream-connectors`.
  3. Keep `PipedreamConnectorProvider` and its `ConnectorProvider`
     implementation in `client.rs`. Build the provider on the SDK client.
  4. Replace the HTTP code of the adapter with SDK calls and models. This
     covers the modules `config`, `http`, `token`, `transport`, `endpoints`,
     and `wire`, and the request code in `proxy`.
  5. Keep the `mapping` module. Change `failure` so that it maps SDK errors
     to the Firna error type.
  6. Keep each kind of Firna policy in the adapter, or record why Firna drops
     it.
  7. Build the provider in `crates/fna-connectors/src/provider_factory.rs`
     from the SDK configuration types.
  8. Decide the transport limits that Firna sets on the SDK client.
  9. Decide whether Firna uses the SDK webhooks crate for trigger
     deliveries.
  10. Keep the adapter tests that check the mapping and the policy. Use the
      SDK mocks. The `unimock` feature of `pipedream-connect` and of
      `pipedream-connect-webhooks` provides them.
- **Impact if no action:** Firna and this repository keep two copies of the
  same Pipedream mechanics. A fix in one copy does not reach the other. Each
  new change to the adapter adds work to the switch. The SDK has no consumer
  that uses it in a product.
- **Needs:** This SDK merged on `main` of `futex-ai/pipedream-connectors`.
  Access to the private repository `futex-ai/firna`. The consent of the user
  to the new Firna plan.
- **Done when:** A plan in `futex-ai/firna` covers each point of the task.
  The plan is committed on the Firna branch that it targets.
- **Related:** `plans/follow-ups/emitter-date-unit.md`, same cause. The Firna
  mapping reads the emitter dates with `DateTime::from_timestamp_millis`. The
  Firna plan must take each date from `Timestamp::as_datetime`, or Firna keeps
  wrong dates after that fix.
