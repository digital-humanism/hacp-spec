# Enforcement Profile Revision Normative Assessment

**Status:** Normative assessment
**Target:** HACP Enforcement profile lifecycle
**Scope:** Explicit profile revision identity for successor Enforcement definitions
**Result:** **POSITIVE**

## 1. Question

Should a HACP compatibility profile be able to evolve through an explicit profile revision that is independent of the HACP wire/protocol version?

For the current Enforcement workstream, the concrete question is:

> Should the successor Enforcement definition be identified as an explicit revision of the existing `HACP-Enforcement` compatibility profile?

This assessment addresses profile identity and lifecycle only. It does not activate Enforcement revision 2 and does not change production behavior.

## 2. Current model

HACP currently defines the compatibility hierarchy:

```text
HACP-Core
⊂ HACP-Runtime
⊂ HACP-Enforcement
```

The active Enforcement profile is defined by:

```text
profiles/enforcement.md
```

The current public claim surface identifies the specification/profile combination, for example:

```text
HACP 0.9-Enforcement
```

The general profile and versioning model distinguishes specification/wire versioning from implementation releases and conformance suites, but it does not currently define a generic profile-revision dimension.

## 3. Successor Enforcement definition

The successor draft is defined by:

```text
profiles/enforcement-v2-draft.md
```

Its metadata states:

```text
Status: Draft — not yet active
Profile revision: 2-draft
Based on: profiles/enforcement.md
Release lineage: successor candidate
```

This establishes an intended lineage:

```text
existing HACP-Enforcement profile
        ↓
successor revision
```

rather than a new independent compatibility tier.

The `2-draft` identifier is currently local metadata of the successor document. It is not yet defined by the general HACP profile, discovery, claim, or conformance identity model.

## 4. Observable semantic difference

The successor Enforcement definition is not merely an editorial rewrite.

In particular, Enforcement revision 2 draft develops request-target binding semantics that can affect externally observable enforcement outcomes.

Representative invariants include:

```text
/x != /x?
```

```text
/a//b != /a/b
```

```text
//a != /a
```

The revision also defines representation-preservation behavior for percent-encoded forms, query-component ordering, and related request-target boundaries.

Therefore:

```text
revision 1 semantics
!= necessarily revision 2 semantics
```

for observable authorization and enforcement behavior.

## 5. Interoperability problem

If a successor profile with materially different normative behavior is activated without explicit revision identity, two implementations could both claim:

```text
HACP 0.9-Enforcement
```

while following different normative Enforcement definitions.

That would leave an operator, integrator, or conformance consumer unable to determine which exact Enforcement semantics are being claimed.

For an enforcement boundary, this is security-relevant because the difference may be observable as:

```text
ALLOW
vs
DENY
```

A conformance claim must therefore identify the normative revision whenever multiple revisions of the same profile can produce different externally observable behavior.

## 6. Why HACP wire version is not sufficient

A profile revision is not necessarily a wire-protocol revision.

HACP already separates signed-object/wire versioning from higher-level profile and conformance concerns. Enforcement revision 2 request-binding semantics do not, by themselves, require a different serialization format for every signed HACP object.

Therefore:

```text
Enforcement profile revision
does not imply
new HACP wire version
```

Using a new `hacp_version` solely to distinguish Enforcement successor semantics would unnecessarily couple two independent dimensions.

## 7. Why implementation/package version is not sufficient

An implementation package version identifies a release of a particular implementation.

It cannot answer the normative question:

> Which HACP Enforcement definition does this implementation conform to?

Independent implementations have independent package versioning. Package versions therefore cannot serve as interoperable profile identity.

## 8. Why a new sibling compatibility profile is not preferred

A model such as:

```text
HACP-Enforcement
HACP-Enforcement-v2
```

would treat a revision of the same functional compatibility level as a new compatibility tier.

That is not consistent with the current profile hierarchy, where `Core`, `Runtime`, and `Enforcement` represent progressively stronger functional profiles.

Enforcement revision 2 is a successor normative definition of the same Enforcement compatibility level, not a new higher-level profile.

Creating a sibling profile would unnecessarily expand profile taxonomy and mix profile identity with profile revision.

## 9. Why silent in-place replacement is not sufficient

A silent replacement of the current Enforcement definition while retaining an undifferentiated:

```text
HACP 0.9-Enforcement
```

claim would remove the ability to distinguish predecessor and successor semantics.

Even if the predecessor has limited external deployment today, the specification should establish unambiguous identity before multiple independent implementations or long-lived claims depend on it.

Profile evolution must therefore remain externally identifiable when normative behavior changes materially.

## 10. Normative conclusion

