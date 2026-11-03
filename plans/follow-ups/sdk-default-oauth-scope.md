# The SDK requests no OAuth scope by default

- **Severity:** Medium. **Likelihood:** Common. **Effort:** Small.
  **Confidence:** High. **Harder later:** Yes.
- **Kind:** Needs decision.
- **Added:** 2026-11-03, from branch `calummoore/pipedream-sdk-n-docs`. The
  first live runs of 2026-11-03 found the rejected bearer.
- **Location:** `crates/pipedream-connect/src/token.rs:80-87` and `:119`,
  `crates/pipedream-connect/src/config.rs:73`, `:100`, and `:113-117`,
  `crates/pipedream-connect/src/builder.rs:67-71`, and
  `docs/protocol/connect-client.md:44`.
- **Reason:** The CLI already requests `connect:*` and the client README
  names the scope for usage, but a default for every host needs a user
  decision.

- **Source:** Implementation.
- **Category:** product bug.
- **Terms:**
  - **Project bearer:** the OAuth access token that the client gets from
    `POST /v1/oauth/token` and sends on each authenticated call.
  - **Scope:** the optional `scope` field of that token request, a
    space-separated list of the permissions that the bearer gets.
  - **Connect route:** an API path under `/v1/connect/`, which every SDK
    resource call uses, except workflow invocation.
  - **Workflow invocation:** the SDK call
    `WorkflowsApi::invoke_for_external_user`, which sends the project bearer
    to a Pipedream workflow URL for one external user.
  - **Custom request:** an `ApiRequest` that the host sends with
    `PipedreamClient::dispatcher()`, which can use `ApiPath::V1` or
    `ApiPath::absolute` to leave `/v1/connect/`.
- **Background:**
  - The client sends `scope` in the token request only when the host sets
    one. The host sets it with `ClientBuilder::scope` at
    `crates/pipedream-connect/src/builder.rs:67-71`, or with
    `ClientConfig::with_scope` at
    `crates/pipedream-connect/src/config.rs:113-117`.
  - `ClientConfig::new` starts with no scope, at
    `crates/pipedream-connect/src/config.rs:100`. Then `TokenRequest` at
    `crates/pipedream-connect/src/token.rs:80-87` leaves out the field.
  - `docs/protocol/connect-client.md:44` says: "The request MAY contain a
    space-separated `scope`." No requirement names a default scope.
  - One client caches one project bearer for all its calls.
    `PipedreamClient::dispatch` at
    `crates/pipedream-connect/src/client.rs:98-102` gets it from one token
    provider. So one scope covers the Connect routes, the workflow
    invocations, and the custom requests of a client.
  - A host with a static bearer never calls the token endpoint. That host
    owns the scope of its bearer, and no option here changes it.
  - The vendored OpenAPI document is
    `crates/pipedream-connect-types/tests/fixtures/pipedream_openapi_swagger.json`.
    Its `CreateOAuthTokenOpts.scope` says: "Optional space-separated scopes
    for the access token. Defaults to `*`."
  - Its `OAuth2` security scheme lists 20 scopes. These four matter here:
    - `*`: "Full access to every OAuth-protected endpoint."
    - `connect:*`: "Full access to all Connect API endpoints (components,
      projects, triggers, accounts, etc.)."
    - `connect:usage:read`: "List Connect usage records for a time window."
    - `connect:workflow:invoke`: "Invoke Connect workflows on behalf of a
      user."
  - The document names a required scope for one operation only: `listUsers`
    needs `connect:users:read`. The usage operation `listUsages` names none.
  - On 2026-11-03, the live document had the same 20 scope names, the same
    default, and the same operation rules. Its URL is
    `https://pipedream.com/docs/pipedream_openapi_swagger.json`.
  - The first live runs of 2026-11-03 found these facts:
    - A token request with no scope got a bearer with the top-level scope
      `*`.
    - `GET /v1/connect/usage` then returned 403 with this body:

      ```json
      {"error":"Please use a \"connect:*\" scope instead of a top-level \"*\" scope"}
      ```

    - A bearer requested with the scope `connect:*` passed the usage, apps,
      accounts, and projects calls.
    - No run checked a `connect:*` bearer on workflow invocation, or on a
      custom request outside `/v1/connect/`.
  - The SDK sends `GET /v1/connect/usage` from `UsageApi::list` at
    `crates/pipedream-connect/src/resources/usage.rs:22`.
    `crates/pipedream-connect/src/failure.rs:77` maps a 403 to
    `Error::Forbidden`, which `crates/pipedream-connect/src/error.rs:91-95`
    defines.
  - That error keeps the Pipedream text in `message`. Its display text is
    only `[pipedream-connect/http] forbidden`. A host that logs only that
    text does not see which scope Pipedream asks for.
  - On 2026-11-03, the page `https://pipedream.com/docs/connect/workflows`
    named no scope. Its example requests a token with no scope, then invokes
    a workflow with the bearer.
  - Since 2026-11-03, the CLI requests `connect:*`. Every CLI command uses a
    Connect route, and no CLI command invokes a workflow. The CLI builds its
    client configuration in `crates/pipedream-cli/src/settings.rs`.
  - Since 2026-11-03, the client README `crates/pipedream-connect/README.md`
    tells hosts which scope usage needs.
  - Pipedream Workflows shut down on 2027-03-31, and each workflow URL then
    stops. `plans/follow-ups/workflows-shutdown.md` decides what the SDK does
    with workflow invocation before that date.
  - No known host uses workflow invocation. The SDK has no release yet, and
    the first release has no date.
  - The Connect token of `POST /v1/connect/{project_id}/tokens` is another
    token. Its default scope is already `connect:*`, as
    `docs/protocol/resources/tokens.md:23` says. This item does not change it.
