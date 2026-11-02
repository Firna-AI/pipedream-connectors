# Accounts protocol names the AccountRef type

- **Severity:** Low. **Effort:** Small. **Confidence:** High.
  **Harder later:** No.
- **Kind:** Outside the scope.
- **Added:** 2026-11-02, from branch `calummoore/pipedream-rust-sdk-plan`. The
  docs implementor of the connection webhook trust rule found it.
- **Location:** `docs/protocol/resources/accounts.md:59-60`.
- **Reason:** The sentence was there before the change that found it, so a
  separate docs branch fixes it.

- **Source:** Implementation.
- **Category:** docs or spec.
- **Terms:**
  - **Protocol doc:** a file under `docs/protocol/` that states the SDK
    requirements as MUST and MAY sentences. It must not name source files,
    functions, or types.
  - **AccountRef:** a Rust type in
    `crates/pipedream-connect-types/src/accounts/models.rs`. It pairs an
    external user ID with an account ID.
  - **Owned account call:** an account retrieval or deletion that names the
    external user who must own the account.
- **Background:**
  - `docs/protocol/resources/accounts.md` states the account rules of the
    Connect SDK.
  - Lines 59-60 say: "Account retrieval and deletion send `external_user_id`
    when the operation uses an `AccountRef` ownership check."
  - The next sentences use the term "owned account call". One says that such
    a call MUST return `NotFound` when the account belongs to another
    external user or has no `external_id`.
  - `AccountRef` is an SDK type. The client methods `AccountsApi::get_owned`
    and `AccountsApi::delete_owned` in
    `crates/pipedream-connect/src/resources/accounts.rs` take it.
- **Problem:** The requirement names an SDK type instead of the behaviour at
  the boundary. The term "owned account call" has no definition in the
  protocol doc.
- **Impact if no action:** A reader who does not know the Rust code cannot
  tell when a call sends `external_user_id`. The sentence goes stale when the
  type changes its name.
- **Root question:** How does the accounts protocol say when an account call
  sends `external_user_id`, without naming code?
- **Options:**
  - **A.** Reword lines 59-60 to state the behaviour and define the term.
    Example: "Account retrieval and deletion MAY name the external user who
    must own the account. Such an owned account call sends
    `external_user_id`." The client README keeps the type names.
    Effort: Small.
  - **B.** Change nothing. Rust readers can find the type. The protocol doc
    then keeps a sentence that breaks the protocol doc rule. Effort: Small.
- **Left out:** Moving the ownership rules to the client README. It fails the
  goal, because the `NotFound` rules are requirements at the boundary, and a
  requirement lives in the protocol doc. Only two approaches are real,
  because the sentence either states the behaviour or keeps the type name.
- **Wider fix:** None. The documentation skill already forbids code names in
  protocol docs.
- **Previous examples:** None found.
- **Recommendation:** A. It keeps the requirement and removes the code name
  with a two-line change.
- **Action:** fix after merge, apply A, because the sentence was there before
  this branch and is outside its review scope.
