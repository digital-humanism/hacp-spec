# HACP Enforcement Profile Revisions

**Status:** Normative lifecycle definition
**Profile family:** HACP-Enforcement
**Scope:** Enforcement profile revision identity, lifecycle, and version relationship

## 1. Purpose

This document defines revision semantics for the `HACP-Enforcement` profile family.

It applies only to Enforcement profile revisions. It does not introduce a generic revision mechanism for all HACP compatibility profiles.

The current profile hierarchy remains unchanged:

```text
HACP-Core
⊂ HACP-Runtime
⊂ HACP-Enforcement
```

This document defines how multiple normative revisions may exist within the existing `HACP-Enforcement` compatibility level.

## 2. Enforcement profile family

`HACP-Enforcement` is one compatibility profile.

It may evolve through explicit normative revisions.

Conceptually:

```text
HACP-Enforcement
    revision 1
        ↓
    revision 2
        ↓
    future revision, if normatively justified
```

A new Enforcement revision does not create a new sibling compatibility profile.

Therefore:

```text
HACP-Enforcement revision 2
```

is preferred over:

```text
HACP-Enforcement-v2
```

when identifying a successor normative definition of the same Enforcement compatibility level.

## 3. Revision identity

An Enforcement profile revision identifies a normative revision of the `HACP-Enforcement` profile.

A revision MUST be independently identifiable when two revisions may produce different externally observable conformance behavior.

A revision identifier MUST NOT be inferred from:

```text
implementation package version
repository release version
architecture/project release version
HACP wire/object version
```

Those identifiers describe different lifecycle dimensions.

## 4. Current revisions

The current Enforcement revision lineage is:

```text
Revision 1
Normative document:
profiles/enforcement.md
```

and:

```text
Revision 2
Status:
draft

Normative draft:
profiles/enforcement-v2-draft.md
```

Revision 2 is the successor candidate to revision 1.

Revision 2 is not active merely because its normative draft exists.

## 5. Relationship to HACP versioning

An Enforcement profile revision is independent of the HACP wire/object version.

A change in Enforcement profile revision does not, by itself, require a change to:

```text
hacp_version
```

A new HACP wire/object version is required only when the underlying HACP object or protocol requirements independently require such a change.

Therefore, the following conceptual combination is valid:

```text
HACP wire/object version: 0.9
Enforcement profile revision: 2
```

provided the applicable HACP specification and activation rules permit that combination.

## 6. Observable semantic changes

A new Enforcement revision MAY introduce materially different normative enforcement behavior within the same compatibility profile.

Such differences may include, for example:

```text
request-binding semantics
verification-order semantics
reason-code semantics
control-state enforcement semantics
other explicitly defined Enforcement invariants
```

When those differences can change externally observable conformance behavior, the applicable revision MUST be identifiable in conformance evidence.

## 7. Lifecycle states

An Enforcement revision may have one of the following lifecycle states:

```text
draft
active
superseded
legacy
```

### draft

The revision is under normative development.

A draft revision MUST NOT be presented as an active Enforcement conformance target.

### active

The revision is an approved Enforcement conformance target.

Activation requires the applicable normative, conformance, and lifecycle prerequisites to be satisfied.

### superseded

The revision has been replaced by a newer active revision for current conformance purposes.

Superseded status does not erase historical conformance evidence.

### legacy

The revision remains recognized for compatibility or historical deployment reasons but is not the preferred target for new implementations.

The exact transition from one lifecycle state to another MUST be recorded explicitly.

## 8. Predecessor and successor relationship

A successor revision MUST identify the revision or normative definition from which it evolves.

A successor revision MUST NOT silently replace a predecessor when the two revisions may produce materially different externally observable behavior.

The predecessor/successor relationship MUST remain externally understandable through normative documentation and conformance evidence.

## 9. Conformance identity

Conformance evidence for `HACP-Enforcement` MUST identify the applicable revision whenever revision-sensitive behavior exists.