- **Problem:** When the host sets no scope, Pipedream issues a `*` bearer. The
  usage route rejects that bearer with 403, so a client with no scope cannot
  list usage.
- **Impact if no action:** Each host that sets no scope gets
  `Error::Forbidden` when it lists usage. After the first release, a new
  default changes the bearer of every host that relies on `*`.
- **Root question:** Which scope does the SDK ask for when the host sets none?
- **Options:**
  - **A.** Keep no default, so Pipedream keeps issuing `*`. Copy the README
    note into the rustdoc of `ClientBuilder::scope` and
    `ClientConfig::with_scope`. Workflow invocation keeps the bearer that the
    Pipedream docs show. Each host that misses the note still gets 403 from
    usage. Effort: Small.
  - **B.** Send `connect:*` when the host sets no scope. A host can still set
    any scope, `*` too, and the default bearer grants less than `*`. Change
    the default, `docs/protocol/connect-client.md:44`, the client README, and
    the expected token body at
    `crates/pipedream-connect/tests/openapi_coverage/wire.rs:87`. Workflow
    invocation and custom requests outside `/v1/connect/` then send a
    `connect:*` bearer, which no run checked. If workflow invocation stays,
    check one live workflow call with the new default before the first
    release. Effort: Small.
  - **C.** Require a scope. The client rejects client credentials with no
    scope when it is built, as the builder rejects a missing project or
    environment. Each host then chooses, and the README says which scope
    fits which call. This adds a new rejection, which needs the approval of
    the user. Each existing client setup must add a scope. Effort: Small.
  - **D.** Answer `plans/follow-ups/workflows-shutdown.md` first, and change
    nothing until then. If that answer removes workflow invocation, apply B.
    If the call stays, make one live workflow call with a `connect:*` bearer,
    which needs credentials and a deployed workflow. Apply B when it passes,
    or E when it fails. Until then, a client with no scope cannot list usage.
    Effort: Small.
  - **E.** Give Connect routes a `connect:*` bearer. Give workflow invocation
    and custom requests outside `/v1/connect/` a second bearer with no scope,
    so they keep `*`. The token provider then caches two bearers, and the
    protocol needs rules for the second one. E keeps the `*` bearer on
    workflow URLs, which leave the API origin. Most of the extra code serves
    a call that ends on 2027-03-31. Effort: Medium.
- **Left out:**
  - A bearer for each operation with the narrowest scope, such as
    `connect:usage:read` for usage. It fails the goal, because the OpenAPI
    document names a scope for one operation only. Each other choice would
    be a guess.
  - A retry after this 403 with a new `connect:*` bearer. E is better on
    every point: it picks the right bearer before the first call, reads no
    error text, and sends no request twice.
  - Wait for Pipedream to accept `*` on usage. It fails the goal, because
    hosts fail now, and the error text asks for `connect:*`.
- **Wider fix:** E. With a bearer for each route kind, no route gets a default
  bearer that it rejects. B gives the same result on every Connect route.
- **Previous examples:** None found for a rejected scope. A related case has
  the same source: OpenAPI text that the live API contradicts. The document
  gives epoch milliseconds for six emitter dates, and a live response of
  2026-11-03 used epoch seconds. Commits `9adf13a` and `754b2ee` decode them
  as seconds, and `docs/protocol/connect-resources.md:54` records that
  decision.
- **Related:** `plans/follow-ups/workflows-shutdown.md`: the same code, and
  work that waits for the answer. Workflow invocation sends the bearer that
  this item scopes. If that item removes the call, E here loses most of its
  purpose, and B loses its unchecked workflow call. D waits for that answer.
- **Recommendation:** B. Every SDK resource call except workflow invocation
  uses a Connect route, and `connect:*` passed each route that the live runs
  tried. E best protects every route, but its second bearer mainly serves
  workflow invocation, which ends on 2027-03-31.
- **Action:** ask the user, because a default scope changes the token request
  of every host, and no requirement covers it.
  `docs/protocol/connect-client.md:44` only allows a scope. No run checked a
  `connect:*` bearer on workflow invocation, so no option is clearly best.
