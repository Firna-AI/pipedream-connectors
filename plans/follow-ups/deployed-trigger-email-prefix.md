# SDK rejects the IDs of email triggers

- **Severity:** Medium. **Likelihood:** Common. **Effort:** Small.
  **Confidence:** High. **Harder later:** No.
- **Kind:** Needs decision.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-sdk-j-docs`. A
  check of the emitter date item, `plans/follow-ups/emitter-date-unit.md`,
  found the email trigger examples on 2026-11-02.
- **Location:** `crates/pipedream-connect-types/src/ids.rs:110-117`,
  `docs/protocol/connect-resources.md:80-108`,
  `crates/pipedream-connect-types/src/triggers/emitters.rs:110-151`,
  `crates/pipedream-cli/src/report/input.rs:146`,
  `crates/pipedream-cli/src/commands/triggers/args.rs:45`,
  `crates/pipedream-cli/README.md:174-175`.
- **Reason:** Each fix changes the deployed trigger ID rule of the protocol
  doc, and only a user decision can change that rule.

- **Source:** Implementation.
- **Category:** product bug.
- **Terms:**
  - **Host:** an application that uses the SDK to run Pipedream Connect for
    its own end users.
  - **Emitter:** one entry of a deployed triggers list or a deploy response,
    with the SDK type `Emitter` in
    `crates/pipedream-connect-types/src/triggers/emitters.rs`.
  - **Email interface:** the emitter of an email trigger, with a unique email
    address that emits an event for each email, and an ID that starts with
    `ei_`.
  - **Trigger ID call:** one of the SDK methods that take a deployed trigger
    ID of the type `DeployedTriggerId`, such as `get` or `delete`.
  - **Common identifier grammar:** the path-safe rule that `common_identifier`
    in `ids.rs` checks: 1 to 250 visible ASCII bytes, without `/`, `\`, `?`,
    or `#`, and not `.` or `..`.
