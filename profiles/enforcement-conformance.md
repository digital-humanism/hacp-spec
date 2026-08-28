# HACP Enforcement Conformance Identity

**Status:** Normative conformance identity definition
**Profile family:** HACP-Enforcement
**Scope:** Revision-bound conformance suites, vector sets, and verification evidence

## 1. Purpose

This document defines how conformance evidence for `HACP-Enforcement` profile revisions is identified and bound to the applicable normative revision.

It applies only to the `HACP-Enforcement` profile family.

It does not define generic HACP conformance-suite revisioning, runner transport semantics, or the final canonical Enforcement revision 2 conformance suite.

## 2. Normative basis

This document builds on:

```text
docs/conformance/
ENFORCEMENT_PROFILE_REVISION_NORMATIVE_ASSESSMENT.md
```

```text
profiles/enforcement-revisions.md
```

and:

```text
profiles/enforcement-identity.md
```

Those documents establish that:

```text
HACP-Enforcement is one compatibility profile,
materially distinct Enforcement revisions must remain externally distinguishable,
and conformance claims must identify the applicable revision.
```

## 3. Conformance identity dimensions

For Enforcement revision-sensitive conformance, the relevant identity dimensions are:

```text
HACP specification version
HACP-Enforcement profile
Enforcement profile revision
conformance suite or vector-set identifier
exact executed vector-set digest
implementation identity
implementation version
result
```

These dimensions describe different concerns and MUST NOT be collapsed into a single ambiguous identifier.

## 4. Revision-bound conformance

Conformance evidence for `HACP-Enforcement` MUST identify the applicable Enforcement revision whenever revision-sensitive behavior exists.

Conceptually:

```text
HACP specification version
+
HACP-Enforcement
+
Enforcement profile revision
+
conformance suite or vector set
+
exact vector-set digest
```

The applicable Enforcement revision MUST be explicit.

A vector-set name alone MUST NOT be treated as a substitute for explicit revision identity.

## 5. Profile revision and vector-set version are distinct

An Enforcement profile revision is a normative lifecycle identifier.

A vector-set or conformance-suite version is an executable-evidence identifier.

Therefore:

```text
Enforcement profile revision
!=
vector-set version
```

A vector set MAY change without creating a new Enforcement profile revision.

Examples include:

```text
adding missing negative vectors
adding coverage for an already-defined invariant
correcting invalid fixture construction
repairing broken fixture signatures
improving verification metadata
reordering non-semantic fixture inventory
```

Such changes do not, by themselves, establish a new normative Enforcement revision.

## 6. Vector-set changes and normative changes

A new Enforcement profile revision SHOULD correspond to meaningful normative evolution.

A new or updated vector set MAY occur within an existing revision when the underlying normative meaning is unchanged.

Conversely, when the normative Enforcement revision changes in a way that materially affects externally observable conformance behavior, conformance evidence MUST identify the new revision even if some vector fixtures remain unchanged.

Executable evidence follows normative identity; it does not define normative identity by naming convention alone.

## 7. Vector-set binding

An Enforcement conformance run MUST be bound to the exact vector set that was executed.

The conformance evidence MUST preserve a digest or equivalent integrity identifier for the executed vector set.

Conceptually:

```text
vector_set
+
vector_set_digest
```

identify the executable fixture collection used for the run.

A verification result without sufficient fixture-set integrity information MUST NOT be treated as equivalent to a result bound to a specific verified vector set.

## 8. Revision binding of a vector set

A vector set intended to provide Enforcement conformance evidence MUST identify the Enforcement revision it targets.

Conceptually:

```text
Profile:
HACP-Enforcement

Revision:
2

Vector set:
<identifier>
```

A vector set MUST NOT claim applicability to multiple materially distinct Enforcement revisions unless that multi-revision applicability has explicit normative basis.

The default assumption SHOULD be one explicit normative Enforcement revision per conformance target.

## 9. Draft conformance targets

