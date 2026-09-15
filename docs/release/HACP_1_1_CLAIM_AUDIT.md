# HACP 1.1.0 — Claim Audit

**Release line:** HACP 1.1.0
**Stage:** R6 — Claim audit
**Contract boundary:** HACP 1.0.0 Variant A inherited floor + HACP-Enforcement revision 2 Active + HC2-55 advertised under explicit revision-2 identity
**Status:** R6 claim-surface assessment

## 1. Purpose

This document records the HACP 1.1.0 R6 public-claim audit.

The audit determines whether current public claims remain no broader than the established HACP 1.1.0 release contract:

```text
HACP 1.0.0 Variant A inherited floor
+
HACP-Enforcement revision 2 Active
+
HC2-55 advertised under explicit revision-2 identity
```

The audit is evidence-bounded.

It does not authorize production changes, new protocol semantics, new conformance vectors, wire/object migration, historical release rewriting, or expansion of the HACP 1.1.0 release scope.

The governing engineering rule remains:

```text
NO PRODUCTION CHANGE WITHOUT NORMATIVE BASIS AND PROVEN RED
```

## 2. Audit scope

The reviewed public claim surface includes:

```text
README.md
PROFILES.md
versioning.md
profiles/
docs/release/
docs/conformance/
docs/ietf/
```

The audit reviewed current and historical claims involving:

```text
HACP 1.0 / 1.0.0
HACP 1.1 / 1.1.0
HACP-Core
canonical 38/38
exact reason-code correspondence
HACP-Enforcement
Enforcement revision 1
Enforcement revision 2
Active / Superseded / Legacy
HC2 / HC2-55 / 55/55
conformance-suite completeness
HTTP request binding
URI normalization
hacp_version
wire/object identity
Humanist Core 2.0
```

Historical statements were interpreted in their original release or engineering-stage context.

Historical documents were not treated as stale merely because a later lifecycle transition changed current state.

## 3. Claim classification model

Suspicious or potentially ambiguous statements were classified using:

```text
CURRENT CORRECT CLAIM
BOUNDED CLAIM
HISTORICAL CLAIM
NEGATIVE / NON-CLAIM
STALE DOCUMENTATION
OVERCLAIM
AMBIGUOUS CLAIM
NORMATIVE CONTRADICTION
OUT OF SCOPE
```

No production or normative change was authorized merely because a wording issue was identified.

## 4. Current HACP 1.1.0 claim boundary

The current release line establishes:

```text
HACP 1.0.0 Variant A
→ inherited decision-level floor

HACP-Enforcement revision 1
→ Superseded predecessor
→ historical normative lineage

HACP-Enforcement revision 2
→ Active successor
→ current preferred HACP-Enforcement conformance target

HC2-55
→ explicitly bound to HACP-Enforcement revision 2
→ bounded HTTP request-binding evidence
```

The retained wire/object family remains:

```text
hacp_version = "0.9"
```

Enforcement revision identity remains independent from HACP wire/object identity.

## 5. HACP 1.0.0 inherited floor

The HACP 1.0.0 Variant A release remains the immutable inherited floor.

Its established executable boundary remains:

```text
HACP-Core v0.9.2
canonical vector set: core-0.9.2
canonical decision-level result: 38/38
```

HACP 1.1.0 does not reinterpret that result as exact historical reason-code `38/38`.

Historical HACP 1.0.0 release records that describe Enforcement revision 2 as draft, non-active, or non-promoted remain historically correct for the HACP 1.0.0 release point.

No historical HACP 1.0.0 release document was found to require rewriting solely because Enforcement revision 2 was activated later.

## 6. Enforcement revision lifecycle

The current public profile surface consistently identifies:

```text
Revision 1
→ Superseded

Revision 2
→ Active
```

Revision 1 does not automatically become `Legacy`.

A transition to `Legacy` requires separate compatibility, deployment, or historical-support justification.

No current public claim was found that silently substitutes revision-2 semantics for revision 1.

No current public claim was found that treats `hacp_version` as the Enforcement revision identifier.

