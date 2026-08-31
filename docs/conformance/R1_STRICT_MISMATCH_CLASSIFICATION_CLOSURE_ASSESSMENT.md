# HACP 1.0.0 R1 Strict Mismatch Classification Closure Assessment

**Release target:** HACP 1.0.0
**Stage:** R1 — Strict mismatch classification
**Contract boundary:** HACP 1.0.0 §1.1 Variant A
**Assessment type:** Release-readiness / strict mismatch classification evidence
**Status:** R1 CLOSURE ASSESSMENT
**Historical strict baseline under assessment:** 15/38 PASS, 23/38 FAIL
**Known baseline property:** all 23 historical strict failures preserved the expected decision outcome and differed at the exact reason-code level

---

## 1. Executive preamble

### 1.1 Why this assessment exists

The HACP 1.0.0 release process deliberately separates two questions that must not be conflated:

```text
Does current HACP behavior preserve the release-critical decision-level contract?
```

and:

```text
Does every historical canonical vector currently reproduce its historical exact reason code?
```

Those questions overlap, but they are not equivalent.

The strict Protocol v1 re-certification work established a historical baseline of:

```text
15/38 PASS
23/38 FAIL
```

For all 23 failing vectors:

```text
expected decision outcome == observed decision outcome

but

expected exact reason code != observed exact reason code
```

The existence of 23 exact-reason mismatches therefore did not establish the existence of 23 production defects.

A mismatch could instead be caused by:

- malformed or incomplete canonical prerequisites;
- placeholder or dummy signatures;
- a vector that cannot reach the semantic boundary it claims to test;
- historical runner state that is not materialized by the current Protocol v1 adapter;
- a historical reason-code vocabulary that no longer has a current normative owner;
- legacy/current reason-code correspondence drift while the decision-level safety invariant remains enforced;
- an already-correct production branch hidden by an earlier canonical rejection gate;
- an explicit normative conflict that must remain on HOLD rather than being converted into an implementation change;
- or a genuine production violation of a current release-critical normative requirement.

The purpose of R1 was therefore not to increase the strict PASS counter mechanically.

The purpose was to determine which of these classes actually existed across the remaining strict surface and whether any unassessed class could materially invalidate the HACP 1.0.0 contract.

---

## 2. R1 objective

The controlling R1 objective was:

```text
reduce uncertainty in the remaining strict mismatch surface
and determine whether any unresolved mismatch is a real
HACP 1.0.0 blocker against the §1.1 release contract
```

R1 explicitly did **not** require:

```text
exact reason-code 38/38 before HACP 1.0.0
```

and did **not** authorize:

```text
mass vector repair
mass production changes
production changes for PASS-counter growth
activation of Enforcement revision 2
advertising the HC2 draft surface as the active 1.0 Enforcement contract
```

The release-critical question was narrower:

> Has the strict mismatch surface been classified deeply enough that no obvious unassessed semantic class remains which could materially invalidate the HACP 1.0.0 decision-level contract?

This assessment concludes that the answer is:

```text
YES
```

subject to the explicit HOLD and deferred work recorded below.

---

## 3. Why the R1 objective is considered achieved

The objective is considered achieved because the investigation no longer contains an unexamined mismatch family whose release impact is unknown.

The historical 23-failure surface has now been reduced to explicit dispositions.

Across that surface, the investigation established all of the following:

1. Canonical prerequisite failures were distinguished from production semantic failures.

2. Intended semantic boundaries were inspected for reachability rather than inferred from vector names.

3. Current normative owners were checked before production behavior was treated as defective.

4. Current sidecar evaluation order was inspected directly.

5. Existing executable production evidence was reused where available.

6. Historical reason-code lineage was distinguished from current standard reason-code ownership.

7. Production changes were prohibited until all of the following were established:

```text
normative requirement
+
valid prerequisites
+
intended-boundary reachability
+
observable production violation
+
focused executable RED
+
reproduced RED
```

8. Where that complete chain was established, production defects were fixed and fully regression-tested.

9. Where that chain was not established, no production change was made merely to satisfy a historical strict expectation.

10. No remaining semantic family currently establishes an unresolved HACP 1.0.0 production blocker.

The investigation therefore achieved the actual R1 goal:

```text
release-risk uncertainty was reduced to classified,
documented, bounded residual work
```

rather than:

```text
an unexplained list of 23 strict failures
```

---

## 4. HACP 1.0.0 contract boundary used by this assessment

This assessment evaluates blockers against the HACP 1.0.0 §1.1 Variant A contract.

The relevant release boundary is:

```text
HACP 1.0.0 =
  stable public HACP-Core contract
+ reproducible decision-level canonical conformance
+ Protocol v1 runner / strict verifier as tooling
+ honestly documented historical / exact-reason scope
+ sidecar as the current implementation of the active Enforcement profile
```

The following are not automatic HACP 1.0.0 requirements:

```text
Enforcement revision 2 active
exact reason-code 38/38
HC2-55 advertised as the active 1.0 Enforcement contract
```

Accordingly, an exact reason-code mismatch is release-blocking only when the evidence demonstrates that it materially violates the advertised 1.0 contract.

---

## 5. Initial strict baseline

The historical strict Protocol v1 baseline entering R1 was:

```text
15/38 PASS
23/38 FAIL
```

