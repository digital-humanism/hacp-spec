# HACP 1.0 Final Verification Matrix

**Release target:** HACP 1.0.0
**Contract boundary:** §1.1 Variant A
**Stage:** R6 — Final Verification Matrix
**Status:** COMPLETE — READY FOR SIGNED R6 CLOSURE

---

## 1. Purpose

This document records the final R6 verification evidence for the HACP 1.0.0 Variant A release boundary.

The governing release contract is intentionally narrow:

```text
HACP 1.0.0 =
  stable public HACP-Core contract
+ reproducible decision-level canonical conformance
+ Protocol v1 runner / strict verifier tooling
+ honest historical and exact-reason verification scope
+ sidecar as the current implementation of the active Enforcement profile
```

The following are not automatic HACP 1.0.0 release requirements:

```text
Enforcement revision 2 active
HC2 active
HC2 mandatory advertised conformance
exact reason-code 38/38
hacp_version = "1.0"
canonical HACP-Core migration from v0.9.2
canonical vector-set migration from core-0.9.2
automatic package-version alignment
```

R6 is a verification stage. No production change, canonical-vector change, active-profile promotion, version migration, or opportunistic cleanup was authorized by this stage.

---

## 2. Candidate Repository Baselines

### 2.1 `hacp-spec`

```text
Repository:      hacp-spec
Branch:          main
Commit:          407ce8932cf8ac20b0a02e4c836f40c7c923c9a8
Short commit:    407ce89
Subject:         docs: complete HACP 1.0 documentation hygiene audit
origin/main:     aligned
Working tree:    clean
Commit signature: Good Git SSH signature
Signing key:     SHA256:0BnXknauq0S7xwJ8Fi48yGHQD8QClVi1+MO/4ymQDgE
```

Classification: **PASS**.

### 2.2 `hacp-sidecar`

```text
Repository:      hacp-sidecar
Branch:          main
Commit:          b6b9e9836a182deb832fa42e33ca9c239790fac1
Short commit:    b6b9e98
Subject:         fix: enforce quantity and destination scope boundaries
origin/main:     aligned
Working tree:    clean
Commit signature: Good Git SSH signature
Signing key:     SHA256:0BnXknauq0S7xwJ8Fi48yGHQD8QClVi1+MO/4ymQDgE
```

Classification: **PASS**.

### 2.3 `humanist-core`

```text
Repository:      humanist-core
Branch:          main
Commit:          e4734bfba41171e97758a9c193dcd4d1ce1984c5
Short commit:    e4734bf
Subject:         docs: polish README release status
origin/main:     aligned
Working tree:    clean
Commit signature: none
```

Technical verification at this commit passed all R6 mandatory surfaces documented below.

The unsigned current HEAD is recorded separately as `R6-HDR-001` under release-lineage / R7-readiness deviation handling. No existing Variant A rule was found that makes this fact an R6 technical blocker, and no history rewrite or synthetic fix was performed.

Classification: **PASS with carried R7 lineage deviation**.

---

## 3. Canonical Manifest Identity

Manifest:

```text
harness/conformance_manifest.json
```

Observed identity:

