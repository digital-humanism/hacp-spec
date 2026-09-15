# HACP 1.1.0 — 1.0 Compatibility Assessment

**Status:** CLOSED / PASS / COMMITTED / SIGNED / PUBLISHED / CLEAN
**Stage:** R5 — 1.0 compatibility
**Release line:** HACP 1.1.0
**Contract boundary:** HACP 1.0.0 Variant A inherited floor + HACP-Enforcement revision 2 Active + HC2-55 advertised under explicit revision-2 identity
**Repository:** `hacp-spec`
**Canonical artifact:** `docs/release/HACP_1_1_COMPATIBILITY.md`

---

## 1. Purpose

This document records the HACP 1.1.0 compatibility assessment required by roadmap stage R5.

R5 determines whether activation of `HACP-Enforcement` revision 2 preserves the inherited HACP 1.0.0 Variant A floor without silently changing the meaning of predecessor Enforcement claims, collapsing independent version domains, or implying a wire/object migration.

The governing engineering rule remains:

> **NO PRODUCTION CHANGE WITHOUT NORMATIVE BASIS AND PROVEN RED.**

This stage is compatibility verification and documentation work.

It does not authorize:

```text
production changes
canonical HACP-Core vector changes
wire/object migration
signed-object schema migration
new HC2 cases
general URI normalization
dot-segment semantics
AuthorityRoot
DelegationGrant
Semantic Checkpoint 2.0
Humanist Core 2.0 claims
Core exact-reason 38/38 work
```

---

## 2. R5 compatibility objective

The HACP 1.1.0 compatibility objective is:

```text
preserve the immutable HACP 1.0.0 Variant A floor
+
activate HACP-Enforcement revision 2 explicitly
+
preserve revision-specific identity
+
prevent silent semantic substitution
+
preserve independent version domains
+
retain wire/object family 0.9
```

R5 specifically verifies that:

1. HACP 1.0.0 decision-level compatibility remains the inherited floor;
2. `HACP-Enforcement` revision 1 remains identifiable as the superseded predecessor;
3. `HACP-Enforcement` revision 2 remains identifiable as the active successor;
4. an unqualified Enforcement claim cannot silently acquire revision-2 semantics;
5. capability advertisement and conformance claims preserve applicable revision identity;
6. Enforcement revision identity is independent from `hacp_version`;
7. activation of revision 2 does not imply a wire/object transition;
8. conformance evidence remains revision-bound;
9. historical release records remain historical evidence and are not rewritten merely because lifecycle state later changed.

---

## 3. Inherited HACP 1.0.0 compatibility floor

The inherited HACP 1.0.0 floor remains Variant A:

```text
Core decision-level contract
+
reproducible canonical 38/38
+
honest documented scope
```

The inherited canonical Core identity remains:

```text
HACP-Core:
0.9.2

vector set:
core-0.9.2

wire/object family:
0.9
```

The HACP 1.0.0 specification release did not migrate signed objects to:

```text
hacp_version = "1.0"
```

A future wire/object version change requires independent normative authorization.

R5 does not reinterpret the HACP 1.0.0 release as having activated Enforcement revision 2.

Historical HACP 1.0.0 release artifacts that state:

```text
Enforcement revision 2
→ draft successor
→ not active
```

remain historically correct for that release point and MUST NOT be rewritten solely because revision 2 was activated later in the HACP 1.1.0 release line.

---

## 4. Current Enforcement lifecycle

The current Enforcement lifecycle is established as:

```text
HACP-Enforcement revision 1
→ superseded predecessor
→ historical normative lineage
```

and:

```text
HACP-Enforcement revision 2
→ active successor
→ current preferred HACP-Enforcement conformance target
```

The current normative documents are:

```text
revision 1:
profiles/enforcement.md

revision 2:
profiles/enforcement-v2.md
```

Revision 1 remains available as predecessor normative evidence.

Revision 1 was not retroactively treated as an active revision solely to create lifecycle symmetry.

### 4.1 Legacy status is not established

Revision 1 is:

```text
Superseded
```

It is not automatically:

```text
Legacy
```

Legacy status requires separate compatibility, deployment, or historical support justification.

Therefore the R5 migration guidance uses:

```text
revision 1
→ superseded predecessor
```

not:

```text
revision 1
→ legacy
```

unless a separate future compatibility assessment establishes that status.

---

## 5. Revision identity and claim semantics