All 23 failures had:

```text
correct decision outcome
incorrect exact reason-code correspondence
```

Before the remaining-family inventory began, the following were already classified and closed:

- `CORE-INV1-005`
- `CORE-INV2-003`
- `CORE-INV2-004`
- `CORE-INV2-007`
- `CORE-INV2-008`

The following remained on explicit normative HOLD:

- `CORE-RUNTIME-005`

The remaining read-only family inventory therefore contained 17 strict mismatches.

Those 17 cases are the primary subject of this closure assessment.

---

## 6. Investigation method

### 6.1 Family-first classification

The remaining 17 cases were not treated as 17 independent bugs.

They were first grouped into semantic families:

- scope / boundary correspondence;
- action-hash binding;
- token-to-envelope binding;
- token expiry;
- token revocation;
- key revocation;
- envelope revocation;
- parent-envelope revocation inheritance;
- traceability / provenance integrity;
- missing provenance vocabulary;
- signature / verification precedence;
- autonomy budget;
- runtime action-hash correspondence.

The family grouping was intentionally provisional.

Families were split or collapsed only when evidence justified doing so.

---

### 6.2 Per-case questions

For each relevant case or representative family, the investigation attempted to establish:

1. exact historical expected outcome;
2. exact historical expected reason;
3. current normative owner;
4. canonical vector construction;
5. signature and key validity;
6. token validity where applicable;
7. temporal validity;
8. required runtime/control state;
9. intended semantic boundary;
10. whether the intended boundary was reachable;
11. first actual winning verification gate;
12. current production branch;
13. existing executable production evidence;
14. whether the case duplicated an already-proven representative;
15. whether the mismatch could plausibly affect the HACP 1.0.0 contract.

---

### 6.3 Production-change rule

The governing engineering rule throughout R1 was:

```text
NO PRODUCTION CHANGE
WITHOUT NORMATIVE BASIS
AND PROVEN RED
```

A historical canonical mismatch by itself was never sufficient authorization for a production change.

Production modification required:

```text
normative requirement established
+
valid prerequisites
+
intended semantic boundary reachable
+
current production violation observed
+
focused executable RED
+
identical RED reproduced
```

Only then was the smallest production correction permitted.

---

## 7. Major evidence findings

## 7.1 Canonical reachability was a major mismatch source

A large subset of historical strict vectors contained construction that prevented the intended semantic boundary from being reached.

Observed examples included:

- `"dummy"` envelope signatures;
- `"PLACEHOLDER"` signatures;
- placeholder `action_hash` values;
- placeholder provenance hashes and signatures;
- historical policy-context state not materialized by the current Protocol v1 runner.

This means that an observed result such as:

```text
INVALID_ENVELOPE
```

or:

```text
SIGNATURE_FAILURE
```

often described the first actual gate reached by the historical vector rather than the behavior of the semantic boundary named by the vector.

R1 therefore treated:

```text
historical strict output
```

and:

```text
valid-prerequisite production behavior
```

as separate evidence questions.

---

## 7.2 Protocol v1 runner state materialization is intentionally narrower than several historical vectors assume

The current conformance runner accepts raw fields including:

```text
policy_context
checkpoint
provenance_event
```

but current `policyContextJSON` materializes only fields such as:

```text
clock
human_required
human_required_verbs
consequence_class
risk_class
checkpoint
```

The runner does not materialize historical vector fields such as:

```text
current_action_count
revoked_keys
revoked_tokens
revoked_envelopes
```

into the evaluator's actual:

```text
BudgetLedger
RevocationStore
```

state.

Likewise, although `ProvenanceEvent` is present in `InputData`, the inspected evaluation wiring does not use the historical provenance controls to construct a provenance-verification state equivalent to the canonical traceability vectors.

This finding explains several strict mismatches as runner/vector reachability problems rather than production enforcement defects.

---

## 7.3 Current production contains explicit security gates hidden by malformed vectors

Direct inspection of the current evaluator established explicit branches for:

```text
KEY_REVOKED
TOKEN_REVOKED
ENVELOPE_REVOKED
TOKEN_EXPIRED
BUDGET_EXHAUSTED
TRACEABILITY_FAILURE
```

as well as:

```text
token ↔ envelope binding
action_hash binding
scope containment
```

Several of these branches also have independent integration evidence.

Therefore a historical vector failing before those gates does not establish that the corresponding production semantic is absent.

---

## 7.4 Historical reason-code vocabulary is not identical to the current normative vocabulary

Historical conformance archaeology established that some reason codes were intentional historical conformance semantics.

Examples include:

```text
HASH_MISMATCH
TOKEN_ENVELOPE_MISMATCH
TRACEABILITY_MISSING
```

They were not arbitrary typographical mistakes.

For example, historical conformance implementation lineage explicitly used:

```text
token.action_hash mismatch
→ HASH_MISMATCH

token.envelope_id mismatch
→ TOKEN_ENVELOPE_MISMATCH

omit_provenance
→ TRACEABILITY_MISSING
```

However, current authoritative reason-code ownership is different.

Current inspected normative/active behavior includes:

```text
action_hash mismatch
→ SIGNATURE_FAILURE
```

and the current error model maps required-provenance failure into:

```text
TRACEABILITY_FAILURE
```

The current sidecar uses:

