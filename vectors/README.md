# HACP Conformance Vectors

Language-independent canonical test cases for verifying HACP implementations.

The vector set is the normative executable baseline for **HACP-Core v0.9.2**.
Vectors are JSON files that define protocol inputs, explicit policy context,
and expected observable outcomes.

They are designed to be **byte-reproducible** across platforms and languages
through deterministic canonicalization, explicit timestamps, fixed test keys,
and a manifest-pinned vector set.

## Current Canonical Baseline

```text
Spec:          HACP-Core v0.9.2
Vector set:    core-0.9.2
Vectors:       38
Draft vectors: 0
Manifest:      verified
Vector digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

Current cross-language verification against this exact baseline:

| Implementation | Result |
|----------------|--------|
| `humanist-core` (Python) | 38/38 ✅ |
| `hacp-ts` (TypeScript) | 38/38 ✅ |
| `hacp-go` (Go) | 38/38 ✅ |
| `hacp-sidecar` runner (Go) | 38/38 ✅ |

Additional verification:

```text
TypeScript total suite:         44/44 PASS
Python full regression:        324/324 PASS
Python statement coverage:        100%
Python branch coverage:           100%
Python ↔ Go real sidecar E2E:      5/5 PASS
```

Detailed TypeScript and Go verification record:

- [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md)

---

## Vector Types

### Golden Vectors

Golden vectors represent valid protocol scenarios that are expected to
complete successfully, usually with `ALLOW`.

Example shape:

```json
{
  "test_id": "CORE-INV5-001",
  "type": "golden",
  "description": "Valid Ed25519 signature over canonicalized payload - verification passes",
  "inputs": {
    "intent_envelope": {},
    "proposed_action": {},
    "decision_token": {}
  },
  "policy_context": {
    "clock": 1786000100
  },
  "expected": {
    "outcome": "ALLOW",
    "reason_codes": []
  },
  "draft_mode": false
}
```

Typical characteristics:

- valid signatures;
- valid timestamps;
- action inside granted scope;
- valid token/envelope binding where a token is present;
- valid provenance where provenance is required;
- `draft_mode: false`.

### Negative Vectors

Negative vectors represent invalid, unsafe, expired, revoked, malformed,
or unresolved scenarios.

Expected outcomes are `DENY` or `CHECKPOINT`, together with the normative
reason code(s).

Example shape:

```json
{
  "test_id": "CORE-INV5-002",
  "type": "negative",
  "description": "Tampered signed payload must fail verification",
  "inputs": {
    "intent_envelope": {},
    "proposed_action": {},
    "decision_token": {}
  },
  "policy_context": {
    "clock": 1786000100
  },
  "expected": {
    "outcome": "DENY",
    "reason_codes": ["SIGNATURE_FAILURE"]
  },
  "draft_mode": false
}
```

Negative vectors cover, among other cases:

- expired envelopes or tokens;
- action-boundary crossings;
- token hash mismatch;
- token/envelope mismatch;
- key, token, or envelope revocation;
- invalid or unsupported cryptographic profile;
- malformed provenance;
- missing provenance;
- duplicate JSON keys;
- exhausted autonomy budget;
- unresolved or invalid checkpoint state.

---

## Vector Structure

### Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_id` | string | ✅ | Unique vector identifier, e.g. `CORE-INV5-001` |
| `type` | string | ✅ | `golden` or `negative` |
| `description` | string | ✅ | Human-readable purpose of the vector |
| `inputs` | object | ✅ | Protocol inputs |
| `policy_context` | object | optional | Explicit runtime/policy state |
| `expected` | object | ✅ | Expected observable result |
| `draft_mode` | boolean | ✅ for baked set | `false` for every canonical v0.9.2 vector |

### `inputs`