```text
spec_version:        0.9.2
profile:             HACP-Core
vector_set:          core-0.9.2
canonicalization:    JCS-RFC8785
digest_algorithm:    SHA-256
total_vectors:       38
vector_digest:       sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

The HACP 1.0.0 specification release version and the HACP wire/object/canonical-vector identity remain separate version dimensions. No manifest version migration was required by Variant A.

Classification: **PASS**.

---

## 4. Canonical Vector Integrity and Digest Verification

A read-only independent digest recomputation was performed using the same canonicalization contract as the harness manifest generator:

```text
38 JSON vectors
sorted vector filenames
strict duplicate-key-aware load
JCS canonicalization
SHA-256 over canonicalized vector bytes
```

Observed:

```text
vector_count = 38
expected      = sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
computed      = sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
match         = True
```

Working tree remained clean.

Classification: **PASS**.

---

## 4.1 Command Path Convention

All commands below are intentionally expressed using repository-relative paths.
No workstation-specific checkout paths, user-profile paths, or machine-local directory names are part of the release record.

When a command needs another repository checkout, the neutral placeholder:

```text
<path-to-hacp-sidecar>
```

means the local root of that repository and is not part of the normative or release identity.

---

## 5. Protocol v1 Strict Verifier Self-Test

Command:

```powershell
# From the hacp-spec repository root
python -m pytest .\harness\protocol_v1_tests\test_reason_code_validation.py -q
```

Observed:

```text
1 passed
exit code 0
```

This proves that the Protocol v1 verifier detects a reason-code mismatch. It does not by itself establish exact-reason 38/38 for every implementation.

Classification: **PASS**.

---

## 6. `hacp-go` Build and Canonical Conformance

### 6.1 Package/build smoke

Command:

```powershell
# From the hacp-spec repository root
Push-Location .\hacp-go
go test ./... -count=1
Pop-Location
```

Observed:

```text
? hacp-go [no test files]
exit code 0
```

This confirms Go package/build health only. It is not the canonical vector run.

Classification: **PASS**.

### 6.2 Binary build

Command:

```powershell
# From the hacp-spec repository root
Push-Location .\hacp-go
go build -o hacp-go.exe .
Pop-Location
```

Observed:

```text
exit code 0
```

### 6.3 Canonical HACP-Core conformance

Command:

```powershell
# From the hacp-spec repository root
Push-Location .\harness

python harness.py `
  --mode cli `
  --binary-path "..\hacp-go\hacp-go.exe" `
  --vectors-dir "..\vectors"

Pop-Location
```

Observed:

```text
HACP Conformance Harness v0.9.2 - Mode: cli
RESULTS: 38/38 passed
exit code 0
```

Working tree remained clean.

Expected baseline: **38/38 canonical decision-level PASS**.
Observed baseline: **38/38 PASS**.

Classification: **PASS**.
HACP 1.0.0 blocker: **NO**.

---

## 7. TypeScript Conformance and Regression

Implementation:

```text
hacp-spec/hacp-ts
```

### 7.1 Canonical conformance

Command:

```powershell
# From the hacp-spec repository root
Push-Location .\hacp-ts
npm run test:conformance
Pop-Location
```

Observed:

```text
38 canonical vectors PASS
1 vector inventory assertion PASS
39 total test items
0 fail
exit code 0
```

Expected canonical baseline: **38/38**.
Observed canonical baseline: **38/38 PASS**.

Classification: **PASS**.

### 7.2 Full TypeScript regression

Command:

```powershell
# From the hacp-spec repository root
Push-Location .\hacp-ts
npm test
Pop-Location
```

Observed:

```text
44 tests
44 pass
0 fail
0 skipped
exit code 0
```

The 44 tests consist of the canonical vector inventory and 38 canonical vectors plus five additional action-hash invariants.

Classification: **PASS**.

---

## 8. Go Sidecar Runner Protocol Conformance

### 8.1 Conformance runner build

Command:

```powershell
# From the hacp-sidecar repository root
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
```

Observed:

```text
exit code 0
working tree clean
```

### 8.2 Protocol v1 canonical runner execution

Command:

```powershell
# From the hacp-spec repository root.
# Replace <path-to-hacp-sidecar> with a local checkout path.
Push-Location .\harness

python harness_runner.py `
  --runner "<path-to-hacp-sidecar>\hacp-conformance-runner.exe" `
  --vectors-dir "..\vectors" `
  --manifest conformance_manifest.json `
  --implementation-name hacp-sidecar `
  --implementation-version 0.3.0 `
  --output console `
  --verbose

Pop-Location
```

Observed manifest gate:

```text
Protocol version: 1
Manifest verified: 0.9.2 (HACP-Core)
Vector set: core-0.9.2
Digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

Observed strict result:

```text
RESULTS: 15/38 passed
exit code 1
```

For every strict failure:

```text
outcome_correct = True
```

Therefore the same execution yields two distinct verification conclusions.

### 8.3 Decision-level canonical result

```text
Exact decision outcome correct: 38/38
```

Expected Variant A decision-level baseline: **38/38**.
Observed: **38/38 decision-correct**.

Classification: **PASS**.
HACP 1.0.0 blocker: **NO**.

### 8.4 Strict exact-reason baseline

Observed:

```text
15/38 exact-reason PASS
23/38 exact-reason mismatch
all 23 mismatches have outcome_correct=True
```

This reproduces the accepted historical strict baseline dispositioned during R1.

Exact reason-code 38/38 is:

```text
NOT ESTABLISHED
NOT CLAIMED
NOT REQUIRED BY VARIANT A
```

Classification: **EXPECTED CLASSIFIED STRICT BASELINE REPRODUCED**.
HACP 1.0.0 blocker: **NO**.

No harness weakening, vector rewrite, or production reason-code mass correction was performed merely to obtain process exit code 0.

---

## 9. Full `hacp-sidecar` Go Regression

Command:

```powershell
# From the hacp-sidecar repository root
go test ./... -count=1
```

Observed passing test-bearing packages include:

```text
cmd/hacp-conformance-runner
cmd/sidecar
internal/controlplane
internal/evaluate
internal/proxy
internal/scope
internal/trust
internal/wire
```

Observed:

```text
no package failures
exit code 0
working tree clean
```

Classification: **PASS**.

---

## 10. Python HACP-Core Canonical Conformance

Repository:

```text
humanist-core @ e4734bfba41171e97758a9c193dcd4d1ce1984c5
```

Command:

```powershell
# From the humanist-core repository root
python -m pytest tests/conformance/test_core_vectors.py -v -rs
```

Observed:

```text
collected 39 items
1 vector inventory assertion PASS
38 canonical vector cases PASS
39 passed
0 failed
0 skipped
exit code 0
working tree clean
```

Canonical claim:

```text
Python HACP-Core v0.9.2: 38/38 PASS
```

Classification: **PASS**.

---

## 11. Python Full Local Regression

Command:

```powershell
# From the humanist-core repository root
python -m pytest tests/ -v -rs
```

Observed:

```text
collected 324 items
319 passed
5 skipped
0 failed
exit code 0
working tree clean
```

The five skips are the environment-dependent external real-sidecar E2E tests. Their skip conditions require the external sidecar environment and signing identity.

Expected local baseline:

```text
319 passed
5 external-sidecar tests conditionally skipped
0 failed
```

Observed baseline: **exact match**.

Classification: **PASS**.

---

## 12. Python Statement and Branch Coverage Gate

Command:

```powershell
# From the humanist-core repository root
python -m pytest tests/ `
  --cov=humanist_core `
  --cov-branch `
  --cov-report=term-missing `
  --cov-fail-under=100 `
  -rs
```

Observed:

```text
319 passed
5 skipped
0 failed

TOTAL statements:        1554
Missed statements:       0
Branches:                420
Partial branches:        0
Coverage:                100.00%
Required coverage gate:  reached
exit code:               0
working tree:            clean
```

Expected:

```text
100% statement coverage
100% branch coverage
BrPart 0
0 missed statements
```

Observed baseline: **exact match**.

Classification: **PASS**.

---

## 13. Python ↔ Go External E2E

Verified pair:

```text
humanist-core  e4734bfba41171e97758a9c193dcd4d1ce1984c5
hacp-sidecar   b6b9e9836a182deb832fa42e33ca9c239790fac1
```

External sidecar mode:

```text
HACP_SIDECAR_EXTERNAL=1
HACP_SIDECAR_URL=http://127.0.0.1:8080
HACP_TEST_SIGNER_KEY_ID=key-ed25519-test-001
```

Command:

```powershell
# From the humanist-core repository root
python -m pytest `
  tests/test_hacp_sidecar_integration.py `
  -vv -rs --tb=long
