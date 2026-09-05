# HACP 1.0.0 — Release Candidate Record

**Release line:** HACP 1.0.0
**Release stage:** R7 — Release Candidate
**Contract boundary:** §1.1 Variant A
**RC identifier:** `v1.0.0-rc.1`
**Status:** RC preparation record

---

## 1. Purpose

This record binds the HACP 1.0.0 release candidate to exact immutable source revisions and records the evidence required by Stage R7.

The release candidate is governed by the HACP 1.0.0 Variant A contract boundary:

```text
1.0.0 =
  stable public HACP-Core contract
+ reproducible 38/38 decision-level canonical conformance
+ Protocol v1 runner / strict verifier tooling
+ honest historical / exact-reason scope
+ sidecar as the current implementation of the active Enforcement profile
```

This release candidate does not activate Enforcement revision 2 and does not advertise HC2-55 as active HACP 1.0 Enforcement conformance.

---

## 2. RC Source Revisions

### `hacp-spec`

```text
commit:
e3b6556ad7c53c3cb22886da84b08f3d24368df8

subject:
docs: record HACP 1.0 final verification matrix

branch:
main

origin/main:
e3b6556ad7c53c3cb22886da84b08f3d24368df8

working tree at R7 readiness inspection:
clean

commit signature:
GOOD

signature type:
SSH / ED25519

signing key:
SHA256:0BnXknauq0S7xwJ8Fi48yGHQD8QClVi1+MO/4ymQDgE
```

### `hacp-sidecar`

```text
commit:
b6b9e9836a182deb832fa42e33ca9c239790fac1

subject:
fix: enforce quantity and destination scope boundaries

branch:
main

origin/main:
b6b9e9836a182deb832fa42e33ca9c239790fac1

working tree at R7 readiness inspection:
clean

commit signature:
GOOD

signature type:
SSH / ED25519

signing key:
SHA256:0BnXknauq0S7xwJ8Fi48yGHQD8QClVi1+MO/4ymQDgE
```

### `humanist-core`

```text
commit:
e4734bfba41171e97758a9c193dcd4d1ce1984c5

subject:
docs: polish README release status

branch:
main

origin/main:
e4734bfba41171e97758a9c193dcd4d1ce1984c5

working tree at R7 readiness inspection:
clean

commit signature:
NONE
```

The unsigned `humanist-core` source commit is recorded as an accepted non-blocking R7 release-lineage deviation.

No history rewrite, amend, synthetic commit, force-push, or version bump was performed to alter the already verified source revision.

The exact commit remains immutable and remote-aligned and was part of the successful R6 technical verification baseline.

---

## 3. R7 Source Readiness Summary

| Repository | Exact source revision | `origin/main` aligned | Clean | Commit signed | R6 technically verified | R7 disposition |
|---|---|---:|---:|---:|---:|---|
| `hacp-spec` | `e3b6556ad7c53c3cb22886da84b08f3d24368df8` | YES | YES | YES | YES | ACCEPT |
| `hacp-sidecar` | `b6b9e9836a182deb832fa42e33ca9c239790fac1` | YES | YES | YES | YES | ACCEPT |
| `humanist-core` | `e4734bfba41171e97758a9c193dcd4d1ce1984c5` | YES | YES | NO | YES | ACCEPT WITH RECORDED LINEAGE DEVIATION |

No source-tree cleanliness blocker remains.

---

## 4. Canonical Manifest and Vector Identity

Canonical conformance manifest:

```text
path:
harness/conformance_manifest.json

profile:
HACP-Core

spec_version:
0.9.2

vector_set:
core-0.9.2

wire/object family:
0.9

Runner Protocol:
1

canonical vector count:
38
```

Canonical vector digest:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

The digest was independently recomputed during R6 and matched the manifest value exactly.

The HACP 1.0.0 specification release version remains distinct from the canonical HACP-Core / wire / vector version dimensions.

---

## 5. Final Verified Technical Baseline

### Canonical HACP-Core

```text
hacp-go:
38/38 PASS

hacp-ts:
38/38 PASS

humanist-core:
38/38 PASS
```