```text
ENVELOPE_BINDING_FAILURE
```

for token-to-envelope binding failure.

This is therefore a legacy/current correspondence question rather than automatic proof of a production safety defect.

---

## 8. Scope family

## 8.1 `CORE-INV2-002` — audience boundary

Historical expectation:

```text
BOUNDARY_CROSSING
```

Historical actual:

```text
INVALID_ENVELOPE
```

The canonical vector contained invalid cryptographic prerequisites and did not provide a reliable observation of the intended audience boundary.

Independent current runner-level evidence exists:

```text
TestConformanceRunnerReportsBoundaryCrossingForAudienceViolation
```

The representative constructs a valid Ed25519-signed envelope with:

```text
granted audience: internal
proposed audience: external
```

and verifies:

```text
DENY / BOUNDARY_CROSSING
```

Disposition:

```text
DUPLICATE / COVERED BY REPRESENTATIVE
+
VECTOR CONSTRUCTION / REACHABILITY
```

Production defect:

```text
NO
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 8.2 `CORE-INV2-005` — quantity ceiling

Historical expectation:

```text
SCOPE_EXCEEDED
```

Historical actual:

```text
INVALID_ENVELOPE
```

Normative requirement:

```text
proposed quantity > granted max_quantity
→ DENY / SCOPE_EXCEEDED
```

The canonical vector was not sufficient to establish production behavior because its cryptographic prerequisites failed earlier.

Independent production inspection then discovered a separate and real omission.

`ScopeGrant` already contained:

```text
MaxQuantity *int
```

but `ParseProposedActionAttributes()` did not extract:

```text
quantity
```

and `DefaultScopeGuard.CheckBoundary()` did not enforce the quantity ceiling.

A valid-prerequisite runner-level RED was constructed.

Observed before fix, twice:

```text
quantity_above_granted_ceiling

expected:
DENY / SCOPE_EXCEEDED

actual:
ALLOW
```

The RED was reproduced without production modification.

Production defect:

```text
ESTABLISHED
```

A minimal production correction was then authorized.

The correction:

- added optional quantity extraction;
- enforced `MaxQuantity`;
- returned `UNKNOWN_ATTRIBUTE` when a configured quantity restriction has no observable proposed quantity;
- returned `SCOPE_EXCEEDED` when the proposed quantity exceeds the granted ceiling;
- preserved the established earlier boundary-precedence ordering.

Focused GREEN:

```text
PASS
```

Near regression:

```text
go test ./internal/scope ./internal/evaluate ./cmd/hacp-conformance-runner
PASS
```

Full regression:

```text
go test ./...
PASS
```

Signed sidecar commit:

```text
b6b9e98
fix: enforce quantity and destination scope boundaries
```

Disposition:

```text
PRODUCTION DEFECT
FIXED
CLOSED
```

HACP 1.0.0 blocker after fix:

```text
NO
```

---

## 8.3 `CORE-INV2-006` — destination allowlist

Historical expectation:

```text
BOUNDARY_CROSSING
```

Historical actual:

```text
SIGNATURE_FAILURE
```

Normative requirement:

```text
proposed destination outside granted destination allowlist
→ DENY / BOUNDARY_CROSSING
```

As with quantity, canonical cryptographic construction shadowed the intended boundary.

Independent inspection discovered that:

```text
ScopeGrant.Destinations
```

already existed, but:

```text
ProposedAction.destination
```

was not extracted into boundary attributes and no destination allowlist enforcement existed in `CheckBoundary()`.

A valid-prerequisite runner-level RED was constructed.

Observed before fix, twice:

```text
destination_outside_allowlist

expected:
DENY / BOUNDARY_CROSSING

actual:
ALLOW
```

Production defect:

```text
ESTABLISHED
```

The same minimal structural fix as quantity:

- added optional destination extraction;
- enforced configured destination allowlists;
- returned `UNKNOWN_ATTRIBUTE` when the policy configures destinations but the proposed action provides no destination;
- returned `BOUNDARY_CROSSING` for an observed destination outside the allowlist.

Focused GREEN, near regression, and full `go test ./...` all passed.

Signed sidecar commit:

```text
b6b9e98
fix: enforce quantity and destination scope boundaries
```

Disposition:

```text
PRODUCTION DEFECT
FIXED
CLOSED
```

HACP 1.0.0 blocker after fix:

```text
NO
```

---

## 9. Action-hash family

Relevant cases:

- `CORE-INV3-002`
- `CORE-INV5-007`
- `CORE-RUNTIME-003`

Historical expected reason:

```text
HASH_MISMATCH
```

Historical archaeology established that `HASH_MISMATCH` was intentional historical conformance vocabulary.

It appeared in early conformance vectors and was later explicitly described in the v0.9.2 cross-language baseline lineage.

The historical semantic was:

```text
valid token
bound to original ProposedAction
presented ProposedAction changed afterward
→ HASH_MISMATCH
```

However, current inspected behavior differs.

Current sidecar computes:

```text
SHA-256(canonical ProposedAction)
```

and compares it with:

```text
DecisionToken.action_hash
```

A mismatch is denied as:

```text
SIGNATURE_FAILURE
```

The active Enforcement lineage likewise maps token action-hash mismatch to `SIGNATURE_FAILURE`.

Current root normative ownership for `HASH_MISMATCH` was not established in the inspected current:

- HACP specification;
- error model;
- active Enforcement profile;
- current standard reason vocabulary.

Therefore the evidence establishes:

```text
historical action-hash enforcement:
PRESENT

