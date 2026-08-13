# HACP Conformance Vectors

Language-independent test cases for verifying HACP implementations.

Vectors are JSON files that specify inputs, policy context, and expected outcomes.
They are designed to be **byte-reproducible** across platforms and languages through
deterministic canonicalization and fixed test keys.

## Vector Types

### Golden Vectors

Golden vectors represent **valid scenarios** that should result in `ALLOW`.

```json
{
  "test_id": "CORE-INV5-001",
  "type": "golden",
  "description": "Valid Ed25519 signature over canonicalized payload - verification passes",
  "inputs": { ... },
  "policy_context": { ... },
  "expected": {
    "outcome": "ALLOW",
    "action_hash": "75e8e48a67b90604b1b1fcbcbfa3382975e525ef7fe1ca8fbb4510aba5eb40cf"
  },
  "draft_mode": false
}
```

**Characteristics:**
- Valid signatures (computed by `bake_vector.py`)
- Valid timestamps (within expiry window)
- Actions within envelope scope
- `expected.outcome: "ALLOW"`

### Negative Vectors

Negative vectors represent **invalid scenarios** that should result in `DENY` or `CHECKPOINT`.

```json
{
  "test_id": "CORE-INV5-002",
  "type": "negative",
  "description": "Flip one byte in signed payload. Verification must fail and result in DENY.",
  "inputs": { ... },
  "policy_context": { ... },
  "expected": {
    "outcome": "DENY",
    "reason_code": "SIGNATURE_FAILURE"
  },
  "draft_mode": false
}
```

**Characteristics:**
- Invalid signatures (tampered or dummy)
- Expired envelopes/tokens
- Actions outside envelope scope
- `expected.outcome: "DENY"` or `"CHECKPOINT"`

## Vector Structure

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `test_id` | string | Unique identifier (e.g., `CORE-INV5-001`) |
| `type` | string | `"golden"` or `"negative"` |
| `description` | string | Human-readable test description |
| `inputs` | object | Input data (see below) |
| `expected` | object | Expected outcome (see below) |
| `draft_mode` | boolean | `true` before baking, `false` after |

### `inputs` Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `intent_envelope` | object | ✅ | The envelope authorizing the action |
| `proposed_action` | object | ✅ | The action being evaluated |
| `decision_token` | object | ⏸ | Pre-issued token (optional, for binding tests) |
| `policy_context` | object | ⏸ | Runtime context (clock, budget, revocations) |
| `checkpoint` | object | ⏸ | Runtime checkpoint state (Phase 3) |
| `provenance_event` | object | ⏸ | Provenance event to verify (INV-4) |

### `expected` Object

| Field | Type | Description |
|-------|------|-------------|
| `outcome` | string | `"ALLOW"`, `"DENY"`, or `"CHECKPOINT"` |
| `action_hash` | string | Expected action hash (golden vectors only) |
| `reason_code` | string | Expected reason code (negative vectors) |
| `provenance_event_id` | string | Expected provenance event ID (INV-4) |

## Vector Categories

### INV-1: Human Final Decision (4 vectors)

Tests that human oversight is enforced for sensitive operations.

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv1_001_golden.json` | golden | Human principal with human-required action → ALLOW |
| `core_inv1_002_negative.json` | negative | System principal with human-required action → CHECKPOINT |
| `core_inv1_005_negative.json` | negative | Expired delegation envelope → DENY |
| `core_inv1_006_negative.json` | negative | Checkpoint timeout exceeded → DENY |

### INV-2: Boundary Re-Authorization (8 vectors)

Tests that actions stay within envelope boundaries.

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv2_001_golden.json` | golden | All attributes within scope → ALLOW |
| `core_inv2_002_negative.json` | negative | Audience boundary crossing (internal → external) → DENY |
| `core_inv2_003_negative.json` | negative | Reversibility boundary crossing → DENY |
| `core_inv2_004_negative.json` | negative | Externality boundary crossing → DENY |
| `core_inv2_005_negative.json` | negative | Quantity scope exceeded → DENY |
| `core_inv2_006_negative.json` | negative | Destination outside allowlist → DENY |
| `core_inv2_007_negative.json` | negative | Data class boundary crossing → DENY |
| `core_inv2_008_negative.json` | negative | Optional security attribute absent → UNKNOWN_ATTRIBUTE |