`HACP-Enforcement` is one compatibility profile that may evolve through explicit normative revisions.

A conformance claim for `HACP-Enforcement` MUST identify the applicable Enforcement revision whenever materially distinct revisions exist.

Conceptually:

```text
applicable HACP specification version
+
HACP-Enforcement
+
applicable Enforcement revision
+
applicable conformance status or evidence
```

A claim MUST NOT use either of the following as a substitute for Enforcement revision identity:

```text
implementation release number
hacp_version
```

A possible human-readable representation is:

```text
HACP 1.0-Enforcement, revision 2
```

This representation is illustrative.

No canonical display grammar, punctuation, or mandatory machine-readable claim serialization is established by the revision identity model.

---

## 6. Silent substitution is prohibited

Revision 2 MUST NOT silently replace revision 1 while leaving materially distinct revisions externally indistinguishable.

The activation transition preserves explicit evidence of:

```text
predecessor revision
successor revision
activation release point
lifecycle disposition
applicable conformance identity
```

Therefore:

```text
unqualified HACP 1.0-Enforcement
!= implicit revision-2 selection
```

and:

```text
revision 2 active
!= authorization for silent revision-1 → revision-2 semantic substitution
```

R5 found no normative basis for automatic revision upgrade, automatic downgrade, fallback, or silent selection.

---

## 7. Capability advertisement and revision negotiation

An implementation that advertises support for `HACP-Enforcement` MUST make the supported revision or revisions unambiguous when multiple materially distinct revisions exist.

The identity model allows:

```text
one implementation
→ one supported Enforcement revision
```

or:

```text
one implementation
→ multiple supported Enforcement revisions
```

The model does not require every implementation to support both revision 1 and revision 2.

Capability advertisement is not itself a conformance claim:

```text
supports Enforcement revision 2
!=
conforms to Enforcement revision 2
```

Advertising multiple supported revisions does not define negotiation.

The following remain outside the current revision identity model:

```text
client/server revision selection
preferred revision field
fallback order
automatic downgrade
automatic upgrade
revision handshake
negotiation failure semantics
```

No negotiation behavior may therefore be inferred solely from the presence of multiple advertised revisions.

If revision negotiation is required in the future, it requires separate normative definition.

---

## 8. Wire/object compatibility

Enforcement profile revision is independent of the HACP wire/object version.

The retained HACP wire/object family is:

```text
hacp_version = "0.9"
```

Revision 2 activation does not, by itself, change:

```text
hacp_version
signed-object schemas
canonical HACP-Core vector identity
runner protocol
implementation package version
```

Revision 1 and revision 2 may differ in Enforcement semantics while remaining compatible with the same HACP wire/object version.

Therefore:

```text
same wire/object version
!= same Enforcement semantics
```

and:

```text
different Enforcement revision
!= automatic wire incompatibility
```

Revision 2 activation does not automatically make revision-1 implementations invalid at the HACP wire level.

Profile conformance and wire compatibility are separate dimensions.

---

## 9. Version-domain matrix

The following domains are independent and MUST NOT be collapsed into one identifier:

| Domain | Current identity / rule | R5 compatibility finding |
|---|---|---|
| HACP specification release | HACP 1.0.0 inherited floor; HACP 1.1.0 release line in preparation | Release identity does not migrate wire/object identity |
| Human-readable compatibility claim | `HACP 1.0-Enforcement` plus applicable revision | Applicable revision must be explicit when revision-sensitive |
| Enforcement profile revision | Revision 1 / Revision 2 | Independent normative lifecycle dimension |
| Enforcement lifecycle status | Rev 1 Superseded; Rev 2 Active | Lifecycle does not substitute for wire version |
| HACP wire/object version | `0.9` | Unchanged |
| Canonical HACP-Core baseline | `HACP-Core 0.9.2` | Retained |
| Canonical Core vector set | `core-0.9.2` | Retained |
| Enforcement evidence set | `HC2-55`, bound to revision 2 | Evidence identity is not profile revision |
| Vector-set digest | Exact executed fixture digest | Must remain bound to evidence record |
| Runner protocol | Protocol v1 | Independent execution-transport contract |
| Implementation version | Repository/package-specific | MUST NOT substitute for Enforcement revision identity |

No R5 evidence supports collapsing any of these domains.

---

## 10. Enforcement conformance evidence boundary

Revision-sensitive Enforcement conformance evidence MUST identify the applicable Enforcement revision.

