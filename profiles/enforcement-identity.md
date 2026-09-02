# HACP Enforcement Profile Identity

**Status:** Normative identity definition
**Profile family:** HACP-Enforcement
**Scope:** Conformance-claim and capability identity for Enforcement profile revisions

## 1. Purpose

This document defines how `HACP-Enforcement` profile revisions are identified in conformance claims and capability advertisements.

It applies only to the `HACP-Enforcement` profile family.

It does not define generic HACP profile identity rules, revision negotiation, or transport-specific discovery schemas.

## 2. Normative basis

This document builds on:

```text
docs/conformance/
ENFORCEMENT_PROFILE_REVISION_NORMATIVE_ASSESSMENT.md
```

and:

```text
profiles/enforcement-revisions.md
```

Those documents establish that:

```text
HACP-Enforcement is one compatibility profile,
it may evolve through explicit revisions,
and materially distinct revisions must remain externally distinguishable.
```

## 3. Enforcement identity dimensions

For Enforcement revision-sensitive interoperability, the relevant identity dimensions are:

```text
HACP specification version
HACP-Enforcement profile
Enforcement profile revision
conformance status or evidence, when applicable
```

These dimensions MUST NOT be inferred from:

```text
implementation package version
repository release version
architecture/project release version
HACP wire/object version
```

They describe different lifecycle concerns.

## 4. Canonical profile identity

The canonical logical identity of an Enforcement revision is:

```text
Profile:
HACP-Enforcement

Revision:
N
```

For the current successor draft:

```text
Profile:
HACP-Enforcement

Revision:
2
```

This document does not define a mandatory single-string serialization of that identity.

A human-readable representation MAY be used, but it MUST preserve the same unambiguous profile/revision meaning.

## 5. Conformance claims

A conformance claim for `HACP-Enforcement` MUST identify the applicable Enforcement revision whenever materially distinct revisions exist.

Conceptually, a claim identifies:

```text
applicable HACP specification version
+
HACP-Enforcement
+
applicable Enforcement revision
+
applicable conformance status or evidence
```

A claim MUST NOT rely on an implementation release number as a substitute for Enforcement revision identity.

A claim MUST NOT rely on `hacp_version` alone as a substitute for Enforcement revision identity.

## 6. Human-readable claim representation

A human-readable claim MAY render the structured identity in a compact form.

For example:

```text
HACP 1.0-Enforcement, revision 2
```

is a possible presentation of the logical identity.

This example is illustrative only.

This document does not define:

```text
final claim grammar
mandatory punctuation
canonical display string
machine-readable claim serialization
```

Those representations MAY be specified separately.

## 7. Capability advertisement

An implementation that advertises support for `HACP-Enforcement` MUST make the supported revision or revisions unambiguous when multiple materially distinct revisions exist.

Conceptually:

```text
HACP-Enforcement
supported revisions:
- 1
- 2
```

An implementation MAY support more than one Enforcement revision.

This document does not require an implementation to support more than one revision.

## 8. Multiple supported revisions

A capability advertisement MUST NOT assume that one implementation supports exactly one Enforcement revision.

The identity model MUST allow:

```text
one implementation
→ one supported Enforcement revision
```

or:

```text
one implementation
→ multiple supported Enforcement revisions
```

The exact representation of multiple supported revisions is transport-specific and intentionally deferred.

## 9. Capability is not a conformance claim

An advertisement of support for an Enforcement revision is not, by itself, a conformance claim.

Therefore:

```text
supports Enforcement revision 2
```

does not necessarily mean:

```text
conforms to Enforcement revision 2
```

A conformance claim remains subject to the applicable HACP conformance rules and required evidence.

An implementation MAY advertise experimental, draft, partial, or otherwise non-conformant support if the surrounding representation makes that status unambiguous.

Such support MUST NOT be presented as active conformance without the required evidence.

## 10. Draft and active revision identity

Revision identity is separate from lifecycle status.

For example:

```text
Revision: 2
Status: draft
```

and:

```text
Revision: 2
Status: active
```

refer to the same normative lineage revision at different lifecycle stages.

A draft revision MUST NOT be represented as an active conformance target.

At the time this document is introduced:

```text
HACP-Enforcement revision 1
→ current normative predecessor
```

