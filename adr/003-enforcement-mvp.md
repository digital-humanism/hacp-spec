# ADR-003: Enforcement MVP Scope

Status: Proposed
Date: 2026-06-16
Phase: 4 Gate D

## Context

HACP requires that an action cannot be executed without passing through `evaluate()`.

Phase 4 introduces enforcement outside the agent process. The enforcement mechanism must not require business-logic changes in the agent, but it also must not introduce hidden compromises, kernel dependencies, or opaque interception.

The project currently has reproducible conformance vectors for decision evaluation, but no production enforcement sidecar.

## Decision

The Phase 4 enforcement MVP will be implemented as a user-space sidecar named `hacp-sidecar`.

The MVP scope is limited to:

1. MCP tool calls routed through the sidecar.
2. HTTP requests sent through an explicit `HTTP_PROXY` to the sidecar.
3. Verification of `X-HACP-Intent-Envelope` and `X-HACP-Decision-Token`.
4. Scope guard checks.
5. Budget and replay checks.
6. Ed25519 signature verification.
7. Revocation via an authenticated control channel.
8. Local provenance ring buffer with asynchronous flush.
9. Docker Compose demonstration with agent, sidecar, mock control plane, and mock upstream.
10. Allow-path latency benchmark with recorded p99 overhead.

The following are explicitly out of scope for Phase 4:

1. eBPF enforcement.
2. Transparent iptables or network-layer interception.
3. Dynamic library injection.
4. Agent bytecode modification.
5. Full OS-level process isolation.
6. Generic non-HTTP tool transports.

MVP enforcement statement:

```text
MCP + explicit HTTP_PROXY only; eBPF = later.
```

## Consequences

Positive:

1. The sidecar can be implemented and audited without kernel tooling.
2. Existing conformance vectors can be reused through a header-generation test harness.
3. The agent can remain unchanged except for proxy or endpoint configuration.
4. Fail-closed behavior is testable at the network boundary.
5. The architecture remains compatible with later OS-level enforcement.

Negative:

1. An agent with unrestricted raw network access can bypass an explicit proxy.
2. HTTPS interception is not free; CONNECT tunneling cannot be blindly allowed.
3. MCP transports other than the supported sidecar transport are not enforced.
4. Revocation freshness depends on control channel availability.

Required deployment mitigation:

A conformant MVP deployment MUST block direct agent egress to enforceable upstreams. The sidecar MUST be the only permitted egress path for enforceable transports.

## Alternatives considered

### 1. Modify agent SDK only

Rejected because it does not prevent a compromised or rewritten agent from bypassing evaluation.

### 2. Transparent proxy

Rejected for MVP because it increases deployment complexity and hides interception assumptions.

### 3. eBPF enforcement

Deferred. It provides stronger bypass resistance but is too heavy for Gate D and reduces portability.

### 4. API gateway only

Rejected as the sole mechanism because HACP enforcement must be protocol-aware, budget-aware, revocation-aware, and traceable by design.