current action-hash enforcement:
PRESENT

historical exact reason:
HASH_MISMATCH

current exact reason:
SIGNATURE_FAILURE
```

This is a:

```text
LEGACY / CURRENT EXACT-REASON CORRESPONDENCE SPLIT
```

not an established decision-level production failure.

`CORE-INV5-007` and `CORE-RUNTIME-003` additionally contain placeholder construction that prevents clean historical observation of their intended boundaries.

Disposition for the family:

```text
HISTORICAL / CURRENT REASON-CODE CORRESPONDENCE
+
NO CURRENT PRODUCTION DEFECT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

Deferred target:

```text
1.0.n exact-reason / normative correspondence hardening
```

---

## 10. Token-to-envelope binding

### `CORE-INV3-003`

Historical expected reason:

```text
TOKEN_ENVELOPE_MISMATCH
```

Historical actual:

```text
SIGNATURE_FAILURE
```

The canonical token and envelope signatures are placeholders, so the intended token-to-envelope binding boundary is shadowed.

Current production explicitly checks:

```text
tok.EnvelopeID != env.EnvelopeID
```

and denies using:

```text
ENVELOPE_BINDING_FAILURE
```

Historical archaeology established that:

```text
TOKEN_ENVELOPE_MISMATCH
```

was intentional historical conformance vocabulary.

However, a current authoritative owner requiring that exact historical reason was not established.

The security semantic itself is enforced:

```text
token for envelope A
used with envelope B
→ DENY
```

Disposition:

```text
VECTOR REACHABILITY
+
LEGACY / CURRENT EXACT-REASON VOCABULARY SPLIT
+
CURRENT BINDING ENFORCEMENT PRESENT
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 11. Token expiry

### `CORE-INV3-004`

Historical expected reason:

```text
TOKEN_EXPIRED
```

Historical actual:

```text
INVALID_ENVELOPE
```

The canonical vector contains invalid/placeholder prerequisites before the token-expiry gate.

Current evaluator order includes:

```text
token key resolution
→ token key revocation
→ token signature verification
→ token revocation
→ token expiry
```

and explicitly returns:

```text
TOKEN_EXPIRED
```

when:

```text
now > token.expires_at + permitted skew
```

No independent current executable test specifically proving the valid signed token-expiry path was found during the R1 inventory.

This absence is recorded as an executable-coverage limitation.

It is not treated as evidence of a defect because the current production branch is explicit and no valid-prerequisite violation was observed.

Disposition:

```text
VECTOR CONSTRUCTION / REACHABILITY
+
CURRENT PRODUCTION SEMANTIC PRESENT
+
NO INDEPENDENT EXPIRY TEST FOUND
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 12. Revocation family

## 12.1 `CORE-INV4-002` — token revocation

Historical expected reason:

```text
TOKEN_REVOKED
```

Historical actual:

```text
INVALID_ENVELOPE
```

The canonical Protocol v1 vector cannot materialize its historical:

```text
policy_context.revoked_tokens
```

into the evaluator's real revocation store.

Independent production integration evidence exists.

The distributed control-plane integration test establishes:

```text
same valid signed authority
→ accepted before revocation
→ distributed token revoke
→ same authority evaluated again
→ DENY / TOKEN_REVOKED
```

Disposition:

```text
RUNNER / VECTOR STATE REACHABILITY
+
ALREADY-CONFORMING PRODUCTION
```

Production defect:

```text
NO
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 12.2 `CORE-INV5-008` — revoked signer key

Historical expected reason:

```text
KEY_REVOKED
```

Historical actual:

```text
SIGNATURE_FAILURE
```

The crypto profile establishes an important precedence rule:

```text
key resolution
→ key revocation
→ only then signature verification
```

A revoked signer key must therefore produce:

```text
KEY_REVOKED
```

before signature verification.

Current production follows that order for both envelope and token signer keys.

Existing integration evidence uses a real generated Ed25519 key, valid signed envelope, revoked signer state, and obtains:

```text
DENY / KEY_REVOKED
```

A separate bad-signature case obtains:

```text
SIGNATURE_FAILURE
```

Disposition:

```text
ALREADY-CONFORMING PRODUCTION
+
HISTORICAL VECTOR / REACHABILITY ISSUE
```

Production defect:

```text
NO
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 12.3 `CORE-INV7-005` — direct envelope revocation

Historical expected reason:

```text
ENVELOPE_REVOKED
```

Historical actual:

```text
INVALID_ENVELOPE
```

The canonical vector attempts to supply revocation state through:

```text
policy_context.revoked_envelopes
```

The current runner does not materialize that field into the evaluator's `RevocationStore`.

Current production nevertheless explicitly checks:

```text
IsEnvelopeRevoked(env.EnvelopeID)
```

after successful envelope signature verification and returns:

```text
ENVELOPE_REVOKED
```

No independent current executable test specifically covering direct envelope revocation was found during R1.

Disposition:

