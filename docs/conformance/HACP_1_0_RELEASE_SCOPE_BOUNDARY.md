# HACP 1.0.0 Release Scope Boundary

**Document:** `docs/conformance/HACP_1_0_RELEASE_SCOPE_BOUNDARY.md`
**Release target:** HACP 1.0.0
**Contract boundary:** §1.1 Variant A
**Stage:** R3 — Release-Scope Boundary
**Repository:** `hacp-spec`
**R1 status:** COMPLETE
**R2 status:** COMPLETE
**Scope status:** FROZEN SUBJECT TO EXPLICIT R4 PRE-TAG ALIGNMENT
**Production changes authorized by this document:** NONE

---

## 1. Purpose

This document freezes the HACP 1.0.0 release scope after completion of:

```text
R1 — Strict mismatch classification
R2 — Normative freeze review
```

Its purpose is to prevent release scope from expanding implicitly during the remaining release-preparation stages.

Every known release-relevant item is classified into one of four categories:

```text
A. MUST — release-critical contract
B. MUST BEFORE TAG — release/version/publication alignment
C. DEFERRED — 1.0.n / later
D. OUT OF HACP 1.0.0 VARIANT A
```

This document does not introduce new protocol semantics.

It records the release boundary established by the engineering release plan and the signed R1/R2 closure evidence.

---

## 2. Governing HACP 1.0.0 Contract

The controlling release boundary is Variant A:

```text
HACP 1.0.0 =
  stable public HACP-Core contract
+ reproducible decision-level canonical conformance
+ Protocol v1 runner / strict verifier as tooling
+ honestly documented historical / exact-reason scope
+ sidecar as current implementation of the active Enforcement profile
```

The following are not implied by HACP 1.0.0:

```text
HACP 1.0.0 ≠ Enforcement revision 2 active
HACP 1.0.0 ≠ exact reason-code 38/38
HACP 1.0.0 ≠ HC2-55 advertised as active Enforcement
```

The governing engineering rule remains:

```text
NO PRODUCTION CHANGE
WITHOUT NORMATIVE BASIS
AND PROVEN RED
```

---

## 3. Entry Evidence

### 3.1 R1

R1 established:

```text
Historical strict mismatches:
23

Dispositioned:
23 / 23

Established production defects:
6

Established production defects fixed:
6 / 6

Normative HOLD:
CORE-RUNTIME-005

Unresolved production defects established by R1:
0

Unresolved HACP 1.0.0 blockers established by R1:
0
```

R1 also established that exact reason-code 38/38 is not itself the HACP 1.0.0 Variant A contract.

Primary evidence:

```text
docs/conformance/R1_STRICT_MISMATCH_CLASSIFICATION_CLOSURE_ASSESSMENT.md
docs/conformance/HACP_1_0_RELEASE_BLOCKER_LEDGER.md
```

### 3.2 R2

R2 established:

```text
UNRESOLVED R2 NORMATIVE BLOCKERS:
NONE

PRODUCTION CHANGES:
0

SCHEMA CHANGES:
0

VECTOR CHANGES:
0

RUNNER CHANGES:
0

ENFORCEMENT REVISION 2:
DRAFT — NOT ACTIVE

HC2:
NOT PROMOTED TO THE ACTIVE HACP 1.0 ENFORCEMENT CONTRACT

EXACT REASON-CODE 38/38:
NOT CLAIMED
```

Primary evidence:

```text
docs/conformance/HACP_1_0_NORMATIVE_FREEZE_ASSESSMENT.md
```

---

## 4. Classification Model

### Category A — MUST

An item belongs in Category A when the Variant A release contract materially depends on it.

If such an item is unresolved at release time, HACP 1.0.0 MUST NOT be tagged.

### Category B — MUST BEFORE TAG

An item belongs in Category B when the underlying semantics are already sufficiently established for Variant A, but final release/version/publication alignment remains necessary.

These are not open R1 or R2 semantic blockers.

They become release blockers if left unresolved at the actual HACP 1.0.0 tag/publication boundary.

### Category C — DEFERRED

An item belongs in Category C when it is known, explicitly bounded, and not required to make the Variant A release claim correct.

These items MUST NOT be silently represented as completed.

### Category D — OUT OF SCOPE

An item belongs in Category D when it is intentionally excluded from the HACP 1.0.0 Variant A contract.

Such work MUST NOT be pulled into HACP 1.0.0 merely because it exists, is partially implemented, has executable evidence, or appears technically mature.

---

## 5. Category A — Release-Critical Contract

### A1. Stable public HACP-Core normative contract

State:

```text
SATISFIED
```

Evidence:

```text
R2 — Normative Freeze Review
COMPLETE
UNRESOLVED R2 NORMATIVE BLOCKERS: NONE
```

### A2. Reproducible canonical decision-level conformance

State:

```text
SATISFIED
```

Established baseline:

```text
Canonical HACP-Core vectors:
38 / 38 PASS
```

This is decision-level canonical conformance, not strict exact-reason 38/38.

### A3. Protocol v1 runner / strict verifier tooling

State:

```text
SATISFIED
```

Protocol v1 runner verification and strict reason-code verification remain part of the verification tooling surface.

### A4. Historical strict mismatch surface classified

State:

```text
SATISFIED
```

R1 result:

```text
23 / 23 dispositioned
```

### A5. Established release-critical production defects fixed

State:

```text
SATISFIED
```

R1 result:

```text
6 production defects established
6 / 6 fixed
```

Unresolved production defects established by R1:

```text
0
```

### A6. No unresolved release-critical normative contradiction

State:

```text
SATISFIED
```

R2 result:

```text
UNRESOLVED HACP 1.0.0 NORMATIVE BLOCKERS:
NONE
```

### A7. Active Enforcement claim remains bounded

The sidecar MAY represent the current implementation of the current HACP-Enforcement profile only within established evidence.

It MUST NOT imply:

```text
Enforcement revision 2 active
HC2-55 active HACP 1.0 Enforcement
complete active revision-2 conformance suite
```

State:

```text
SATISFIED
```

### A8. Honest exact-reason scope

The HACP 1.0.0 release MUST preserve the distinction between:

```text
canonical decision-level 38/38
```

and:

```text
strict exact-reason 38/38
```

State:

```text
SATISFIED
```

The latter is not claimed.

---

## 6. Category B — Must Be Completed Before the HACP 1.0.0 Tag

These are release-alignment obligations and are handed to R4.

### B1. Public specification version labels

Current release-facing normative files still contain development-era labels including:

```text
Version: 0.9.3
Status: Draft
Status: Draft for public review
```

Observed in release-facing files including:

```text
HACP-SPEC-0.9-draft.md
canonicalization.md
error-model.md
versioning.md
PROFILES.md
profiles/enforcement.md
README.md
```

Disposition:

```text
MUST BE ALIGNED BEFORE HACP 1.0.0 TAG
OWNER: R4
```

This does not imply a change to the HACP signed-object wire identifier.

### B2. Claim-string alignment

Current public examples include:

```text
HACP 0.9-Core
HACP 0.9-Runtime
HACP 0.9-Enforcement
```

Disposition:

```text
MUST BE REVIEWED AND ALIGNED FOR THE FINAL 1.0.0 RELEASE CLAIM SURFACE
OWNER: R4
```

Any change must preserve the distinction between specification release version and `hacp_version` wire/object version.

### B3. README release/version/status metadata

`README.md` currently contains:

```text
Version: 0.9.3
Phase 1–3 complete · Phase 4 Gates A–E closed
```

and multiple historical/canonical `0.9.2` references.

Disposition:

```text
MUST BE REVIEWED BEFORE TAG
OWNER: R4
```

Not every `0.9.2` occurrence is automatically stale.

The following may represent historical or canonical conformance-suite identity rather than the release version:

```text
HACP Conformance Harness v0.9.2
HACP-Core v0.9.2
spec_version: 0.9.2
vector_set: core-0.9.2
canonical HACP-Core v0.9.2 vector suite
```

R4 MUST classify those references individually before changing them.

### B4. Phase / Gate terminology where release-facing

Current public documents contain development-era wording such as:

```text
Phase 4
Gate D
Gate E
Phase 4 MVP
Normative for Gate D
```

Disposition:

```text
REVIEW REQUIRED BEFORE TAG
OWNER: R4 / R5 AS APPROPRIATE
```

Historical engineering-gate statements may remain when they accurately describe project history.

Release-facing wording MUST NOT create ambiguity about current profile status, revision lifecycle, release status, or conformance status.

### B5. Manifest, package, and release metadata consistency

Before the HACP 1.0.0 tag, all release-facing machine-readable and package/repository metadata must be classified as one of:

```text
release version metadata
historical suite identity
wire/object version
tool version
implementation version
```

Disposition:

```text
MUST BE VERIFIED BEFORE TAG
OWNER: R4 / later release-hygiene stages
```

No automatic wire/object version migration is authorized.

---

## 7. Category C — Deferred to 1.0.n / Later

### C1. Exact reason-code correspondence hardening

Residual historical/current reason-code correspondence work where no current release-critical production defect was established.

Disposition:

```text
DEFERRED
TARGET: 1.0.n / later
```

### C2. Canonical vector reachability cleanup

Historical vectors may contain:

```text
dummy signatures
placeholder signatures
placeholder action hashes
placeholder provenance hashes/signatures
invalid or incomplete prerequisites
```

Disposition:

```text
DEFERRED
TARGET: separately scoped vector-maintenance work
```