A draft Enforcement revision MAY have executable vectors and verification evidence.

Such evidence demonstrates behavior against the applicable draft conformance target only.

Therefore:

```text
PASS against a draft Enforcement revision
!=
active Enforcement conformance
```

A draft-suite PASS MUST NOT be represented as active profile conformance.

At the time this document is introduced:

```text
HACP-Enforcement revision 2
→ draft successor
→ not active
```

Any existing revision 2 vector results remain draft-revision evidence until activation prerequisites are satisfied.

## 10. Current Enforcement v2 draft vectors

The current:

```text
vectors/enforcement-v2-draft/
```

collection contains executable evidence for behavior defined by the Enforcement revision 2 draft.

Its existence does not establish that the directory already constitutes the complete mandatory conformance suite for an active Enforcement revision 2.

In particular:

```text
verified request-binding cases
```

do not imply:

```text
complete Enforcement revision 2 conformance coverage
```

The current draft vector collection MUST NOT be interpreted as complete active-profile conformance merely because all currently defined vectors pass.

## 11. Verification evidence

A verification record for Enforcement conformance SHOULD identify at least:

```text
implementation
implementation version
applicable HACP specification version
profile
profile revision
vector-set identifier
vector-set digest
passed count
total count
overall result
```

Where additional metadata is required by the applicable harness or suite, it SHOULD also be preserved.

A verification record MUST NOT omit the applicable Enforcement revision when revision-sensitive behavior exists.

## 12. Claim boundary

A result such as:

```text
55/55 request-binding cases passed
```

demonstrates only the behavior covered by those 55 cases.

It does not imply:

```text
general URI normalization conformance
complete Enforcement revision 2 conformance
active Enforcement revision 2 status
```

Conformance evidence MUST remain bounded by the normative scope and executable coverage actually verified.

## 13. Fixture integrity

Conformance evidence is meaningful only when fixture integrity is preserved.

A golden failure MUST be distinguished from:

```text
semantic implementation failure
```

when the fixture itself is invalid, corrupted, incorrectly signed, or otherwise fails before the intended semantic boundary is reached.

Fixture repair MAY change the vector set without changing the Enforcement profile revision when the normative meaning is unchanged.

Fixture integrity validation SHOULD occur before interpreting a failure as evidence of an implementation defect.

## 14. Runner protocol independence

The HACP conformance runner protocol is an execution transport contract.

Enforcement profile revision is a normative conformance identity.

Therefore:

```text
runner protocol version
!=
Enforcement profile revision
```

A change in Enforcement profile revision does not, by itself, require a new runner protocol version.

A runner protocol change is required only when the execution transport contract independently requires such a change.

## 15. Per-vector evaluation boundary

Enforcement profile revision identity belongs primarily to suite/run metadata and verification evidence.

This document does not require the profile revision to be added to every individual runner evaluation request or response.

A runner MAY receive revision context through suite-level metadata, harness configuration, or other defined mechanisms.

The exact execution representation is intentionally deferred.

## 16. Canonical Core manifest boundary

The current canonical HACP-Core conformance manifest and vector set are outside the scope of this document.

This document does not:

```text
modify the canonical HACP-Core manifest
reinterpret existing HACP-Core vector identity
require profile revision fields for HACP-Core
```

Enforcement revision-specific conformance identity is defined locally for the `HACP-Enforcement` profile family.

## 17. Enforcement manifest model

A future Enforcement conformance manifest SHOULD be able to identify conceptually:

```text
spec_version
profile
profile_revision
vector_set
canonicalization
digest_algorithm
vector_digest
total_vectors
status
```

The exact field names, schema, filename, and repository placement are not defined by this document.

For a draft conformance target, the lifecycle status SHOULD remain distinguishable from an active target.

## 18. Conformance status and revision status

Conformance result and revision lifecycle status are separate concepts.

For example:

```text
Revision: 2
Revision status: draft
Run result: PASS
```

