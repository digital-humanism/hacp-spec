# HACP 1.0 Clean-Clone Verification

**Release:** HACP 1.0.0
**Stage:** R8 — Clean-clone validation
**Contract boundary:** Variant A
**Status:** PASS
**Unresolved R8 release blockers:** 0

---

## 1. Purpose

This record documents clean-clone verification of the HACP 1.0.0 release candidate and its required implementation surfaces.

The objective of R8 is to establish that the HACP 1.0.0 Variant A release path can be reproduced from fresh public repository clones at exact immutable source revisions without dependency on:

- uncommitted developer state;
- local developer repository contents;
- `.restricted` engineering material;
- hidden generated source artifacts;
- machine-specific repository state;
- post-RC production changes.

The R8 exit path is:

```text
fresh clone
→ exact immutable source revisions
→ documented setup
→ build
→ required tests
→ Variant A conformance
→ cross-language interoperability
→ clean post-execution repository state
→ PASS
```

R8 is a validation stage. It does not authorize production feature work, semantic expansion, Enforcement revision 2 activation, HC2 promotion, strict 38/38 exact-reason chasing, history rewriting, or post-1.0 hardening.

---

## 2. HACP 1.0 Variant A Boundary

The HACP 1.0.0 release candidate is validated under Variant A.

Variant A includes:

- the stable public HACP Core contract;
- reproducible decision-level canonical conformance;
- canonical HACP-Core vector execution;
- Protocol v1 runner and strict verifier tooling;
- the active Enforcement profile as implemented by `hacp-sidecar`;
- documented historical and exact-reason limitations.

Variant A does not require:

- Enforcement revision 2 activation;
- HC2 promotion as the advertised HACP 1.0 Enforcement contract;
- exact reason-code 38/38 sidecar conformance.

For `hacp-sidecar`, the accepted strict baseline remains:

```text
decision outcome correctness: 38/38
exact reason-code match:       15/38
classified exact mismatch:     23/38
```

The 23 exact reason-code mismatches are previously classified and are not reopened by R8.

---

## 3. Immutable Source Anchors

### 3.1 hacp-spec

Release-candidate tag:

```text
v1.0.0-rc.1
```

Tagged commit:

```text
71c0e01018edb282614f7b047737ea3a425e2542
```

Annotated tag object:

```text
c3e0f94e6ff2bbd8d1491838c7e2dffcb8b9d6d3
```

Remote peeled target:

```text
71c0e01018edb282614f7b047737ea3a425e2542
```

The tag and commit were independently verified with a valid SSH/ED25519 signature.

Signing-key fingerprint:

```text
SHA256:0BnXknauq0S7xwJ8Fi48yGHQD8QClVi1+MO/4ymQDgE
```

At final R8 record preparation:

```text
HEAD        = 71c0e01018edb282614f7b047737ea3a425e2542
origin/main = 71c0e01018edb282614f7b047737ea3a425e2542
working tree = clean
```

### 3.2 hacp-sidecar

Exact source revision used for R8 verification:

```text
b6b9e9836a182deb832fa42e33ca9c239790fac1
```

The fresh-clone checkout was detached at this exact revision and remained clean after build, regression, conformance, and E2E execution.

### 3.3 humanist-core

Exact source revision used for the technical R8 verification:

```text
e4734bfba41171e97758a9c193dcd4d1ce1984c5
```

This revision was used for:

- full local regression;
- statement and branch coverage verification;
- CLI smoke verification;
- wheel build;
- external Python ↔ Go sidecar E2E verification.

This source revision is intentionally preserved as the technical verification anchor.

A later R8 documentation-only correction was committed separately as:

```text
6d9ae82ed7fefe47b395e4ef68d0a18807866473
docs: fix HACP sidecar integration links
```

That commit does not replace or retroactively alter the technical source revision verified by R8.

---

## 4. Reference Environment

R8 clean-clone verification was executed on Windows using the following observed toolchain:

```text
Git              2.54.0.windows.1
Go               1.26.5 windows/amd64
Python           3.13.14
Node.js          24.19.0
npm              11.17.0
protoc           35.1
protoc-gen-go    1.36.12
```