**Result: POSITIVE**

A HACP compatibility profile may evolve through explicit profile revisions independently of the HACP wire/protocol version.

When two normative revisions of the same profile may produce different externally observable conformance behavior, a conformance claim **MUST** identify the applicable profile revision.

For the current Enforcement successor:

```text
Profile: HACP-Enforcement
Revision: 2
```

is the preferred identity model.

Conceptually:

```text
HACP-Enforcement
    revision 1
        ↓
    revision 2
```

This preserves one Enforcement compatibility level while making its normative evolution explicit.

## 11. Identity dimensions

The HACP lifecycle should distinguish the following concepts:

```text
1. HACP specification version
2. HACP wire/object version
3. Compatibility profile
4. Profile revision
5. Conformance suite/vector-set version
6. Implementation/package version
7. Architecture/project release version
```

These dimensions may be related by release policy, but they are not interchangeable identifiers.

For conformance, the minimum conceptual identity becomes:

```text
specification version
+
profile
+
profile revision
+
conformance suite
```

## 12. Where profile revision must be visible

Profile revision must be representable wherever compatibility or conformance identity is communicated.

At minimum, this includes:

```text
profile metadata
conformance claims
capability discovery
conformance-suite metadata
verification reports
```

The exact syntax and transport representation are intentionally deferred.

## 13. Where profile revision is not automatically required

This assessment does **not** establish a requirement to add profile revision fields to:

```text
IntentEnvelope
ProposedAction
DecisionToken
ProvenanceEvent
other signed HACP objects
```

No signed-object schema change is justified by this assessment alone.

Signed objects continue to use the protocol-version field defined by the applicable HACP object schema.

Profile revision primarily belongs to the implementation/service conformance surface.

## 14. Conformance implications

A conformance suite for a revision-sensitive profile must be unambiguously associated with the revision it verifies.

The conceptual association becomes:

```text
specification version
+
profile
+
profile revision
+
suite/vector set
```

This prevents a test result for one normative revision from being presented as evidence for another revision with materially different behavior.

## 15. Capability-discovery implications

An implementation claiming Enforcement support must eventually be able to communicate the applicable Enforcement revision.

Conceptually:

```text
profile:
  name: Enforcement
  revision: 2
```

This assessment does not prescribe a JSON schema, API field, handshake format, or other transport representation.

Those are follow-on integration decisions.

## 16. Lifecycle implications

This assessment does not require immediate deprecation of the predecessor Enforcement definition.

A valid lifecycle may include states such as:

```text
revision 1 — active
revision 2 — draft
```

followed later by:

```text
revision 1 — superseded or legacy
revision 2 — active
```

The exact coexistence, deprecation, and activation policy must be defined separately.

## 17. Activation requirement

Enforcement revision 2 **MUST NOT** become active through an ambiguous silent replacement that leaves materially different normative revisions externally indistinguishable.

Before activation, HACP must define at least:

```text
profile revision semantics
claim identity
capability/discovery identity
conformance-suite identity
predecessor/successor lifecycle semantics
```

This requirement is an activation prerequisite, not a production implementation requirement.

## 18. Explicit non-goals

This assessment does not define:

```text
the final conformance-claim string syntax
the final capability-discovery schema
new fields in signed HACP objects
a new HACP wire version
a HACP 1.0 transition
a new cryptographic primitive
new sidecar production behavior
request-target classes beyond the closed HC2 workstream
dot-segment processing
AuthorityRoot
DelegationGrant
Semantic Checkpoint 2.0
```

It also does not require implementation refactoring.

## 19. Production impact

```text
Production changes: 0
```

No production defect is established by this assessment.

No RED condition is expected or required at this stage.

The governing engineering rule remains:

```text
no production changes without normative basis and proven RED
```

## 20. Recommended follow-on sequence

If this assessment is accepted, the next normative work should remain incremental:

```text
1. Record this normative assessment.
2. Define minimal generic profile-revision semantics.
3. Define claim and discovery identity for profile revisions.
4. Define conformance-suite revision identity.
5. Define predecessor/successor lifecycle semantics.
6. Continue remaining Enforcement v2 activation-readiness audits.
7. Evaluate activation only after all identified blockers are closed.
```

Generic profile/versioning rules should not be changed before the normative basis in this assessment is accepted.

## 21. Final determination

The successor Enforcement definition should be modeled as:

```text
HACP-Enforcement
revision 2
```

not as:

```text
a new HACP wire version
```

and not as:

```text
a separate sibling compatibility profile
```

Explicit profile revision identity is required whenever normative revisions can produce different externally observable conformance behavior.

**Assessment result: POSITIVE.**
