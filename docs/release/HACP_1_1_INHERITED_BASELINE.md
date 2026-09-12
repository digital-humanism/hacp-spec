# HACP 1.1.0 — Inherited Baseline

**Release line:** HACP 1.1.0 preparation
**Roadmap stage:** R0
**Master-plan phase:** S0 — Inherit HACP 1.0.0 baseline
**Status:** Accepted inherited baseline
**Date:** 2026-09-12

## 1. Purpose

This document records the inherited baseline for HACP 1.1.0.

It establishes the immutable HACP 1.0.0 Variant A floor, current signed repository identities, current public-claim state, and fresh reproduction of the inherited canonical decision-level conformance baseline.

This document does not activate HACP-Enforcement revision 2, does not advertise HC2-55 as active conformance, does not change production behavior, and does not change the HACP wire/object version.

The governing engineering rule remains:

```text
NO PRODUCTION CHANGE WITHOUT NORMATIVE BASIS AND PROVEN RED
```

---

## 2. Release-line boundary

HACP 1.0.0 is the immutable inherited floor for HACP 1.1.0.

The inherited HACP 1.0.0 contract is:

```text
stable HACP-Core decision-level contract
+
reproducible canonical 38/38
+
honest documented release scope
+
existing sidecar Enforcement implementation
```

The HACP 1.0.0 contract boundary remains:

```text
Variant A
```

The following version domains remain distinct:

```text
HACP specification release:
1.0.0

canonical HACP-Core:
0.9.2

canonical vector set:
core-0.9.2

HACP wire/object family:
0.9
```

The HACP 1.0.0 specification release did not imply:

```text
hacp_version = "1.0"
```

A future wire/object version change requires separate normative authorization.

---

## 3. Current repository identities

Fresh read-only repository identity capture was performed on 2026-09-12.

### 3.1 hacp-spec

```text
repository:
hacp-spec

branch:
main

HEAD:
c468c9bb0427448e564bcf3e7d9c8a3a004b8513

origin/main:
c468c9bb0427448e564bcf3e7d9c8a3a004b8513

ahead / behind:
0 / 0

working tree:
CLEAN

HEAD commit:
docs: add HACP 1.0 release notes

commit signature:
Good
```

The current `hacp-spec` HEAD remains exactly the stable HACP 1.0.0 tag target.

### 3.2 hacp-sidecar

```text
repository:
hacp-sidecar

branch:
main

HEAD:
1bc10acbe79620a165cee573c5b260cd8639280f

origin/main:
1bc10acbe79620a165cee573c5b260cd8639280f

ahead / behind:
0 / 0

working tree:
CLEAN

HEAD commit:
test: cover representative reason branches

commit signature:
Good
```

### 3.3 humanist-core

```text
repository:
humanist-core

branch:
main

HEAD:
6d9ae82ed7fefe47b395e4ef68d0a18807866473

origin/main:
6d9ae82ed7fefe47b395e4ef68d0a18807866473

ahead / behind:
0 / 0

working tree:
CLEAN

HEAD commit:
docs: fix HACP sidecar integration links

commit signature:
Good
```

All three current repository identities are therefore:

```text
full SHA captured
signed commit verified
HEAD = origin/main
working tree clean
```

No planning snapshot or historical R11 anchor is being substituted for the fresh R0 record.

---

## 4. Immutable HACP 1.0.0 release identity

Fresh verification of the stable `hacp-spec` release tag established:

```text
tag:
v1.0.0

tag object:
009cb86e2adada511ebcb3b2a6f343be8c968971

peeled commit:
c468c9bb0427448e564bcf3e7d9c8a3a004b8513

tag signature:
Good

tagged commit signature:
Good
```

The observed tag object and peeled target exactly match the recorded HACP 1.0.0 stable release identity.

No retagging, tag movement, or release-history rewrite was performed.

---

## 5. Public-claim audit

Fresh inspection of the current public `hacp-spec` tree established the following state.

### HACP 1.0.0 status

```text
HACP specification release:
1.0.0

status:
Stable
```

Result:

```text
ESTABLISHED
```

### Variant A boundary

Current release, conformance, verification, and release-note material continues to identify the HACP 1.0.0 release boundary as:

```text
Variant A
```

Result:

```text
ESTABLISHED
```

### Wire/object version

Current public release and profile material continues to identify:

```text
wire/object version:
0.9
```

The specification release version and wire/object version remain explicitly separate.

Result:

```text
ESTABLISHED
```

### Enforcement revision 2 lifecycle

Current revision-2 profile material records:

```text
Status: Draft — not yet active
```

The transition model also records that the revision-2 Activation Gate is not yet closed.

Result:

```text
Enforcement revision 2:
NOT ACTIVE
```

### HC2-55 advertised conformance

Current release and conformance material continues to classify HC2-55 as draft successor evidence and not as the active HACP 1.0 Enforcement conformance contract.

Result:

```text
HC2-55:
NOT PROMOTED
NOT ADVERTISED AS ACTIVE CONFORMANCE
```

### Humanist Core 2.0

No reviewed release-facing claim establishes Humanist Core 2.0 as completed or released.

Result:

```text
Humanist Core 2.0:
NOT CLAIMED
```

Overall public-claim result:

```text
PUBLIC CLAIMS <= CURRENT EVIDENCE
```

---

## 6. Canonical inherited baseline

The current canonical manifest identifies:

```text
spec_version:
0.9.2

profile:
HACP-Core

vector_set:
core-0.9.2

canonicalization:
JCS-RFC8785

digest_algorithm:
SHA-256

vector_digest:
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58

total_vectors:
38

generated_at:
2026-08-17T19:28:00Z
```

Fresh integrity verification was executed with:

```text
python .\tools\bake_vector.py --check
```

Observed result:

```text
CHECK RESULTS:
38/38 passed
```

Fresh canonical decision-level execution was performed with:

```text
python .\harness\harness.py --mode local
```

Observed result:

```text
RESULTS:
38/38 passed
```

The `hacp-spec` working tree remained clean after verification.

Therefore:

```text
canonical vector integrity:
PASS — 38/38

canonical decision-level inherited baseline:
PASS — 38/38

working-tree mutation:
NONE
```

This result reproduces the HACP 1.0.0 Variant A canonical decision-level floor.

It does not establish or require exact historical reason-code 38/38 correspondence.

---

## 7. Post-1.0 closure inheritance

The restricted HACP 1.0.n hardening backlog was inspected read-only.

Backlog status precedence for HACP 1.1.0 planning is:

```text
latest explicit closure / disposition
→ detailed item body / child disposition
→ summary table only as historical index
```

Accordingly, stale summary rows do not override later closure evidence.

### R8-DOC-003

Latest explicit disposition:

```text
CLOSED
```

Closure evidence includes:

```text
hacp-sidecar
4cd664f2b0e960ef2e5db272523f924efbd496eb

docs: document native external E2E startup
```

Disposition for HACP 1.1.0:

```text
DO NOT REOPEN
DO NOT ENTER R1
```

### REPRESENTATIVE-REASON-COVERAGE

Latest explicit disposition:

```text
CLOSED
```

The focused evaluator coverage gap was closed for:

```text
TOKEN_EXPIRED
direct ENVELOPE_REVOKED
```

No production defect was established and no production semantics were changed.

Disposition for HACP 1.1.0:

```text
DO NOT REOPEN
DO NOT ENTER R1
```

### VECTOR-REACHABILITY-CLEANUP

A stale summary row still records this workstream as `OPEN`.

The later explicit closure review controls current planning status and records:

```text
VECTOR-REACHABILITY-CLEANUP
STATUS: CLOSED
```

Closure accounting:

```text
original strict cases reviewed:
17

unclassified cases:
0

current production defects established:
0

new production REDs warranted:
0

canonical vector edits authorized:
0
```

Disposition for HACP 1.1.0:

```text
CLOSED
DO NOT REOPEN
DO NOT ENTER R1
```

Historical strict mismatch inventory must not be mechanically converted into the HACP 1.1.0 activation ledger.

---

## 8. Known HOLD and CANDIDATE boundaries

The following deferred items remain bounded and do not become HACP 1.1.0 activation blockers by default.

### CORE-RUNTIME-005

```text
status:
HOLD

classification:
normative conflict

production defect:
NOT ESTABLISHED

production RED:
NOT ESTABLISHED

production change:
NOT AUTHORIZED
```

