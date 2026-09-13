# P1 — normative claim check: ID-B vs HACP Specification v1.0.0

Source I-D: `draft-cassandres-hacp-agency-core-00`
Frozen contract: HACP Specification v1.0.0, tag `v1.0.0`, commit `c468c9bb0427448e564bcf3e7d9c8a3a004b8513`
Status: normative provenance mapping; **not a new normative document**.

| ID-B location | Claim | Frozen spec owner | Match |
|---|---|---|---|
| Abstract | fail-closed | crypto/fail-closed + INV-5 | YES |
| Abstract / §5 | deterministic; no LLM on evaluate path | §1.2 / §5.1 | YES |
| Abstract / §10 | 38-vector core-0.9.2; wire hacp_version 0.9 | §4.1 / §10 | YES |
| §3 | enforcement point required else advisory | deployment assumptions | YES |
| §3 | bounded clock skew MAY | deployment assumptions | YES |
| §4 | JSON Schema + frozen HACP canonicalization profile | §4 + canonicalization.md | YES |
| §4.1 | IntentEnvelope fields; hacp_version 0.9 | §4.1 | YES |
| §4.2 | ProposedAction required/optional/security-relevant attributes | §4.2 | YES |
| §4.3 | action_hash SHA-256; Ed25519; no alg negotiation; no token on DENY/CHECKPOINT | §4.3 / §5.2 | YES |
| §4.4 | reason_codes/token/checkpoint/provenance presence rules | §4.4 | YES |
| §5 steps 1–12 | evaluation order and disposition | §5.1 steps 1–12 | YES |
| §5 | key revocation before signature | §5.3 verification order | YES |
| §5 | envelope/token revocation after successful signature | §5.3 | YES |
| §6 | meaningful boundary MUST NOT ALLOW | §6 rule 1 | YES |
| §6 | quantity ceiling is SCOPE_EXCEEDED, not automatically BOUNDARY_CROSSING | §6 + boundary matrix | YES |
| §6 | absent optional attrs ungranted unless policy defaults | §6 rule 3 | YES |
| §6 | frozen boundary matrix governs on conflict | §5.3 / §6 | YES |
| §7 | budget monotonic; exhaustion MUST NOT ALLOW system | §7 | YES |
| §8 | Ed25519 production; HMAC development-only; policy_digest signed | §8 | YES |
| §9 | EVALUATED / ISSUED / REVOKED + prev_event_hash | §9 | YES |
| §10 INV-1..5, INV-7 | invariant statements | §10 | YES |
| §10 | INV-6 reserved | §10 | YES |
| §10 | vector digest `sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58` | published core-0.9.2 pin | YES |
| §10 | MUST NOT special-case vector IDs | conformance rules | YES |
| §11 | token / transitive envelope / key revocation | §5.3 | YES |
| §13 | no implicit ALLOW; mutated action denied; TEST ONLY keys | crypto + INV-3 + INV-5 | YES |
| §14 | no IANA actions | Variant A | YES |

## Canonicalization note

The prior draft said “compatible with RFC 8785 expectations.” This Internet-Draft **does not upgrade that wording into a claim of full RFC 8785/JCS conformance**. ID-B now explicitly points back to the frozen HACP v1.0.0 canonicalization profile and treats RFC 8785 as related canonicalization work. This avoids creating a new normative promise in the IETF restatement.

## Deliberately out of ID-B

- Enforcement revision 2 as active 1.0 contract.
- HC2 request-binding classes as active 1.0 contract.
- Exact reason-code 38/38 correspondence.
- General URI normalization.
- Architecture v2.0 completion claims.
- New IANA registries.

## Residual review rule

Any edit that changes a MUST, MUST NOT, SHALL, SHOULD, MAY, field list, evaluation order, reason code disposition, boundary rule, cryptographic primitive, or revocation behavior requires a fresh P1 comparison against tag `v1.0.0` before publication in any subsequent Internet-Draft revision.