Historical evidence MUST NOT be casually rewritten merely to increase a PASS counter.

### C3. Additional executable strict coverage

Disposition:

```text
DEFERRED
TARGET: 1.0.n / later hardening
```

### C4. Protocol v1 adapter observability for historical inputs

Disposition:

```text
DEFERRED
TARGET: tooling / coverage hardening
```

### C5. `CORE-RUNTIME-005`

Current status:

```text
NORMATIVE CONFLICT
HOLD
PRODUCTION RED NOT ESTABLISHED
NO PRODUCTION CHANGE
```

Historical mismatch:

```text
expected:
HUMAN_RESOLUTION_REQUIRED

current production:
SELF_APPROVAL_DENIED
```

Disposition:

```text
DEFERRED
TARGET: 1.0.n / later normative adjudication
```

### C6. Parent-envelope revocation inheritance

No current authoritative normative owner for a stable parent-envelope revocation-inheritance requirement was established.

Disposition:

```text
DEFERRED
```

If such inheritance is intended to become stable, explicit normative ownership is required before production behavior changes.

### C7. General post-1.0 semantic hardening

Disposition:

```text
DEFERRED
```

---

## 8. Category D — Explicitly Outside HACP 1.0.0 Variant A

### D1. Enforcement revision 2 activation

Current state:

```text
Revision: 2
Status: draft
Active: NO
```

Disposition:

```text
OUT OF HACP 1.0.0 VARIANT A
```

### D2. HC2-55 promotion to active Enforcement

HC2/revision-2 executable evidence MUST NOT be represented as:

```text
the advertised active HACP 1.0 Enforcement contract
```

Disposition:

```text
OUT OF HACP 1.0.0 VARIANT A
```

### D3. Declaration of revision-2 suite completeness

Passing the currently available revision-2 draft vectors does not establish:

```text
complete mandatory active revision-2 conformance suite
```

Disposition:

```text
OUT OF HACP 1.0.0 VARIANT A
```

### D4. Exact reason-code 38/38 as a release requirement

Disposition:

```text
OUT OF HACP 1.0.0 RELEASE REQUIREMENTS
```

### D5. Production changes solely for strict PASS-counter growth

Production change requires:

```text
normative requirement
→ valid prerequisites
→ reachable semantic boundary
→ observable production violation
→ executable RED
→ reproduced RED
```

Disposition:

```text
OUT OF SCOPE
```

### D6. Automatic `hacp_version` migration to `1.0`

R2 established:

```text
specification release version
!=
wire/object version
```

Therefore HACP specification 1.0.0 does not itself require:

```text
hacp_version = "1.0"
```

Disposition:

```text
OUT OF SCOPE WITHOUT SEPARATE NORMATIVE BASIS
```

Existing HACP 1.0.0 signed objects may continue to use:

```text
hacp_version = "0.9"
```

unless a separately justified wire/object transition is adopted.

---

## 9. Consolidated Release-Scope Matrix

| Item | Category | Current state | Required for 1.0.0? | Blocking if unresolved at tag? | Disposition / owner |
|---|---|---|---:|---:|---|
| Stable HACP-Core normative contract | A | Satisfied | YES | YES | Closed by R2 |
| Canonical decision-level conformance | A | 38/38 baseline established | YES | YES | Preserve through final gates |
| Protocol v1 runner / verifier tooling | A | Established | YES | YES | Preserve through final gates |
| Historical strict surface classified | A | 23/23 | YES | YES | Closed by R1 |
| R1 production defects | A | 6/6 fixed | YES | YES | Closed by R1 |
| Unresolved R1 production blocker | A | None | YES | YES | None established |
| Unresolved R2 normative blocker | A | None | YES | YES | None established |
| Honest exact-reason scope | A | Satisfied | YES | YES | Do not claim strict 38/38 |
| Public 0.9.3 / Draft labels | B | Open | YES before tag | YES at tag | R4 |
| Final claim-string alignment | B | Open | YES before tag | YES at tag | R4 |
| README release metadata | B | Open | YES before tag | YES at tag | R4 |
| Phase/Gate release-facing wording | B | Review required | YES where release-facing | YES if misleading at tag | R4/R5 |
| Manifest/package/release metadata | B | Review required | YES before tag | YES at tag | R4/later release hygiene |
| Exact-reason correspondence hardening | C | Residual | NO | NO | 1.0.n/later |
| Vector reachability cleanup | C | Known residual | NO | NO | separate vector maintenance |
| Additional strict executable coverage | C | Optional hardening | NO | NO | 1.0.n/later |
| Protocol v1 historical observability | C | Optional hardening | NO | NO | later tooling work |
| `CORE-RUNTIME-005` | C | HOLD | NO under Variant A | NO | 1.0.n/later adjudication |
| Parent-envelope inheritance clarification | C | No current owner established | NO | NO | later normative work |
| Enforcement revision 2 activation | D | Draft / not active | NO | NO | separate activation process |
| HC2-55 advertised as active 1.0 Enforcement | D | Not advertised | NO | Scope violation if claimed | MUST remain out |
| Rev2 suite-completeness declaration | D | Not established | NO | Scope violation if claimed | separate future determination |
| Exact reason 38/38 release requirement | D | Not achieved / not claimed | NO | NO | excluded from Variant A |
| Production work for PASS-counter growth only | D | Prohibited | NO | N/A | do not perform |
| Automatic `hacp_version = "1.0"` migration | D | Not authorized | NO | N/A | separate normative basis required |

