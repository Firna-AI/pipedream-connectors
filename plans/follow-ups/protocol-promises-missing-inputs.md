# Three protocol sentences promise missing inputs

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  final review of 2026-10-22 raised it as finding 22.
- **Location:** `docs/protocol/resources/tokens.md:43-44`,
  `docs/protocol/resources/accounts.md:54-55`,
  `docs/protocol/resources/proxy.md:36`.
- **Reason:** Each option changes protocol requirements, so the item waits for
  a user decision, and the user deferred it on 2026-11-02 as not urgent.

- **Source:** Review.
- **Category:** docs or spec.
- **Terms:**
  - **Protocol doc:** a file under `docs/protocol/` that states the SDK
    requirements as MUST and MAY sentences.
  - **OpenAPI document:** the API schema that Pipedream publishes. A copy
    fetched on 2026-10-20 is the test fixture
    `crates/pipedream-connect-types/tests/fixtures/pipedream_openapi_swagger.json`.
  - **Common cursor contract:** the rules under "Cursor pagination" in
    `docs/protocol/connect-resources.md`. A paged request MAY send `limit`,
    `after`, or `before`, and a paged response holds `page_info`.
  - **Connect token:** a token that belongs to one project, environment, and
    external user. Account creation sends it.
- **Background:**
  - The repository is a Rust SDK for the Pipedream Connect API. The
    `pipedream-connect-types` crate holds the models, and `pipedream-connect`
    is the client.
  - Nothing generates code or docs from the OpenAPI document. The Rust models
    and the protocol text are hand-written.
  - The `openapi_coverage` test in `crates/pipedream-connect/tests/` and the
    `conformance` tests in `crates/pipedream-connect-types/tests/` compare the
    code with the OpenAPI document. They cover the models and the 68 Connect
    operations.
  - The `openapi_coverage` test also reads the operation tables of the
    protocol docs. No test reads their input sentences.
  - The OpenAPI document requires `start_ts` and `end_ts` for the usage list,
    `listUsages`. It gives that operation no cursor query and no `page_info`.
  - It requires `connect_token` in the account creation body,
    `createAccount`.
  - It requires the `external_user_id` and `account_id` query values for each
    proxy operation.
  - The code follows the OpenAPI document:
    - `ListUsageRequest` in
      `crates/pipedream-connect-types/src/admin/requests.rs` holds only
      `start_ts` and `end_ts`.
    - `CreateAccountRequest.connect_token` in
      `crates/pipedream-connect-types/src/accounts/models.rs` is not optional.
    - `ProxyRequest` in `crates/pipedream-connect/src/resources/proxy.rs`
      takes an `AccountRef`, which holds the external user ID and the account
      ID.
  - `tokens.md:41` also says that usage records accept "filters defined by the
    operation". The operation defines no filter.
- **Problem:** `tokens.md:43-44` says that usage lists use the common cursor
  contract "when Pipedream returns page information". `accounts.md:54-55`
  calls the Connect token of account creation optional. `proxy.md:36` says
  that a proxy request "MAY include an account ID".
- **Impact if no action:** Readers expect inputs that the SDK and the API do
  not support. The protocol docs then disagree with the OpenAPI document and
  the code.
- **Root question:** How do the protocol docs state only the inputs that the
  API and the SDK support?
- **Options:**
  - **A.** Correct the three sentences. Usage lists have no cursors, account
    creation requires a Connect token, and a proxy request requires an account
    ID. Effort: Small.
  - **B.** Add the inputs to the models. This sends inputs that the API does
    not define. Effort: Large.
  - **C.** Generate an operation reference from the OpenAPI document with an
    xtask command. Each entry gives the method, the path, and the required and
    optional inputs. The generated pages replace the hand-written input text.
    Effort: Large.
- **Left out:** Not recorded.
- **Wider fix:** C. Generated input lists cannot promise inputs that the
  OpenAPI document does not have.
- **Previous examples:** Commit `0163d0f` of 2026-10-21 corrected two
  sentences of this class in `docs/protocol/resources/catalog.md`. One
  promised cursor pagination for the app category list, which returns a bare
  array. The other said that an action run MAY accept a blocking flag.
- **Recommendation:** A now. C is worth a separate change if the user wants
  generated reference docs.
- **Action:** ask the user, because each option changes requirement sentences
  in a protocol doc, and no recorded decision covers them.