### INV-3: Token Binding (4 vectors)

Tests that tokens are cryptographically bound to specific actions.

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv3_001_golden.json` | golden | Valid token with correct action_hash → ALLOW |
| `core_inv3_002_negative.json` | negative | Token with modified action field (hash mismatch) → DENY |
| `core_inv3_003_negative.json` | negative | Cross-envelope token replay → DENY |
| `core_inv3_004_negative.json` | negative | Token presented after expires_at → DENY |

### INV-4: Traceability (5 vectors)

Tests that provenance events are correctly recorded and verified.

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv4_001_golden.json` | golden | Valid signed EVALUATED provenance event → ALLOW |
| `core_inv4_002_golden.json` | golden | Token revoked after issuance → REVOKED provenance |
| `core_inv4_003_negative.json` | negative | Tampered provenance payload_hash → DENY |
| `core_inv4_004_negative.json` | negative | Decision without provenance event → DENY |
| `core_inv4_005_negative.json` | negative | Broken prev_event_hash linkage → DENY |

### INV-5: Cryptographic Integrity (8 vectors)

Tests Ed25519 signatures, canonicalization, and key validation.

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv5_001_golden.json` | golden | Valid Ed25519 signature → ALLOW |
| `core_inv5_002_negative.json` | negative | Tampered signature (one byte flipped) → DENY |
| `core_inv5_003_negative.json` | negative | Signature by unknown key → SIGNATURE_FAILURE |
| `core_inv5_004_negative.json` | negative | Wrong algorithm (HMAC vs Ed25519) → DENY |
| `core_inv5_005_golden.json` | golden | JCS canonicalization with reordered keys → ALLOW |
| `core_inv5_006_negative.json` | negative | Duplicate JSON keys rejected → DENY |
| `core_inv5_007_negative.json` | negative | Non-canonical serialization hash mismatch → DENY |
| `core_inv5_008_negative.json` | negative | Revoked signer_key_id → KEY_REVOKED |

### INV-7: Bounded Autonomy (4 vectors)

Tests autonomy budget enforcement.

| Vector | Type | Tests |
|--------|------|-------|
| `core_inv7_001_golden.json` | golden | System principal within budget (1 of 2) → ALLOW |
| `core_inv7_002_negative.json` | negative | Budget exhausted (N+1)-th action → BUDGET_EXHAUSTED |
| `core_inv7_005_negative.json` | negative | Envelope revoked mid-budget → ENVELOPE_REVOKED |
| `core_inv7_006_negative.json` | negative | Parent envelope revoked → child inherits revocation |

### Runtime: Checkpoint State Machine (5 vectors)

Tests Phase 3 checkpoint lifecycle.

| Vector | Type | Tests |
|--------|------|-------|
| `runtime_001_golden.json` | golden | Checkpoint resolved ALLOW by human → resume ALLOW |
| `runtime_002_negative.json` | negative | Checkpoint timeout (OPEN past expires_at) → EXPIRED → DENY |
| `runtime_003_negative.json` | negative | Resume with token bound to different action_hash → DENY |
| `runtime_004_negative.json` | negative | Resume while checkpoint still OPEN → CHECKPOINT (not ALLOW) |
| `runtime_005_negative.json` | negative | System principal resolves own checkpoint → SELF_APPROVAL_DENIED |

## Adding New Vectors

### Step 1: Create the Vector File

Create a new JSON file in `vectors/` following the naming convention:

```
{category}_{inv}_{number}_{type}.json
```

Examples:
- `core_inv2_009_negative.json` — new INV-2 negative test
- `core_inv6_001_golden.json` — new INV-6 golden test (if added)
- `runtime_006_golden.json` — new runtime test

### Step 2: Write the Vector

Start with `draft_mode: true` and placeholder signature:

```json
{
  "test_id": "CORE-INV2-009",
  "type": "negative",
  "description": "Your test description here",
  "inputs": {
    "intent_envelope": {
      "hacp_version": "0.9",
      "envelope_id": "22222222-2222-2222-2222-222222222222",
      "principal": "human_admin_01",
      "principal_kind": "human",
      "intent_statement": "Test envelope",
      "scope": { ... },
      "issued_at": 1786000000,
      "expires_at": 1786003600,
      "signer_key_id": "key-ed25519-test-001",
      "signature": "PLACEHOLDER"
    },
    "proposed_action": { ... }
  },
  "policy_context": {
    "clock": 1786000100
  },
  "expected": {
    "outcome": "DENY",
    "reason_code": "BOUNDARY_CROSSING"
  },
  "draft_mode": true
}
```

### Step 3: Bake the Vector

Run the baking tool to compute hashes and signatures:

```bash
python tools/bake_vector.py
```

This will:
1. Compute `action_hash = SHA-256(JCS(proposed_action))`
2. Sign the envelope: `signature = Ed25519(test_sk, JCS(envelope_without_signature))`
3. Set `draft_mode: false`
4. Update `policy_context.clock: explicit` (no `time.time()` in runner)

### Step 4: Verify Integrity

```bash
python tools/bake_vector.py --check
```

All vectors should show `✓` (valid hashes and signatures).

### Step 5: Test Locally

```bash
python harness/harness.py --mode local
```

Your new vector should pass (or fail as expected for negative tests).

### Step 6: Regenerate Manifest

```bash
python harness/generate_manifest.py
```

This updates `harness/conformance_manifest.json` with the new vector digest.

### Step 7: Commit Everything

```bash
git add vectors/your_new_vector.json
git add harness/conformance_manifest.json
git commit -m "feat(vectors): add CORE-INV2-009 negative test

