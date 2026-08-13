# HACP Conformance Runner Protocol

**Version:** 1
**Status:** Draft for public review
**License:** CC BY 4.0

This document defines the language-neutral protocol between the HACP conformance harness and implementation runners.

## 1. Architecture

```text
hacp-spec
   │
   ├── normative spec
   ├── canonical vectors
   ├── manifest
   └── language-neutral harness
            │
            │ stdin/stdout JSON
            ▼
        black-box runner
            │
      ┌─────┼─────┬─────────┐
      ▼     ▼     ▼         ▼
   Python   Go    TS      Sidecar
```

**Principle:** Conformance validates observable protocol behavior, not implementation structure.

The harness MUST NOT import implementation code directly. Each implementation provides a runner that accepts JSON on stdin and returns JSON on stdout.

## 2. Protocol Version

All messages MUST include a `protocol_version` field.

```json
{
  "protocol_version": "1"
}
```

The harness MUST reject a runner response with an incompatible `protocol_version`.

Version compatibility:
- Harness and runner with the same major version MUST be compatible.
- Harness MAY support multiple minor versions.
- Harness MUST reject unknown major versions with exit code 3.

## 3. Message Format

### 3.1 Request (Harness → Runner)

The harness sends one JSON object per line to the runner's stdin.

```json
{
  "protocol_version": "1",
  "operation": "evaluate",
  "vector_id": "core_inv1_001",
  "input": {
    "intent_envelope": { ... },
    "proposed_action": { ... },
    "decision_token": { ... },
    "policy_context": { ... }
  }
}
```

Required fields:

| Field | Type | Description |
|---|---|---|
| `protocol_version` | string | Protocol version (currently "1") |
| `operation` | string | Operation to perform: `evaluate`, `revoke`, `explain` |
| `vector_id` | string | Identifier of the conformance vector |
| `input` | object | Operation-specific input data |

### 3.2 Response (Runner → Harness)

The runner MUST emit exactly one JSON object per line to stdout.

```json
{
  "protocol_version": "1",
  "decision": "ALLOW",
  "reason_codes": [],
  "action_hash": "5dd0154322d58283f02c6c623e29b66c77b74ea659d1b6a43aaf064d3555a69e",
  "metrics": {
    "latency_ns": 12345
  }
}
```

Required fields:

| Field | Type | Description |
|---|---|---|
| `protocol_version` | string | Protocol version (must match request) |
| `decision` | string | One of: `ALLOW`, `DENY`, `CHECKPOINT` |
| `reason_codes` | array | List of reason codes (empty for ALLOW) |
| `action_hash` | string | SHA-256 hex of canonicalized ProposedAction |

Optional fields:

| Field | Type | Description |
|---|---|---|
| `metrics` | object | Performance metrics (not normative for conformance) |
| `metrics.latency_ns` | integer | Processing latency in nanoseconds |

### 3.3 Output Constraints

The runner MUST adhere to the following constraints:

1. The runner MUST emit exactly one JSON response per input request.
2. The runner MUST NOT write non-JSON data to stdout.
3. Diagnostic output MUST go to stderr.
4. The runner MUST NOT write partial JSON responses.
5. The runner MUST flush stdout after each response.

## 4. Operations

### 4.1 evaluate

Evaluates a proposed action against the policy context.

**Input:**

```json
{
  "operation": "evaluate",
  "input": {
    "intent_envelope": { ... },
    "proposed_action": { ... },
    "decision_token": { ... },
    "policy_context": { ... }
  }
}
```

**Output:**

```json
{
  "decision": "ALLOW",
  "reason_codes": [],
  "action_hash": "abc123..."
}
```

### 4.2 revoke

Revokes a token, envelope, or key.

**Input:**

```json
{
  "operation": "revoke",
  "input": {
    "target_id": "33333333-3333-3333-3333-333333333333",
    "target_kind": "token",
    "reason": "COMPROMISED"
  }
}
```

**Output:**

```json
{
  "decision": "OK",
  "reason_codes": []
}
```

### 4.3 explain

Returns explanation for a decision.

**Input:**

```json
{
  "operation": "explain",
  "input": {
    "decision": "DENY",
    "reason_codes": ["BOUNDARY_CROSSING"]
  }
}
```

**Output:**

```json
{
  "decision": "OK",
  "explanation": {
    "human_readable": "...",
    "policy_rules": [...]
  }
}
```

## 5. Timeout Semantics