### TypeScript regression

```text
44/44 PASS
```

### Python regression

```text
319 PASS
5 expected external-sidecar SKIP
0 FAIL
```

### Python coverage

```text
1554 statements
0 missed

420 branches
0 partial

100% statement coverage
100% branch coverage
```

### Python ↔ Go external E2E

```text
5/5 PASS
```

### Sidecar canonical decision baseline

```text
38/38 decision outcomes correct
```

### Sidecar strict exact-reason baseline

```text
15/38 exact reason PASS
23/38 classified exact-reason mismatch
38/38 decision outcomes correct
```

The exact-reason result is an accepted and classified Variant A baseline.

Exact reason-code `38/38` is not a HACP 1.0.0 Variant A release requirement.

### Protocol v1 strict verifier self-test

```text
PASS
```

### Build / package surface

```text
hacp-go build:
PASS

hacp-ts build:
PASS

hacp-sidecar build:
PASS

hacp-sidecar conformance runner build:
PASS

humanist-core wheel build:
PASS
```

Verified wheel filename:

```text
humanist_core-0.5.0-py3-none-any.whl
```

The R6 wheel was an ephemeral verification artifact and was not retained as an immutable repository-tracked release artifact.

---

## 6. Build Environment Snapshot

The R7 readiness environment was:

```text
Operating system / architecture:
Windows / amd64

Go:
go1.26.5 windows/amd64

Python:
3.13.14

pytest:
9.1.1

Protocol Buffers compiler:
libprotoc 35.1

protoc-gen-go:
v1.36.12

Node.js:
v24.19.0

npm:
11.17.0

Git:
2.54.0.windows.1
```

This snapshot records the environment used for R7 release preparation.

Clean-clone and isolated-environment reproducibility are Stage R8 responsibilities and are not asserted by this record.

---

## 7. Release Artifact Hashes

No binary, wheel, archive, executable, or package artifact is tracked in the `hacp-spec` release tree for this RC preparation state.

Therefore:

```text
repository-tracked binary/package artifact hashes:
N/A
```

The canonical vector digest remains the immutable content digest relevant to the canonical conformance input surface:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

If independently published binary/package artifacts are added later in the release process, their hashes must be recorded separately before publication.

---

## 8. Release Blocker Ledger Snapshot

The substantive release-blocker state at R7 preparation is:

```text
R1-established production defects:
6

fixed:
6/6

unresolved production defects:
0

unresolved HACP 1.0.0 blockers:
0
```

`CORE-RUNTIME-005` remains:

```text
status:
HOLD

classification:
normative conflict

1.0.0 blocker:
NO

target:
1.0.n / later adjudication
```

The blocker ledger still contains stale administrative header metadata:

```text
Current stage: R1
Status: ACTIVE
```

This metadata does not alter the substantive ledger conclusion and is not treated as an R7 release blocker.

No opportunistic ledger rewrite was performed solely for R7 cosmetic alignment.

---

## 9. Enforcement Revision 2 / HC2 Snapshot

The successor enforcement surface remains:

```text
Enforcement revision 2:
DRAFT successor
NOT ACTIVE

HC2:
draft successor evidence

HC2-55:
NOT active HACP 1.0 Enforcement conformance
```

The HACP 1.0.0 RC may ship these draft successor artifacts in the source tree, but they are not part of the advertised active HACP 1.0 Enforcement contract.

The RC does not:

```text
activate Enforcement revision 2
promote HC2
advertise HC2-55 as mandatory HACP 1.0 conformance
make HC2 a release gate
```

---

## 10. Known Non-Blocking Issues and Deviations

### R7-LIN-001 — `humanist-core` unsigned source HEAD

```text
repository:
humanist-core

commit:
e4734bfba41171e97758a9c193dcd4d1ce1984c5

commit signature:
NONE

production impact:
NONE

technical verification impact:
NONE

1.0.0 blocker:
NO

disposition:
ACCEPTED NON-BLOCKING RELEASE-LINEAGE DEVIATION
```