It may enter the HACP 1.1.0 activation ledger only if R1 establishes that it lies on the advertised revision-2 path.

Otherwise it remains explicitly dispositioned outside the activation blocker set.

### CORE-INV7-006 / PARENT-ENVELOPE-REVOCATION-INHERITANCE

These names refer to one underlying normative question:

```text
parent-envelope revocation inheritance
```

Current disposition:

```text
HOLD / normative ownership unresolved

current authoritative normative owner:
NOT ESTABLISHED

production RED:
NOT AUTHORIZED

production change:
NOT AUTHORIZED
```

Alias rule:

```text
CORE-INV7-006
PARENT-ENVELOPE-REVOCATION-INHERITANCE

→ one question
→ one HOLD
→ never two activation blockers
```

It enters R1 only if the advertised revision-2 contract requires this semantic.

### Other non-default activation items

The following remain outside the R1 activation blocker set absent new concrete evidence:

```text
CONTROL-PLANE-EXTERNAL-PERSISTENCE
DURABLE-MULTI-REGION-PERSISTENCE
CONTROL-PLANE-TRANSPORT-AUTHENTICATION
DOT-SEGMENT-SEMANTICS
R11-ACTION-TOOL-IDENTITY
PROTOCOL-V1-ADAPTER-INPUT-SEMANTICS
```

Existing dispositions also remain inherited:

```text
final canonical claim-string syntax
→ not a standalone activation blocker

final capability-discovery transport schema
→ not a standalone activation blocker

final universal conformance manifest schema
→ not required as a standalone activation blocker
```

None of these dispositions authorizes production change.

---

## 9. HACP 1.1.0 activation root

The only HACP 1.1.0 activation root inherited into the next stage is:

```text
REV2-ACTIVATION-READINESS
```

Its activation-readiness children are:

```text
REV2-NORMATIVE-CLOSURE-REVIEW
REV2-SUITE-COMPLETENESS-ASSESSMENT
REV2-REMAINING-BLOCKER-REVIEW
HC2-55-ADVERTISED-CONFORMANCE
```

These are inputs to R1 / S1.

They are not resolved merely by this inherited-baseline record.

R1 must classify genuine activation prerequisites as:

```text
YES
NO
UNRESOLVED
```

R1 remains open while:

```text
UNRESOLVED > 0
```

R2 becomes eligible only when:

```text
UNRESOLVED = 0
```

---

## 10. R0 exit assessment

The R0 inherited-baseline evidence establishes:

```text
immutable HACP 1.0.0 v1.0.0 identity verified:
YES

hacp-spec current full signed identity verified:
YES

hacp-sidecar current full signed identity verified:
YES

humanist-core current full signed identity verified:
YES

repository cleanliness and remote alignment verified:
YES

Variant A inheritance verified:
YES

release-version / wire-object-version separation verified:
YES

canonical 38/38 inherited baseline verified:
YES

public claims <= evidence:
YES

closed post-1.0 work remains closed:
YES

activation root identified without historical blocker resurrection:
YES
```

Final review state:

```text
HACP_1_1_INHERITED_BASELINE.md reviewed:
YES
```

Therefore the current R0 decision is:

```text
R0 EVIDENCE:
COMPLETE

R0 CANONICAL ARTIFACT:
REVIEWED AND ACCEPTED

R0 EXIT:
CLOSED
```

With R0 closed:

```text
R1:
AUTHORIZED TO BEGIN

R2:
NOT AUTHORIZED UNTIL R1 UNRESOLVED = 0

REV2 ACTIVATION:
NOT AUTHORIZED

HC2-55 ADVERTISEMENT:
NOT AUTHORIZED

PROFILE Draft → Active EDIT:
NOT AUTHORIZED

PRODUCTION CHANGE:
NOT AUTHORIZED WITHOUT NORMATIVE BASIS AND PROVEN RED

WIRE/OBJECT VERSION CHANGE:
NOT AUTHORIZED
```

With this document reviewed and accepted, R0 is closed and R1 may begin with creation of:

```text
docs/conformance/HACP_1_1_ACTIVATION_BLOCKER_LEDGER.md
```