- **Background:**
  - `DeployedTriggerId::new` in `crates/pipedream-connect-types/src/ids.rs`
    accepts an ID only when it starts with `dc_`, `hi_`, or `ti_`, and then
    has one or more ASCII letters or digits. JSON decoding uses the same rule.
  - The "Identifiers" section of `docs/protocol/connect-resources.md` gives
    the same three prefixes for a deployed trigger ID. Its decision says that
    the SDK adds no prefix rule beyond the eight listed prefixes without a
    user decision.
  - That decision enforces the eight prefixes because the OpenAPI document
    specifies them. The three trigger prefixes come from the `id` patterns of
    the schemas `DeployedComponent`, `HttpInterface`, and `TimerInterface`.
  - In the OpenAPI document, the `trigger_id` path parameter of each deployed
    trigger operation has the schema `{"type": "string"}`, with no pattern.
  - The OpenAPI enum `EmitterType` has the values `email`, `http`, `source`,
    and `timer`. The SDK has the same enum as a list filter. A list with the
    filter `email` asks for the email triggers of one end user.
  - The vendored OpenAPI copy,
    `crates/pipedream-connect-types/tests/fixtures/pipedream_openapi_swagger.json`,
    has no `ei_` value and no email interface schema. The live OpenAPI
    document of 2026-11-02 has neither. In both, the `Emitter` schema lists
    only the other three kinds.
  - The Pipedream Connect page "Deploying Triggers" deploys the trigger
    `email-new-email`. Its example response is the email interface
    `ei_QaJTb0`, with the `email_address` `xxxxxxxxxx@upload.pipedream.net`.
    Its `created_at` and `updated_at` are both 1744499847.
  - The classic REST API docs of February 2024 show the email interface
    `ei_QbGT3D`, with the `email_address`
    `em5tdwgfgbw9piv@upload.pipedream.net`. It is in the response of "Create
    a Workflow", beside `dc_`, `hi_`, and `ti_` emitters.
  - Neither example has a `type` field. The Connect page shows no `type`
    field on any emitter. So no known source gives the `type` value of an
    email interface.
  - `Emitter` decodes by its `type` field. A value without a known `type`
    decodes as `Emitter::Other` and keeps its raw JSON. So a list or a deploy
    response keeps an email interface and re-encodes it unchanged.
  - A probe on 2026-11-02 ran the types crate:
    - `DeployedTriggerId::new("ei_QaJTb0")` returned
      `InvalidIdentifier { kind: DeployedTrigger }`, with the text
      `[pipedream-connect-types/ids] invalid deployed trigger identifier`.
    - JSON decoding of the same string failed with the same text.
    - The example email interface decoded as `Emitter::Other`, with and
      without `"type": "EmailInterface"`.
  - The trait `DeployedTriggersApi` in
    `crates/pipedream-connect/src/resources/deployed_triggers.rs` has eleven
    methods. Only `list` takes no trigger ID.
  - On 2026-11-02 the SDK had ten trigger ID calls: `get`, `update`,
    `delete`, `list_events`, `get_workflows`, `set_workflows`,
    `list_webhooks`, `set_webhooks`, `get_webhook`, and
    `regenerate_webhook_signing_key`.
  - The OpenAPI refresh in `plans/follow-ups/openapi-fixture-refresh.md`
    adds an eleventh trigger ID call. It sends the new operation
    `resetDeployedTriggerWebhookDeliveryState`, whose path holds a deployed
    trigger ID.
  - The CLI uses the same type. `triggers get`, `triggers delete`, and
    `triggers events` exit with code 2 for an `ei_` ID. They print
    `error: <TRIGGER> is not a valid deployed trigger ID.` and the hint
    `Use an ID that starts with dc_, hi_, or ti_ and then has 1 to 247 ASCII
    letters or digits.`
  - The CLI has copies of the prefix list in
    `crates/pipedream-cli/src/report/input.rs:146`,
    `crates/pipedream-cli/src/commands/triggers/args.rs:45`, and
    `crates/pipedream-cli/README.md:174-175`.
  - These tests check the prefix list:
    - `crates/pipedream-connect-types/src/_tests_/ids_tests.rs:110-112`.
    - `crates/pipedream-cli/src/report/_tests_/input/hint_tests.rs:27-32`
      and `:104-106`.
    - `crates/pipedream-cli/src/commands/triggers/_tests_/deployed/read_tests.rs:156-166`.
  - `PipedreamClient::dispatcher`, `ApiRequest`, and `ApiPath` in
    `crates/pipedream-connect` are public. A host can send a request with an
    `ei_` ID through them. The host must then build the path and decode the
    response itself.
  - Every option keeps two requirements of
    `docs/protocol/connect-resources.md`. The client validates each ID before
    it requests a bearer token. Each ID keeps the common identifier grammar,
    because it enters a URL path.
- **Problem:** Pipedream docs show email trigger IDs that start with `ei_`.
  The SDK rejects that prefix before it sends a request. So a host cannot
  read, change, or delete an email trigger, or manage its events, workflows,
  or webhooks, through the SDK.
- **Impact if no action:** A host can deploy and list an email trigger, but it
  cannot remove the trigger through the SDK. Each host that offers email
  triggers must build each trigger ID call itself, and the CLI cannot send
  them.
- **Root question:** How does the SDK accept the ID of every deployed trigger
  that Pipedream creates, while each ID stays safe in a URL path?