Fresh Python virtual environments were created outside source repositories.

The verification did not rely on the normal developer virtual environments.

---

## 5. Isolation Method

For this record:

```text
<R8_ISOLATED_ROOT>
```

denotes an arbitrary temporary directory located outside the normal developer repository trees.

The R8 clean-clone layout was:

```text
<R8_ISOLATED_ROOT>\hacp-spec
<R8_ISOLATED_ROOT>\hacp-sidecar
<R8_ISOLATED_ROOT>\humanist-core
```

Python virtual environments were created outside repository trees:

```text
<R8_ISOLATED_ROOT>\.venv-hacp-spec
<R8_ISOLATED_ROOT>\.venv-humanist-core
```

External E2E runtime artifacts were also isolated outside source repositories:

```text
<R8_ISOLATED_ROOT>\.e2e-runtime
```

The normal developer repositories were not used as clean-clone execution evidence.

The exact filesystem location of `<R8_ISOLATED_ROOT>` is not part of the release contract and may vary by machine or operating environment.

---

## 6. hacp-spec Verification

### 6.1 Exact RC checkout

Fresh `hacp-spec` clone:

```text
HEAD = 71c0e01018edb282614f7b047737ea3a425e2542
```

The checkout was clean before verification.

### 6.2 Canonical manifest

The canonical manifest used by R8 was:

```text
harness/conformance_manifest.json
```

Observed contract identifiers:

```text
profile:          HACP-Core
spec_version:     0.9.2
vector_set:       core-0.9.2
wire/object:      0.9
runner protocol:  1
vector count:     38
```

Manifest digest:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

### 6.3 Python canonical verification

A fresh isolated Python virtual environment was created.

Tracked harness dependencies were installed from:

```text
harness/requirements.txt
```

Vector bake check:

```text
CHECK RESULTS: 38/38 passed
```

Local canonical harness:

```text
RESULTS: 38/38 passed
```

Result:

```text
PASS
```

No dependency on developer Python state was observed.

### 6.4 TypeScript verification

Dependencies were installed from the tracked lock file using:

```text
npm ci
```

Build:

```text
PASS
```

Full TypeScript test suite:

```text
44 tests
44 passed
0 failed
```

The suite included the canonical 38-vector inventory.

Result:

```text
PASS
```

### 6.5 Go verification

From the fresh `hacp-go` source:

```text
go test ./...
go build ./...
```

Both commands completed successfully.

The Go package currently reports no local unit test files; this is not treated as a release failure because the required build path completed successfully and canonical verification is exercised through the release conformance surfaces.

Result:

```text
PASS
```

### 6.6 Post-execution repository state

After Python, TypeScript, and Go verification:

```text
git status --short
```

returned no tracked changes.

Result:

```text
CLEAN
```

---

## 7. hacp-sidecar Verification

### 7.1 Exact source checkout

Fresh source revision:

```text
b6b9e9836a182deb832fa42e33ca9c239790fac1
```

Initial working tree:

```text
CLEAN
```

### 7.2 Regression

The following full regression command completed successfully:

```text
go test ./... -count=1
```

Result:

```text
PASS
```

### 7.3 Static verification

```text
go vet ./...
```

completed successfully with no reported issues.

Result:

```text
PASS
```

### 7.4 Binary build

Sidecar binary:

```text
go build -o hacp-sidecar.exe ./cmd/sidecar
```

Result:

```text
PASS
```

Conformance runner:

```text
go build -o hacp-conformance-runner.exe ./cmd/hacp-conformance-runner
```

Result:

```text
PASS
```

### 7.5 Variant A canonical execution

The fresh sidecar conformance runner was executed against the fresh `hacp-spec` canonical manifest and vectors.

Observed strict result:

```text
15/38 exact reason-code matches
23/38 exact reason-code mismatches
```

For every one of the 23 strict mismatches:

```text
outcome_correct = True
```

Therefore the decision-level result remained:

```text
38/38 correct
```

This exactly reproduces the accepted HACP 1.0 Variant A baseline.

