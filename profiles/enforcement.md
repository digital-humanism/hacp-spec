# HACP Enforcement Profile

Status: Draft
Version: 0.9.0-draft-enforcement.2
Phase: 4 Gate D
Normative references:
- HACP-SPEC-0.9-draft.md
- INVARIANTS.md
- PROFILES.md
- canonicalization.md
- wire/encoding.md
- wire/crypto-profile.md
- api/decision-api.md
- error-model.md
- checkpoint-protocol.md

## 1. Scope

This profile defines normative behavior for an enforcement point that prevents execution of an agent action unless the request carries a valid HACP ALLOW decision token.

This profile applies to the Phase 4 enforcement MVP.

Requires: HACP-Runtime (and therefore HACP-Core).

## 2. Enforcement point

The enforcement point MUST be positioned between the agent and every enforceable tool transport.

For Phase 4 MVP, enforceable transports are:

1. MCP tool calls carried through the sidecar.
2. HTTP requests sent through an explicit HTTP_PROXY to the sidecar.

All other transports are out of scope for Gate D and MUST be treated as non-enforced unless deployment-level isolation prevents their use.

The enforcement point MUST be fail-closed.

## 3. Core invariant

A request MUST NOT be forwarded unless the enforcement point has verified all of the following:

1. Required HACP headers are present.
2. The intent envelope parses and is canonically valid.
3. The decision token parses and is canonically valid.
4. The token decision is ALLOW.
5. The signer key is not revoked.
6. The token signature is valid.
7. The envelope signature is valid.
8. The envelope is not revoked.
9. The token is not revoked.
10. The envelope is not expired.
11. The token is not expired.
12. The token action_hash matches the envelope action_hash.
13. The token is bound to the current request.
14. The scope check passes.
15. The budget check passes.
16. Revocation state is fresh.
17. A provenance record can be appended.

If any check fails, the enforcement point MUST deny the request and MUST NOT forward any part of the request payload upstream.

## 4. Verification order (normative)

The enforcement point MUST evaluate in the following fixed order:

1. Accept request.
2. Require HACP headers (`X-HACP-Intent-Envelope`, `X-HACP-Decision-Token`).
3. Parse and decode headers (RFC 8785 canonical JSON).
4. Verify envelope schema and required fields.
5. Verify token schema and required fields.
6. Verify token decision is ALLOW (reject DENY/CHECKPOINT immediately).
7. Resolve `signer_key_id` for both envelope and token.
8. Verify signer key is not revoked (key revocation check).
9. Verify token signature.
10. Verify envelope signature.
11. Check envelope revocation state.
12. Check token revocation state.
13. Verify envelope expiry.
14. Verify token expiry.
15. Verify token `action_hash` matches envelope `action_hash`.
16. Verify token binding (request parameters).
17. Verify scope containment.
18. Verify budget and replay state.
19. Verify revocation freshness (bounded staleness).
20. Append provenance record.
21. Forward request.

The enforcement point MUST stop evaluation on first failure.

## 5. Enforcement modes

| Mode | Behavior | Conformance |
|---|---|---|
| enforce | Fail-closed verification and forwarding. | Normative for Gate D. |
| shadow | Logs verification results but does not deny. | Non-conformant. |
| disabled | Bypasses verification. | Non-conformant. |

The default mode MUST be `enforce`.

A conformant deployment MUST NOT silently downgrade to `shadow` or `disabled`.

## 6. Fail modes

The enforcement point MUST deny on any failure.

The following table defines normative HACP reason codes per `error-model.md`.

| Condition | Reason code |
|---|---|
| Missing required HACP header. | `INVALID_ENVELOPE` |
| Header value cannot be decoded. | `INVALID_ENVELOPE` |
| Envelope or token JSON cannot be parsed. | `INVALID_ENVELOPE` or `INVALID_ACTION` |
| Required claim is missing. | `INVALID_ENVELOPE` or `INVALID_ACTION` |
| Unsupported protocol version. | `INVALID_ENVELOPE` |
| Envelope signature invalid. | `SIGNATURE_FAILURE` |
| Token signature invalid. | `SIGNATURE_FAILURE` |
| Signer key cannot be resolved. | `SIGNATURE_FAILURE` |
| Signer key revoked. | `KEY_REVOKED` |
| Envelope expired. | `ENVELOPE_EXPIRED` |
| Token expired. | `TOKEN_EXPIRED` |
| Envelope revoked. | `ENVELOPE_REVOKED` |
| Token revoked. | `TOKEN_REVOKED` |
| Token already consumed. | `TOKEN_REVOKED` |
| Token action_hash mismatch. | `SIGNATURE_FAILURE` |
| Request binding mismatch. | `SCOPE_EXCEEDED` |
| Request method, path, or tool_name outside token scope. | `SCOPE_EXCEEDED` |
| Unknown scope attribute. | `UNKNOWN_ATTRIBUTE` |
| Request crosses declared boundary. | `BOUNDARY_CROSSING` |
| Budget exhausted. | `BUDGET_EXHAUSTED` |
| Budget ledger unavailable. | `BUDGET_EXHAUSTED` |
| Decision token is DENY. | Use token-supplied reason if present; otherwise `POLICY_DENIED`. |
| Decision token is CHECKPOINT and unresolved. | `HUMAN_REQUIRED` |
| Checkpoint not resolved before expiry. | `CHECKPOINT_TIMEOUT` |
| Provenance record cannot be appended. | `TRACEABILITY_FAILURE` |
| Revocation state is stale. | `TRACEABILITY_FAILURE` |
| Control channel is unavailable beyond allowed staleness. | `TRACEABILITY_FAILURE` |
| Provenance chain integrity broken. | `TRACEABILITY_FAILURE` |

