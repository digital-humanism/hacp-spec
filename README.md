# HACP Specification

**Version:** `0.9.0-draft`  
**Status:** Draft for public review  
**Specification License:** CC BY 4.0  
**Reference Implementation:** `humanist-core` — AGPLv3 with Commercial Dual Licensing  

---

## 1. What is HACP?

**HACP** — **Human Agency Continuity Protocol** — is an open protocol for preserving human agency in AI-assisted and autonomous agent systems.

HACP defines how a proposed action is represented, evaluated, authorized, revoked, and cryptographically recorded **before execution**.

The protocol is designed for enterprise environments where AI agents can perform consequential operations such as:

- modifying tickets or records;
- sending external communications;
- executing tool calls;
- exporting data;
- triggering workflows;
- calling external APIs;
- performing irreversible or high-risk operations.

HACP ensures that a human retains the final semantic decision, while still allowing automation within explicitly authorized boundaries.

---

## 2. Why HACP?

Many current AI governance approaches rely on post-hoc mechanisms:

- output watermarking;
- content provenance metadata;
- advisory risk scoring;
- LLM-based safety filters;
- logging after execution.

These mechanisms may help with audit or compliance, but they do not prevent unauthorized or unsafe actions from being executed.

HACP takes a different approach.

It moves control to the **pre-execution boundary**:

```text
IntentEnvelope
   ↓
ProposedAction
   ↓
ScopeGuard / Boundary Evaluation
   ↓
DecisionToken
   ↓
Enforced Execution or Denial
   ↓
Provenance Event
```

The decision is made before the action is performed, and the token is cryptographically bound to the exact action being authorized.

---

## 3. Core Principles

HACP is built around the following principles:

1. **Human final semantic decision**  
   A human or explicitly authorized policy authority must be able to approve, deny, or checkpoint consequential actions.

2. **Deterministic hot path**  
   The core allow/deny/checkpoint decision must not require an LLM call at execution time.

3. **Cryptographic honesty**  
   Decisions, intents, and provenance events should be signed and tamper-evident.

4. **Revocability**  
   Tokens, envelopes, and signer keys must support revocation.

5. **Auditability**  
   Every decision should be traceable to a policy version, principal, signer key, and action hash.

6. **Minimal trust in LLM output**  
   LLM signals may be used as telemetry or explanation, but not as the sole mandatory authority for execution.

7. **Enforcement over advisory control**  
   HACP is intended to support real enforcement points, not only recommendations.

---

## 4. HACP Profiles

HACP is divided into compatibility profiles.

### 4.1 HACP-Core

The minimal protocol layer.

A Core-compatible implementation must support:

- `IntentEnvelope`;
- `ProposedAction`;
- `DecisionToken`;
- `evaluate()` decision function;
- token verification;
- revocation interface;
- cryptographic signing or verification;
- provenance event generation.

### 4.2 HACP-Runtime

Adds support for asynchronous human interaction.

Includes:

- checkpoint creation;
- timeout handling;
- resume semantics;
- state expiration;
- notification payloads;
- human signer assurance.

### 4.3 HACP-Enforcement

Adds support for external enforcement.

Includes:

- sidecar or gateway interception;
- request headers or protocol bindings;
- token binding to request/action;
- fail-closed behavior for high-risk actions;
- MCP and HTTP binding profiles.

---

## 5. Relationship to AI Output Watermarking

HACP is complementary to, but distinct from, AI output watermarking and content provenance systems such as C2PA.

Watermarking attempts to mark content after generation. HACP authorizes actions before execution.

| Concern | Watermarking / C2PA | HACP |
|---|---|---|
| Timing | Post-hoc | Pre-execution |
| Primary goal | Content provenance | Action authorization |
| Prevents unauthorized execution | No | Yes, when enforced |
| Cryptographic binding to action | Weak or removable | Strong, via signed DecisionToken |
| Human final decision | Not guaranteed | Protocol-level requirement |
| Revocation | Limited | Supported |

HACP does not replace watermarking. It addresses a different problem: preventing unauthorized agent actions, not merely labeling generated content.

---

## 6. Repository Structure

```
hacp-spec/
├── LICENSE                          # CC BY 4.0
├── README.md                        # This file
├── requirements.txt                 # Python dependencies for conformance suite
│
├── HACP-SPEC-0.9-draft.md          # Normative specification
├── INVARIANTS.md                    # Testable invariants (INV-1 through INV-7)
├── PROFILES.md                      # Core / Runtime / Enforcement profiles
├── NON-GOALS.md                     # Explicit out-of-scope items
├── canonicalization.md              # Deterministic serialization rules (JCS)
├── threat-model.md                  # Deployment assumptions and threat model
├── versioning.md                    # Compatibility and versioning policy
├── error-model.md                   # Error codes and reason codes
│
├── schemas/                         # JSON Schema definitions
│   ├── intent_envelope.json
│   ├── proposed_action.json
│   ├── decision_token.json
│   ├── agency_decision.json
│   ├── provenance_event.json
│   └── revocation_record.json
│
├── api/                             # Programmatic interface contracts
│   └── decision-api.md              # evaluate, issue_token, revoke, explain
│                                    # + Section 3: Conformance Testing Interface
│
├── wire/                            # Transport and encoding specifications
│   ├── encoding.md                  # JSON serialization, HTTP bindings
│   └── crypto-profile.md            # Ed25519, SHA-256, Base64url
│
├── vectors/                         # Language-independent conformance vectors
│   ├── core_inv1_*.json             # INV-1: Human Final Decision
│   ├── core_inv2_*.json             # INV-2: Boundary Re-Authorization
│   ├── core_inv3_*.json             # INV-3: Token Binding
│   ├── core_inv4_*.json             # INV-4: Traceability
│   ├── core_inv5_*.json             # INV-5: Cryptographic Integrity
│   └── core_inv7_*.json             # INV-7: Bounded Autonomy
│
├── harness/                         # Cross-language conformance testing harness
│   ├── harness.py                   # Test runner (local / http / cli modes)
│   ├── requirements.txt             # Harness dependencies
│   └── README.md                    # Harness usage documentation
│
└── runner.py                        # Legacy runner (deprecated, use harness/)
```

