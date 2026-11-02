# Workflow invocation stops on 2027-03-31

- **Severity:** Medium. **Effort:** Medium. **Confidence:** High.
  **Harder later:** Yes.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  post-merge follow-up work found the shutdown notice in the Pipedream docs.
  The user confirmed it on 2026-11-02.
- **Location:** `crates/pipedream-connect/src/resources/workflows.rs:22-31`,
  `crates/pipedream-connect/src/resources/admin_operations.rs:206-232`,
  `crates/pipedream-connect-types/src/admin/requests.rs:9-26`, and
  `docs/protocol/resources/proxy.md:109-138`.
- **Reason:** The workflow URLs work until 2027-03-31, and each option changes
  an API surface that the user chose, so the item waits for the user.

- **Source:** Implementation.
- **Category:** docs or spec.
- **Terms:**
  - **Workflow invocation:** the SDK call
    `WorkflowsApi::invoke_for_external_user` in `crates/pipedream-connect`. It
    sends a request to a Pipedream workflow URL for one external user.
  - **Pipedream Workflows:** the Pipedream product that runs hosted
    workflows. An HTTPS URL starts each workflow. It is not part of Connect.
  - **Project bearer:** the OAuth access token of the Connect project. It
    authorizes every Connect call of the project.
  - **First release:** the first crates.io release of the library crates. It
    waits until the error crate `internal-error` is on crates.io.
- **Background:**
  - Workflow invocation sends GET, POST, PUT, PATCH, or DELETE to an HTTPS
    workflow URL. It sends the project bearer, `X-PD-External-User-ID`, and
    `X-PD-Environment`. It returns every status, header, and body byte.
  - `docs/protocol/resources/proxy.md:109-138` states these rules. Lines 1-4
    and the table row at line 16 also name the call.
  - The call is not in the Pipedream OpenAPI document. The Connect docs page
    `https://pipedream.com/docs/connect/workflows` describes it.
  - On 2026-10-20 the user chose the API surface of the SDK: every Connect
    operation, the OAuth token, the rate-limit token, and workflow
    invocation. `docs/protocol/connect-resources.md:3-5` states it.
  - Workflow invocation is the only resource call that sends the project
    bearer away from the API origin. `docs/protocol/connect-client.md:17-18`
    states this exception.
  - On 2026-10-26 the user chose to let the call send the bearer to any HTTPS
    host, and to document the risk. The final review had graded that risk
    High. `docs/architecture.md:123-125` says that the host MUST pass only
    trusted workflow URLs.
  - The public request API can also reach any HTTPS host.
    `ApiPath::absolute` at `crates/pipedream-connect/src/request.rs:42-50`
    builds such a target, and `PipedreamClient::dispatcher()` sends it with
    the bearer. It shares the workflow URL rule and the error
    `InvalidRequest::WorkflowEndpoint`.
  - The page `https://pipedream.com/docs/workflows` says these facts. The
    user confirmed them on 2026-11-02:
    - Pipedream Workflows and String shut down on 2027-03-31. String is the
      Pipedream tool that builds workflows with AI.
    - Workflows are in maintenance mode, with no new signups and no new
      features.
    - Connect, the MCP servers, and the API proxy are not affected.
    - Pipedream deletes all Workflows data by 2027-04-30.
  - So each workflow URL stops on 2027-03-31, and the call then has no
    target.
  - Only workflow invocation uses these parts:
    - The trait `WorkflowsApi` and its handle in
      `crates/pipedream-connect/src/resources/workflows.rs`.
    - The `workflows()` accessors at
      `crates/pipedream-connect/src/resources/accessors.rs:24`, `:59`,
      `:137`, and `:218`.
    - `invoke_workflow` at
      `crates/pipedream-connect/src/resources/admin_operations.rs:206-232`.
    - `ApiPath::workflow_endpoint` and `ResponseMode::WorkflowPassThrough` in
      `crates/pipedream-connect/src/request.rs:52-59` and `:90-91`.
    - `InvalidRequest::WorkflowMethod` at
      `crates/pipedream-connect/src/error.rs:51`.
    - `WorkflowRequest` at
      `crates/pipedream-connect-types/src/admin/requests.rs:9-26`.
  - Tests of the call are in
    `crates/pipedream-connect/src/resources/_tests_/workflows_tests.rs`,
    `crates/pipedream-connect-types/src/_tests_/requests_tests.rs`, and
    `crates/pipedream-cli/src/report/_tests_/api/status_tests.rs`.
  - `README.md:51` and `crates/pipedream-connect/README.md` lines 20,
    180-181, 187-190, 212, 281, and 288 describe the call.
  - No CLI command calls workflow invocation.
    `crates/pipedream-cli/src/report/api.rs:166-174` only maps its two input
    errors to text.
  - The CLI option `triggers deploy --workflow-id` uses another route. It
    sends trigger events to a workflow through the Connect deploy route.
  - The deployed-trigger routes that send events to workflows are Connect
    operations of the OpenAPI document: `listDeployedTriggerWorkflows`,
    `updateDeployedTriggerWorkflows`, and the `workflow_id` of a trigger
    deploy. The resources protocol requires each Connect operation of that
    document.
  - No known host uses workflow invocation. The Firna adapter trait
    `ConnectorProvider` has no workflow method.
  - The SDK has no release yet, and the first release has no date.