The enforcement point MUST NOT invent success semantics when a failure occurs.

## 7. Token binding

An ALLOW decision token MUST be cryptographically bound to the exact proposed action via `action_hash`.

The token MAY include additional binding via the `constraints` object for request-level narrow binding.

Minimum required binding:

1. `action_hash`: SHA-256(JCS(proposed_action)) — MUST match envelope.
2. `envelope_id`: MUST match the envelope header.

Optional binding in `constraints` for enforcement:

1. `method`
2. `path`
3. `tool_name`
4. `payload_hash`

The enforcement point MUST recompute the request payload_hash over the exact request body.

For requests without a body, the payload_hash MUST be the SHA-256 hash of the empty byte string:

```text
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

If any binding claim does not match the current request, the request MUST be denied with `SCOPE_EXCEEDED`.

## 8. Scope guard

The enforcement point MUST verify that the current request is inside the token scope.

The scope check MUST include at least:

1. Transport type.
2. Method.
3. Path.
4. Tool name.
5. Boundary constraints, if present.

If the token scope contains an attribute unknown to the enforcement point, the request MUST be denied with `UNKNOWN_ATTRIBUTE`.

## 9. Budget and replay protection

A decision token MAY contain a budget in `constraints` or inherit from the envelope.

The enforcement point MUST maintain local replay state for consumed tokens and budget counters.

Budget rules:

1. A token MUST NOT be used more times than its budget permits.
2. A consumed single-use token MUST be denied on replay with `TOKEN_REVOKED`.
3. Budget counters MUST be checked atomically before forwarding.
4. If budget state is unavailable, the request MUST be denied fail-closed with `BUDGET_EXHAUSTED`.

## 10. Control channel

The enforcement point MUST support a control channel for revocation and policy freshness.

For Phase 4 MVP, the control channel MUST provide:

1. Authenticated streaming delivery (gRPC streaming recommended).
2. Signed revocation events.
3. Monotonic sequence numbers.
4. Full snapshot resynchronization after reconnect or sequence gap.
5. Local denylist persistence for the current runtime.

Revocation targets MUST include at least:

1. Token identifiers.
2. Envelope identifiers.
3. Signing key identifiers.

The control channel MUST NOT grant ALLOW decisions.

The control channel MUST NOT override a DENY decision by itself.

A CHECKPOINT resolution delivered through the control channel MUST still result in a new evaluator-signed decision token before any action is allowed.

The enforcement point MUST define a maximum revocation staleness threshold.

Default:

```text
max_revocation_staleness_ms = 5000
```

If revocation state is older than this threshold, the enforcement point MUST deny requests fail-closed with `TRACEABILITY_FAILURE`.

## 11. Provenance

The enforcement point MUST maintain a provenance ring buffer.

Each record MUST include at least:

1. Timestamp (Unix seconds).
2. Request identifier.
3. Envelope identifier.
4. Token identifier.
5. Action hash.
6. Decision.
7. Reason code.
8. Enforcement latency.

The flush MAY be asynchronous, but record acceptance MUST happen before the request is forwarded.

If the ring buffer cannot accept a record, the request MUST be denied with `TRACEABILITY_FAILURE`.

Provenance records MUST NOT include the full request payload unless explicitly required by a declared audit policy.

Payload hashes and identifiers are sufficient by default.

## 12. Failure isolation

The enforcement point MUST NOT forward a partial request before verification is complete.

If the enforcement process crashes, the deployment MUST fail closed by preventing direct agent egress.

The enforcement point MUST NOT expose upstream services to the agent without a valid ALLOW decision.

## 13. Deployment requirements for conformance

A conformant Phase 4 deployment MUST ensure that the agent cannot bypass the enforcement point for enforceable transports.

Minimum MVP requirements:

1. The agent MUST be configured with an explicit HTTP_PROXY or MCP endpoint pointing to the sidecar.
2. The agent container or process MUST NOT have unrestricted direct egress.
3. Direct upstream access MUST be blocked by network policy, container policy, or equivalent isolation.
4. Sidecar failure MUST result in action denial for enforceable transports.

Kernel-level enforcement, such as eBPF, is planned after Gate D and is not required for Phase 4 MVP conformance.