## 7. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** in this specification are to be interpreted as described in RFC 2119 and RFC 8174.

---

## 8. Implementer Quick Start

To implement HACP-Core:

1. Read `HACP-SPEC-0.9-draft.md`.
2. Implement the core data structures defined in `schemas/`.
3. Implement an `evaluate()` function that receives:
   - `IntentEnvelope`;
   - `ProposedAction`;
   - policy context;
   - revocation state.
4. Return one of:
   - `ALLOW`;
   - `DENY`;
   - `CHECKPOINT`.
5. For `ALLOW`, issue a signed `DecisionToken`.
6. Bind the token to the canonicalized hash of the `ProposedAction`.
7. Record the decision in a `ProvenanceEvent`.
8. Support token and envelope revocation.

Implementations MUST NOT require an LLM call to produce a mandatory allow/deny decision on the hot path.

---

## 9. Canonicalization and Cryptography

HACP requires deterministic serialization before hashing or signing.

Implementations MUST follow `canonicalization.md`.

The current draft recommends:

- JSON Canonicalization Scheme (JCS) for JSON payloads;
- SHA-256 for action and payload hashing;
- Ed25519 for production signatures;
- HMAC only for local development profiles;
- explicit key identifiers;
- no dynamic algorithm negotiation.

A `DecisionToken` is not valid unless it is cryptographically bound to the exact action it authorizes.

---

## 10. Conformance

A public conformance suite is planned.

It will include:

- golden vectors;
- negative vectors;
- canonicalization tests;
- revocation tests;
- expired-token tests;
- forged-signature tests;
- boundary matrix tests;
- checkpoint timeout tests;
- enforcement binding tests.

The compatibility marks:

- `HACP-Core Compatible`
- `HACP-Runtime Compatible`
- `HACP-Enforcement Compatible`

may only be used by implementations that pass the corresponding conformance suite once the conformance policy is published.

---

## 11. Security Model

HACP assumes that deployments may contain imperfect or adversarial LLM behavior.

Therefore:

- LLM output is not trusted as an authority.
- Proposed actions should arrive through a mediated interface.
- Tool calls should be schema-constrained.
- High-risk actions should require explicit authorization.
- Enforcement points should fail closed when token validation cannot be completed.
- Revocation state should be propagated as quickly as deployment constraints allow.

HACP does not claim to solve prompt injection by itself. It defines a control boundary that can be enforced when actions are mediated and interceptable.

See `threat-model.md` for details.

---

## 12. Non-Goals

HACP is intentionally scoped.

The following are not goals of HACP-Core:

- building a global identity mesh;
- replacing DLP systems;
- providing OS-level sandboxing;
- acting as a general MCP firewall;
- detecting AI-generated content after the fact;
- guaranteeing removal of AI watermarking;
- solving all prompt-injection risks inside unmediated LLM reasoning;
- replacing service mesh infrastructure.

See `NON-GOALS.md`.

---

## 13. Licensing

This specification is provided under **Creative Commons Attribution 4.0 International (CC BY 4.0)**.

The reference implementation, `humanist-core`, is licensed under **AGPLv3**, with commercial dual licensing available for closed-source and enterprise embedding.

The separation is intentional:

- the specification remains open;
- conformance is intended to be public;
- commercial support and closed embedding are available through separate licensing.

---

## 14. Trademark and Compatibility Claims

The name **HACP** and compatibility statements are intended to be protected through conformance, not marketing.

Once the conformance process is published:

- implementations should not claim HACP compatibility without passing the relevant test suite;
- partial implementations should clearly state which profile and version they support;
- experimental implementations should identify themselves as draft or non-conformant.

---

## 15. Versioning

This is a draft specification.

The versioning policy is:

- `0.9.x` — draft stabilization;
- `1.0.0` — normative freeze after:
  - public review;
  - conformance suite publication;
  - at least one independent clean-room implementation;
  - validation of Core revocation and token-binding semantics.

Backward compatibility requirements will be defined before `1.0.0`.

---

## 16. Roadmap

Current draft focus:

1. Stabilize Core objects.
2. Define canonicalization and signing rules.
3. Publish JSON Schemas.
4. Define deterministic boundary attributes.
5. Define revocation semantics.
6. Publish conformance vectors.
7. Define Runtime checkpoint state machine.
8. Define Enforcement bindings for MCP and HTTP.

---

## 17. Reference Implementation

The current reference implementation is maintained in the `humanist-core` project.

It includes:

- authority core;
- boundary detection;
- risk engine;
- autonomy budget;
- cryptographic provenance;
- runtime integration;
- evaluation metrics;
- ROI and enterprise deployment examples.

The reference implementation is not the specification. In case of conflict, the specification and conformance suite are authoritative.

---

## 18. Contribution

Contributions should focus on:

- testability;
- implementation independence;
- deterministic behavior;
- clear normative language;
- security review;
- enterprise deployment realism.

Philosophical or marketing changes are not accepted into normative documents without clear engineering justification.

---

## 19. Summary

HACP is an open protocol for enforcing human agency in AI agent systems.

It does not attempt to mark AI output after the fact. It authorizes, denies, or checkpoints actions before execution, using deterministic policy evaluation and cryptographic decision tokens.

The goal is simple:

> **No consequential agent action should be executable without a verifiable, revocable, and auditable decision.**