## 7. HC2-55 claim boundary

HC2-55 is the current revision-bound executable evidence set for the explicitly covered Enforcement revision-2 HTTP request-binding semantics.

Its established identity is:

```text
Profile: HACP-Enforcement
Revision: 2
Evidence set: HC2-55
Total vectors: 55
Canonicalization: JCS-RFC8785
Digest algorithm: SHA-256
Vector-set digest:
sha256:fcf2b2ee93bf2623c0e088d8b02527713f517220bb63cb0f77b08ed3d2c3ba8a
```

The recertified result is:

```text
bake/check: 55/55 PASS
black-box:   55/55 PASS
```

HC2-55 establishes only the explicitly defined HTTP request-binding semantics covered by those cases.

HC2-55 does not establish:

```text
the complete mandatory HACP-Enforcement revision 2 conformance suite
general URI normalization conformance
HACP-Core replacement
HACP-Core manifest extension
wire/object migration
new HACP object schemas
```

Passing all 55 HC2 cases therefore does not imply conformance beyond the established revision-bound evidence surface.

## 8. URI and request-target claims

The current Enforcement revision-2 profile defines narrow request-target binding rules.

The reviewed wording explicitly excludes broader inference concerning:

```text
general URI normalization
general percent-decoding
dot-segment processing
router or framework path cleaning
generic slash normalization
generic query-parameter equivalence
scheme or authority reconstruction
```

No current general URI-normalization conformance claim was established.

Occurrences of phrases such as `URI normalization is implied` found by broad text search were reviewed in context and were part of explicit negative constructions such as:

```text
No percent-decoding, repair, or additional URI normalization is implied.
```

They are therefore classified as `NEGATIVE / NON-CLAIM`, not as overclaims.

## 9. Wire/object and release-version claims

The audit found the current version domains to remain distinct:

```text
HACP release line
!=
HACP wire/object version
!=
Enforcement profile revision
!=
conformance evidence-set identity
!=
runner protocol version
!=
implementation/package version
```

HACP 1.1.0 does not imply:

```text
hacp_version = "1.0"
```

The current wire/object family remains:

```text
hacp_version = "0.9"
```

No wire/object migration claim was established.

## 10. Humanist Core 2.0 and deferred scope

HACP 1.1.0 is not Humanist Core 2.0.

The audit found no current release-facing claim that HACP 1.1.0 completes or implements:

```text
Humanist Core 2.0
AuthorityRoot
DelegationGrant
Semantic Checkpoint 2.0
general URI normalization
control-plane persistence
control-plane transport authentication
Core exact reason-code 38/38
```

Those items remain outside the HACP 1.1.0 release contract unless separately established by future normative work.

## 11. R6-FINDING-001 — Capability discovery wording

**Surface:** `versioning.md` §5 Capability Discovery
**Classification:** `AMBIGUOUS CLAIM`

The generic capability-discovery wording states that, in the absence of explicit discovery, verifiers default to strict validation based on the `hacp_version` present in the payload.

For generic HACP wire/object validation, strict validation of `hacp_version` remains correct.

For `HACP-Enforcement`, however, the current revision-identity rules establish that:

```text
hacp_version
!=
Enforcement revision identity
```

and:

```text
a claim MUST NOT rely on hacp_version alone
as a substitute for Enforcement revision identity
```

Therefore, the generic wording could be misread as allowing `hacp_version` alone to resolve a revision-sensitive Enforcement identity.

Disposition:

```text
production defect:             NO
wire/object defect:            NO
normative semantic defect:     NO
documentation ambiguity:       YES
docs-only clarification:       AUTHORIZED
```

Required clarification:

- preserve strict validation of `hacp_version`;
- preserve the generic statement that absence of discovery is not proof of absence of support;
- explicitly state that `hacp_version` alone cannot infer an applicable Enforcement revision;
- preserve the revision-specific identity rules in `profiles/enforcement-identity.md`.

## 12. R6-FINDING-002 — Generic conformance-suite wording

**Surface:** `PROFILES.md` §5 Claim Rules
**Classification:** `AMBIGUOUS CLAIM`