Tests: [description of what the vector validates]

Expected: [expected outcome and reason code]"
```

## File Naming Convention

```
{category}_{invariant}_{number}_{type}.json
```

**Categories:**
- `core` — Core invariants (INV-1 through INV-7)
- `runtime` — Phase 3 runtime (checkpoint state machine)

**Types:**
- `golden` — Valid scenario, expected ALLOW
- `negative` — Invalid scenario, expected DENY or CHECKPOINT

**Numbers:**
- Three digits, zero-padded (001, 002, ..., 009, 010, ...)
- Sequential within each invariant category

## Deterministic Baking

All golden vectors are **byte-reproducible** through:

1. **Fixed test keypair** — `harness/keys/test-ed25519-001.seed`
2. **JCS canonicalization** — RFC 8785 (sorted keys, no whitespace)
3. **Explicit timestamps** — No `time.time()` in vectors or runners
4. **Deterministic baking** — Same inputs → same signatures

This ensures vectors produce identical results on any platform.

## Canonicalization

All hashing and signing uses strict JCS canonicalization:

```json
{
  "audiences": ["internal", "external"],
  "data_classes": ["confidential"],
  "envelope_id": "22222222-2222-2222-2222-222222222222"
}
```

**Rules:**
- Keys sorted lexicographically (UTF-8)
- Numbers without `.0` suffix
- Strings with JSON escape rules
- No duplicate keys
- No non-finite floats (NaN, Infinity)

Same logical payload → same canonical bytes → same hash on any platform.

## Policy Context

Vectors may specify `policy_context` to control runtime behavior:

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

**Important:** All timestamps in vectors and policy context must be explicit Unix seconds. Never use `time.time()` or dynamic timestamps — this breaks reproducibility.

## Checkpoint Vectors (Phase 3)

Runtime vectors may include a `checkpoint` object in inputs:

```json
{
  "inputs": {
    "intent_envelope": { ... },
    "proposed_action": { ... },
    "checkpoint": {
      "checkpoint_id": "88888888-8888-8888-8888-888888888888",
      "action_hash": "75e8e48a67b90604b1b1fcbcbfa3382975e525ef7fe1ca8fbb4510aba5eb40cf",
      "state": "RESOLVED_ALLOW",
      "resolver_principal": "human_admin_01",
      "resolver_principal_kind": "human",
      "expires_at": 1786000400
    }
  }
}
```

**Checkpoint states:**
- `OPEN` — Awaiting human approval
- `RESOLVED_ALLOW` — Human approved
- `RESOLVED_DENY` — Human denied
- `EXPIRED` — Timeout exceeded

## References

- Invariants specification: [`../INVARIANTS.md`](../INVARIANTS.md)
- Baking tool: [`../tools/bake_vector.py`](../tools/bake_vector.py)
- Harness documentation: [`../harness/README.md`](../harness/README.md)
- Runner protocol: [`../harness/runner_protocol.md`](../harness/runner_protocol.md)
- Test keypair: [`../harness/keys/KEYS.md`](../harness/keys/KEYS.md)