Recent inspected `humanist-core` history is also unsigned, including the commits referenced by the historical `v0.5.0` and `v0.5.0-rc.1` release tags.

Those historical tags are lightweight and unsigned.

No evidence established a requirement to rewrite the verified `humanist-core` source history.

The R7 release lineage instead introduces a signed immutable RC release anchor.

### R7-LEDGER-001 — stale administrative blocker-ledger header

```text
impact:
administrative metadata only

substantive blocker state:
0 unresolved HACP 1.0.0 blockers

1.0.0 blocker:
NO
```

### CORE-RUNTIME-005

```text
status:
HOLD

1.0.0 blocker:
NO
```

No production change is authorized without new normative adjudication.

---

## 11. RC Tagging Model

The HACP 1.0.0 RC release anchor is owned by `hacp-spec`.

The intended RC tag is:

```text
v1.0.0-rc.1
```

The tag model is:

```text
annotated
SSH-signed
immutable
```

The tag must point to the exact signed `hacp-spec` commit that contains this completed RC record.

That commit binds the other participating repositories by their full immutable commit IDs recorded in Section 2.

Separate RC tags in `hacp-sidecar` and `humanist-core` are not required by the current Variant A R7 model.

Historical tag practice is not treated as proof of current R7 signature compliance:

```text
hacp-spec v0.9.3-rc.1:
annotated but unsigned

hacp-spec v0.9.3:
lightweight / unsigned

hacp-sidecar v0.5.0:
lightweight / unsigned

hacp-sidecar v0.5.0-rc.1:
lightweight / unsigned

humanist-core v0.5.0:
lightweight / unsigned

humanist-core v0.5.0-rc.1:
lightweight / unsigned
```

R7 therefore establishes the signed RC tag requirement explicitly rather than rewriting historical release objects.

---

## 12. RC Tag Verification

This section must be completed only after the RC record itself is committed and the signed RC tag is created.

Expected tag:

```text
v1.0.0-rc.1
```

Required verification:

```text
tag object:
annotated

signature:
GOOD

signature type:
SSH / ED25519

target:
exact commit containing this completed RC record
```

Until those facts are established, the tag is:

```text
NOT YET CREATED
```

---

## 13. R7 Exit Assessment

At RC record preparation time:

```text
all RC source heads explicitly chosen:             YES
all source heads remote-aligned:                    YES
all source trees clean:                             YES
exact commits recorded:                            YES
source commit signature state recorded:            YES
humanist-core lineage deviation classified:        YES
canonical manifest identity recorded:              YES
canonical vector digest recorded:                  YES
build environment recorded:                        YES
artifact-hash applicability classified:            YES
known non-blocking issues recorded:                YES
CORE-RUNTIME-005 recorded correctly:               YES
rev2 / HC2 draft status recorded correctly:        YES
release blocker snapshot recorded:                 YES
unresolved Variant A blockers:                     0

RC record committed:
NOT YET

signed RC tag created:
NOT YET

signed RC tag verified:
NOT YET
```

Therefore:

```text
R7 STATUS:
IN PROGRESS
```

Remaining R7 actions are release-only:

```text
1. finalize this RC record;
2. verify public hygiene and diff;
3. commit the RC record with a signed commit;
4. verify the signed commit;
5. create annotated SSH-signed tag v1.0.0-rc.1;
6. verify the tag signature and exact target;
7. verify clean working tree;
8. push the signed commit and tag;
9. verify remote refs;
10. update this record only if the final immutable evidence model requires an additional binding step.
```

No R8 clean-clone work is authorized until R7 is closed.

---

## 14. Release Candidate Boundary Statement

The HACP 1.0.0 release candidate represents:

> A stable HACP-Core decision-level contract with reproducible 38-vector canonical conformance, Protocol v1 verification tooling, an honestly classified exact-reason baseline, passing implementation regressions and interoperability evidence, and zero unresolved blocker established against the Variant A release boundary.

It does not represent activation of Enforcement revision 2 or promotion of HC2-55 to the active HACP 1.0 Enforcement contract.