Common fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `intent_envelope` | object | ✅ | Envelope authorizing the proposed action |
| `proposed_action` | object | ✅ | Action being evaluated |
| `decision_token` | object/null | optional | Pre-issued token used by binding/lifecycle vectors |
| `checkpoint` | object | optional | Runtime checkpoint state |
| `checkpoint_state` | object | optional | Additional checkpoint lifecycle state where applicable |
| `provenance_event` | object | optional | Provenance event to verify |
| `revocation_record` | object | optional | Revocation fixture used by revocation/provenance vectors |
| `omit_provenance` | boolean | optional | Explicit traceability-negative test control |

### `expected`

Common fields:

| Field | Type | Description |
|-------|------|-------------|
| `outcome` | string | `ALLOW`, `DENY`, or `CHECKPOINT` |
| `reason_codes` | array[string] | Normative reason code list; empty for successful vectors |
| `action_hash` | string | Expected action hash when the vector asserts one |
| `provenance_event_id` | string | Expected provenance event identifier when applicable |
| `provenance_events` | array/object | Expected provenance event collection when asserted by the vector |

Implementations should compare observable protocol behavior, not internal
implementation structure.

---

## Vector Inventory

### INV-1 — Human Final Decision / Authority (4 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv1_001_golden.json` | golden | Human principal with human-required consequence → `ALLOW` |
| `core_inv1_002_negative.json` | negative | System principal attempts human-required action → `CHECKPOINT` or normative denial path |
| `core_inv1_005_negative.json` | negative | Expired delegation envelope → `ENVELOPE_EXPIRED` |
| `core_inv1_006_negative.json` | negative | Checkpoint timeout → denial |

### INV-2 — Boundary Re-Authorization (8 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv2_001_golden.json` | golden | All action attributes within scope → `ALLOW` |
| `core_inv2_002_negative.json` | negative | Audience boundary crossing |
| `core_inv2_003_negative.json` | negative | Reversibility boundary crossing |
| `core_inv2_004_negative.json` | negative | Externality boundary crossing |
| `core_inv2_005_negative.json` | negative | Quantity exceeds scope |
| `core_inv2_006_negative.json` | negative | Destination outside allowlist |
| `core_inv2_007_negative.json` | negative | Data-class boundary crossing |
| `core_inv2_008_negative.json` | negative | Security-relevant attribute absent → `UNKNOWN_ATTRIBUTE` |

### INV-3 — Token Binding (4 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv3_001_golden.json` | golden | Token presented with the exact bound action |
| `core_inv3_002_negative.json` | negative | Action changed after token issuance (`quantity` added) → `HASH_MISMATCH` |
| `core_inv3_003_negative.json` | negative | Token bound to a different envelope → denial |
| `core_inv3_004_negative.json` | negative | Token presented after `expires_at` → `TOKEN_EXPIRED` |

`CORE-INV3-002` is fully baked and normative. It is no longer a draft
placeholder.

### INV-4 — Traceability / Provenance (5 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv4_001_golden.json` | golden | Valid signed `EVALUATED` provenance event |
| `core_inv4_002_golden.json` | golden | Revocation-related provenance behavior |
| `core_inv4_003_negative.json` | negative | Tampered provenance `payload_hash` |
| `core_inv4_004_negative.json` | negative | Required provenance omitted |
| `core_inv4_005_negative.json` | negative | Broken `prev_event_hash` linkage |

### INV-5 — Cryptographic Integrity (8 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv5_001_golden.json` | golden | Valid Ed25519 signature |
| `core_inv5_002_negative.json` | negative | Signed payload modified after signing → `SIGNATURE_FAILURE` |
| `core_inv5_003_negative.json` | negative | Unknown/untrusted signing key → `SIGNATURE_FAILURE` |
| `core_inv5_004_negative.json` | negative | Unsupported HMAC signer/profile → `SIGNATURE_FAILURE` |
| `core_inv5_005_golden.json` | golden | Reordered logical JSON produces identical JCS action hash |
| `core_inv5_006_negative.json` | negative | Duplicate JSON keys are rejected |
| `core_inv5_007_negative.json` | negative | Non-canonical serialization / incorrect direct hash → denial |
| `core_inv5_008_negative.json` | negative | Revoked signer key → `KEY_REVOKED` |