The revision-2 evidence identity is:

```text
Profile:
HACP-Enforcement

Revision:
2

Evidence set:
HC2-55

Total vectors:
55

Canonicalization:
JCS-RFC8785

Digest algorithm:
SHA-256

Vector-set digest:
sha256:fcf2b2ee93bf2623c0e088d8b02527713f517220bb63cb0f77b08ed3d2c3ba8a
```

HC2-55 demonstrates the explicitly defined HTTP request-binding semantics covered by those 55 cases.

It does not establish:

```text
general URI normalization
complete mandatory Enforcement revision 2 conformance suite
wire/object migration
canonical HACP-Core replacement
Humanist Core 2.0 completion
```

The profile revision and executable evidence identity remain separate:

```text
Enforcement profile revision
!=
vector-set version
```

and:

```text
vector-set name
!=
revision identity
```

---

## 11. HACP-Core and Enforcement evidence remain separate

The canonical HACP-Core executable baseline and the HACP-Enforcement revision-2 evidence surface remain distinct.

The HACP 1.1.0 Enforcement activation does not:

```text
rename core-0.9.2
replace the canonical Core manifest
migrate HACP-Core to HC2-55
change the Core runner protocol
reinterpret historical Core conformance evidence
```

The HACP 1.0.0 decision-level compatibility floor therefore remains an independent release requirement.

---

## 12. Public profile and discovery surface

The current profile index identifies:

```text
profiles/enforcement-v2.md
→ HACP-Enforcement revision 2
→ Active
```

and preserves:

```text
profiles/enforcement.md
→ revision 1
→ superseded historical normative lineage
```

The public claim guidance identifies:

```text
HACP 1.0-Enforcement
+
applicable Enforcement revision
```

Generic capability discovery guidance remains intentionally general.

For `HACP-Enforcement`, the more specific revision identity rules apply:

```text
advertised HACP-Enforcement support
→ supported revision(s) must be unambiguous
```

This is not a normative contradiction.

It is a specialization of generic profile discovery for a profile family with materially distinct revisions.

R5 does not establish a need to change generic capability discovery semantics.

---

## 13. Migration guidance

The HACP 1.1.0 Enforcement transition is:

```text
HACP-Enforcement revision 1
→ superseded predecessor
→ historical normative lineage retained

HACP-Enforcement revision 2
→ active successor
→ current preferred conformance target
```

A client, implementation, verification record, capability advertisement, or conformance claim MUST NOT infer revision-2 semantics solely from:

```text
HACP 1.0-Enforcement
hacp_version = "0.9"
implementation release number
active profile family name alone
```

where revision-sensitive behavior exists.

The applicable Enforcement revision must remain explicit and externally understandable.

Revision 2 activation does not imply:

```text
automatic upgrade from revision 1
automatic downgrade to revision 1
revision negotiation
fallback behavior
wire/object version transition
schema transition
general URI-normalization compatibility
```

Both revisions may remain compatible with wire/object family `0.9` while having materially different Enforcement semantics.

An implementation is not required by the revision identity model to support both revisions.

---

## 14. Historical-document treatment

Historical release and assessment documents MUST be interpreted according to the lifecycle state at the time they were produced.

Examples include HACP 1.0.0 records that correctly describe revision 2 as:

```text
draft
not active
future successor
```

Those statements are historical evidence.

They are not current lifecycle declarations.

R5 found no justification to rewrite historical HACP 1.0.0 release records merely because the HACP 1.1.0 activation process later changed revision-2 lifecycle status.

This preserves release evidence and avoids retrospective history rewriting.

---

## 15. Compatibility classifications

### 15.1 Normative compatibility model

```text
GREEN
```

The normative model distinguishes:

```text
release identity
wire/object identity
profile revision identity
lifecycle status
conformance evidence identity
runner protocol identity
implementation identity
```

### 15.2 Revision lifecycle

```text
GREEN
```

Revision 1 and revision 2 have explicit predecessor/successor lifecycle states.

### 15.3 Silent substitution

```text
GREEN
```

Silent replacement of materially distinct revisions is explicitly prohibited.

### 15.4 Wire/object compatibility

```text
GREEN
```

No wire/object migration is implied or required by revision-2 activation.

### 15.5 Revision negotiation

```text
NOT DEFINED
```

No implicit upgrade, downgrade, fallback, or preferred-revision selection may be inferred.

This is not a compatibility defect.

