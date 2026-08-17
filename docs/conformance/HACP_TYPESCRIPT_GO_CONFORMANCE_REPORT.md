# HACP TypeScript and Go Conformance Report

**Project:** Humanist / Human Agency Continuity Protocol (HACP)  
**Repository:** `hacp-spec`  
**Target protocol:** HACP-Core v0.9.2  
**Report date:** 2026-08-17  
**Status:** ✅ TypeScript and Go conformance completed successfully

---

## 1. Purpose

This report records the implementation, hardening, and verification work completed for the TypeScript and Go HACP implementations against the canonical HACP-Core v0.9.2 conformance vector set maintained in `hacp-spec`.

The objective of this stage was to verify that independent implementations converge on the same normative protocol behavior using one shared source of canonical test data.

Acceptance criterion:

```text
Canonical vector set: hacp-spec/vectors
Expected vectors:     38
Failures:             0
Skipped vectors:      0
```

Final result:

```text
TypeScript   38/38 PASS
Go           38/38 PASS
```

Both implementations were validated against the same manifest-verified vector set.

---

## 2. Canonical conformance baseline

The normative vector set is stored under:

```text
hacp-spec/
└── vectors/
```

The verified inventory contains exactly 38 canonical JSON vectors.

| Group | Purpose | Vectors |
|---|---|---:|
| INV1 | Principal, delegation, authority invariants | 4 |
| INV2 | Semantic/action boundary enforcement | 8 |
| INV3 | Decision-token binding | 4 |
| INV4 | Provenance / traceability | 5 |
| INV5 | Cryptographic profile / canonicalization | 8 |
| INV7 | Autonomy budget and revocation | 4 |
| Runtime | Checkpoint / resume semantics | 5 |
| **Total** |  | **38** |

The canonical manifest is:

```text
hacp-spec/harness/conformance_manifest.json
```

Final manifest parameters:

```text
spec_version:      0.9.2
profile:           HACP-Core
vector_set:        core-0.9.2
canonicalization:  JCS-RFC8785
digest_algorithm:  SHA-256
total_vectors:     38
```

Final verified digest:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

The manifest verification gate passed before the final Go conformance run.

---

## 3. Finalization of the canonical vector set

Before the final cross-language run, two vectors that had previously been marked as draft were converted into fully executable deterministic normative vectors:

```text
CORE-INV3-002
CORE-INV5-002
```

### CORE-INV3-002

Purpose:

```text
Token replayed against a semantically changed action must fail binding.
```

Final behavior:

- the token is validly signed;
- the token is bound to the original action;
- the presented action contains a changed `quantity`;
- the computed `action_hash` no longer matches;
- expected result: `DENY / HASH_MISMATCH`.

### CORE-INV5-002

Purpose:

```text
A signed token whose payload is modified after signing must fail cryptographic verification.
```

Final behavior:

- a valid token is generated and signed;
- a signed field is modified without re-signing;
- Ed25519 verification fails;
- expected result: `DENY / SIGNATURE_FAILURE`.

After these changes, the manifest digest was recalculated and updated to the value recorded above.

---

# 4. TypeScript implementation

## 4.1 Initial state

The TypeScript implementation was located under:

```text
hacp-spec/
└── hacp-ts/
    ├── package.json
    ├── package-lock.json
    ├── tsconfig.json
    └── src/
        ├── canonical.ts
        ├── cli.ts
        ├── crypto.ts
        └── evaluate.ts
```

At the start of this task:

- canonicalization support already existed;
- SHA-256 and Ed25519 verification already existed;
- a basic runtime evaluator already existed;
- a CLI existed;
- there was no dedicated normative conformance test layer;
- there was no conformance test runner;
- the runtime evaluator primarily returned `ALLOW / DENY / CHECKPOINT`, without the complete normative reason-code surface required for direct vector comparison.

The existing runtime evaluator was intentionally preserved.

A separate clean-room conformance evaluator was added instead of modifying runtime behavior merely to satisfy test vectors.

---

## 4.2 Final TypeScript conformance structure

```text
hacp-ts/
├── src/
│   ├── canonical.ts
│   ├── cli.ts
│   ├── crypto.ts
│   ├── evaluate.ts
│   └── conformance.ts
├── tests/
│   ├── action-hash.test.ts
│   └── conformance.test.ts
├── package.json
├── package-lock.json
└── tsconfig.json
```

