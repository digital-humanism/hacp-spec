# HACP Enforcement Revision Transition

**Status:** Normative transition definition
**Profile family:** HACP-Enforcement
**Scope:** Revision 1 to revision 2 lifecycle transition and activation gate

## 1. Purpose

This document defines the lifecycle transition from the current `HACP-Enforcement` predecessor definition to Enforcement revision 2.

It applies only to the current Enforcement revision lineage.

It records the explicit revision 2 activation transition and predecessor disposition. The transition does not change production behavior.

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

## 3. Lineage at introduction

At the time this document was introduced, the Enforcement lineage was:

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

This section records the pre-activation lineage state and is retained as transition provenance.

Revision 1 was not treated as a completed active conformance target.

Revision 2 was not activated merely because its draft profile and executable draft vectors existed.

## 4. Revision 1 disposition

Revision 1 is the superseded predecessor normative definition from which revision 2 evolved.

Its current role is:

```text
superseded predecessor
+
historical normative lineage source
```

Revision 1 was not required to become active before revision 2 activation.

A lifecycle transition of the form:

```text
revision 1 draft
→ revision 1 active
→ revision 1 superseded
```

MUST NOT be introduced solely to create artificial lifecycle symmetry.

## 5. Current revision 2 status

Revision 2 is the active successor normative definition.

Its current state is:

```text
Revision: 2
Status: active
```

Revision 2 is the current preferred HACP-Enforcement conformance target.

Activation required explicit closure of the Enforcement revision 2 Activation Gate; executable vectors and verification evidence did not, by themselves, activate the revision.

## 6. Lifecycle transition model

The revision 1 to revision 2 lifecycle transition is:

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

With revision 2 active, revision 1 is:

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

The revision 2 normative document is:

```text
profiles/enforcement-v2.md
```

The transition from the draft lifecycle document:

```text
profiles/enforcement-v2-draft.md
        ↓
profiles/enforcement-v2.md
```

was performed as part of the explicit revision 2 activation transition after the applicable activation prerequisites were satisfied.

The active normative document MUST use a filename and status that do not falsely imply draft lifecycle state.

## 21. Revision 1 normative document

The superseded predecessor document:

```text
profiles/enforcement.md
```

SHOULD remain available as historical revision 1 evidence unless a separate archival policy establishes another location.

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

This transition does not:

```text
modify versioning.md
deprecate revision 1 as legacy
rename profiles/enforcement.md
define final claim-string syntax
define final capability-discovery schema
define final conformance manifest schema
declare HC2-55 to be the complete mandatory Enforcement revision 2 conformance suite
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

The revision 2 Activation Gate is closed.

Current Enforcement revision lineage:

```text
Revision 1
→ superseded predecessor
→ historical normative lineage retained
→ defined by profiles/enforcement.md
```

```text
Revision 2
→ active
→ current preferred HACP-Enforcement conformance target
→ defined by profiles/enforcement-v2.md
```

The activation transition records:

```text
revision 1
→ superseded

revision 2
→ active
```

Revision 1 was not retroactively treated as an active revision solely to create lifecycle symmetry.

Revision 2 activation does not imply:

```text
a HACP wire/object version transition
general URI normalization conformance
HC2-55 completeness as the mandatory Enforcement revision 2 conformance suite
runner protocol changes
harness implementation changes
production sidecar changes
```

HC2-55 remains bounded revision-2 HTTP request-binding conformance evidence.

## 27. Final rules

For the current HACP-Enforcement revision lineage:

```text
revision 1
→ superseded predecessor
```

```text
revision 2
→ active successor
→ current preferred HACP-Enforcement conformance target
```

```text
revision 1
→ legacy
ONLY with separate compatibility justification
```

```text
silent revision replacement
→ prohibited
```

```text
profile revision identity
→ independent from HACP wire/object version
```

```text
HC2-55
→ bounded revision-2 HTTP request-binding conformance evidence
→ not the complete mandatory Enforcement revision 2 conformance suite
```