R8 did not weaken the verifier, modify canonical vectors, reinterpret the strict baseline, or introduce reason-code production changes.

Result:

```text
PASS — accepted Variant A baseline reproduced
```

### 7.6 Post-execution repository state

After regression, static verification, binary builds, canonical execution, and external E2E execution:

```text
git status --short
```

returned no tracked changes.

Result:

```text
CLEAN
```

---

## 8. humanist-core Verification

### 8.1 Exact technical source checkout

Fresh source revision:

```text
e4734bfba41171e97758a9c193dcd4d1ce1984c5
```

Initial working tree:

```text
CLEAN
```

### 8.2 Fresh Python environment

A fresh virtual environment outside the repository was used.

The project and development dependencies were installed from the checked-in Python project metadata.

No normal developer virtual environment was used as technical verification evidence.

### 8.3 Full local regression and coverage

The full suite was executed with statement and branch coverage enforcement.

Observed result:

```text
319 passed
5 skipped
0 failed
```

The five skipped tests were the expected external real-sidecar integration tests when an external sidecar was not enabled.

Coverage:

```text
1554 statements
0 missed

420 branches
0 partial

statement coverage: 100%
branch coverage:    100%
```

Result:

```text
PASS
```

### 8.4 CLI smoke verification

The installed CLI entry point was executed:

```text
humanist --help
```

The CLI loaded successfully and exposed the expected HACP command surfaces including:

```text
envelope
request
token
```

Result:

```text
PASS
```

### 8.5 Wheel build

A wheel was built from the fresh source checkout using the standard Python build frontend.

Produced artifact:

```text
humanist_core-0.5.0-py3-none-any.whl
```

Result:

```text
PASS
```

The wheel was an ephemeral validation artifact and was not treated as a repository-tracked release hash anchor.

---

## 9. External Python ↔ Go Sidecar E2E

The five conditionally skipped external E2E tests were then reproduced against the fresh Go sidecar.

### 9.1 Isolated runtime

An ephemeral runtime directory under:

```text
<R8_ISOLATED_ROOT>\.e2e-runtime
```

was used for local runtime-only artifacts.

A minimal local upstream was started at:

```text
http://127.0.0.1:18081
```

The upstream returned HTTP 200 for the test request path.

### 9.2 Deterministic conformance identity

The shared conformance Ed25519 identity was generated outside repository trees using:

```text
seed = SHA256("hacp-conformance-v0.9-key-001")
```

Signer key ID:

```text
key-ed25519-test-001
```

The private key was exported as an ephemeral PKCS8 PEM file under the isolated runtime directory.

No private test key was committed to either repository.

### 9.3 Fresh sidecar runtime configuration

The fresh sidecar binary was started with explicit test trust mode and isolated runtime configuration:

```text
HACP_TRUST_MODE=test
HACP_SIDECAR_PORT=18080
HACP_UPSTREAM=http://127.0.0.1:18081
HACP_PROVENANCE_FLUSH_PATH=<R8_ISOLATED_ROOT>\.e2e-runtime\provenance.jsonl
```

Sidecar health check:

```text
GET /healthz
HTTP 200
```

Result:

```text
PASS
```

### 9.4 humanist-core external mode

The fresh Python test process used:

```text
HACP_SIDECAR_EXTERNAL=1
HACP_SIDECAR_URL=http://127.0.0.1:18080
HACP_TEST_PRIVATE_KEY=<R8_ISOLATED_ROOT>\.e2e-runtime\hacp-test-key.pem
HACP_TEST_SIGNER_KEY_ID=key-ed25519-test-001
```

The following external E2E tests passed:

```text
test_real_sidecar_is_fail_closed_without_hacp_headers
test_http_action_hash_matches_sidecar_shape
test_python_envelope_and_token_signatures_are_self_consistent
test_real_sidecar_allows_python_signed_request
test_sidecar_client_gets_allow
```

Observed result:

```text
5 passed
0 failed
```

This reproduced the complete cross-language chain:

```text
Python IntentEnvelope
→ Python DecisionToken
→ JCS canonicalization
→ Ed25519 signatures
→ SHA-256 action binding
→ Base64url wire transport
→ fresh Go hacp-sidecar
→ signature and binding verification
→ scope / constraint / budget evaluation
→ real HTTP forwarding
→ ALLOW
```

Result:

```text
PASS
```

### 9.5 Runtime cleanup

The sidecar and upstream processes were terminated after verification.

Ports:

```text
18080
18081
```

were confirmed no longer in use by the R8 test processes.

---

## 10. Hidden Local State Assessment

At the final R8 clean-state checkpoint:

### hacp-spec

```text
HEAD:
71c0e01018edb282614f7b047737ea3a425e2542

working tree:
CLEAN

.restricted:
ABSENT
```

### hacp-sidecar

```text
HEAD:
b6b9e9836a182deb832fa42e33ca9c239790fac1

working tree:
CLEAN

.restricted:
ABSENT
```

### humanist-core

```text
HEAD:
e4734bfba41171e97758a9c193dcd4d1ce1984c5

working tree:
CLEAN

.restricted:
ABSENT
```

All required technical verification paths completed without dependence on `.restricted`.

No uncommitted source files were required.

No normal developer repository was used as a substitute for a clean-clone execution path.

Result:

```text
PASS
```

---

## 11. Documentation Findings

### R8-DOC-001 — Immutable RC Record Temporal Wording

The release-candidate record contained wording indicating that the signed RC tag was not yet created and that R7 remained in progress.

Independent repository verification established that:

```text
v1.0.0-rc.1
```

exists as a signed annotated tag and resolves to:

```text
71c0e01018edb282614f7b047737ea3a425e2542
```

Classification:

```text
release-documentation temporal inconsistency
```

Impact:

```text
production behavior:      none
conformance result:       none
RC cryptographic state:   none
source integrity:         none
```

Disposition:

```text
NON-BLOCKING
RECORDED
NO HISTORY REWRITE
NO RC RETAGGING
```

The immutable RC is not rewritten to alter historical wording.

---

### R8-DOC-002 — Broken humanist-core Integration Links

The verified `humanist-core` source contained four README references to:

```text
docs/HACP Integration Verification Guide.md
```

No tracked document existed at that path.

The actual tracked integration document was:

```text
docs/Integration with HACP Sidecar.md
```

The actual document contained sufficient protocol and external-E2E information to reconstruct and successfully reproduce the fresh Python ↔ Go integration path.

The broken README links were therefore classified as:

```text
public release-path documentation defect
production impact: none
technical E2E blocker: no
```

A minimal documentation-only correction was made after technical R8 verification.

Correction commit:

```text
6d9ae82ed7fefe47b395e4ef68d0a18807866473
docs: fix HACP sidecar integration links
```

The commit:

- modifies only `README.md`;
- removes stale references;
- redirects operational integration references to the tracked integration document;
- removes duplicate references created by the path consolidation;
- contains no production, protocol, API, semantic, or version changes;
- carries a valid SSH/ED25519 signature;
- was pushed to `origin/main`.

Disposition:

```text
CLOSED
```

---

### R8-DOC-003 — Sidecar Native E2E Startup Documentation Completeness

The `hacp-sidecar` README explicitly documents:

```text
HACP_TRUST_MODE=test
```

for test/conformance trust mode.

It does not provide a complete native external-E2E startup procedure covering all runtime variables required by the `humanist-core` external test path.

The exact runtime surface was reconstructed from tracked sidecar source:

```text
HACP_SIDECAR_PORT
HACP_UPSTREAM
HACP_PROVENANCE_FLUSH_PATH
HACP_TRUST_MODE
```

The reconstructed procedure successfully reproduced the external E2E suite 5/5 from fresh source.

Classification:

```text
documentation completeness limitation
```

Impact:

```text
production behavior:          none
clean-clone reproducibility:  not blocked
release conformance:          not affected
```

Disposition:

```text
NON-BLOCKING
RECORDED
NO R8 DOCUMENTATION REDESIGN
```

A broader sidecar tutorial or documentation restructuring is outside the R8 scope.

---