`CORE-INV5-002` is fully baked and normative. It tests **payload
tampering after signing**, not merely a random malformed signature.

### INV-7 — Bounded Autonomy (4 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv7_001_golden.json` | golden | System principal remains within autonomy budget |
| `core_inv7_002_negative.json` | negative | Budget exhausted → `BUDGET_EXHAUSTED` |
| `core_inv7_005_negative.json` | negative | Envelope revoked during budget lifecycle → `ENVELOPE_REVOKED` |
| `core_inv7_006_negative.json` | negative | Parent-envelope revocation inherited by child |

### Runtime — Checkpoint State Machine (5 vectors)

| Vector | Type | Tests |
|--------|------|-------|
| `runtime_001_golden.json` | golden | Human resolves checkpoint `ALLOW`; execution resumes |
| `runtime_002_negative.json` | negative | Open checkpoint passes expiry → denial |
| `runtime_003_negative.json` | negative | Resume token bound to different action hash |
| `runtime_004_negative.json` | negative | Resume attempted while checkpoint remains `OPEN` |
| `runtime_005_negative.json` | negative | System principal attempts self-resolution → `SELF_APPROVAL_DENIED` |

---

## Deterministic Cryptographic Baseline

The fixed test identity is intentionally public and exists only for
reproducible conformance testing.

Seed derivation:

```text
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
```

Deterministic seed:

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

**Security notice:** this key is public test material and MUST NOT be
used for production signing.

---

## Deterministic Baking

Canonical vectors are byte-reproducible through:

1. fixed test identity;
2. JCS canonicalization;
3. explicit timestamps;
4. deterministic SHA-256 action hashing;
5. deterministic Ed25519 signatures for fixed input;
6. manifest pinning.

Canonical action binding:

```text
action_hash = SHA256(JCS(proposed_action))
```

Envelope signing:

```text
signature = Ed25519(test_sk, JCS(intent_envelope_without_signature))
```

Token signing:

```text
signature = Ed25519(test_sk, JCS(decision_token_without_signature))
```

No vector or conformant runner should depend on `time.time()` or any
other wall-clock value during normative evaluation.

---

## Canonicalization and Malformed JSON

All values that reach canonicalization use the HACP JCS/RFC 8785
canonicalization profile:

- object keys are sorted deterministically;
- no insignificant whitespace;
- strings follow JSON escaping rules;
- non-finite numeric values are rejected;
- equivalent logical objects produce identical canonical bytes.

### Duplicate-key exception for negative testing

Canonical HACP JSON **does not permit duplicate object keys**.

However, `CORE-INV5-006` intentionally contains duplicate raw JSON keys
as a malformed-input negative vector.

That vector must be rejected **before canonicalization**.

Implementations must not silently normalize the duplicate-key payload by
keeping the first or last value and then treating the result as valid
canonical JSON.

Expected fail-closed behavior:

```text
DENY / INVALID_ACTION
```

This distinction is important:

```text
valid HACP JSON        → canonicalize
duplicate-key raw JSON → reject
```

---

## Policy Context

Vectors may provide explicit top-level `policy_context` to control
runtime behavior deterministically.

Example:

```json
{
  "policy_context": {
    "clock": 1786000100,
    "current_action_count": 0,
    "revoked_tokens": ["token-id-123"],
    "revoked_envelopes": ["envelope-id-456"],
    "revoked_keys": ["key-id-789"],
    "human_required_verbs": ["delete"],
    "checkpoint_timeout_seconds": 3600
  }
}
```

Common context controls include:

```text
clock / current_time
current_action_count
revoked_tokens
revoked_envelopes
revoked_keys
trusted_keys
human_required_verbs
checkpoint_timeout_seconds
```

All timestamps must be explicit Unix seconds.

---

## Checkpoint Vectors

Runtime vectors may include a checkpoint object:

```json
{
  "inputs": {
    "intent_envelope": {},
    "proposed_action": {},
    "checkpoint": {
      "checkpoint_id": "88888888-8888-8888-8888-888888888888",
      "state": "RESOLVED_ALLOW",
      "resolver_principal": "human_admin_01",
      "resolver_principal_kind": "human",
      "expires_at": 1786000400
    }
  }
}
```

Common checkpoint states:

- `OPEN` — waiting for resolution;
- `RESOLVED_ALLOW` — approved;
- `RESOLVED_DENY` — denied;
- `EXPIRED` — timeout exceeded.

A system principal may not resolve its own human-required checkpoint.

---

## Adding or Modifying Vectors

Changes to the canonical vector set are protocol-significant because
they change the executable conformance baseline.

### Step 1 — Create or edit the vector

Use the naming convention:

```text
{category}_{invariant}_{number}_{type}.json
```

Examples:

```text
core_inv2_009_negative.json
core_inv6_001_golden.json
runtime_006_golden.json
```

### Step 2 — Start new bakeable vectors as draft

For vectors that require generated hashes/signatures:

```json
{
  "draft_mode": true
}
```

Use placeholders only during vector authoring.

A release/conformance claim must never be made while canonical vectors
remain in draft mode.

### Step 3 — Bake

From the repository root:

```bash
python tools/bake_vector.py
```

The baking process computes or refreshes the deterministic cryptographic
material required by the vector.

### Step 4 — Verify vector integrity

```bash
python tools/bake_vector.py --check
```

### Step 5 — Run local/spec validation

```bash
python harness/harness.py --mode local
```

### Step 6 — Regenerate the manifest

Any intentional vector modification changes the vector-set digest.

```bash
python harness/generate_manifest.py
```

The updated manifest and vector change must be committed together.

### Step 7 — Re-run cross-language conformance

At minimum, verify all implementations whose conformance is claimed.

For the Go sidecar runner:

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

Expected current baseline:

```text
Manifest verified
RESULTS: 38/38 passed
```

For TypeScript:

```powershell
cd ...\GitHub\Dev\hacp-spec\hacp-ts
npm ci
npm run build
npm test
```

Expected current baseline:

```text
44 tests
44 pass
0 fail
0 skipped
```

Python and other downstream implementations must likewise be revalidated
against the updated canonical set before their conformance claims are
renewed.

---

## Manifest Discipline

The canonical vector set is pinned by:

```text
harness/conformance_manifest.json
```

Current digest:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

A digest mismatch means one of two things:

1. vectors changed without regenerating the manifest; or
2. the harness is pointed at a different vector directory.

Do **not** disable or bypass manifest verification to force a conformance
run to continue.

The manifest gate exists specifically to prevent silent vector drift.

---

## File Naming Convention

```text
{category}_{invariant}_{number}_{type}.json
```

Categories:

- `core` — HACP-Core invariant vectors;
- `runtime` — checkpoint/runtime vectors.

Types:

- `golden` — expected successful/valid behavior;
- `negative` — expected fail-closed or checkpoint behavior.

Numbers are three-digit, zero-padded identifiers within each group.

---

## Assurance Boundary

The vector set provides:

```text
normative executable examples
cross-language conformance baseline
regression protection
reproducible cryptographic fixtures
interoperability evidence
```

Passing all vectors is not by itself:

```text
a formal proof of correctness
a complete security proof
a substitute for fuzzing
a substitute for property-based testing
a substitute for adversarial production testing
```

The 38-vector manifest-verified baseline should remain a release gate
while additional assurance mechanisms are added.

---

## References

- Invariants specification: [`../INVARIANTS.md`](../INVARIANTS.md)
- Harness documentation: [`../harness/README.md`](../harness/README.md)
- Runner protocol: [`../harness/runner_protocol.md`](../harness/runner_protocol.md)
- Conformance manifest: [`../harness/conformance_manifest.json`](../harness/conformance_manifest.json)
- Baking tool: [`../tools/bake_vector.py`](../tools/bake_vector.py)
- Test keypair: [`../harness/keys/KEYS.md`](../harness/keys/KEYS.md)
- TypeScript and Go conformance report: [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md)

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)
