# HACP Decision API Contract

**Version:** 0.9.3
**Status:** Draft for public review
**License:** CC BY 4.0

This document defines the language-agnostic programmatic interface for HACP-Core. Implementations in Python, Go, Rust, or any other language MUST adhere to these semantics.

## 1. Core Interface

### 1.1 `evaluate`

```text
function evaluate(
    envelope: IntentEnvelope, 
    action: ProposedAction, 
    context: PolicyContext
) -> AgencyDecision
```

**Semantics:**
1. Validates schema and signatures of `envelope` and `action`.
2. Checks expiry and revocation state.
3. Evaluates scope, boundaries, and autonomy budget against `context`.
4. Returns `ALLOW`, `DENY`, or `CHECKPOINT`.
5. **Constraint:** MUST be deterministic. MUST NOT invoke LLMs or external network calls on the hot path.

### 1.2 `issue_token`

```text
function issue_token(
    envelope_id: UUID, 
    action: ProposedAction, 
    principal: string, 
    constraints: object
) -> DecisionToken
```

**Semantics:**
1. Computes `action_hash` over the canonicalized `action`.
2. Generates a unique `token_id` and sets `expires_at`.
3. Signs the token payload using the active Ed25519 private key.
4. **Constraint:** MUST only be called if `evaluate()` returned `ALLOW`.

### 1.3 `revoke`

```text
function revoke(
    target_id: string, 
    target_kind: enum, 
    reason: string
) -> RevocationRecord
```

**Semantics:**
1. Creates a signed `RevocationRecord`.
2. Updates the local revocation state (denylist).
3. **Constraint:** MUST be idempotent. Revoking an envelope MUST invalidate all descendant tokens.

### 1.4 `explain`

```text
function explain(
    decision: AgencyDecision
) -> Explanation
```

**Semantics:**
1. Returns deterministic `reason_codes` and the specific policy rules that triggered the decision.
2. **Constraint:** MUST NOT require LLMs. MUST be safe to expose to auditors.

## 2. Error Handling and Fail-Closed Mandate

1. Any unexpected exception, missing dependency, or schema validation failure during `evaluate()` or token verification MUST result in a `DENY` decision with the reason code `INTERNAL_ERROR`.
2. Implementations MUST NEVER default to `ALLOW` in the event of an internal failure.
3. All API functions MUST return structured error objects, not raw language exceptions, when crossing module boundaries.

## 3. Conformance Testing Interface

To enable cross-language verification of clean-room implementations, HACP defines two standard conformance testing interfaces. Implementations MUST support at least one of these interfaces to be eligible for conformance certification.

### 3.1 HTTP Interface (Recommended for Enterprise)

Implementations MUST expose an HTTP endpoint that accepts complete test vectors and returns structured decisions.

#### Endpoint: `POST /evaluate`

**Request:**

```http
POST /evaluate HTTP/1.1
Host: localhost:8080
Content-Type: application/json
Accept: application/json

{
  "test_id": "CORE-INV3-001",
  "type": "golden",
  "description": "Token presented with the exact bound action passes verification.",
  "inputs": {
    "intent_envelope": { ... },
    "proposed_action": { ... },
    "decision_token": { ... }
  },
  "policy_context": {
    "current_action_count": 0
  },
  "expected": {
    "outcome": "ALLOW"
  }
}
```

**Response (Success):**

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "decision": "ALLOW",
  "decision_token": {
    "hacp_version": "0.9",
    "token_id": "33333333-3333-3333-3333-333333333333",
    "envelope_id": "22222222-2222-2222-2222-222222222222",
    "action_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "policy_digest": "a1b2c3d4...",
    "principal": "human_admin_01",
    "signer_key_id": "key-ed25519-prod-001",
    "issued_at": 1786000000,
    "expires_at": 1786003600,
    "decision": "ALLOW",
    "signature": "MEUCIQC..."
  },
  "reason_codes": [],
  "explanation": "Action within scope, budget available, valid signature"
}
```

**Response (Failure):**

```http
HTTP/1.1 200 OK
Content-Type: application/json

{
  "decision": "DENY",
  "reason_codes": ["SCOPE_EXCEEDED"],
  "explanation": "Action audience not in envelope scope"
}
```

**Response (Error):**

```http
HTTP/1.1 500 Internal Server Error
Content-Type: application/json

