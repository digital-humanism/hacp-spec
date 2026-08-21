# HACP — Human Agency Continuity Protocol

[![conformance](https://github.com/digital-humanism/hacp-spec/actions/workflows/conformance.yml/badge.svg)](https://github.com/digital-humanism/hacp-spec/actions/workflows/conformance.yml)
[![release](https://img.shields.io/github/v/release/digital-humanism/hacp-spec?label=release)](https://github.com/digital-humanism/hacp-spec/releases/latest)

**Version:** 0.9.3
**Status:** Phase 1–3 complete · Phase 4 Gates A–E closed  
**License:** CC BY 4.0

HACP is a language-agnostic protocol for preserving human agency in AI agent systems.

It defines a deterministic, cryptographically verifiable authorization layer that evaluates proposed actions **before execution**, binds decisions to exact action semantics, constrains delegated autonomy, and fails closed when authority or distributed control state cannot be trusted.

HACP is informed by the human-agency principles articulated in the [Digital Humanism Manifesto](https://github.com/digital-humanism/manifesto), particularly the preservation of human agency and the use of semantic checkpoints in agentic systems.

The Manifesto is broader than HACP. This specification defines one concrete technical realization of selected principles rather than a normative implementation of the Manifesto as a whole.

---

## Why HACP

Agent systems can act across APIs, infrastructure, financial systems, data stores, and other machine-controlled environments faster than humans can continuously supervise them.

HACP addresses that problem by separating:

```text
intent
  ↓
authorization
  ↓
cryptographic decision
  ↓
pre-execution enforcement
  ↓
action
```

The protocol is designed so that an agent cannot silently reinterpret a human authorization after the decision has been issued.

---

## Core Principles

1. **Pre-execution enforcement** — authorization is evaluated before action execution.
2. **Deterministic hot path** — no LLM is required on the enforcement decision path.
3. **Cryptographic binding** — decision tokens are bound to exact canonical action hashes.
4. **Fail-closed mandate** — internal, cryptographic, scope, freshness, or distributed-state failures never become implicit `ALLOW`.
5. **Scope containment** — actions must remain within the authority granted by the intent envelope.
6. **Bounded autonomy** — system principals may act only within explicitly granted budgets.
7. **Traceability** — accepted actions remain attributable through provenance records.
8. **Distributed convergence** — enforcement replicas converge on the same revocation state and security outcome.

---

# Quick Start

## Run the canonical conformance suite

```bash
pip install -r harness/requirements.txt

python harness/harness.py --mode local
```

Expected result:

```text
============================================================
HACP Conformance Harness v0.9.2 - Mode: local
============================================================

...
============================================================
RESULTS: 38/38 passed
============================================================
```

The canonical HACP-Core v0.9.2 vector set contains **38 normative vectors**.

---

## Run conformance through the runner protocol

For language-neutral verification over stdin/stdout JSON:

```bash
python harness/harness_runner.py \
  --runner "./path/to/implementation-runner" \
  --vectors-dir vectors \
  --manifest harness/conformance_manifest.json \
  --implementation-name my-impl \
  --implementation-version 0.9.2
```

Expected result:

```text
============================================================
HACP Conformance Harness v0.9.2 - Runner Mode

Protocol version: 1
Spec: 0.9.2 (HACP-Core)
============================================================

Manifest verified: 0.9.2 (HACP-Core)
Vector set: core-0.9.2

...
============================================================
RESULTS: 38/38 passed
============================================================
```

Runner protocol specification:

[`harness/runner_protocol.md`](harness/runner_protocol.md)

---

## Verify vector integrity

```bash
python tools/bake_vector.py --check
```

This verifies the canonical vector hashes and baked signatures.

---

# Cross-Language Conformance Baseline

HACP-Core v0.9.2 has converged across the independent Python, TypeScript, Go, and enforcement-sidecar implementations.

```text
Canonical vectors: 38

Go clean-room implementation        38/38 PASS
TypeScript clean-room implementation 38/38 PASS
Python reference implementation      38/38 PASS
Go enforcement sidecar               38/38 PASS

Normative failures:                   0
Skipped normative vectors:            0
Manifest verified:                   YES
```

Additional regression evidence:

```text
TypeScript total suite:              44/44 PASS
Python full regression:            324/324 PASS
Python statement coverage:             100%
Python branch coverage:                100%
Python ↔ Go real sidecar E2E:           5/5 PASS
```

This is a reproducible interoperability and regression milestone. It is **not** presented as a formal security proof.

---

# Reproducibility Guarantees

HACP conformance vectors are designed to be byte-reproducible across supported implementations.

## Fixed test keypair

```text
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
public_key = Ed25519_derive_public(seed)
```

The deterministic test keypair is stored under:

```text
harness/keys/
├── KEYS.md
├── test-ed25519-001.pub
└── test-ed25519-001.seed
```

> **Security notice:** these keys are intentionally public and exist only for reproducible conformance testing. They MUST NOT be used in production.

---

## Deterministic vector baking

```bash
python tools/bake_vector.py
```

For each golden vector:

1. `action_hash = SHA-256(JCS(proposed_action))`
2. `signature = Ed25519(test_sk, JCS(token_without_signature))`
3. `draft_mode = false`
4. `policy_context.clock` is explicit and deterministic

No runtime wall clock is required to evaluate canonical vectors.

---

## Conformance manifest

The canonical vector set is pinned by:

[`harness/conformance_manifest.json`](harness/conformance_manifest.json)

Example:

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

The harness validates the manifest before executing the vector set.

Regenerate it after an intentional vector update:

```bash
python harness/generate_manifest.py
```

---

# Canonicalization and Cryptography

All hashing and signing operate on deterministic JSON canonicalization compatible with RFC 8785 expectations.

Core properties:

- object keys sorted deterministically;
- canonical numeric encoding;
- standard JSON escaping;
- duplicate JSON keys rejected;
- non-finite numbers rejected;
- no presentation whitespace in canonical bytes.

The resulting invariant is:

```text
same logical payload
→ same canonical bytes
→ same SHA-256 digest
→ same signature input
```

The current cryptographic profile uses:

- SHA-256;
- Ed25519;
- deterministic canonical payload encoding.

See:

- [`canonicalization.md`](canonicalization.md)
- [`wire/crypto-profile.md`](wire/crypto-profile.md)
- [`wire/encoding.md`](wire/encoding.md)

---

# Verified Implementations

| Implementation | Role | Language | HACP-Core v0.9.2 |
|---|---|---|---:|
| `hacp-go` | Clean-room implementation | Go | ✅ 38/38 |
| `hacp-ts` | Clean-room implementation | TypeScript | ✅ 38/38 |
| [`humanist-core`](https://github.com/digital-humanism/humanist-core) | Reference SDK | Python | ✅ 38/38 |
| [`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar) | Enforcement proxy | Go | ✅ 38/38 |

Any implementation can be tested through the language-neutral runner protocol without embedding the implementation into the harness.

---

# Conformance Testing Workflow

A new implementation should:

1. implement the normative evaluation semantics;
2. implement canonicalization exactly;
3. verify Ed25519 signatures over canonical payload bytes;
4. expose a runner compatible with [`harness/runner_protocol.md`](harness/runner_protocol.md);
5. execute the canonical manifest-pinned vector set;
6. return the expected outcome and reason semantics for all 38 vectors.

A conformant implementation must not special-case vector IDs.

---

# Test Coverage

## Normative invariants

| Invariant | Description | Vectors |
|---|---|---:|
| **INV-1** | Human Final Decision | 4 |
| **INV-2** | Boundary Re-Authorization | 8 |
| **INV-3** | Token Binding | 4 |
| **INV-4** | Traceability | 5 |
| **INV-5** | Cryptographic Integrity | 8 |
| **INV-7** | Bounded Autonomy | 4 |
| **Runtime** | Checkpoint state machine | 5 |
| **Total** |  | **38** |

The canonical set includes golden and negative vectors covering valid authority, tampering, replay, expiry, scope violations, provenance failures, revocation, checkpoint behavior, and bounded autonomy.

---

# Distributed Control Plane

Phase 4 Gate E adds a distributed revocation/control-state protocol for enforcement sidecars.

Normative gRPC contract:

```text
proto/hacp/control/v1/control_plane.proto
```

The control plane supports:

- atomic revocation snapshots;
- resumable server-streaming revocation updates;
- durable monotonic revisions;
- transport-local stream sequence numbers;
- heartbeats;
- replay after reconnect;
- explicit `ResetRequired` recovery when replay history is unavailable.

## Revision semantics

For a sidecar whose highest fully materialized revision is `R`:

```text
event.revision == R + 1
→ apply event
→ advance to R + 1

event.revision <= R
→ duplicate / old event
→ ignore

event.revision > R + 1
→ revision gap
→ unsafe / fail closed
→ recover
```

`last_seen_revision` means the highest revision whose state has been fully materialized locally.

It does **not** mean the highest revision merely observed on the wire.

---

## Recovery model

Startup:

```text
GetRevocationSnapshot
        ↓
snapshot @ revision R
        ↓
materialize local state
        ↓
WatchRevocations(after_revision=R)
```

Reconnect:

```text
disconnect
    ↓
reconnect(after_revision=last_seen_revision)
    ↓
replay missed events
    ↓
resume live stream
```

Replay history unavailable:

```text
ResetRequired
    ↓
fresh snapshot
    ↓
atomic local replacement
    ↓
resume stream
```

---

## Freshness and fail-closed behavior

A short control-plane disconnect does not immediately make an enforcement sidecar unusable.

The sidecar may continue using the last fully materialized state while that state remains within the configured freshness interval.

When distributed control state becomes stale or unsafe:

```text
DENY
CONTROL_STATE_STALE
```

Examples include:

- freshness interval exceeded;
- revision gap;
- inconsistent heartbeat;
- unsafe distributed state pending snapshot recovery.

Heartbeats refresh freshness but **never** advance durable revision state or skip missing revocation events.

---

## Multi-sidecar convergence

Gate E validates multiple independent sidecars connected to the same control plane.

After a distributed revocation at revision `N`:

```text
Control Plane
     │
     ├──────────────┐
     ▼              ▼
Sidecar A       Sidecar B
revision=N      revision=N
revoked         revoked
DENY            DENY
```

The tested system converges on both:

1. the same fully materialized revision;
2. the same security outcome.

---

# Repository Structure

```text
hacp-spec/
├── LICENSE
├── README.md
├── requirements.txt
│
├── HACP-SPEC-0.9-draft.md
├── INVARIANTS.md
├── PROFILES.md
├── NON-GOALS.md
├── canonicalization.md
├── threat-model.md
├── versioning.md
├── error-model.md
│
├── schemas/
│   ├── intent_envelope.json
│   ├── proposed_action.json
│   ├── decision_token.json
│   ├── agency_decision.json
│   ├── provenance_event.json
│   └── revocation_record.json
│
├── api/
│   └── decision-api.md
│
├── wire/
│   ├── encoding.md
│   └── crypto-profile.md
│
├── proto/
│   └── hacp/
│       └── control/
│           └── v1/
│               └── control_plane.proto
│
├── vectors/
│   ├── core_inv1_*.json
│   ├── core_inv2_*.json
│   ├── core_inv3_*.json
│   ├── core_inv4_*.json
│   ├── core_inv5_*.json
│   └── core_inv7_*.json
│
├── harness/
│   ├── harness.py
│   ├── harness_runner.py
│   ├── runner_protocol.md
│   ├── conformance_manifest.json
│   ├── generate_manifest.py
│   ├── requirements.txt
│   ├── keys/
│   │   ├── KEYS.md
│   │   ├── test-ed25519-001.pub
│   │   └── test-ed25519-001.seed
│   └── README.md
│
├── tools/
│   ├── gen_test_keys.py
│   └── bake_vector.py
│
├── hacp-go/
│   ├── go.mod
│   ├── main.go
│   ├── canonical.go
│   ├── crypto.go
│   └── evaluate.go
│
└── hacp-ts/
    ├── package.json
    ├── package-lock.json
    ├── tsconfig.json
    ├── src/
    │   ├── cli.ts
    │   ├── canonical.ts
    │   ├── crypto.ts
    │   ├── evaluate.ts
    │   └── conformance.ts
    └── tests/
        ├── action-hash.test.ts
        └── conformance.test.ts
```

---

# API Contract

See [`api/decision-api.md`](api/decision-api.md) for the language-neutral interface.

The protocol defines core operations around:

- evaluation;
- token issuance;
- revocation;
- explanation;
- conformance execution.

The runner protocol is the preferred black-box interoperability boundary for independent implementations.

---

# Implementation Status

## Python reference implementation

Repository:

[`humanist-core`](https://github.com/digital-humanism/humanist-core)

Current verified baseline:

```text
HACP-Core vectors:       38/38 PASS
Full regression:        324/324 PASS
Statement coverage:         100%
Branch coverage:            100%
Python ↔ Go sidecar E2E:    5/5 PASS
```

---

## Go and TypeScript clean-room implementations

Both independent implementations pass the complete canonical HACP-Core v0.9.2 vector suite.

| Language | Directory | Conformance |
|---|---|---:|
| Go | `hacp-go/` | ✅ 38/38 |
| TypeScript | `hacp-ts/` | ✅ 38/38 |

TypeScript additional suite:

```text
44/44 PASS
```

---

## Enforcement sidecar

Repository:

[`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar)

The sidecar implements pre-execution enforcement and the distributed Gate E control-plane runtime.

Verified capabilities include:

- canonical protocol enforcement;
- semantic boundary matrix;
- reference deployment;
- operational benchmarks;
- distributed revocation propagation;
- reconnect/replay;
- snapshot recovery;
- heartbeat freshness;
- stale fail-closed behavior;
- multi-sidecar convergence.

---

# Phase 4 Gate Status

| Gate | Purpose | Status |
|---|---|---|
| **Gate A** | Protocol correctness | ✅ Closed |
| **Gate B** | Semantic completeness / boundary matrix | ✅ Closed |
| **Gate C** | Deployability / reference stack | ✅ Closed |
| **Gate D** | Operational viability / benchmark | ✅ Closed |
| **Gate E** | Distributed management / gRPC control plane | ✅ Closed |

Gate E completion establishes the distributed revocation/control-state layer required for multiple independent enforcement sidecars to converge safely.

---

# Roadmap

## Phase 1 — Specification ✅

- [x] HACP-Core v0.9.2 normative baseline
- [x] JSON schemas
- [x] canonicalization profile
- [x] cryptographic profile
- [x] 38-vector canonical conformance set
- [x] deterministic test keypair
- [x] manifest-pinned reproducibility

## Phase 2 — Clean-Room Verification ✅

- [x] Go implementation
- [x] TypeScript implementation
- [x] cross-language canonical convergence
- [x] runner-based black-box verification
- [ ] Rust implementation — optional future work

## Phase 3 — Runtime / Production Foundation ✅

- [x] checkpoint state machine
- [x] runtime vectors
- [x] language-neutral runner protocol
- [x] Python reference implementation synchronization
- [x] Go enforcement sidecar
- [x] Python ↔ Go real sidecar E2E
- [x] full Python statement and branch coverage baseline

## Phase 4 — Enforcement and Distributed Operation ✅

- [x] Gate A — protocol correctness
- [x] Gate B — semantic boundary matrix
- [x] Gate C — reference deployment
- [x] Gate D — operational benchmark
- [x] Gate E — distributed gRPC control plane

## Next

The next project work should focus on hardening and ecosystem maturity rather than changing the HACP-Core 0.9.2 conformance baseline without an explicit version transition.

Candidate areas:

- security review / audit;
- production key-management integrations;
- durable external control-plane persistence;
- observability and operational dashboards;
- additional SDK integrations;
- public conformance registry;
- certification and interoperability program;
- optional Rust implementation.

---

# Contributing

## Adding or changing canonical vectors

1. create or modify a vector under `vectors/`;
2. follow the invariants defined in `INVARIANTS.md`;
3. bake golden hashes/signatures intentionally;
4. verify vector integrity;
5. regenerate the manifest;
6. execute the complete canonical suite;
7. verify independent implementations before accepting a normative change.

Example:

```bash
python tools/bake_vector.py
python tools/bake_vector.py --check
python harness/generate_manifest.py
python harness/harness.py --mode local
```

A change to canonical vectors is a protocol-level change and should not be treated as an ordinary test edit.

---

# Philosophy

HACP is built around a simple architectural position:

> AI systems may assist with reasoning and execution, but authority should remain explicit, bounded, inspectable, and attributable.

The project follows a digital-humanist approach:

- human agency is a first-class architectural concern;
- authorization is explicit rather than inferred;
- machine autonomy is bounded rather than assumed;
- cryptography is used to preserve decision integrity;
- enforcement behavior is deterministic;
- failures are visible and fail closed;
- interoperability is demonstrated through open conformance artifacts.

---

# Security Scope

HACP provides protocol semantics, conformance artifacts, and enforcement primitives.

Passing the conformance suite demonstrates compatibility with the specified behavior. It does not by itself prove:

- implementation memory safety;
- secure deployment configuration;
- secure key custody;
- absence of supply-chain compromise;
- absence of implementation-specific vulnerabilities.

Production deployments should still use normal security engineering practices, including independent review, secrets management, least privilege, observability, and operational controls.

---

# References

- [RFC 8785 — JSON Canonicalization Scheme (JCS)](https://www.rfc-editor.org/rfc/rfc8785)
- [RFC 8032 — Edwards-Curve Digital Signature Algorithm (Ed25519)](https://www.rfc-editor.org/rfc/rfc8032)
- [OAuth 2.0](https://oauth.net/2/)
- [C2PA](https://c2pa.org/)

---

# License

**Specification:** [CC BY 4.0](LICENSE)

Reference and enforcement implementations may use their own repository-specific licenses.

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)
