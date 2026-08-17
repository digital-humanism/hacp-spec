# HACP Vector Tooling

Offline tooling for generating, baking, and verifying the deterministic
HACP conformance vector set.

These tools support the canonical executable baseline for:

```text
HACP-Core v0.9.2
Vector set: core-0.9.2
Vectors: 38
```

Current canonical vector digest:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

> **Important:** tooling under `tools/` exists for conformance-fixture
> generation and verification. It is not production key-management or
> production signing infrastructure.

---

## 1. Tooling Overview

Current tooling:

```text
tools/
├── bake_vector.py
├── gen_test_keys.py
└── README.md
```

| Tool | Purpose |
|------|---------|
| `bake_vector.py` | Deterministically compute/update vector hashes and signatures, and verify baked vectors |
| `gen_test_keys.py` | Generate the deterministic TEST ONLY Ed25519 conformance key material |

The canonical vector set itself lives in:

```text
vectors/
```

The pinned vector-set manifest lives in:

```text
harness/conformance_manifest.json
```

---

## 2. Reproducibility Model

The HACP conformance baseline is designed to produce the same cryptographic
results across languages and platforms.

The reproducibility chain is:

```text
fixed test seed
    ↓
deterministic Ed25519 keypair
    ↓
explicit vector inputs
    ↓
JCS canonicalization
    ↓
SHA-256 action hash
    ↓
Ed25519 signatures
    ↓
baked vectors
    ↓
vector-set manifest digest
```

No canonical vector should depend on:

```text
wall-clock time
random runtime key generation
machine-specific paths
locale-dependent serialization
implementation-specific object ordering
```

---

## 3. Deterministic Test Identity

The conformance key is intentionally deterministic and public.

Seed derivation:

```text
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
```

Derived seed:

```text
4f656d4e80b0ae758c8035ece5fd076f443497f714a134c481ed72f58ed34017
```

Raw Ed25519 public key:

```text
9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3
```

Key identifier:

```text
key-ed25519-test-001
```

Expected key material is documented under:

```text
harness/keys/
```

Typical files:

```text
harness/keys/KEYS.md
harness/keys/test-ed25519-001.pub
harness/keys/test-ed25519-001.seed
```

**Security notice:** these keys are intentionally public and MUST NOT be used
for production authentication, signing, authorization, or trust decisions.

---

## 4. `gen_test_keys.py`

`gen_test_keys.py` generates the deterministic conformance test identity.

Run from the repository root:

```bash
python tools/gen_test_keys.py
```

Its output is intended only for:

```text
canonical vector baking
test fixture reproduction
cross-language cryptographic verification
```

It MUST NOT:

```text
generate production credentials
replace a production KMS
manage key rotation
store private production keys
```

If deterministic test-key output changes unexpectedly, stop and investigate
before rebaking vectors. A changed keypair invalidates existing canonical
signatures and will change the vector-set digest.

---

## 5. `bake_vector.py`

`bake_vector.py` is the offline deterministic vector baking tool.

Typical command:

```bash
python tools/bake_vector.py
```

Its role is to transform authoring-time vector fixtures into baked,
reproducible conformance vectors.

Depending on vector semantics, baking may compute or refresh:

```text
action_hash
IntentEnvelope signature
DecisionToken signature
deterministic cryptographic fixture values
draft_mode
```

Canonical action binding is:

```text
action_hash = SHA256(JCS(proposed_action))
```

IntentEnvelope signing is:

```text
signature = Ed25519(
    test_private_key,
    JCS(intent_envelope_without_signature)
)
```

DecisionToken signing is:

```text
signature = Ed25519(
    test_private_key,
    JCS(decision_token_without_signature)
)
```

All hashes and signatures are calculated over deterministic canonical bytes.

---

## 6. Draft and Baked Vectors

New bakeable vectors may begin with:

```json
{
  "draft_mode": true
}
```

and authoring placeholders where generated cryptographic material is not yet
available.

After baking, a canonical release vector must be in its normative baked state.

For the current HACP-Core v0.9.2 vector set:

```text
Canonical vectors: 38
Draft vectors:     0
```

A public conformance claim MUST NOT rely on vectors still requiring draft
baking.

---

## 7. Intentional Negative Cryptographic Vectors

The baking tool must preserve the intended failure semantics of negative
vectors.

Not every negative vector should be "fixed" into a valid signature.

Two important examples are:

### `CORE-INV3-002`

Semantics:

```text
valid token
bound to original ProposedAction
presented ProposedAction changed afterward
→ HASH_MISMATCH
```

The baker must preserve the valid original binding while leaving the presented
action semantically different.

Expected result:

```text
DENY / HASH_MISMATCH
```

### `CORE-INV5-002`

Semantics:

```text
valid signed token
signed payload modified after signing
→ signature verification fails
```

The baker must not re-sign the already tampered payload, because doing so would
destroy the purpose of the vector.

Expected result:

```text
DENY / SIGNATURE_FAILURE
```

This distinction is essential:

```text
baking establishes the intended fixture state
baking must not normalize away the negative condition being tested
```

---

## 8. Duplicate-Key Negative Vector

`CORE-INV5-006` intentionally exercises malformed raw JSON containing duplicate
object keys.