Conceptually, Enforcement conformance identity includes:

```text
HACP specification version
+
HACP-Enforcement profile
+
Enforcement profile revision
+
applicable conformance suite or vector set
```

This document does not define the final claim-string syntax or manifest schema.

Those representations are defined separately.

## 10. Capability and discovery identity

An implementation that advertises `HACP-Enforcement` support MUST eventually be able to communicate the applicable Enforcement revision when multiple materially distinct revisions exist.

This document defines the requirement for unambiguous revision identity only.

It does not define:

```text
JSON field names
API schema
handshake representation
service-discovery transport
CLI syntax
```

Those are follow-on integration decisions.

## 11. Activation requirements

A draft Enforcement revision MUST NOT become active until all applicable activation prerequisites are closed.

At minimum, activation requires explicit determination of:

```text
normative revision status
claim identity
capability/discovery identity
conformance-suite identity
predecessor/successor lifecycle status
known normative blockers
required executable conformance evidence
```

Additional activation requirements MAY be defined by the applicable Enforcement revision.

## 12. Revision numbering

Revision numbers identify normative Enforcement lineage.

Revision numbers SHOULD increase monotonically.

A revision number MUST NOT be changed solely for editorial modifications that do not alter normative behavior.

A new revision SHOULD correspond to a meaningful normative change or lifecycle transition.

Draft status MAY be represented separately from the numeric revision.

For example:

```text
Revision: 2
Status: draft
```

is preferred conceptually over treating `2-draft` as a distinct permanent revision number.

This document does not require immediate metadata changes to existing draft files.

## 13. Independence from implementation releases

An implementation release MAY add, remove, or update support for an Enforcement revision.

The implementation release number is not the Enforcement revision number.

Independent implementations may use different release numbers while conforming to the same Enforcement revision.

## 14. Independence from HACP-Core and Runtime revisioning

This document does not establish revision semantics for:

```text
HACP-Core
HACP-Runtime
```

No generic HACP profile-revision abstraction is introduced by this document.

If another HACP profile later requires materially distinct normative revisions, that requirement MUST be assessed independently before any generic profile-revision mechanism is introduced.

## 15. Explicit non-goals

This document does not:

```text
modify PROFILES.md
modify versioning.md
change the HACP profile hierarchy
activate Enforcement revision 2
deprecate Enforcement revision 1
define final conformance-claim syntax
define final capability-discovery schema
change signed HACP object schemas
change hacp_version
define a new HACP wire version
change production sidecar behavior
define new request-target classes
define dot-segment behavior
define AuthorityRoot
define DelegationGrant
define Semantic Checkpoint 2.0
```

## 16. Production impact

```text
Production changes: 0
```

This lifecycle definition does not establish a production defect.

No production implementation change is justified by this document alone.

The governing engineering rule remains:

```text
no production changes without normative basis and proven RED
```

## 17. Current Enforcement lifecycle

At the time this document is introduced:

```text
HACP-Enforcement revision 1
→ current normative predecessor
→ defined by profiles/enforcement.md
```

```text
HACP-Enforcement revision 2
→ draft successor
→ defined by profiles/enforcement-v2-draft.md
→ not active
```

Revision 2 activation remains subject to the Enforcement v2 Activation Readiness workstream and its remaining blockers.

## 18. Normative basis

This lifecycle model follows the positive determination recorded in:

```text
docs/conformance/
ENFORCEMENT_PROFILE_REVISION_NORMATIVE_ASSESSMENT.md
```

That assessment established that explicit revision identity is required when Enforcement revisions may produce materially different externally observable conformance behavior.

## 19. Final rule

`HACP-Enforcement` is one compatibility profile that may evolve through explicit normative revisions.

Enforcement profile revision is independent of HACP wire/object versioning.

Materially distinct revisions MUST remain externally distinguishable in conformance and lifecycle evidence.

Revision 2 remains a draft successor until its activation prerequisites are explicitly satisfied.