{
  "error": "INTERNAL_ERROR",
  "message": "Schema validation failed",
  "details": { ... }
}
```

#### Requirements:

1. **Accept complete vectors**: The endpoint MUST accept the full test vector JSON, not individual fields.
2. **Return AgencyDecision**: Response MUST conform to `schemas/agency_decision.json`.
3. **Include token for ALLOW**: If `decision` is `ALLOW`, response MUST include a valid `decision_token` with correct `action_hash` and signature.
4. **Fail-closed errors**: Any internal error MUST return HTTP 500 with `INTERNAL_ERROR` code.
5. **CORS headers**: MUST include appropriate CORS headers for cross-origin testing.

#### Optional Endpoints:

- `POST /issue_token` — Direct token issuance (for testing token generation).
- `POST /revoke` — Revocation testing.
- `GET /explain?decision=<base64>` — Explanation retrieval.

### 3.2 CLI Interface (Simpler for Development)

Implementations MUST provide a command-line binary that accepts vector file paths and outputs JSON to stdout.

#### Command: `evaluate`

```bash
./hacp-impl evaluate --vector <path-to-vector.json>
```

**Arguments:**
- `--vector`: Required. Path to JSON test vector file.
- `--public-key`: Optional. Path to public key file for signature verification.
- `--output`: Optional. Output format: `json` (default) or `text`.

**Example:**

```bash
$ ./hacp-go evaluate --vector vectors/core_inv3_001_golden.json
{
  "decision": "ALLOW",
  "decision_token": { ... },
  "reason_codes": [],
  "explanation": "Action within scope"
}
```

**Exit Codes:**
- `0`: Test completed successfully (decision returned).
- `1`: Test failed (error occurred).
- `2`: Invalid arguments.

#### Requirements:

1. **Read from file**: MUST read vector from file path, not stdin.
2. **Output to stdout**: MUST write JSON response to stdout, not stderr.
3. **Structured errors**: Errors MUST be written as JSON to stderr with exit code 1.
4. **No interactive prompts**: MUST NOT require user interaction.
5. **Deterministic**: Same vector MUST produce same output across runs.

#### Error Output:

```bash
$ ./hacp-go evaluate --vector invalid.json 2>&1
{
  "error": "SCHEMA_VALIDATION_FAILED",
  "message": "Invalid intent_envelope structure",
  "details": { ... }
}
```

### 3.3 Conformance Testing Workflow

The official HACP Conformance Harness (`harness/harness.py`) tests implementations as follows:

1. **Load vector**: Read JSON test vector from `vectors/` directory.
2. **Send to target**:
   - HTTP mode: `POST /evaluate` with complete vector.
   - CLI mode: Execute binary with `--vector` flag.
3. **Parse response**: Extract `decision` and `decision_token` (if present).
4. **Verify outcome**: Compare `decision` with `expected.outcome` from vector.
5. **Verify crypto** (for ALLOW decisions):
   - Compute `action_hash` from canonicalized `proposed_action`.
   - Compare with `decision_token.action_hash`.
   - Verify Ed25519 signature if public key available.
6. **Record result**: Pass/fail with detailed diagnostics.

#### Example Test Run:

```bash
$ python harness/harness.py --mode http --target-url http://localhost:8080
============================================================
HACP Conformance Harness v0.9.2 - Mode: http
============================================================

[PASS] CORE-INV3-001: Token presented with the exact bound action passes verification.
[PASS] CORE-INV3-002: Token with mismatched action_hash correctly rejected.
[FAIL] CORE-INV5-002: Token with invalid signature correctly rejected.
       Details: {'signature_invalid': True}

============================================================
RESULTS: 2/3 passed
============================================================
```

### 3.4 Clean-Room Verification Guarantee

This conformance testing architecture provides mathematical proof of clean-room implementation because:

1. **Data isolation**: Test vectors are language-independent JSON files.
2. **Interface isolation**: Communication via HTTP or CLI only.
3. **No code sharing**: Harness has no access to target implementation internals.
4. **Cryptographic verification**: Validates signatures, hashes, and binding.
5. **Deterministic testing**: Same inputs MUST produce same outputs.

If an implementation passes all conformance vectors, it proves:
- The specification is complete and unambiguous.
- The implementation correctly handles all invariants (INV-1 through INV-7).
- Cryptographic operations are implemented correctly.
- Error handling follows the fail-closed mandate.

This enables independent verification of Go, TypeScript, Rust, or any other language implementation without requiring access to the reference Python code.

## 4. Wire Format and Encoding

See `wire/encoding.md` and `wire/crypto-profile.md` for:
- JSON serialization rules.
- Canonicalization algorithm.
- Base64url encoding for signatures.
- HTTP header conventions.