The generic rule currently states:

```text
Claims MUST NOT assert a profile whose conformance suite has not been passed.
```

The current Enforcement revision-2 model distinguishes:

```text
profile revision
required conformance evidence
vector-set identity
suite completeness
```

HC2-55 is explicitly bounded revision-2 evidence and is explicitly not established as the complete mandatory Enforcement revision-2 conformance suite.

The generic wording could therefore be read as assuming one complete suite for every valid profile claim.

Disposition:

```text
production defect:             NO
Enforcement behavior defect:   NO
HC2 evidence defect:           NO
suite-completeness claim:      NO
documentation ambiguity:       YES
docs-only clarification:       AUTHORIZED
```

Required clarification:

- make the generic rule depend on the applicable conformance requirements and required evidence for the specific profile or profile revision;
- do not declare HC2-55 to be a complete mandatory revision-2 suite;
- do not introduce a new conformance model.

## 13. Related observations not classified as independent findings

The following reviewed items do not establish additional R6 findings.

### PROFILES.md capability discovery

The generic statement that absence of profile discovery is not evidence of absence of support does not itself infer an Enforcement revision.

It is related to R6-FINDING-001 but is not independently contradictory.

### HACP-SPEC-0.9-draft.md filename

The retained filename is a document identifier.

The audit did not establish that the filename itself represents the current HACP-Core lifecycle status as draft.

No rename is authorized by R6.

### Historical activation records

Earlier HACP 1.1.0 stage records correctly describe the lifecycle state that existed at the time of those stages.

For example, R1 correctly records revision 2 as not yet active before the activation pack.

Those statements are classified as `HISTORICAL CLAIM`, not stale documentation.

## 14. Claim-audit results

The audit established:

```text
current correct / bounded claims:        established
historical claims requiring rewrite:    0
ambiguous current claims:                2
overclaims:                              0
normative contradictions:               0
production defects:                     0
production REDs:                        0
wire/object migration claims:           0
general URI normalization claims:       0
complete HC2-55 / Rev2 suite claims:    0
Humanist Core 2.0 completion claims:    0
automatic Revision-1 Legacy claims:     0
silent Revision-2 substitution claims:  0
```

The two ambiguous claims are documentation-level reconciliation items only.

They do not authorize production, protocol-semantic, profile-semantic, vector, schema, runner, or wire/object changes.

## 15. Authorized R6 corrections

R6 authorizes exactly two narrow documentation clarifications:

```text
versioning.md §5 Capability Discovery
PROFILES.md §5 Claim Rules, rule 2
```

The corrections MUST NOT alter:

```text
HACP-Enforcement revision-2 semantics
HACP-Enforcement revision lifecycle
HC2-55 vector semantics
HC2-55 vector count or digest
HACP-Core vectors or manifest
hacp_version
signed-object schemas
runner protocol
sidecar production behavior
historical release records
```

No other correction is authorized by this audit.

## 16. R6 exit assessment

The HACP 1.1.0 release-facing claim surface remains bounded by established evidence.

After the two documentation clarifications identified in this record, the current public claim surface satisfies:

```text
claims <= evidence
```

The established HACP 1.1.0 contract remains:

```text
HACP 1.0.0 Variant A inherited floor
+
HACP-Enforcement revision 2 Active
+
HC2-55 advertised under explicit revision-2 identity
```

with:

```text
wire/object family = 0.9
Humanist Core 2.0 = NOT CLAIMED
general URI normalization = NOT CLAIMED
Core exact reason-code 38/38 = NOT CLAIMED
HC2-55 complete mandatory Rev2 suite = NOT CLAIMED
```

R6 requires no production change.

```text
R6 CLAIM AUDIT:
PASS

PRODUCTION CHANGES AUTHORIZED:
0

PROFILE SEMANTIC CHANGES AUTHORIZED:
0

WIRE/OBJECT CHANGES AUTHORIZED:
0

VECTOR CHANGES AUTHORIZED:
0

DOCS-ONLY CLARIFICATIONS AUTHORIZED:
2
```
