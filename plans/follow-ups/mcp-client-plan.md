# SDK has no decision on an MCP client

- **Severity:** Low. **Effort:** Small. **Confidence:** Medium.
  **Harder later:** No.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  plan `plans/pipedream-connect-sdk.md` recorded it as a post-merge task with
  two questions. Finding 82 of round 2 of the branch review split the
  questions, and a design call settled the second one.
- **Location:** `docs/protocol/connect-resources.md:3-20`, the scope of the
  SDK.
- **Reason:** An MCP client adds scope beyond the API surface that the user
  chose on 2026-10-20, and on 2026-11-02 the user moved work that is not
  urgent to follow-up items.

- **Source:** Design.
- **Category:** docs or spec.
- **Terms:**
  - **MCP:** the Model Context Protocol. An AI agent uses an MCP server to
    list tools and call them.
  - **Pipedream MCP server:** a hosted MCP server that offers the actions of
    each Connect app as tools. It is part of Pipedream Connect.
  - **Host:** an application that uses the SDK to run Pipedream Connect for
    its own end users.
  - **Firna:** the private product in the repository `futex-ai/firna`. It
    uses Pipedream Connect for its connectors.
  - **Legacy workflow REST API:** the Pipedream REST API for the account's
    own workflows, sources, subscriptions, and events. It is not part of
    Connect.
- **Background:**
  - The SDK covers the Connect routes under `/v1/connect/*`, the OAuth token
    route `POST /v1/oauth/token`, the custom rate-limit token, and workflow
    invocation for external users.
  - `docs/protocol/connect-resources.md:3-20` states this scope.
  - On 2026-10-20 the user chose this API surface. The user rejected the four
    legacy routes `/v1/apps`, `/v1/apps/{app_id}`, `/v1/app_categories`, and
    `/v1/app_categories/{id}`, because they duplicate Connect routes.
  - From its start on 2026-10-20, the SDK left out an MCP server transport
    and the legacy workflow REST API on purpose.
  - The Pipedream OpenAPI document has no route of the legacy workflow REST
    API. Its routes outside `/v1/connect/*` are the four legacy routes above
    and `POST /v1/oauth/token`.
  - The SDK already runs a Connect action over REST. `ActionsApi::run` in
    `crates/pipedream-connect/src/resources/actions.rs` sends the operation
    `runAction`, `POST /v1/connect/{project_id}/actions/run`.
  - The public SDK trait `AccessTokenProvider` in
    `crates/pipedream-connect/src/token.rs` gives the project bearer.
  - The page `https://pipedream.com/docs/connect/mcp/developers` gave these
    facts on 2026-11-02:
    - The URL of the Pipedream MCP server is
      `https://remote.mcp.pipedream.net/v3`. It supports the SSE and
      streamable HTTP transports.
    - It takes the same `Authorization: Bearer` token that the SDK gets with
      OAuth client credentials.
    - It requires the headers `x-pd-project-id`, `x-pd-environment`,
      `x-pd-external-user-id`, and `x-pd-app-slug`.
    - It also accepts `x-pd-account-id`, `x-pd-oauth-app-id`, and
      `x-pd-registry` with the value `public`, `private`, or `all`.
    - The page had no section on tool modes.
  - The Pipedream docs said on 2026-11-02 that Pipedream Workflows shut down
    on 2027-03-31, and that Connect, the MCP servers, and the API proxy do
    not change.
  - The Firna connectors plan is `plans/pipedream-connectors.md` on the
    branch `calummoore/tel-aviv-v4` of `futex-ai/firna`. On 2026-11-02:
    - Its section "Non-Goals (V1)" lists the MCP server transport.
    - Its post-merge follow-up says: "Evaluate Pipedream's MCP server
      transport as an alternative execution path once its tool-mode contract
      is documented."
    - So Firna is a possible consumer of an MCP client, after its first
      version. No other known host needs one.
  - A second scope question asked whether the SDK gets a legacy workflow
    REST API client. A design call of 2026-11-02 settled it: do not plan a
    legacy REST client. An agent made the call, because the facts below make
    that answer clearly best. The user can reverse it. These are its reasons:
    - Pipedream Workflows end on 2027-03-31, and Pipedream deletes all
      Workflows data by 2027-04-30.
    - On 2026-11-02 the REST reference page
      `https://pipedream.com/docs/rest-api` returned HTTP 404. The docs index
      `https://pipedream.com/docs/llms.txt` listed no reference for the
      legacy workflow REST API.
    - No known consumer needs the client.
    - The call keeps the API surface that the user chose on 2026-10-20.
- **Problem:** No recorded decision says whether the SDK gets an MCP client.
  Firna plans to evaluate the Pipedream MCP server as another execution path,
  and the SDK gives that path no support.
- **Impact if no action:** A host that wants the Pipedream MCP server builds
  its own MCP client and sets the Connect headers itself. Each such host
  repeats that code, and nobody plans for the Firna evaluation.
- **Root question:** How does a host run Connect actions through the
  Pipedream MCP server?
- **Options:**
  - **A.** Keep the scope of the SDK. A host uses a general MCP client
    library. It takes the bearer from `AccessTokenProvider` and sets the
    `x-pd-*` headers itself. Add one sentence to the scope in
    `docs/protocol/connect-resources.md` that says that the SDK has no MCP
    client. Each host repeats the header code. Effort: Small.
  - **B.** Plan a small helper in `pipedream-connect`. It gives the MCP
    server URL, the bearer, and the `x-pd-*` headers for one external user
    and one app. A host passes them to any MCP client library. The SDK adds
    no MCP protocol code and no new dependency. It adds public API, a
    protocol section, and tests for a path that no host uses yet.
    Effort: Medium.
  - **C.** Plan an MCP client crate, such as `pipedream-connect-mcp`. It
    connects over streamable HTTP, lists the tools of an app, and calls them.
    It adds a crate, an MCP library dependency, and a protocol doc. Pipedream
    has not documented the tool-mode contract that Firna waits for, so the
    tool models can change. Effort: Large.
- **Left out:**
  - An MCP server in the SDK that offers Connect actions as tools through
    `ActionsApi::run`. It fails the goal, because the goal is the Pipedream
    MCP server. It also builds again a server that Pipedream hosts.
  - Keep this item open until the Firna evaluation ends. It is not a distinct
    approach. The SDK stays as A says until then, and a later plan can still
    add B or C.
  - A legacy workflow REST API client. The design call above settled it. It
    does not answer the root question.
- **Wider fix:** None. The item is a scope choice, not a defect class.
- **Previous examples:** None found.
- **Related:** `plans/follow-ups/workflows-shutdown.md`, the same cause. The
  Workflows shutdown on 2027-03-31 ends the SDK workflow invocation call of
  that item. It is also a reason of the design call against a legacy workflow
  REST API client. The MCP server stays after the shutdown.
- **Recommendation:** A. No known host needs an MCP client yet, and Pipedream
  has not documented the tool-mode contract that Firna waits for.
- **Action:** ask the user, because the best option depends on whether Firna
  adopts the Pipedream MCP server as an execution path, and only the user
  knows that product plan.