```

Observed:

```text
collected 5 items
5 passed
0 failed
0 skipped
exit code 0
working tree clean
```

The five verified interoperability assertions were:

```text
real sidecar fails closed without HACP headers
Python HTTP action_hash matches sidecar shape
Python envelope/token signatures are self-consistent
real sidecar accepts a valid Python-signed request
SidecarClient receives ALLOW from the real sidecar
```

Classification: **PASS**.

---

## 14. Package and Build Verification

### 14.1 `hacp-go`

```powershell
go build -o hacp-go.exe .
```

Result: **PASS**, exit code 0.

### 14.2 `hacp-ts`

```powershell
npm run build
```

The build was executed as part of both `npm run test:conformance` and `npm test`.

Result: **PASS**.

### 14.3 `hacp-sidecar` conformance runner

```powershell
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
```

Result: **PASS**, exit code 0.

### 14.4 `hacp-sidecar` binary

```powershell
go build -o hacp-sidecar.exe .\cmd\sidecar
```

Observed:

```text
exit code 0
working tree clean
```

Result: **PASS**.

### 14.5 `humanist-core` wheel build

Build metadata:

```text
backend: setuptools.build_meta
project: humanist-core
version: 0.5.0
```

Command:

```powershell
# From the humanist-core repository root
$wheelOut = Join-Path $env:TEMP "humanist-core-r6-wheel"
New-Item -ItemType Directory -Force -Path $wheelOut | Out-Null

python -m pip wheel . `
  --no-deps `
  --wheel-dir $wheelOut