```text
HACP-Enforcement revision 2
→ draft successor
→ not active
```

## 11. Discovery does not imply negotiation

Advertising supported Enforcement revisions does not define revision negotiation.

The following questions are out of scope for this document:

```text
client/server revision selection
preferred revision
fallback order
automatic downgrade
automatic upgrade
revision handshake
negotiation failure semantics
```

No negotiation behavior may be inferred solely from the presence of multiple advertised revisions.

Revision negotiation, if required, MUST have separate normative definition.

## 12. Active or preferred revision

This document does not define an `active_revision`, `preferred_revision`, or equivalent selection field.

An implementation MAY internally distinguish preferred or default behavior, but such concepts are not part of the normative identity model defined here.

If a public preferred/default revision concept is later required, it MUST be assessed separately.

## 13. Signed-object boundary

Enforcement profile revision identity belongs to the conformance and capability surface.

This document does not require adding Enforcement revision fields to:

```text
IntentEnvelope
ProposedAction
DecisionToken
ProvenanceEvent
other signed HACP objects
```

No signed-object schema change is justified by this identity definition alone.

The existing HACP object-version field remains independent of Enforcement profile revision identity.

## 14. Relationship to hacp_version

`hacp_version` identifies the applicable HACP wire/object version as defined by the HACP specification.

It does not identify the Enforcement profile revision.

Therefore:

```text
hacp_version = 0.9
```

does not by itself mean:

```text
Enforcement revision = 1
```

or:

```text
Enforcement revision = 2
```

The two dimensions MUST remain conceptually distinct.

## 15. Relationship to implementation releases

An implementation release MAY support different Enforcement revisions over time.

For example:

```text
implementation release X
→ supports Enforcement revision 1
```

```text
implementation release Y
→ supports Enforcement revision 2
```

The release numbers do not become Enforcement revision identifiers.

Independent implementations may use different package/release numbering while supporting the same Enforcement revision.

## 16. Conformance evidence identity

Any verification report, certification record, or equivalent Enforcement conformance evidence MUST identify the applicable Enforcement revision whenever revision-sensitive behavior exists.

A statement such as:

```text
HACP-Enforcement conformance passed
```

is insufficient when multiple materially distinct Enforcement revisions are recognized.

The applicable revision MUST be explicit.

## 17. Capability metadata requirements

Capability metadata MUST preserve the following semantic distinction:

```text
profile identity
revision identity
support status
conformance status
```

These concepts MUST NOT be collapsed in a way that makes them indistinguishable.

This document does not prescribe field names or schema structure.

## 18. Explicit non-goals

This document does not:

```text
modify PROFILES.md
modify versioning.md
define generic HACP profile revision semantics
define final claim-string syntax
define final JSON or API schema
define HTTP headers
define gRPC fields
define CLI flags
define revision negotiation
define preferred-revision logic
define downgrade or fallback behavior
define conformance manifest schema
activate Enforcement revision 2
deprecate Enforcement revision 1
change hacp_version
change signed HACP object schemas
change production sidecar behavior
change conformance runner behavior
define new request-target semantics
define dot-segment semantics
define AuthorityRoot
define DelegationGrant
define Semantic Checkpoint 2.0
```

## 19. Production impact

```text
Production changes: 0
```

This identity definition does not establish a production defect.

No implementation change is justified by this document alone.

The governing engineering rule remains:

```text
no production changes without normative basis and proven RED
```

## 20. Current Enforcement identity state

At the time this document is introduced:

```text
HACP-Enforcement revision 1
→ predecessor
```

and:

```text
HACP-Enforcement revision 2
→ draft successor
→ not active
```

Revision 2 may be identified in draft/support contexts.

Revision 2 MUST NOT be presented as active conformance until its activation and conformance prerequisites are closed.

## 21. Final rules

For `HACP-Enforcement`:

```text
claim identity
→ MUST identify applicable revision when revision-sensitive behavior exists
```

```text
capability identity
→ MUST make supported revision(s) unambiguous
```

```text
capability advertisement
!= conformance claim
```

```text
revision advertisement
!= revision negotiation
```

```text
Enforcement revision
!= hacp_version
```

```text
Enforcement revision
!= implementation release version
```

These rules preserve unambiguous Enforcement interoperability without changing the generic HACP profile hierarchy or signed-object wire model.