```text
RUNNER / VECTOR STATE REACHABILITY
+
CURRENT PRODUCTION SEMANTIC PRESENT
+
NO INDEPENDENT DIRECT-REVOCATION TEST FOUND
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 12.4 `CORE-INV7-006` — parent-envelope revocation inheritance

Historical expected reason:

```text
ENVELOPE_REVOKED
```

Historical actual:

```text
SIGNATURE_FAILURE
```

The historical vector asserts:

```text
revoked parent envelope
→ child inherits revocation
```

Current control-plane infrastructure recognizes:

```text
REVOCATION_KIND_PARENT_ENVELOPE
```

but local materialization stores the event subject as an envelope revocation ID.

The evaluator itself checks:

```text
IsEnvelopeRevoked(env.EnvelopeID)
```

for the currently evaluated envelope.

A search of current normative HACP-Core sources found:

```text
parent_envelope_id
→ OPTIONAL; delegation chain
```

but did not establish a normative rule requiring:

```text
parent revoked
→ child automatically revoked
```

or requiring the exact reason:

```text
ENVELOPE_REVOKED
```

for such inheritance.

Therefore the production-defect authorization chain fails at the normative-owner step.

Disposition:

```text
HISTORICAL VECTOR / CONFORMANCE EXPECTATION
WITHOUT ESTABLISHED CURRENT NORMATIVE OWNER
```

Production RED:

```text
NOT AUTHORIZED
```

Production change:

```text
NOT AUTHORIZED
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

Deferred target:

```text
future normative clarification if parent-revocation inheritance
is intended to become a stable requirement
```

---

## 13. Traceability family

## 13.1 `CORE-INV4-003` — provenance payload integrity

Historical expectation:

```text
TRACEABILITY_FAILURE
```

Historical actual:

```text
SIGNATURE_FAILURE
```

The canonical vector contains placeholder envelope/provenance cryptographic material.

The current Protocol v1 runner does not translate the historical provenance-event controls into a production-equivalent configured provenance verification state.

Current production maps provenance acceptance failure to:

```text
TRACEABILITY_FAILURE
```

Disposition:

```text
VECTOR / RUNNER REACHABILITY
+
CURRENT TRACEABILITY_FAILURE SEMANTIC PRESENT
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 13.2 `CORE-INV4-005` — provenance chain linkage integrity

Historical expectation:

```text
TRACEABILITY_FAILURE
```

Historical actual:

```text
SIGNATURE_FAILURE
```

The vector tests a different provenance integrity mechanism from `CORE-INV4-003`, but both belong to the same high-level current reason family:

```text
TRACEABILITY_FAILURE
```

The historical vector again contains placeholder prerequisite material and runner provenance-state reachability limitations.

For R1 release-risk classification, `CORE-INV4-003` and `CORE-INV4-005` therefore belong to one traceability-integrity family even though their detailed failure mechanisms differ.

Disposition:

```text
VECTOR / RUNNER REACHABILITY
+
DUPLICATE HIGH-LEVEL TRACEABILITY FAILURE FAMILY
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 13.3 `CORE-INV4-004` — missing provenance

Historical expected reason:

```text
TRACEABILITY_MISSING
```

Historical actual:

```text
SIGNATURE_FAILURE
```

Historical archaeology established that:

```text
TRACEABILITY_MISSING
```

was intentionally used by historical conformance implementation lineage.

Current error-model semantics instead include required provenance-event absence under:

```text
TRACEABILITY_FAILURE
```

The current Protocol v1 runner also does not materialize the vector's:

```text
omit_provenance
```

control into an equivalent production provenance state.

Disposition:

```text
RUNNER / VECTOR REACHABILITY
+
LEGACY / CURRENT EXACT-REASON VOCABULARY SPLIT
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

Deferred target:

```text
1.0.n exact-reason / normative correspondence
```

---

## 14. Signature / expiry precedence

### `CORE-INV5-002`

Historical expected reason:

```text
SIGNATURE_FAILURE
```

Historical actual:

```text
ENVELOPE_EXPIRED
```

Current evaluator ordering for envelopes is:

```text
resolve key
→ key revocation
→ envelope signature
→ envelope revocation
→ envelope expiry
```

Current token ordering similarly places token signature before token expiry.

Therefore current production structure gives invalid signatures precedence over later expiry checks.

This is consistent with the current authenticated-claims model.

The repository also contains focused DecisionToken authentication-precedence evidence showing that a modified token signature is rejected before signed decision semantics are trusted.

The historical vector mismatch therefore does not establish a current production precedence defect.

Disposition:

```text
CURRENT VERIFICATION ORDER STRUCTURALLY CONFORMING
+
HISTORICAL STRICT CORRESPONDENCE ISSUE
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 15. Autonomy budget

### `CORE-INV7-002`

Historical expected reason:

```text
BUDGET_EXHAUSTED
```

Historical actual:

```text
INVALID_ENVELOPE
```

The vector supplies:

```text
autonomy_budget.max_actions = 5
policy_context.current_action_count = 5
```

Current production does not use `policy_context.current_action_count`.

It uses the runtime:

```text
BudgetLedger.ConsumeAutonomy(...)
```

mechanism.

The current Protocol v1 runner does not materialize the historical action-count field into the production budget ledger.

Current production explicitly returns:

```text
BUDGET_EXHAUSTED
```

when envelope autonomy budget consumption fails.

Disposition:

```text
RUNNER / VECTOR STATE REACHABILITY
+
CURRENT PRODUCTION BUDGET SEMANTIC PRESENT
```

Production defect:

```text
NOT ESTABLISHED
```

HACP 1.0.0 blocker:

```text
NO
```

---

## 16. Complete remaining-17 disposition matrix

| Test ID | Historical expected reason | Historical actual | Final R1 disposition | Production defect? | 1.0.0 blocker? |
|---|---|---|---|---|---|
| `CORE-INV2-002` | `BOUNDARY_CROSSING` | `INVALID_ENVELOPE` | Vector reachability + covered by valid audience representative | No | No |
| `CORE-INV2-005` | `SCOPE_EXCEEDED` | `INVALID_ENVELOPE` | Real quantity-enforcement defect found by independent RED; fixed in `b6b9e98` | **Yes, fixed** | No after fix |
| `CORE-INV2-006` | `BOUNDARY_CROSSING` | `SIGNATURE_FAILURE` | Real destination-enforcement defect found by independent RED; fixed in `b6b9e98` | **Yes, fixed** | No after fix |
| `CORE-INV3-002` | `HASH_MISMATCH` | `ENVELOPE_EXPIRED` | Historical/current exact-reason correspondence split; current action-hash enforcement present | No current defect established | No |
| `CORE-INV3-003` | `TOKEN_ENVELOPE_MISMATCH` | `SIGNATURE_FAILURE` | Vector reachability + legacy/current binding-reason vocabulary split | No current defect established | No |
| `CORE-INV3-004` | `TOKEN_EXPIRED` | `INVALID_ENVELOPE` | Vector reachability; current explicit `TOKEN_EXPIRED` branch present | No defect established | No |
| `CORE-INV4-002` | `TOKEN_REVOKED` | `INVALID_ENVELOPE` | Runner state reachability; production independently proves `TOKEN_REVOKED` | No | No |
| `CORE-INV4-003` | `TRACEABILITY_FAILURE` | `SIGNATURE_FAILURE` | Provenance/vector reachability; current traceability-failure family present | No defect established | No |
| `CORE-INV4-004` | `TRACEABILITY_MISSING` | `SIGNATURE_FAILURE` | Runner reachability + legacy/current reason-vocabulary split | No current defect established | No |
| `CORE-INV4-005` | `TRACEABILITY_FAILURE` | `SIGNATURE_FAILURE` | Provenance/vector reachability; same high-level integrity family as `INV4-003` | No defect established | No |
| `CORE-INV5-002` | `SIGNATURE_FAILURE` | `ENVELOPE_EXPIRED` | Current signature-before-expiry order is structurally present | No current defect established | No |
| `CORE-INV5-007` | `HASH_MISMATCH` | `SIGNATURE_FAILURE` | Placeholder reachability + hash legacy/current correspondence | No current defect established | No |
| `CORE-INV5-008` | `KEY_REVOKED` | `SIGNATURE_FAILURE` | Current key-revocation-before-signature semantics independently proven | No | No |
| `CORE-INV7-002` | `BUDGET_EXHAUSTED` | `INVALID_ENVELOPE` | Runner cannot materialize historical action count; production budget semantic present | No defect established | No |
| `CORE-INV7-005` | `ENVELOPE_REVOKED` | `INVALID_ENVELOPE` | Runner revocation-state reachability; current direct envelope-revocation branch present | No defect established | No |
| `CORE-INV7-006` | `ENVELOPE_REVOKED` | `SIGNATURE_FAILURE` | Historical parent-inheritance expectation lacks established current normative owner | No defect established | No |
| `CORE-RUNTIME-003` | `HASH_MISMATCH` | `SIGNATURE_FAILURE` | Placeholder reachability + hash legacy/current correspondence | No current defect established | No |

---

## 17. Previously classified strict mismatches

The 17-case family inventory supplements the earlier R1 classifications.

Previously closed:

```text
CORE-INV1-005
CORE-INV2-003
CORE-INV2-004
CORE-INV2-007
CORE-INV2-008
```

Previously placed on HOLD:

```text
CORE-RUNTIME-005
```

The combined result is that the full historical 23-failure strict surface now has an explicit release disposition.

---

## 18. Production defects established by R1

Across the strict mismatch classification work, six production defects were established through the required evidence chain and corrected.

They are:

```text
CORE-INV2-003
CORE-INV2-004
CORE-INV2-005
CORE-INV2-006
CORE-INV2-007
CORE-INV2-008
```

The latest pair:

```text
CORE-INV2-005
CORE-INV2-006
```

was fixed in:

```text
hacp-sidecar b6b9e98
fix: enforce quantity and destination scope boundaries
```

For that change:

```text
normative owner:
ESTABLISHED

valid boundary reachability:
ESTABLISHED

RED run #1:
REPRODUCED

RED run #2:
REPRODUCED

pre-fix observed result:
ALLOW

required results:
quantity    → DENY / SCOPE_EXCEEDED
destination → DENY / BOUNDARY_CROSSING

focused GREEN:
PASS

near regression:
PASS

full go test ./...:
PASS

signed commit:
GOOD
```

No unrelated production work was included.

---

## 19. Normative HOLD

### `CORE-RUNTIME-005`

Historical strict mismatch:

```text
expected:
HUMAN_RESOLUTION_REQUIRED

actual:
SELF_APPROVAL_DENIED
```

Evidence remains normatively inconsistent across:

- canonical expectation;
- later documentation wording;
- current production behavior;
- historical normative lineage.

Current classification remains:

```text
NORMATIVE CONFLICT
HOLD
PRODUCTION RED NOT ESTABLISHED
NO PRODUCTION CHANGE
```

R1 does not attempt to resolve this conflict by choosing one historical source opportunistically.

The item must be adjudicated through independent normative ownership and migration-history work.

Under the HACP 1.0.0 Variant A release boundary, this HOLD does not currently establish a release blocker because:

- the decision-level behavior remains deny/fail-closed;
- no release-critical production violation has been established;
- exact reason-code 38/38 is not itself the 1.0.0 contract;
- the conflict is explicitly documented and deferred rather than hidden.

Deferred target:

```text
1.0.n / later normative adjudication
```

---

## 20. R1 aggregate result

The historical strict surface entering R1 was:

```text
23 exact reason-code mismatches
```

The final R1 result is:

```text
23 / 23 strict failures now have an explicit release disposition

6 production defects established and fixed

1 normative conflict remains explicitly on HOLD

16 other cases do not establish a new production defect
under the current evidence
```

The 16 non-production-defect cases include overlapping classes such as:

- canonical/vector construction defects;
- intended-boundary reachability defects;
- Protocol v1 runner state-materialization limitations;
- already-conforming production behavior;
- duplicate/representative coverage;
- historical/current exact-reason correspondence splits;
- historical expectations without an established current normative owner;
- explicit current production branches for which no release-critical violation was observed.

These categories are not treated as mutually exclusive accounting buckets.

The important release property is:

```text
no historical strict mismatch remains semantically unexamined
at the family level
```

and:

```text
no unresolved production defect currently established by R1
remains unfixed
```

---

## 21. Release-blocker assessment

R1 applies the release-blocker rule against §1.1 Variant A.

An item may block HACP 1.0.0 when evidence establishes, for example:

- a release-critical normative contradiction that makes advertised behavior ambiguous;
- a production violation of an established release-critical normative requirement;
- unsafe behavior;
- broken reproducible canonical decision-level verification;
- a release claim materially stronger than available evidence;
- an unresolved defect making the advertised contract incorrect.

Conversely, an item normally does not block HACP 1.0.0 merely because it is:

- an exact reason-code mismatch with correct decision outcome;
- a vector reachability defect;
- legacy/current reason-code correspondence work;
- deeper hardening suitable for 1.0.n;
- an explicitly bounded normative HOLD that does not invalidate the advertised 1.0 decision-level contract.

After the R1 family inventory:

```text
UNRESOLVED HACP 1.0.0 PRODUCTION BLOCKERS
ESTABLISHED BY STRICT-MISMATCH EVIDENCE:

NONE
```

This conclusion does not claim that every historical exact reason code has been reconciled.

It claims that the remaining exact-reason surface has been classified deeply enough to determine its release impact.

---

## 22. R1 exit-criteria mapping

### Criterion 1

> Release-relevant mismatches have explicit classification against §1.1.

Result:

```text
SATISFIED
```

All 23 historical strict failures now have an explicit disposition, including the 17-case family inventory and the previously classified/HOLD cases.

---

### Criterion 2

> No unknown strict mismatch obviously remains as an unassessed release blocker.

Result:

```text
SATISFIED
```

The remaining surface was examined by semantic family rather than only by individual vector ID.

No obvious unassessed family remains.

---

### Criterion 3

> `CORE-RUNTIME-005` is adjudicated or remains explicitly HOLD/non-blocking.

Result:

```text
SATISFIED FOR R1
```

The item remains explicitly:

```text
HOLD
NORMATIVE CONFLICT
NO PRODUCTION RED
NO PRODUCTION CHANGE
```

Its release impact is bounded and documented.

---

### Criterion 4

> Blocker ledger exists.

Result:

```text
SATISFIED
```

Artifact:

```text
docs/conformance/HACP_1_0_RELEASE_BLOCKER_LEDGER.md
```

The ledger should be updated to reflect this closure assessment.

---

### Criterion 5

> Each unresolved item has a future target.

Result:

```text
SATISFIED
```

Residual classes naturally map to:

```text
1.0.n exact-reason correspondence
vector reachability cleanup
additional executable coverage
normative conflict resolution
future parent-envelope semantics clarification if required
```

---

### Criterion 6

> No mass production change is required before 1.0.0.

Result:

```text
SATISFIED
```

The only newly established production defects from the 17-case inventory were the quantity and destination boundary omissions.

They were corrected by one narrow, regression-clean sidecar change.

No remaining family currently authorizes another production change.

---

## 23. R1 closure conclusion

The R1 strict mismatch classification stage achieved its intended purpose.

It began with:

```text
23 historical exact reason-code failures
```

whose release meaning was unknown.

It ends with:

```text
a classified semantic surface
```

in which:

- real production defects were separated from malformed canonical observations;
- genuine defects were fixed only after normative ownership and reproduced RED;
- already-conforming behavior was not modified to satisfy broken vector reachability;
- historical reason-code vocabulary was not silently rewritten into current normative semantics;
- unresolved normative conflict was preserved as HOLD rather than guessed through;
- no remaining obvious semantic family establishes an unresolved HACP 1.0.0 production blocker.