```

Observed:

```text
Successfully built humanist-core
humanist_core-0.5.0-py3-none-any.whl
exit code 0
working tree clean
```

This verifies package artifact construction from current `pyproject.toml` metadata. Clean-clone installation and fresh-environment installation remain R8 responsibilities.

Classification: **PASS**.

---

## 15. Release Blocker Ledger Review

The current release blocker ledger records the R1 strict-mismatch disposition against Variant A.

Final R1 totals:

```text
Historical strict mismatches:                 23
Dispositioned:                                23/23
Established production defects:               6
Established production defects fixed:         6/6
Normative conflicts on HOLD:                  1
Unresolved production defects established:    0
Unresolved HACP 1.0.0 blockers established:   0
```

`CORE-RUNTIME-005` remains:

```text
status:          HOLD
classification:  normative conflict
1.0.0 blocker:   NO
future target:   1.0.n / later adjudication
```

R2 separately closed all established normative-freeze blockers. R5 closed all documentation-hygiene blockers.

R6 established no new mandatory-suite blocker.

Final Variant A blocker state:

```text
UNRESOLVED VARIANT A BLOCKERS: 0
```

Classification: **PASS**.

### 15.1 Stale ledger header metadata

The blocker ledger still contains administrative header metadata indicating:

```text
Current stage: R1 — Strict mismatch classification
Status: ACTIVE
```

Its substantive conclusion, totals, and exit determination record R1 as complete with no unresolved HACP 1.0.0 blocker.

Classification:

```text
STALE RELEASE-LEDGER METADATA
production impact: none
R6 blocker: no
```

No opportunistic metadata edit was performed during R6.

---

## 16. Enforcement Revision 2 / HC2 Boundary

Release documentation consistently establishes:

```text
Enforcement revision 2 = draft successor
Enforcement revision 2 = not active
HC2 = draft successor evidence
HC2-55 = not active HACP 1.0 Enforcement conformance
current draft vector PASS != complete active profile conformance
```

Under the HACP 1.0.0 release scope boundary, Enforcement revision 2 activation and HC2 promotion are explicitly outside Variant A.

Therefore:

```text
HC2/revision-2 execution required as mandatory R6 gate: NO
revision-2 activation performed:                         NO
HC2 promotion performed:                                NO
accidental active-profile claim established:            NO
```

Classification: **PASS — correctly excluded from the mandatory R6 gate**.

---

## 17. Deviations and Deferred Items

### R6-HDR-001 — unsigned `humanist-core` HEAD

```text
Repository:       humanist-core
Commit:           e4734bfba41171e97758a9c193dcd4d1ce1984c5
Finding:          current HEAD has no Git commit signature
Branch alignment: main == origin/main
Working tree:     clean
Technical R6:     all mandatory surfaces PASS
```

Classification:

```text
RELEASE-LINEAGE / R7 READINESS DEVIATION
```

R6 Variant A blocker: **NO**.

Reasoning:

- the fact is established and preserved;
- no existing Variant A rule was found that converts this current `humanist-core` commit-signature fact into an R6 technical blocker;
- no history rewrite or artificial corrective commit is justified by R6;
- lineage policy, if required for release-candidate assembly, must be handled in R7 rather than retroactively changing R6 verification evidence.

Disposition: **CARRY FORWARD TO R7**.

### R6-REP-001 — local ignored artifacts

Locally generated binaries and Python caches may exist, including:

```text
*.exe
__pycache__/
```

They are ignored and untracked under the verified repository hygiene rules, and all relevant working trees remained clean after verification.

Classification: **EXPECTED LOCAL IGNORED ARTIFACTS**.
R6 blocker: **NO**.

### Strict reason-code correspondence

```text
15/38 exact-reason PASS
23/38 classified mismatch
38/38 decision outcome correct
```

Classification: **known accepted Variant A limitation**.
Future target: **1.0.n / separately authorized work**.

### `CORE-RUNTIME-005`

Classification: **normative HOLD**.
R6 blocker: **NO**.
Future target: **1.0.n / later adjudication**.

---

## 18. Final Verification Matrix

| ID | Repository / Surface | Exact commit | Verification | Expected | Observed | Deviation | 1.0.0 blocker | Result |
|---|---|---|---|---|---|---|---|---|
| R6-V01 | `hacp-spec` baseline | `407ce8932cf8ac20b0a02e4c836f40c7c923c9a8` | branch / origin / clean / signature | aligned, clean, signed | aligned, clean, Good signature | none | NO | PASS |
| R6-V02 | `hacp-sidecar` baseline | `b6b9e9836a182deb832fa42e33ca9c239790fac1` | branch / origin / clean / signature | aligned, clean, signed | aligned, clean, Good signature | none | NO | PASS |
| R6-V03 | `humanist-core` baseline | `e4734bfba41171e97758a9c193dcd4d1ce1984c5` | branch / origin / clean / signature | aligned, clean | aligned, clean, unsigned | R6-HDR-001 | NO in R6 | PASS / carry to R7 |
| R6-V04 | manifest identity | `407ce89` | manifest inspection | HACP-Core v0.9.2 / 38 | exact match | none | NO | PASS |
| R6-V05 | vector integrity | `407ce89` | independent digest recompute | digest match | exact match | none | NO | PASS |
| R6-V06 | Protocol v1 verifier | `407ce89` | pytest verifier self-test | mismatch detected | 1 passed | none | NO | PASS |
| R6-V07a | `hacp-go` package smoke | `407ce89` | `go test ./... -count=1` | build/package success | exit 0 | none | NO | PASS |
| R6-V07b | `hacp-go` canonical | `407ce89` | legacy CLI harness | 38/38 | 38/38 | none | NO | PASS |
| R6-V08 | `hacp-ts` canonical | `407ce89` | `npm run test:conformance` | 38/38 | 38/38 | none | NO | PASS |
| R6-V09 | `hacp-ts` regression | `407ce89` | `npm test` | 44/44 | 44/44 | none | NO | PASS |
| R6-V10 | sidecar decision-level canonical | `b6b9e98` + `407ce89` | Protocol v1 runner | 38/38 decisions | 38/38 decisions correct | strict reason mismatch exists separately | NO | PASS |
| R6-V11 | sidecar strict baseline | `b6b9e98` + `407ce89` | strict Protocol v1 verifier | classified baseline | 15/38 strict + 23 decision-correct mismatches | expected | NO | PASS / classified |
| R6-V12 | sidecar Go regression | `b6b9e98` | `go test ./... -count=1` | no failures | no failures | none | NO | PASS |
| R6-V13 | Python canonical | `e4734bf` | `test_core_vectors.py` | 38/38 | 38/38 | none | NO | PASS |
| R6-V14 | Python local regression | `e4734bf` | `pytest tests/` | 319 pass / 5 expected skip | exact match | none | NO | PASS |
| R6-V15 | Python coverage | `e4734bf` | 100% stmt + branch gate | 100%, BrPart 0 | 100%, 0 missed, 0 partial | none | NO | PASS |
| R6-V16 | Python ↔ Go external E2E | `e4734bf` + `b6b9e98` | 5 real-sidecar tests | 5/5 | 5/5 | none | NO | PASS |
| R6-V17 | package/build | all current candidates | Go/TS/sidecar/wheel builds | build success | all PASS | none | NO | PASS |
| R6-V18 | Variant A blocker ledger | release artifacts | blocker review | zero unresolved blockers | zero unresolved blockers | stale ledger header only | NO | PASS |
| R6-V19 | HC2/rev2 scope boundary | release artifacts | scope review | draft / excluded | draft / excluded | none | NO | PASS |

---

## 19. R6 Exit Determination

Against the HACP 1.0.0 §1.1 Variant A release boundary:

```text
candidate repository baselines established:        YES
canonical vector inventory verified:               YES
manifest digest independently verified:            YES
Protocol v1 strict verifier functioning:           YES
hacp-go canonical conformance:                      38/38 PASS
hacp-ts canonical conformance:                      38/38 PASS
hacp-ts full regression:                            44/44 PASS
hacp-sidecar canonical decision outcomes:           38/38 CORRECT
strict exact-reason baseline honestly reproduced:  15/38 + 23 classified mismatches
hacp-sidecar full Go regression:                    PASS
humanist-core canonical conformance:                38/38 PASS
humanist-core local regression:                     319 PASS / 5 expected SKIP
humanist-core statement coverage:                   100%
humanist-core branch coverage:                      100%
Python ↔ Go external E2E:                          5/5 PASS
package/build verification:                         PASS
unresolved Variant A blockers:                      0
HC2/revision-2 accidentally promoted:               NO
production changes during R6:                       NONE
canonical vector changes during R6:                 NONE
R1–R5 reopened:                                     NO
```

Final R6 determination:

```text
R6 COMPLETE

MANDATORY VARIANT A VERIFICATION SURFACES: PASS
UNRESOLVED HACP 1.0.0 VARIANT A BLOCKERS: 0

READY FOR SIGNED R6 CLOSURE
NEXT: R7 — Release Candidate Assembly
```

The `humanist-core` unsigned current HEAD remains explicitly carried as `R6-HDR-001` for R7 release-lineage/readiness handling. It does not invalidate the technical R6 verification results recorded here.

---

## 20. Reproducibility Principle

The R6 record intentionally preserves the distinction between:

```text
technical verification evidence
release-lineage evidence
strict exact-reason evidence
decision-level canonical evidence
draft successor evidence
active release claims
```

No failing or non-zero strict-verifier result was hidden, weakened, or rewritten to obtain a cosmetically green release record.

The release conclusion is therefore bounded and reproducible:

> HACP 1.0.0 Variant A has a reproducible 38-vector HACP-Core decision-level baseline across independent implementations, a functioning Protocol v1 strict verifier with honestly classified residual reason-code mismatches, passing Go/TypeScript/Python regressions, 100% Python statement and branch coverage, successful Python ↔ Go real-sidecar E2E interoperability, successful package/build verification, and no unresolved blocker established against the defined Variant A release contract.