- **Options:**
  - **A.** Add `ei_` as a fourth deployed trigger prefix. Change the
    validator, the protocol table and decision, the CLI hint, the argument
    help, the CLI README, and the tests. The `ei_` prefix rests on docs
    examples only, not on an OpenAPI pattern. The next new emitter kind breaks
    the trigger ID calls again. Effort: Small.
  - **B.** Accept a deployed trigger ID in the shape that the four known
    trigger prefixes share: lowercase letters, an underscore, then ASCII
    letters or digits. A new prefix in the same shape also passes. The CLI
    still rejects a value such as `trigger-1` before a request. No Pipedream
    source states this shape, so B adds a rule that no requirement covers.
    Effort: Small.
  - **C.** Accept any deployed trigger ID with the common identifier grammar,
    as the SDK does for app override IDs and webhook IDs. Keep `dc_`, `hi_`,
    `ti_`, and `ei_` in the protocol doc as known values, not as a rule.
    Change the CLI hint to the common hint, and update the help, the README,
    and the tests. A wrong ID, such as an account ID, then reaches Pipedream
    and gets `NotFound`, as a mistyped `dc_` ID does today. A delete with
    `missing_ok` then returns success for it. Effort: Small.
  - **D.** Ask Pipedream to add the email interface schema and its `ei_`
    pattern to the OpenAPI document. Then refresh the OpenAPI copy, and add
    `ei_` as A does, from OpenAPI evidence. Email triggers stay unusable
    through the SDK until Pipedream changes its document, at an unknown date.
    The request is public, so the user approves it first. Effort: Small.
  - **E.** Do A, and add a typed `EmailInterface` emitter with `id`,
    `email_address`, `created_at`, `updated_at`, `type`, and unknown fields.
    Add the variant `Emitter::EmailInterface` and its arm in `check_trigger`,
    in `crates/pipedream-connect/src/resources/trigger_operations.rs`. No
    source gives the `type` value, and the emitter date unit is still open,
    so a live run with Pipedream credentials must confirm both. The new
    variant breaks each host `match` on `Emitter` without a wildcard arm,
    because the enum is not `#[non_exhaustive]`. Hosts that read email
    interfaces from `Emitter::Other` also stop getting them there.
    Effort: Medium.
- **Left out:**
  - Change nothing, and let each host send these requests with `ApiRequest`
    and `PipedreamClient::dispatcher`. It fails the goal, because the trigger
    ID calls still reject the ID, and the CLI has no such path.
  - A free-form ID grammar. It breaks a requirement that stays, because these
    IDs enter URL paths.
  - One ID type for each emitter kind. It still needs the `ei_` prefix, as A
    does, and it breaks every caller of `DeployedTriggerId`. A gives the same
    result for the goal at a lower cost, and the `Emitter` variant already
    names the kind.
  - A constructor that skips the prefix check only for an ID from a response.
    It fails the goal, because a host that stores the ID and builds it again
    with `DeployedTriggerId::new` still gets the error.
  - A test that compares the prefix list with the OpenAPI copy. It fails the
    goal, because the OpenAPI copy has no email interface.
- **Wider fix:** C. Without a prefix list, no new emitter kind can meet this
  error again. B prevents the class only for IDs in the shape that it guesses.
- **Previous examples:** Commit `a4c09f1` of 2026-10-22 fixed the same class.
  `DeployedTriggerId` then accepted only `dc_`, so the SDK rejected the IDs of
  HTTP and timer interfaces. The commit added `hi_` and `ti_` from the OpenAPI
  response patterns.
- **Related:**
  - `plans/follow-ups/emitter-date-unit.md`, same code. Both items change the
    emitter model in `emitters.rs`. The date unit that it picks also applies
    to the two dates of the typed model in E.
  - `plans/follow-ups/openapi-fixture-refresh.md`, same code. The OpenAPI copy
    lacks the email interface, and so does the live document of 2026-11-02.
    So that refresh does not fix this item. D needs one more refresh after
    Pipedream adds the schema. The refresh also adds the call for
    `resetDeployedTriggerWebhookDeliveryState`, which takes a deployed
    trigger ID. So the rule that this item picks applies to that call too.
- **Recommendation:** C. It accepts every ID that Pipedream creates, as the
  OpenAPI path parameter does, and it keeps each ID safe in a URL path.
- **Action:** ask the user, because each option adds a prefix or ends the
  prefix rule in `docs/protocol/connect-resources.md`, and no recorded
  decision covers it.

The email trigger examples come from
<https://pipedream.com/docs/connect/components/triggers>, read on 2026-11-02,
and from
<https://github.com/PipedreamHQ/pipedream/blob/3319f49c20485af0e39408369d50a9ef4d3c4673/docs-v2/pages/rest-api/index.mdx>.
