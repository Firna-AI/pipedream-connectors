# MCP and legacy REST clients have no plan

- **Severity:** Low. **Effort:** Medium. **Confidence:** Medium.
  **Harder later:** No.
- **Kind:** Post-merge task.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  plan `plans/pipedream-connect-sdk.md` recorded it as a post-merge task.
- **Location:** `docs/protocol/connect-resources.md:3-20`, the scope of the
  SDK.
- **Reason:** Each client is new scope beyond the API surface that the user
  chose for this SDK on 2026-10-20, so each needs its own plan.

- **Terms:**
  - **MCP:** the Model Context Protocol. An AI agent uses an MCP server to
    list tools and call them.
  - **Pipedream MCP server:** a hosted MCP server that offers the actions of
    each Connect app as tools. It is part of Pipedream Connect.
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
    API. Its only routes outside Connect are the four legacy routes above.
  - The Pipedream docs said this on 2026-11-02:
    - Pipedream Workflows and String shut down on 2027-03-31. String is the
      Pipedream tool that builds workflows with AI.
    - Workflows get no new signups and no new features.
    - Pipedream deletes all Workflows data by 2027-04-30.
    - Connect, the MCP servers, and the API proxy do not change.
    - The docs index at `https://pipedream.com/docs/llms.txt` listed no
      reference for the legacy workflow REST API.
    - The page `https://pipedream.com/docs/rest-api` returned HTTP 404.
  - The shutdown also ends the targets of the SDK workflow invocation call.
    `plans/follow-ups/workflows-shutdown.md` holds that separate problem.
  - The Pipedream MCP server had these facts on 2026-11-02:
    - Its URL is `https://remote.mcp.pipedream.net/v3`. It supports the SSE
      and streamable HTTP transports.
    - It takes the same `Authorization: Bearer` token that the SDK gets with
      OAuth client credentials.
    - It requires the headers `x-pd-project-id`, `x-pd-environment`,
      `x-pd-external-user-id`, and `x-pd-app-slug`.
    - It also accepts `x-pd-account-id`, `x-pd-oauth-app-id`, and
      `x-pd-registry` with the value `public`, `private`, or `all`.
  - The Firna connectors plan has this follow-up: evaluate the Pipedream MCP
    server transport as another execution path, once Pipedream documents its
    tool-mode contract. So Firna is a possible consumer of an MCP client.
  - That Firna plan is `plans/pipedream-connectors.md` on the branch
    `calummoore/tel-aviv-v4` of the private repository `futex-ai/firna`.
  - No known consumer needs a legacy workflow REST API client.
- **Task:**
  1. Check the facts above against the current Pipedream docs.
  2. Ask the user two questions in one message, with the
     `decisions-and-findings` skill. Should this repository plan an MCP
     client? Should it plan a legacy workflow REST API client?
  3. Give the facts with each question. The workflow shutdown on 2027-03-31
     counts against a legacy REST client.
  4. Write a separate plan with the `plans` skill for each part that the user
     accepts. The plan updates the scope in
     `docs/protocol/connect-resources.md`.
- **Impact if no action:** The SDK stays limited to the Connect REST API. A
  host that wants the Pipedream MCP server builds its own MCP client and sets
  the Connect headers itself. It can take the bearer from the public SDK trait
  `AccessTokenProvider` in `crates/pipedream-connect/src/token.rs`.
- **Needs:** The answer of the user to each question. No credentials.
- **Done when:** The user answered both questions. Each part that the user
  accepted has its own committed plan under `plans/`.
- **Related:** `plans/follow-ups/workflows-shutdown.md`: the same cause. The
  Workflows shutdown on 2027-03-31 ends the SDK workflow invocation call, and
  it counts against a legacy workflow REST API client.
