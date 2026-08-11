# HACP Threat Model and Security Assumptions

**Version:** 0.9.0-draft
**Status:** Draft for public review
**License:** CC BY 4.0

This document defines the security assumptions, trust boundaries, and threat vectors addressed by HACP. It is normative for understanding the limits of HACP-Core and the necessity of HACP-Enforcement.

## 1. Trust Boundaries

HACP operates across three primary trust boundaries:

1. **Agent Boundary:** The automated component proposing actions. Assumed to be potentially imperfect, misconfigured, or compromised.
2. **Enforcement Boundary:** The point where `DecisionToken` is verified and action is executed (e.g., sidecar, gateway, SDK hook). Assumed to be trusted and correctly configured.
3. **Human Boundary:** The principal providing final semantic decisions. Assumed to be authenticated via external mechanisms (HACP-Runtime defines minimal signer assurance).

## 2. Core Security Assumptions

1. **Mediated Interface:** Proposed actions arrive through a schema-constrained, mediated interface (e.g., MCP tools). Unmediated execution is out of scope for Core.
2. **Cryptographic Integrity:** Ed25519 private keys used for signing envelopes and tokens are stored securely and are not compromised.
3. **Bounded Clock Skew:** Verifiers and issuers have synchronized clocks within a defined, bounded tolerance.
4. **Deterministic Evaluation:** `evaluate()` is a pure function of its inputs (envelope, action, policy, revocation state) and does not rely on non-deterministic external calls (like LLM inference) on the hot path.

## 3. Threat Vectors and Mitigations

### 3.1 Token Replay and Hijacking
**Threat:** An attacker intercepts a valid `DecisionToken` and attempts to use it to authorize a different action or replay it after expiry.
**Mitigation:** 
- `action_hash` strictly binds the token to the exact canonicalized `ProposedAction`.
- `expires_at` enforces temporal bounds.
- Revocation interface allows invalidating tokens and envelopes post-issuance.

### 3.2 Scope Creep and Boundary Crossing
**Threat:** An agent gradually expands its actions beyond authorized limits (e.g., changing `audience` from `internal` to `external`).
**Mitigation:** 
- Deterministic boundary matrix in `evaluate()`.
- Any meaningful boundary crossing forces `CHECKPOINT` or `DENY`.
- Fail-closed behavior on unknown attributes.

### 3.3 LLM Prompt Injection and Jailbreaking
**Threat:** An attacker manipulates the LLM to generate malicious `ProposedAction` payloads.
**Mitigation:** 
- HACP-Core assumes the interface is mediated. It does not analyze the prompt.
- HACP-Enforcement (sidecar/gateway) is required to enforce boundaries at the network/tool level if the agent runtime is untrusted.

### 3.4 Revocation Bypass
**Threat:** An enforcement point executes an action using a token that has been revoked by the control plane.
**Mitigation:** 
- Local denylist with bounded staleness.
- Fail-closed mandate: if revocation state cannot be verified, high-risk actions MUST be denied.

### 3.5 Key Compromise
**Threat:** An attacker compromises a `signer_key_id` and issues fraudulent tokens.
**Mitigation:** 
- Immediate revocation of the `signer_key_id`.
- Short-lived tokens (`expires_at`) minimize the window of exposure.
- Policy digest binding ensures tokens cannot be reused if policy changes.

## 4. Deployment Tiers and Agent Trust

HACP-Core is designed for **Cooperative** and **Managed** agent environments. 

- **Cooperative Agent:** The agent runtime honestly proposes actions and respects `DENY`/`CHECKPOINT` decisions. HACP-Core is sufficient.
- **Managed Runtime:** The runtime restricts network/tool access. HACP-Core + basic hooks are sufficient.
- **Hostile/Untrusted Agent:** The agent actively attempts to bypass controls. HACP-Core is insufficient; **HACP-Enforcement** (external sidecar/gateway with strict network mediation) is strictly required.

## 5. Residual Risks

HACP does not eliminate all risks. Residual risks include:
- Compromise of the human authentication mechanism (out of scope for Core).
- Zero-day vulnerabilities in the enforcement point itself.
- Social engineering of the human approver (HACP verifies the decision, not the cognitive state of the approver).