Therefore the R1 exit question:

```text
Has strict mismatch uncertainty been reduced enough that
no obvious unassessed mismatch class can materially invalidate
the HACP 1.0.0 §1.1 decision-level contract?
```

is answered:

```text
YES
```

Recommended stage status:

```text
R1 — COMPLETE
```

Recommended next release stage:

```text
R2 — Normative freeze review
```

---

## 24. What this conclusion does not claim

R1 closure must not be misrepresented as any of the following:

```text
exact reason-code 38/38 achieved
```

```text
all historical vectors repaired
```

```text
all legacy/current reason-code correspondence adjudicated
```

```text
CORE-RUNTIME-005 resolved
```

```text
Enforcement revision 2 activated
```

```text
HC2-55 is now the advertised HACP 1.0 Enforcement contract
```

```text
all possible post-1.0 semantic hardening completed
```

None of those statements is required for this R1 exit decision.

---

## 25. Deferred 1.0.n / later work

The following classes are appropriate for structured post-1.0 follow-up:

### Exact reason-code correspondence

Including:

```text
HASH_MISMATCH
TOKEN_ENVELOPE_MISMATCH
TRACEABILITY_MISSING
ENVELOPE_BINDING_FAILURE correspondence
```

where historical and current vocabularies differ.

### Vector reachability cleanup

Replace placeholder/dummy prerequisites only under a separately defined vector-maintenance scope.

Do not mutate historical evidence casually.

### Protocol v1 adapter observability

Potential future improvements may include clearer handling or explicit documentation of historical inputs such as:

```text
current_action_count
revoked_*
provenance_event controls
```

if those inputs are intended to remain part of the supported conformance interface.

### Additional executable production evidence

Optional coverage improvements may include focused tests for:

```text
TOKEN_EXPIRED
direct ENVELOPE_REVOKED
```

These are coverage-hardening opportunities, not currently established 1.0.0 blockers.

### Normative conflict resolution

`CORE-RUNTIME-005` remains a dedicated normative adjudication item.

### Parent-envelope semantics

If parent-envelope revocation inheritance is intended to become a stable requirement, it requires an explicit normative owner before production behavior is changed.

---

## 26. Evidence anchors

### Release planning

```text
HACP_1.0.0_ENGINEERING_RELEASE_PLAN_final.md
```

### Initial remaining-family inventory

```text
HACP_1.0.0_R1_REMAINING_17_STRICT_MISMATCH_INVENTORY.md
```

### Strict re-certification status

```text
docs/conformance/STRICT_REASON_CODE_RECERTIFICATION_STATUS.md
```

### Release blocker ledger

```text
docs/conformance/HACP_1_0_RELEASE_BLOCKER_LEDGER.md
```

### Current normative / semantic sources inspected during R1

```text
HACP-SPEC-0.9-draft.md
INVARIANTS.md
boundary-matrix.md
error-model.md
wire/crypto-profile.md
profiles/enforcement.md
profiles/enforcement-v2-draft.md
```

### Verification-order evidence

```text
docs/conformance/ENFORCEMENT_VERIFICATION_ORDER_NORMATIVE_ASSESSMENT.md
docs/conformance/ENFORCEMENT_VERIFICATION_PRECEDENCE_VERIFICATION.md
```

### Current production surfaces inspected

```text
hacp-sidecar/internal/evaluate/pipeline.go
hacp-sidecar/internal/evaluate/scope.go
hacp-sidecar/internal/scope/action.go
hacp-sidecar/internal/evaluate/revocation.go
hacp-sidecar/internal/controlplane/adapter.go
hacp-sidecar/internal/controlplane/subscriber.go
hacp-sidecar/cmd/hacp-conformance-runner/main.go
```

### Representative executable evidence

```text
TestConformanceRunnerReportsBoundaryCrossingForAudienceViolation
TestConformanceRunnerEnforcesQuantityAndDestinationBoundaries
TestAtomicTrustStorePipelineRevokedSignerFailsClosed
distributed token revocation pipeline integration tests
TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision
```

### Latest production correction from the remaining-17 inventory

```text
hacp-sidecar
b6b9e98
fix: enforce quantity and destination scope boundaries
```

Verification:

```text
focused quantity/destination RED:
REPRODUCED TWICE

focused post-fix GREEN:
PASS

near regression:
PASS

full:
go test ./...
PASS

commit signature:
GOOD
```

---

## 27. Final R1 statement

```text
R1 RESULT:
COMPLETE

HISTORICAL STRICT FAILURE SURFACE:
23 / 23 dispositioned

UNRESOLVED PRODUCTION DEFECTS ESTABLISHED BY R1:
0

NEWLY DISCOVERED DEFECTS FROM REMAINING-17 INVENTORY:
2

NEWLY DISCOVERED DEFECTS FIXED:
2 / 2

NORMATIVE HOLD:
CORE-RUNTIME-005

UNASSESSED OBVIOUS RELEASE-RELEVANT SEMANTIC FAMILIES:
NONE IDENTIFIED

HACP 1.0.0 STRICT-MISMATCH BLOCKER:
NONE ESTABLISHED

NEXT:
R2 — NORMATIVE FREEZE REVIEW
```