means:

```text
the implementation passed the executed revision 2 draft vector set
```

It does not mean:

```text
revision 2 is active
```

and does not mean:

```text
the implementation has active revision 2 conformance
```

## 19. Multiple suites for one revision

More than one vector set or suite release MAY exist for the same Enforcement revision.

For example:

```text
Revision 2
→ vector set A
→ vector set B
```

provided both are explicitly bound to revision 2 and their relationship is documented.

A later vector set MAY supersede an earlier executable evidence set without creating a new normative revision.

Historical verification evidence SHOULD retain the exact vector-set identity and digest that was executed.

## 20. Superseded vector sets

When a vector set is superseded because of:

```text
fixture correction
coverage extension
metadata correction
suite maintenance
```

historical results remain interpretable only when their original vector-set identity and digest are preserved.

A superseded vector set MUST NOT be silently replaced in historical conformance records.

## 21. Conformance-suite completeness

A vector collection MUST NOT be called the complete mandatory conformance suite for an Enforcement revision unless suite completeness has been explicitly established.

Suite completeness requires a separate determination that the executable surface adequately covers the normative requirements required for the intended conformance claim.

Passing all currently available vectors is not, by itself, evidence that the suite is complete.

## 22. Activation relationship

Activation of an Enforcement revision requires more than successful execution of available draft vectors.

Before an Enforcement revision becomes an active conformance target, the applicable activation process MUST determine at least:

```text
normative revision closure
required conformance surface
suite completeness
revision-bound manifest identity
claim identity
capability/discovery identity
known blocker closure
required implementation evidence
```

This document does not itself activate any Enforcement revision.

## 23. Harness implementation impact

This document establishes conformance identity requirements.

It does not establish that the current harness implementation is defective.

Any future harness change to carry or emit Enforcement revision metadata MUST be justified by an adopted executable or manifest requirement.

Until such a requirement is defined:

```text
harness production changes: 0
```

## 24. Production impact

```text
Production changes: 0
```

This conformance identity definition does not establish a sidecar or runtime production defect.

No production implementation change is justified by this document alone.

The governing engineering rule remains:

```text
no production changes without normative basis and proven RED
```

## 25. Explicit non-goals

This document does not:

```text
modify PROFILES.md
modify versioning.md
define generic HACP suite revisioning
modify runner_protocol.md
modify the canonical HACP-Core manifest
modify harness code
modify sidecar code
define final Enforcement manifest schema
define final manifest filename or path
define suite negotiation
define revision negotiation
define preferred revision selection
activate Enforcement revision 2
deprecate Enforcement revision 1
declare current enforcement-v2-draft vectors complete
change signed HACP object schemas
change hacp_version
define new request-target semantics
define dot-segment semantics
define AuthorityRoot
define DelegationGrant
define Semantic Checkpoint 2.0
```

## 26. Current Enforcement conformance state

At the time this document is introduced:

```text
HACP-Enforcement revision 1
→ current normative predecessor
```

```text
HACP-Enforcement revision 2
→ draft successor
→ executable draft vector evidence exists
→ not active
```

The existing revision 2 draft vector collection provides bounded evidence for explicitly defined draft semantics only.

It is not automatically the complete active revision 2 conformance suite.

## 27. Final rules

For `HACP-Enforcement`:

```text
conformance evidence
→ MUST identify applicable Enforcement revision
```

```text
vector set
→ MUST be bound to an explicit Enforcement revision
```

```text
vector-set identifier
!= profile revision
```

```text
profile revision
!= vector-set version
```

```text
draft-suite PASS
!= active profile conformance
```

```text
runner protocol version
!= Enforcement profile revision
```

```text
current draft vectors
!= automatically complete Enforcement revision 2 suite
```

These rules preserve revision-specific conformance integrity without changing the generic HACP profile hierarchy, canonical HACP-Core conformance surface, runner transport contract, or production enforcement implementation.
