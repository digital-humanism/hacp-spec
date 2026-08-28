# HACP Enforcement Revision Transition

**Status:** Normative transition definition
**Profile family:** HACP-Enforcement
**Scope:** Revision 1 to revision 2 lifecycle transition and activation gate

## 1. Purpose

This document defines the lifecycle transition from the current `HACP-Enforcement` predecessor definition to Enforcement revision 2.

It applies only to the current Enforcement revision lineage.

It does not activate Enforcement revision 2, rename any profile document, or change production behavior.

## 2. Normative basis

This document builds on:

```text
docs/conformance/
ENFORCEMENT_PROFILE_REVISION_NORMATIVE_ASSESSMENT.md
```

```text
profiles/enforcement-revisions.md
```

```text
profiles/enforcement-identity.md
```

and:

```text
profiles/enforcement-conformance.md
```

Those documents establish that:

```text
HACP-Enforcement is one compatibility profile,
it may evolve through explicit revisions,
revision identity is independent of hacp_version,
and materially distinct revisions must remain externally distinguishable.
```

## 3. Current lineage

At the time this document is introduced, the Enforcement lineage is:

```text
Revision 1
→ predecessor normative draft
→ defined by profiles/enforcement.md
```

and:

```text
Revision 2
→ successor normative draft
→ defined by profiles/enforcement-v2-draft.md
→ not active
```

Revision 1 is not treated by this document as a completed active conformance target.

Revision 2 is not active merely because its draft profile and executable draft vectors exist.

## 4. Current revision 1 status

Revision 1 is the predecessor normative definition from which revision 2 evolves.

Its current role is:

```text
historical/current predecessor definition
+
normative lineage source
```

This document does not require revision 1 to be activated before revision 2 may later become active.

A lifecycle transition of the form:

```text
revision 1 draft
→ revision 1 active
→ revision 1 superseded
```

MUST NOT be introduced solely to create artificial lifecycle symmetry.

## 5. Current revision 2 status

Revision 2 is the successor normative draft.

Its current state is:

```text
Revision: 2
Status: draft
```

Revision 2 remains subject to the Enforcement v2 Activation Readiness workstream.

Existing draft vectors and verification evidence do not, by themselves, activate revision 2.

## 6. Target lifecycle transition

The intended lifecycle transition is:

```text
revision 1
predecessor normative draft
        │
        │ successor development
        ▼
revision 2
draft
        │
        │ Activation Gate PASS
        ▼
revision 2
active
```

At the revision 2 activation transition:

```text
revision 1
→ superseded
```

and:

```text
revision 2
→ active
```

The predecessor/successor transition MUST be recorded explicitly.

## 7. Superseded status for revision 1

When revision 2 becomes active, revision 1 SHOULD become:

```text
superseded
```

unless a separate compatibility assessment establishes a need for another lifecycle status.

`superseded` means that revision 1 remains part of the historical normative lineage but is no longer the preferred current Enforcement conformance target.

Superseded status MUST NOT erase historical normative or verification evidence.

## 8. Legacy status is not automatic

Revision 1 MUST NOT automatically become:

```text
legacy
```

at revision 2 activation.

Legacy status requires separate justification based on real compatibility, deployment, or historical support requirements.

Absent such evidence, the expected disposition is:

```text
revision 1
→ superseded
```

not:

```text
revision 1
→ legacy
```

## 9. Silent replacement is forbidden

Revision 2 MUST NOT silently replace revision 1 while leaving the two revisions externally indistinguishable.

The activation transition MUST preserve explicit evidence of:

```text
predecessor revision
successor revision
activation date or release point
lifecycle disposition
applicable conformance identity
```

The exact publication format is defined separately.

## 10. Activation Gate

Revision 2 MUST NOT become active until the Enforcement revision 2 Activation Gate is satisfied.

The Activation Gate is a lifecycle condition, not a runtime protocol mechanism.

The gate is satisfied only when all mandatory activation categories have been explicitly closed.

## 11. Activation Gate — normative closure

Before activation, the revision 2 normative surface MUST be sufficiently closed for the intended conformance claim.

At minimum, this includes explicit disposition of:

```text
profile scope
known normative ambiguities
reason-code semantics
verification-order semantics
control-state and freshness ownership
historical draft wording that no longer reflects current lifecycle state
other identified activation blockers
```

Open normative blockers prevent activation.

## 12. Activation Gate — identity closure

Before activation, Enforcement revision 2 identity MUST be unambiguous.

At minimum, this includes:

```text
revision lifecycle identity
claim identity
capability/discovery identity
conformance evidence identity
```

The applicable revision MUST remain externally distinguishable from revision 1.

## 13. Activation Gate — conformance closure

Before activation, the required executable conformance surface MUST be explicitly determined.

At minimum, this includes:

```text
mandatory normative surface inventory
suite completeness assessment
revision-bound suite or manifest identity
fixture integrity requirements
required negative and positive evidence
required implementation pass criteria
```

Passing all currently available draft vectors is not sufficient unless suite completeness has been separately established.

