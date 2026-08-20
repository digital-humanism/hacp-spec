# HACP Wire Headers

Status: Draft
Version: 0.9.3
Phase: 4 Gate D
Normative references:
- HACP-SPEC-0.9-draft.md
- INVARIANTS.md
- wire/encoding.md
- wire/crypto-profile.md
- api/decision-api.md
- profiles/enforcement.md
- schemas/intent_envelope.json
- schemas/decision_token.json

## 1. Required request headers

Every enforceable request MUST carry both headers:

```text
X-HACP-Intent-Envelope
X-HACP-Decision-Token
```

Both header values MUST be base64url-encoded, without padding, of a canonical RFC 8785 JSON object.

Header values MUST contain only ASCII characters.

Maximum header value size:

```text
8 KB per header
```

If a header is missing, malformed, or oversized, the enforcement point MUST deny the request with `INVALID_ENVELOPE` or `INVALID_ACTION`.

## 2. Hash and signature encoding

Per `wire/crypto-profile.md`, this profile uses:

1. SHA-256 hashes as lowercase hexadecimal strings in JSON.
2. Ed25519 signatures (RFC 8032, pure mode) as base64url strings without padding.
3. Timestamps as Unix seconds (UTC).

Canonicalization MUST use RFC 8785 JSON Canonicalization Scheme.

Algorithm negotiation is prohibited. The algorithm is implicitly Ed25519 by virtue of the HACP version.

## 3. X-HACP-Intent-Envelope

The intent envelope header carries the proposed action and the human-agency signature over that action.

Decoded JSON object per `schemas/intent_envelope.json`:

```json
{
  "hacp_version": "0.9",
  "envelope_id": "22222222-2222-2222-2222-222222222222",
  "principal": "human_admin_01",
  "principal_kind": "human",
  "intent_statement": "Export customer records for audit",
  "scope": {
    "verbs": ["export"],
    "resource_classes": ["customer_record"],
    "audiences": ["external"],
    "reversibility": ["irreversible"],
    "externality": ["external"],
    "data_classes": ["confidential"]
  },
  "issued_at": 1786000000,
  "expires_at": 1786003600,
  "signer_key_id": "key-ed25519-test-001",
  "signature": "zDDnVtgJOGKqiLNVpB89kTFzjuu4z-JTsUBzmVsFtO15a626G_rkHDI3aFK8APYBk6JXRc-QcfI-o9MUPnhrBA"
}
```

Required claims per schema:

| Claim | Requirement |
|---|---|
| hacp_version | MUST be "0.9". |
| envelope_id | MUST be present and unique (UUID). |
| principal | MUST be present. |
| principal_kind | MUST be "human" or "system". |
| intent_statement | MUST be present. |
| scope | MUST be present (ScopeGrant). |
| issued_at | MUST be present (Unix seconds). |
| expires_at | MUST be present (Unix seconds). |
| signer_key_id | MUST be present. |
| signature | MUST be present (base64url, no padding). |

The envelope signature MUST be computed as follows:

1. Remove the `signature` member from the envelope object.
2. Canonicalize the remaining object with RFC 8785.
3. Sign the UTF-8 bytes of the canonical output with Ed25519 (RFC 8032, pure mode).
4. Encode the signature as base64url without padding.

## 4. X-HACP-Decision-Token

The decision token header carries an evaluator-signed decision for a specific proposed action and a specific request.

Decoded JSON object per `schemas/decision_token.json`:

```json
{
  "hacp_version": "0.9",
  "token_id": "33333333-3333-3333-3333-333333333333",
  "envelope_id": "22222222-2222-2222-2222-222222222222",
  "action_hash": "5dd0154322d58283f02c6c623e29b66c77b74ea659d1b6a43aaf064d3555a69e",
  "policy_digest": "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2",
  "principal": "human_admin_01",
  "signer_key_id": "key-ed25519-test-001",
  "issued_at": 1786000000,
  "expires_at": 1786003600,
  "decision": "ALLOW",
  "constraints": {
    "method": "POST",
    "path": "/tools/email.send",
    "tool_name": "email.send",
    "payload_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  },
  "signature": "_5TKPo_xMVHrR-ucaVxKuysTuZaPnoSQgG7o7YVLyzYQW9NlwrjZePdNnlP1D2VgAdhwwJDJTLKYN6XP2I45Cw"
}
```

Required claims per schema:

| Claim | Requirement |
|---|---|
| hacp_version | MUST be "0.9". |
| token_id | MUST be present and unique (UUID). |
| envelope_id | MUST match the envelope header. |
| action_hash | MUST match the envelope action hash (SHA-256 of canonical ProposedAction). |
| policy_digest | MUST be present. |
| principal | MUST be present. |
| signer_key_id | MUST be present. |
| issued_at | MUST be present (Unix seconds). |
| expires_at | MUST be present (Unix seconds). |
| decision | MUST be "ALLOW", "DENY", or "CHECKPOINT". |
| signature | MUST be present (base64url, no padding). |

Optional claims for enforcement binding:

| Claim | Requirement |
|---|---|
| constraints | MAY be present for request-level narrow binding. |

The token signature MUST be computed as follows:

1. Remove the `signature` member from the token object.
2. Canonicalize the remaining object with RFC 8785.
3. Sign the UTF-8 bytes of the canonical output with Ed25519 (RFC 8032, pure mode).
4. Encode the signature as base64url without padding.

Only tokens with `decision = ALLOW` may be forwarded by an enforcement point.

Tokens with `decision = DENY` or `decision = CHECKPOINT` MUST NOT be forwarded.

## 5. Request binding rules

The enforcement point MUST verify two levels of binding:

### Level 1: Action binding (MUST)

The token `action_hash` MUST match the SHA-256 hash of the canonical ProposedAction from the envelope.

### Level 2: Request binding (enforcement profile)

If the token includes `constraints`, the enforcement point MUST compare the current request against the token binding claims.

For HTTP requests:

| Binding claim | Value |
|---|---|
| method | Uppercase HTTP method. |
| path | Request path including query string, excluding scheme and authority. |
| tool_name | Stable tool identifier declared by the deployment. |
| payload_hash | SHA-256 (lowercase hex) of the raw HTTP request body. |

For MCP tool calls transported through the sidecar:

| Binding claim | Value |
|---|---|
| method | "MCP" |
| path | "/mcp" |
| tool_name | MCP tool name from the JSON-RPC request. |
| payload_hash | SHA-256 (lowercase hex) of the canonical MCP request body accepted by the sidecar. |

If the request has no body, `payload_hash` MUST be:

```text
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

Any binding mismatch MUST result in denial with `SCOPE_EXCEEDED`.

## 6. Response headers

An enforcement point MUST include the following response headers on denied requests:

```text
X-HACP-Decision: DENY
X-HACP-Reason: <reason_code>
```

On allowed requests, the enforcement point MAY include:

```text
X-HACP-Decision: ALLOW
```

The enforcement point MAY include:

```text
X-HACP-Request-Id: <opaque_request_id>
```

The `X-HACP-Request-Id` header is diagnostic and not part of the signed request.

## 7. Clock tolerance

The enforcement point MAY allow a small clock skew.

Default:

```text
max_clock_skew_seconds = 5
```

Expiry checks MUST remain fail-closed.

If timestamp validation cannot be completed, the request MUST be denied with `ENVELOPE_EXPIRED` or `TOKEN_EXPIRED`.