---

## 10. R4 Handoff Boundary

R3 hands the following bounded work to R4:

```text
public release/version metadata
status labels
claim strings
README release-version alignment
release-facing Phase/Gate wording
manifest/package/release metadata classification
```

R4 MUST distinguish among:

```text
specification release version
wire/object version
canonical suite identity
runner/tool version
implementation version
historical engineering milestone
profile/revision lifecycle status
```

R4 MUST NOT mechanically replace every:

```text
0.9.2
0.9.3
0.9
Draft
Phase 4
Gate D
Gate E
```

with a 1.0 label.

Each occurrence must be classified by semantic ownership before modification.

---

## 11. Scope-Freeze Rule

After this R3 boundary is accepted:

```text
NO NEW ITEM MAY ENTER HACP 1.0.0 SCOPE
BY DEFAULT
```

A proposed scope expansion requires a separate explicit GO / NO-GO review.

At minimum, that review must state:

```text
the proposed new release requirement
why Variant A is insufficient without it
the normative owner
the implementation impact
the conformance impact
the schedule/risk impact
whether production changes are required
whether executable RED exists where applicable
```

Without that review:

```text
new work remains outside HACP 1.0.0 scope
```

---

## 12. R3 Exit Criteria

### Criterion 1 — Every known release-relevant class is assigned

Result:

```text
SATISFIED
```

### Criterion 2 — Every unresolved item has an explicit HACP 1.0.0 blocker disposition

Result:

```text
SATISFIED
```

### Criterion 3 — Unresolved release-critical contract blockers

```text
0
```

Result:

```text
SATISFIED
```

### Criterion 4 — Open pre-tag alignment work has a next-stage owner

Result:

```text
SATISFIED
```

Owner:

```text
R4 — Version Alignment and Release Metadata
```

### Criterion 5 — Deferred work is not represented as completed

Result:

```text
SATISFIED
```

### Criterion 6 — HC2-55 / revision 2 are not represented as active HACP 1.0 Enforcement

Result:

```text
SATISFIED
```

### Criterion 7 — Exact reason-code 38/38 is not represented as a HACP 1.0.0 requirement

Result:

```text
SATISFIED
```

### Criterion 8 — Scope expansion after R3 requires explicit GO / NO-GO review

Result:

```text
SATISFIED BY THIS DOCUMENT
```

---

## 13. Final R3 Determination

The HACP 1.0.0 release scope is now bounded as follows:

```text
CATEGORY A — RELEASE-CRITICAL CONTRACT
SATISFIED ON CURRENT EVIDENCE

CATEGORY B — PRE-TAG ALIGNMENT
OPEN
BOUNDED
OWNED BY R4 / LATER RELEASE-HYGIENE STAGES

CATEGORY C — 1.0.n / LATER
EXPLICITLY DEFERRED

CATEGORY D — OUT OF HACP 1.0.0 VARIANT A
EXPLICITLY EXCLUDED
```

Current blocker state:

```text
OPEN RELEASE-CRITICAL CONTRACT BLOCKERS:
NONE

OPEN PRE-TAG ALIGNMENT WORK:
YES

UNRESOLVED R1 PRODUCTION BLOCKERS:
NONE

UNRESOLVED R2 NORMATIVE BLOCKERS:
NONE

NORMATIVE HOLD:
CORE-RUNTIME-005
→ NON-BLOCKING UNDER VARIANT A
→ DEFERRED

ENFORCEMENT REVISION 2:
DRAFT — NOT ACTIVE

HC2-55:
NOT THE ADVERTISED ACTIVE HACP 1.0 ENFORCEMENT CONTRACT

EXACT REASON-CODE 38/38:
NOT A HACP 1.0.0 RELEASE REQUIREMENT
```

Therefore the R3 exit question:

```text
Is the HACP 1.0.0 scope explicit enough that
remaining release work can proceed without
silently expanding the Variant A contract?
```

is answered:

```text
YES
```

Final R3 status:

```text
R3 — RELEASE-SCOPE BOUNDARY

COMPLETE

RELEASE SCOPE:
FROZEN

NEXT:
R4 — VERSION ALIGNMENT AND RELEASE METADATA
```