## 14. Activation Gate — lifecycle closure

Before activation, the predecessor/successor lifecycle MUST be explicit.

At minimum:

```text
revision 1 disposition
revision 2 active status
canonical normative document for revision 2
transition record
claim boundary
```

No lifecycle status may be inferred solely from filenames.

## 15. Activation Gate — documentation and release closure

Before activation, public documentation MUST be internally consistent with the active revision status.

At minimum:

```text
obsolete draft-only lifecycle wording resolved
known limitations explicit
revision identity consistent across public artifacts
verification evidence recorded
signed public history preserved
repository state clean at the activation point
```

The exact release mechanics are outside the scope of this document.

## 16. Activation result

A successful revision 2 activation means:

```text
HACP-Enforcement revision 2
→ active Enforcement conformance target
```

and:

```text
HACP-Enforcement revision 1
→ superseded predecessor
```

unless a separately justified lifecycle disposition is adopted.

Activation does not imply that every future Enforcement revision is frozen permanently.

## 17. Activation does not imply HACP 1.0

Activation of Enforcement revision 2 does not, by itself, imply:

```text
HACP 1.0
```

Enforcement revision identity remains independent of the HACP wire/object version.

A HACP specification or wire-version transition requires its own normative basis.

## 18. Activation does not imply broader URI conformance

Revision 2 activation does not imply:

```text
general URI normalization conformance
```

Request-target claims remain limited to semantics explicitly defined and verified by the applicable Enforcement revision.

Existing request-binding evidence MUST retain its established claim boundaries.

## 19. Activation does not imply automatic wire incompatibility

Revision 1 and revision 2 may differ in Enforcement semantics while remaining compatible with the same HACP wire/object version.

Therefore, revision 2 activation does not automatically make revision 1 implementations invalid at the HACP wire level.

Profile conformance and wire compatibility are separate dimensions.

## 20. Normative document transition

The current revision 2 normative draft is:

```text
profiles/enforcement-v2-draft.md
```

This document does not rename it.

If revision 2 later becomes active, the active normative document SHOULD use a filename and status that do not falsely imply draft lifecycle state.

A likely transition is:

```text
profiles/enforcement-v2-draft.md
        ↓
profiles/enforcement-v2.md
```

but the exact file operation MUST occur only when activation prerequisites are satisfied.

## 21. Revision 1 normative document

The current predecessor document:

```text
profiles/enforcement.md
```

SHOULD remain available as historical revision 1 evidence when revision 2 becomes active, unless a separate archival policy establishes another location.

The predecessor MUST NOT be silently overwritten in a way that destroys revision lineage.

## 22. Transition record

Revision 2 activation SHOULD produce a public transition record that identifies at least:

```text
profile
activated revision
superseded revision
activation basis
conformance basis
known limitations
verification status
```

The exact document name and format are not defined here.

## 23. Forbidden transitions

The following transitions are forbidden without separate normative basis:

```text
revision 2 draft
→ active while known activation blockers remain
```

```text
revision 1
→ silently replaced without explicit lifecycle disposition
```

```text
revision 1
→ legacy without compatibility justification
```

```text
revision 2
→ HACP 1.0 solely because the Enforcement revision changed
```

```text
draft-suite PASS
→ active conformance without suite completeness determination
```

```text
document rename
→ inferred activation
```

## 24. Production impact

```text
Production changes: 0
```

This transition definition does not establish a production defect.

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
activate Enforcement revision 2
deprecate revision 1 as legacy
rename profiles/enforcement-v2-draft.md
rename profiles/enforcement.md
define final claim-string syntax
define final capability-discovery schema
define final conformance manifest schema
declare the current draft vectors complete
change runner protocol
change harness code
change sidecar code
change hacp_version
change signed HACP object schemas
define new request-target semantics
define dot-segment semantics
define AuthorityRoot
define DelegationGrant
define Semantic Checkpoint 2.0
```

## 26. Current activation status

At the time this document is introduced:

```text
Revision 1
→ predecessor normative draft
```

```text
Revision 2
→ draft successor
→ Activation Readiness in progress
→ not active
```

The revision 2 Activation Gate is not yet closed.

Known remaining Activation Readiness work includes at least:

```text
reason-code normative consistency
verification-order correspondence
control-state and freshness normative ownership
conformance-suite completeness
remaining activation blocker review
```

## 27. Final rules

For the current `HACP-Enforcement` revision lineage:

```text
revision 1
→ predecessor normative draft
```

```text
revision 2
→ draft successor
```

```text
revision 2 draft
→ active
ONLY after Activation Gate PASS
```

```text
at revision 2 activation:
revision 1
→ superseded
```

```text
revision 1
→ legacy
ONLY with separate compatibility justification
```

```text
silent replacement
→ forbidden
```

```text
revision 2 activation
!= HACP 1.0 transition
```

```text
revision 2 activation
!= general URI normalization conformance
```

These rules preserve explicit Enforcement lineage and prevent activation from being inferred from draft maturity, filename changes, or incomplete executable evidence.