### `src/conformance.ts`

A dedicated conformance evaluator was introduced.

Its responsibility is to evaluate raw wire dictionaries from canonical vector files rather than depend on application-facing SDK models.

The evaluator returns a structured result containing:

```text
decision
reason_codes
action_hash
canonical_action
canonical_envelope
canonical_token
envelope_signature_valid
token_signature_valid
provenance_valid
provenance_event_id
```

This keeps normative verification independent from the runtime evaluator.

---

## 4.3 Normative evaluation order

The final TypeScript conformance evaluator implements a fail-closed gate order:

```text
Gate 0  malformed / duplicate-key input
Gate 1  checkpoint and runtime state
Gate 2  key profile, token lifecycle and token binding
Gate 3  provenance / traceability
Gate 4  envelope lifecycle and revocation
Gate 5  autonomy budget, human authority and scope
Gate 6  Ed25519 signature verification
ALLOW
```

This order is significant because some negative vectors contain more than one invalid condition.

The evaluator must return the normative reason associated with the first applicable security gate rather than an implementation-specific later failure.

Examples include:

```text
TOKEN_EXPIRED vs ENVELOPE_EXPIRED
HASH_MISMATCH vs SIGNATURE_FAILURE
KEY_REVOKED vs SIGNATURE_FAILURE
HUMAN_RESOLUTION_REQUIRED vs generic checkpoint denial
```

---

## 4.4 JCS and action binding

The existing TypeScript canonicalization module was reused.

Canonical action binding is:

```text
action_hash = SHA256(JCS(proposed_action))
```

The test suite verifies that:

- object key order does not change the action hash;
- changes to security-relevant action semantics do change the hash.

Dedicated action-hash tests cover:

```text
field ordering
audience
reversibility
externality
data_class
```

---

## 4.5 Ed25519 verification

The existing TypeScript crypto layer was reused for:

```text
SHA-256
raw 32-byte Ed25519 public-key loading
SPKI wrapping
Ed25519 verification
base64url signature decoding
```

Canonical test identity:

```text
key_id:
key-ed25519-test-001
```

Raw Ed25519 public key:

```text
9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3
```

This is the same deterministic test identity used across the other implementations.

---

## 4.6 Duplicate JSON key handling

`CORE-INV5-006` requires duplicate JSON keys to be rejected.

Standard JavaScript `JSON.parse()` silently keeps the last duplicate key, so a lightweight JSON syntax walker was added before normal parsing.

Duplicate keys are recorded as metadata:

```text
_duplicate_json_keys
```

The evaluator then fails closed with:

```text
DENY / INVALID_ACTION
```

This preserves the intended semantics of the canonical negative vector.

---

## 4.7 TypeScript test infrastructure

The project now uses Node's built-in test runner:

```text
node:test
```

No Jest dependency was required.

Build/test commands:

```text
npm run build
npm test
npm run test:conformance
```

The implementation and tests compile under TypeScript strict mode.

A strict-nullability issue in the JSON parser was fixed with an explicit `throw`, allowing TypeScript control-flow analysis to prove that the regex match is non-null.

---

## 4.8 Final TypeScript result

Final command:

```powershell
cd ...\GitHub\Dev\hacp-spec\hacp-ts
npm test
```

Final result:

```text
tests 44
pass 44
fail 0
cancelled 0
skipped 0
todo 0
```

Composition:

```text
38 canonical HACP-Core vectors
 5 action-hash invariants
 1 vector inventory test
---------------------------------
44 total tests
```

All 38 normative vectors passed, including:

```text
CORE-INV3-002
CORE-INV5-002
CORE-INV5-006
CORE-RUNTIME-005
```

Final TypeScript status:

```text
HACP-Core v0.9.2: 38/38 PASS
Failures:          0
Skipped:           0
```

---

# 5. Go implementation / hacp-sidecar

## 5.1 Validation scope

The Go implementation is provided by:

```text
hacp-sidecar
```

The normative vector runner is:

```text
cmd/hacp-conformance-runner
```

The runtime sidecar is:

```text
cmd/sidecar
```

The conformance runner uses:

```text
ProtocolVersion = 1
```

and communicates with the language-neutral harness over JSON stdin/stdout.

---

## 5.2 Go repository test and build verification

Repository-wide test command:

```powershell
cd ...\GitHub\Dev\hacp-sidecar
go test ./...
```

Observed result:

```text
hacp-sidecar/internal/scope     ok
```

Other packages compiled successfully and reported `[no test files]`.

No Go package failed compilation or testing.

The conformance runner was rebuilt:

```powershell
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
```

The sidecar binary can be rebuilt with:

```powershell
go build -o hacp-sidecar.exe .\cmd\sidecar
```

---

## 5.3 Runner-mode conformance harness

The Go runner was validated through the normative harness stored in `hacp-spec`.

The harness ran from:

```text
hacp-spec/harness
```

against:

```text
hacp-sidecar/hacp-conformance-runner.exe
```

Command pattern:

```powershell
python harness_runner.py `
  --runner "...\GitHub\Dev\hacp-sidecar\hacp-conformance-runner.exe" `
  --vectors-dir "...\GitHub\Dev\hacp-spec\vectors" `
  --manifest conformance_manifest.json `
  --implementation-name hacp-sidecar `
  --implementation-version 0.3.0 `
  --output console `
  --verbose
```

The harness verified:

```text
Spec:       0.9.2
Profile:    HACP-Core
Vector set: core-0.9.2
Protocol:   1
```

Manifest verification succeeded using:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

---

## 5.4 Final Go conformance result

Final output:

```text
RESULTS: 38/38 passed
```

No vectors were skipped.

All invariant groups passed:

### INV1 — authority and principal invariants

```text
CORE-INV1-001 PASS
CORE-INV1-002 PASS
CORE-INV1-005 PASS
CORE-INV1-006 PASS
```

### INV2 — semantic/action boundaries

```text
CORE-INV2-001 through CORE-INV2-008 PASS
```

Including:

```text
audience crossing
reversibility crossing
externality crossing
quantity limits
destination allowlists
data-class boundaries
missing security-relevant attributes
```

### INV3 — token binding

```text
CORE-INV3-001 through CORE-INV3-004 PASS
```

Including the finalized:

```text
CORE-INV3-002 PASS
```

### INV4 — provenance

```text
CORE-INV4-001 through CORE-INV4-005 PASS
```

Including:

```text
signed EVALUATED event
revocation provenance
payload tamper detection
missing provenance
broken prev_event_hash
```

### INV5 — cryptographic profile

```text
CORE-INV5-001 through CORE-INV5-008 PASS
```

Including:

```text
valid Ed25519 verification
tampered signed payload
unknown signer rejection
HMAC profile rejection
JCS key-order invariance
duplicate JSON key rejection
non-canonical serialization mismatch
revoked key handling
```

The finalized vector `CORE-INV5-002` also passed.

### INV7 — budget and revocation

```text
CORE-INV7-001 PASS
CORE-INV7-002 PASS
CORE-INV7-005 PASS
CORE-INV7-006 PASS
```

### Runtime checkpoint semantics

```text
CORE-RUNTIME-001 through CORE-RUNTIME-005 PASS
```

Including:

```text
human checkpoint resolution
checkpoint timeout
resume token mismatch
unresolved checkpoint
system self-resolution rejection
```

Final Go status:

```text
HACP-Core v0.9.2: 38/38 PASS
Failures:          0
Skipped:           0
Manifest:          verified
```

---

# 6. Real sidecar environment verification

In addition to black-box conformance, the real local sidecar environment was verified.

Reference deployment:

```text
mock upstream     127.0.0.1:8000
control-plane     127.0.0.1:5000
hacp-sidecar      127.0.0.1:8080
```

Observed behavior:

```text
upstream :8000       HTTP 200
control :5000        HTTP server reachable
sidecar :8080        fail-closed without HACP credentials
```

An unauthenticated request to the sidecar returned:

```text
Forbidden
```

This confirms that the sidecar boundary was active and did not silently forward requests without HACP credentials.

---

# 7. Python ↔ Go interoperability verification

Although this report focuses on TypeScript and Go, the Go implementation was additionally checked through a real Python-to-Go E2E path.

Path:

```text
humanist-core
    ↓
signed IntentEnvelope + DecisionToken
    ↓
HTTP
    ↓
hacp-sidecar
    ↓
validation
    ↓