### 15.6 Public discovery / claim identity

```text
GREEN
```

The profile index identifies revision 2 as Active and identifies revision 1 as the superseded historical predecessor.

Revision-aware claim identity is explicitly documented.

### 15.7 Historical wording

```text
PRESERVE
```

Historically correct predecessor release records are not current contradictions.

### 15.8 Production compatibility defect

```text
NOT ESTABLISHED
```

### 15.9 Production RED

```text
NOT ESTABLISHED
```

---

## 16. Changes authorized by R5

R5 authorizes the creation and publication of this compatibility / migration record.

R5 does not currently establish normative or production evidence requiring changes to:

```text
profiles/enforcement.md
profiles/enforcement-v2.md
profiles/enforcement-revisions.md
profiles/enforcement-identity.md
profiles/enforcement-transition.md
profiles/enforcement-conformance.md
versioning.md
PROFILES.md
README.md
signed-object schemas
wire/object version
canonical HACP-Core vectors
HC2 vectors
runner protocol
production sidecar behavior
```

Any later change to those surfaces still requires its own stage-local basis.

---

## 17. Fresh R5 regression requirement

R4 already recorded preservation of the HACP 1.0.0 decision-level floor:

```text
decision-level:
38/38 correct
```

with historical strict reason mismatch surface remaining separately classified.

R5 must not convert that inherited evidence into a fresh R5 execution result.

Fresh R5 decision-level canonical Core verification was performed on the
current R5 candidate state:

```text
command:
python .\harness\harness.py
```

observed result:
HACP Conformance Harness v0.9.2
Mode: local
38/38 PASS

Therefore:

R5 compatibility model:
ESTABLISHED

Fresh R5 HACP-Core decision-level verification:
38/38 PASS

R5 Core regression:
GREEN

---

## 18. R5 exit criteria

R5 may be formally closed when all of the following are true:

```text
[x] revision-1 predecessor lifecycle is explicit
[x] revision-2 active lifecycle is explicit
[x] applicable revision identity is required where revision-sensitive
[x] silent revision substitution is prohibited
[x] capability advertisement does not imply negotiation
[x] wire/object family remains 0.9
[x] version domains remain distinct
[x] HC2-55 remains revision-2-bound and bounded
[x] historical 1.0.0 records remain preserved as historical evidence
[x] migration guidance is recorded
[x] fresh R5 HACP-Core decision-level 38/38 verification recorded
[x] canonical artifact reviewed
[x] git diff --check PASS
[x] signed commit verified
[x] publication verified
[x] working tree clean
```

R6 MUST NOT begin before all R5 exit criteria are satisfied.

---

## 19. R5 counters

Current R5 assessment counters:

```text
production compatibility defects established:
0

production REDs established:
0

production changes authorized:
0

production changes performed:
0

profile semantic changes authorized:
0

wire/object migrations authorized:
0

wire/object migrations performed:
0

schema changes authorized:
0

canonical HACP-Core vector changes authorized:
0

HC2 vector changes authorized:
0

runner protocol changes authorized:
0

historical release documents requiring rewrite:
0

silent semantic substitution findings:
0
```

---

## 20. Current R5 disposition

Current disposition:

```text
R5 compatibility model:
ESTABLISHED

fresh R5 Core regression:
38/38 PASS / GREEN

canonical artifact:
REVIEWED

git diff --check:
PASS

commit:
b3614389aa47bf59f48d0d4ce657127152a32bab

signature:
Good

publication:
VERIFIED

HEAD == origin/main:
VERIFIED

working tree:
CLEAN

R5:
CLOSED / PASS / COMMITTED / SIGNED / PUBLISHED / CLEAN
```

R5 closure is complete. This document records the verified publication and clean repository state.

---

## 21. Final R5 closure record

R5 was formally closed after:

1. fresh HACP-Core decision-level verification: 38/38 PASS;
2. canonical compatibility artifact review;
3. git diff --check PASS;
4. signed commit creation and signature verification;
5. publication to origin/main;
6. remote identity verification;
7. clean working-tree verification.

The initial R5 compatibility artifact was published in:

b3614389aa47bf59f48d0d4ce657127152a32bab

The present document revision synchronizes the canonical artifact with the
already verified R5 publication closure state.

At publication closure:

HEAD == origin/main
working tree == clean

Final R5 status:

```text
CLOSED / PASS / COMMITTED / SIGNED / PUBLISHED / CLEAN
```