- **Problem:** The SDK offers a public call whose targets stop on 2027-03-31.
  The protocol and the READMEs describe it as a working feature with no end
  date.
- **Impact if no action:** A host can adopt the call, and each call then
  fails after 2027-03-31. Each release that ships the call adds hosts that
  must migrate.
- **Root question:** What does the SDK do with workflow invocation before
  Pipedream Workflows stop on 2027-03-31?
- **Options:**
  - **A.** Document the end date now in the protocol section, the client
    README, and the rustdoc of `WorkflowsApi`. Remove the call in the first
    release after 2027-03-31. Hosts get no compiler warning, and a release
    before the date ships the call. The work now is Small, and the later
    removal is the work of option C. Effort: Medium.
  - **B.** Mark the call `#[deprecated]` in the first release, with the end
    date in the note. Remove it in the first release after the date. Hosts
    get a compiler warning, but the first release ships the call. Effort:
    Medium.
  - **C.** Remove workflow invocation before the first release. Delete each
    part that only the call uses, its tests, its protocol rules, and its
    README text. Keep `ApiPath::absolute`, which shares the URL rule and the
    error. No release ships the call. A host that still needs a workflow
    until the date sends the request itself. Effort: Medium.
  - **D.** Move workflow invocation behind an opt-in cargo feature, such as
    `workflows`. Remove the feature after the date. The default build has no
    workflow call, and a host can still opt in. The feature needs its own
    docs and tests, and the gate builds only with all features. Effort:
    Medium.
- **Left out:**
  - Change nothing. It fails the goal, because hosts get no warning and the
    docs keep a promise that ends.
  - A guard that rejects each call after 2027-03-31. It fails the goal,
    because it acts only after the date, and hosts still adopt the call
    before it.
  - Remove the deployed-trigger workflow routes too. It breaks a requirement
    that stays: the SDK covers each Connect operation of the OpenAPI
    document. `plans/follow-ups/openapi-fixture-refresh.md` removes them when
    Pipedream drops them from that document.
- **Wider fix:** None. Each option settles this call. The rate-limit token is
  the only other call outside the OpenAPI document, and it has no end date.
- **Previous examples:** None found.
- **Related:**
  - `plans/follow-ups/mcp-client-plan.md`: the same cause. That item records
    a design call of 2026-11-02 not to plan a legacy workflow REST API
    client. The Workflows shutdown is one of its reasons.
  - `plans/follow-ups/openapi-fixture-refresh.md`: the same cause. The
    shutdown can make Pipedream drop the deployed-trigger workflow routes
    from the OpenAPI document. This item leaves those routes to that refresh,
    as **Left out** says. The live document of 2026-11-02 still has them.
  - `plans/follow-ups/publish-crates-io.md`: work that waits for the answer.
    Options B and C act in or before the first release, so answer this item
    before that release.
- **Recommendation:** C. No known host uses the call and no release exists,
  so a removal now migrates nobody. It also ends the only resource call that
  sends the project bearer away from the API origin.
- **Action:** ask the user, because removing the call changes the API surface
  that the user chose on 2026-10-20 and deletes the workflow rules of
  `docs/protocol/resources/proxy.md`. No recorded user decision covers that
  change.