mock upstream
```

Final E2E result:

```text
5 passed
0 failed
0 skipped
```

The suite verified:

```text
fail-closed behavior without HACP headers
HTTP ProposedAction/action_hash compatibility
Python envelope/token signature consistency
real sidecar ALLOW for valid Python-signed requests
SidecarClient → real sidecar ALLOW
```

This adds a real interoperability layer beyond independent vector evaluators.

---

# 8. Cross-language result

After the TypeScript and Go work described above, all three implementations converge on the same normative vector set:

```text
Python       38/38 PASS
TypeScript   38/38 PASS
Go           38/38 PASS
```

All results refer to the same canonical manifest-verified set:

```text
HACP-Core v0.9.2
core-0.9.2
38 vectors
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

Normative failures:

```text
0
```

Skipped normative vectors:

```text
0
```

This establishes cross-language model convergence for the current HACP-Core v0.9.2 vector baseline.

---

# 9. Files added or materially updated

## `hacp-spec`

```text
vectors/
harness/conformance_manifest.json
hacp-ts/
```

The manifest digest was updated after finalizing the vector set.

## `hacp-ts`

Added:

```text
src/conformance.ts
tests/conformance.test.ts
tests/action-hash.test.ts
```

Updated:

```text
package.json
package-lock.json
tsconfig.json
```

Existing modules retained:

```text
src/canonical.ts
src/crypto.ts
src/evaluate.ts
src/cli.ts
```

## `hacp-sidecar`

No duplicate vector set was introduced.

The implementation boundary remains:

```text
cmd/hacp-conformance-runner
```

The final runner binary was rebuilt from current source before verification.

---

# 10. Reproducibility commands

## TypeScript

```powershell
cd ...\GitHub\Dev\hacp-spec\hacp-ts

npm ci
npm run build
npm test
```

Expected:

```text
44 tests
44 pass
0 fail
0 skipped
```

## Go repository test/build

```powershell
cd ...\GitHub\Dev\hacp-sidecar

go test ./...
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
go build -o hacp-sidecar.exe .\cmd\sidecar
```

## Go normative conformance

```powershell
cd ...\GitHub\Dev\hacp-spec\harness

python harness_runner.py `
  --runner "...\GitHub\Dev\hacp-sidecar\hacp-conformance-runner.exe" `
  --vectors-dir "...\GitHub\Dev\hacp-spec\vectors" `
  --manifest conformance_manifest.json `
  --implementation-name hacp-sidecar `
  --implementation-version 0.3.0 `
  --output console `
  --verbose
```

Expected:

```text
Manifest verified
RESULTS: 38/38 passed
```

---

# 11. Assurance boundary

The current result demonstrates deterministic agreement between independent implementations on the HACP-Core v0.9.2 canonical vector set.

It should be interpreted as:

```text
normative conformance baseline
cross-language model convergence
regression/reproducibility evidence
```

It is not, by itself:

```text
a formal proof of protocol correctness
a complete security proof
a substitute for fuzzing
a substitute for property-based testing
a substitute for adversarial production testing
```

The conformance baseline should therefore be preserved as a release gate while additional assurance layers are added.

---

# 12. Recommended next steps

1. Commit the final `hacp-spec` vector manifest and TypeScript conformance implementation.
2. Add TypeScript conformance execution to CI.
3. Add Go runner conformance execution to CI using the same canonical manifest.
4. Pin `hacp-spec` consumers to an immutable release tag or commit SHA for release-grade reproducibility.
5. Add a common cross-language report generator recording implementation/version, vector id, decision, reason codes, hashes/signature outcomes, latency, and pass/fail.
6. Produce a single black-box differential report across Python, TypeScript, and Go.
7. Continue with property-based testing and fuzzing around JCS, duplicate-key JSON, number edge cases, scope mutation, token binding, checkpoints, revocation, and provenance chains.

---

# 13. Final status

The TypeScript and Go HACP-Core v0.9.2 conformance milestone is complete.

```text
================================================
HACP-Core v0.9.2 Conformance
================================================

Canonical vectors:              38
Manifest verified:              YES

TypeScript:                     38/38 PASS
Go:                             38/38 PASS

TypeScript total suite:         44/44 PASS
Go repository tests/build:      PASS

Python ↔ Go real E2E:            5/5 PASS

Normative failures:                0
Skipped normative vectors:         0
================================================
```

Together with the independently validated Python implementation, the current HACP-Core v0.9.2 model demonstrates deterministic cross-language convergence across Python, TypeScript, and Go.


---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)