Normal canonical HACP JSON forbids duplicate keys.

Therefore:

```text
valid JSON object     → JCS canonicalization
duplicate-key raw JSON → reject before canonicalization
```

The baking/checking workflow must not silently parse and rewrite this vector in
a way that removes its malformed-input semantics.

Expected fail-closed result:

```text
DENY / INVALID_ACTION
```

---

## 9. Verify Baked Vector Integrity

After baking:

```bash
python tools/bake_vector.py --check
```

The check step is intended to detect unexpected differences in generated
cryptographic fixture data.

A successful integrity check does not replace the conformance harness.

Use both:

```text
bake/check
    ↓
manifest verification
    ↓
cross-language conformance
```

---

## 10. Manifest Update

Any intentional modification to the canonical vector set changes its digest.

After vector changes and successful integrity checks:

```bash
python harness/generate_manifest.py
```

Current canonical manifest:

```text
harness/conformance_manifest.json
```

Current baseline:

```json
{
  "spec_version": "0.9.2",
  "profile": "HACP-Core",
  "vector_set": "core-0.9.2",
  "canonicalization": "JCS-RFC8785",
  "digest_algorithm": "SHA-256",
  "vector_digest": "sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58",
  "total_vectors": 38
}
```

The modified vectors and regenerated manifest MUST be reviewed and committed
together.

A manifest mismatch must not be bypassed.

---

## 11. Required Validation Sequence After Vector Changes

Recommended sequence from repository root:

```bash
python tools/bake_vector.py
python tools/bake_vector.py --check
python harness/generate_manifest.py
python harness/harness.py --mode local
```

Then re-run every implementation whose conformance is claimed.

### TypeScript

```powershell
cd ...\GitHub\Dev\hacp-spec\hacp-ts
npm ci
npm run build
npm test
```

Current expected result:

```text
44 tests
44 pass
0 fail
0 skipped
```

### Go sidecar runner

Build:

```powershell
cd ...\GitHub\Dev\hacp-sidecar
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
```

Run:

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

Current expected result:

```text
Manifest verified
RESULTS: 38/38 passed
```

### Python

The `humanist-core` implementation must be revalidated against the same updated
canonical vector set before its conformance claim is renewed.

Current baseline:

```text
Python HACP-Core:          38/38 PASS
Python full regression:  324/324 PASS
Statement coverage:         100%
Branch coverage:            100%
```

---

## 12. Canonicalization Requirements

All generated hashes and signatures use deterministic HACP canonicalization
based on RFC 8785 / JCS.

Required properties include:

```text
deterministic object key ordering
UTF-8 canonical bytes
no insignificant whitespace
valid JSON string escaping
no non-finite numbers
no duplicate keys in valid canonical payloads
```

Equivalent logical payloads must generate identical canonical bytes and hashes
across implementations.

---

## 13. Timestamps

Canonical vectors use explicit Unix timestamps.

Tooling and runners must not use dynamic wall-clock time for normative vector
evaluation.

Incorrect:

```python
time.time()
```

for deciding canonical vector expiry during a conformance run.

Correct:

```text
use the explicit clock supplied by the vector/policy context
```

This is required for deterministic reproduction.

---

## 14. Change Discipline

Canonical vector modifications are protocol-significant.

A vector change should be treated similarly to a specification change because
it alters executable normative behavior.

A vector-set change is complete only when:

```text
vector semantics reviewed
baking completed
integrity check passed
manifest regenerated
manifest digest verified
TypeScript revalidated
Go revalidated
Python revalidated
documentation updated
```

Do not update expected outputs merely to make an implementation pass unless the
underlying normative semantics have been deliberately changed and reviewed.

---

## 15. Current Cross-Language Baseline

The current baked vector set has converged across three implementation
languages:

```text
Python       38/38 PASS
TypeScript   38/38 PASS
Go           38/38 PASS
```

Additional current verification:

```text
TypeScript suite:             44/44 PASS
Python full regression:      324/324 PASS
Python statement coverage:      100%
Python branch coverage:         100%
Python ↔ Go sidecar E2E:         5/5 PASS
```

Normative failures:

```text
0
```

Skipped normative vectors:

```text
0
```

Detailed report:

- [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md)

---

## 16. Assurance Boundary

The tools in this directory provide deterministic fixture generation and
verification.

They do not provide:

```text
formal verification
production key management
production signing services
security auditing
fuzz testing
property-based testing
deployment certification
```

Their purpose is narrower and explicit:

```text
make canonical HACP conformance fixtures reproducible
```

---

## 17. References

- [`../vectors/README.md`](../vectors/README.md) — canonical vector documentation
- [`../harness/README.md`](../harness/README.md) — conformance harness
- [`../harness/runner_protocol.md`](../harness/runner_protocol.md) — black-box runner protocol
- [`../harness/conformance_manifest.json`](../harness/conformance_manifest.json) — current vector-set manifest
- [`../harness/keys/KEYS.md`](../harness/keys/KEYS.md) — deterministic test-key documentation
- [`../canonicalization.md`](../canonicalization.md) — HACP canonicalization
- [`../wire/crypto-profile.md`](../wire/crypto-profile.md) — cryptographic profile
- [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md) — current TypeScript/Go verification report

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)