## 12. R8 Release-Path Corrections

R8 produced one minimal correction:

```text
repository:
humanist-core

commit:
6d9ae82ed7fefe47b395e4ef68d0a18807866473

change:
documentation links only
```

No production implementation changes were required by R8.

No canonical vectors were modified.

No protocol semantics were changed.

No HACP version domain was changed.

No Enforcement revision was promoted.

No release history was rewritten.

---

## 13. Final Verification Matrix

| Surface | Verification | Result |
|---|---|---|
| `hacp-spec` RC checkout | exact `v1.0.0-rc.1` target | PASS |
| RC signature | SSH/ED25519 verification | PASS |
| Canonical manifest | 38-vector inventory and digest | PASS |
| Python canonical path | 38/38 | PASS |
| TypeScript canonical/regression | 44/44 | PASS |
| Go spec build | `go build ./...` | PASS |
| `hacp-sidecar` exact source | `b6b9e983...` | PASS |
| Sidecar regression | `go test ./... -count=1` | PASS |
| Sidecar vet | `go vet ./...` | PASS |
| Sidecar build | sidecar binary | PASS |
| Sidecar runner build | conformance runner | PASS |
| Sidecar decision outcome | 38/38 correct | PASS |
| Sidecar exact reason | 15/38 + 23 classified mismatch | ACCEPTED |
| `humanist-core` exact technical source | `e4734bf...` | PASS |
| Python local suite | 319 PASS / 5 expected SKIP / 0 FAIL | PASS |
| Statement coverage | 100% | PASS |
| Branch coverage | 100% | PASS |
| CLI smoke | installed CLI loads correctly | PASS |
| Wheel build | `humanist_core-0.5.0-py3-none-any.whl` | PASS |
| External Python ↔ Go E2E | 5/5 | PASS |
| Fresh-clone repository cleanliness | all three clean | PASS |
| `.restricted` dependency | none observed | PASS |
| R8-DOC-001 | immutable temporal wording | NON-BLOCKING |
| R8-DOC-002 | broken integration links | CLOSED |
| R8-DOC-003 | startup-doc completeness | NON-BLOCKING |
| Unresolved R8 release blockers | 0 | PASS |

---

## 14. Exit-Criteria Assessment

R8 required evidence of:

```text
fresh clone
→ exact immutable source revisions
→ documented setup
→ build
→ required tests
→ Variant A conformance
→ PASS
```

Observed result:

```text
fresh clone                         PASS
exact source anchors                PASS
isolated dependency setup           PASS
required builds                     PASS
required regressions                PASS
canonical HACP-Core                 PASS
Variant A sidecar baseline          PASS
Python ↔ Go external E2E            PASS
hidden-local-state assessment       PASS
post-execution clean state          PASS
```

Where documentation was incomplete, only tracked source was used to reconstruct the required runtime configuration; the reconstructed path was then experimentally verified from clean source.

No release blocker remained after verification.

---

## 15. Conclusion

HACP 1.0.0 R8 clean-clone validation is complete.

The release candidate and required implementation surfaces were reproduced from fresh isolated clones at exact source anchors.

The canonical HACP-Core vector baseline reproduced successfully.

The accepted HACP 1.0 Variant A sidecar baseline reproduced without reinterpretation or strict-reason expansion.

The `humanist-core` Python implementation reproduced its full local regression and 100% statement and branch coverage from a fresh environment.

The real Python ↔ Go interoperability path reproduced successfully against a freshly built `hacp-sidecar`:

```text
5/5 external E2E tests passed
```

All source repositories remained clean after technical verification.

No dependency on `.restricted`, uncommitted developer state, or normal developer repository artifacts was observed.

One broken public documentation path discovered during R8 was corrected with a minimal signed documentation-only commit. Two additional documentation observations are recorded as non-blocking and do not alter the verified release contract.

Final R8 disposition:

```text
R8 — CLEAN-CLONE VALIDATION: PASS

Unresolved R8 release blockers: 0
Production corrections required: 0
Variant A boundary preserved: YES
```

R8 is closed.

No R9 GO/NO-GO decision is made by this record.