The harness MAY configure a timeout for runner responses.

```text
default_timeout_ms = 5000
```

If a runner exceeds the timeout:
1. The harness MAY terminate the runner process.
2. The harness MUST record exit code 3 (runner execution error).
3. The harness MUST NOT count this as a conformance failure.

## 6. Error Handling

### 6.1 Runner Errors

If the runner encounters an internal error, it MUST return:

```json
{
  "protocol_version": "1",
  "decision": "ERROR",
  "reason_codes": ["INTERNAL_ERROR"],
  "error_message": "description of the error"
}
```

The harness MUST treat this as exit code 3 (runner execution error), not as conformance failure.

### 6.2 Malformed Responses

If the runner emits malformed JSON or non-JSON data to stdout:
1. The harness MUST record exit code 3.
2. The harness MUST NOT attempt to parse the malformed output.

### 6.3 Protocol Version Mismatch

If the runner returns a `protocol_version` that the harness does not support:
1. The harness MUST record exit code 3.
2. The harness MUST terminate the runner.

## 7. Exit Codes

| Code | Meaning | Description |
|---|---|---|
| 0 | Conformant | All vectors passed |
| 1 | Conformance failure | One or more vectors failed |
| 2 | Harness/configuration error | Harness setup or vector loading failed |
| 3 | Runner execution/protocol error | Runner crashed, malformed output, or timeout |

## 8. Conformance Manifest

The harness MUST load a conformance manifest that describes the vector set.

```json
{
  "spec_version": "0.9.2",
  "profile": "HACP-Core",
  "vector_set": "core-0.9.2",
  "canonicalization": "JCS-RFC8785",
  "digest_algorithm": "SHA-256",
  "vector_digest": "sha256:abcdef...",
  "total_vectors": 38
}
```

Required fields:

| Field | Type | Description |
|---|---|---|
| `spec_version` | string | HACP specification version |
| `profile` | string | Conformance profile: `HACP-Core`, `HACP-Runtime`, `HACP-Enforcement` |
| `vector_set` | string | Identifier of the vector set |
| `canonicalization` | string | Canonicalization algorithm used for vector digest |
| `digest_algorithm` | string | Hash algorithm used for vector digest |
| `vector_digest` | string | Digest of the canonical vector set |
| `total_vectors` | integer | Total number of vectors in the set |

The harness MUST verify the vector digest before running conformance tests. If the digest does not match, the harness MUST exit with code 2.

## 9. Output Formats

The harness MUST support two output formats:

### 9.1 Human-readable

```text
HACP Conformance Suite
Spec: 0.9.2
Profile: HACP-Core
Vector digest: sha256:abcdef...

[PASS] core_inv1_001
[PASS] core_inv2_001
[FAIL] core_inv3_005 (expected DENY, got ALLOW)
...

37 passed
1 failed

CONFORMANCE FAILURE
```

### 9.2 Machine-readable (JSON)

```json
{
  "spec_version": "0.9.2",
  "profile": "HACP-Core",
  "implementation": "hacp-sidecar",
  "implementation_version": "0.3.0",
  "protocol_version": "1",
  "vector_set_digest": "sha256:abcdef...",
  "vectors": {
    "total": 38,
    "passed": 38,
    "failed": 0
  },
  "result": "conformant",
  "started_at": "2026-08-13T01:00:00Z",
  "duration_ms": 184,
  "failures": []
}
```

## 10. Implementation Requirements

An implementation MUST provide a runner that:

1. Accepts JSON requests on stdin.
2. Returns JSON responses on stdout.
3. Handles all operations defined in Section 4.
4. Emits exactly one response per request.
5. Writes diagnostics to stderr, not stdout.
6. Exits cleanly when stdin is closed.
7. Supports the protocol version declared in the harness manifest.

## 11. Harness Requirements

The harness MUST:

1. Load vectors from a configurable path (`--vectors` flag or `HACP_VECTORS_PATH`).
2. Verify the vector digest before running tests.
3. Send requests to the runner via stdin.
4. Read responses from the runner via stdout.
5. Compare responses to expected outcomes.
6. Support both human-readable and JSON output formats.
7. Exit with the appropriate code per Section 7.
8. Handle runner timeouts per Section 5.

## 12. Reference

- HACP-SPEC-0.9-draft.md — normative specification
- error-model.md — reason codes
- canonicalization.md — JCS canonicalization rules
- wire/crypto-profile.md — cryptographic requirements