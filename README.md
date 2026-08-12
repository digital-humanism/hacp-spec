# HACP — Human Agency Continuity Protocol

![tests](https://github.com/digital-humanism/hacp-spec/actions/workflows/conformance.yml/badge.svg)

**Version:** 0.9.0-draft  
**Status:** Phase 1 + Phase 2 Complete  
**License:** CC BY 4.0

A language-agnostic protocol for preserving human agency in AI agent systems. HACP enforces pre-execution policy decisions through cryptographic tokens, preventing autonomous M2M loops and ensuring human oversight.

## Core Principles

1. **Pre-execution enforcement** — Decisions made BEFORE action execution
2. **Deterministic hot path** — No LLMs on the decision path
3. **Cryptographic binding** — Tokens bound to exact action hashes (SHA-256)
4. **Fail-closed mandate** — Internal errors → DENY, never ALLOW
5. **Scope containment** — Actions must stay within envelope boundaries

## Quick Start

### Run Conformance Tests (Local Mode)

```bash
# Install dependencies
pip install -r harness/requirements.txt

# Run all 28 test vectors
python harness/harness.py --mode local
```

Expected output:

```
============================================================
HACP Conformance Harness v0.9.2 - Mode: local
============================================================

[PASS] CORE-INV1-001: Human principal with human-required consequence class
[PASS] CORE-INV2-001: All attributes within granted scope
[PASS] CORE-INV3-001: Token presented with the exact bound action
[PASS] CORE-INV5-002: Flip one byte in signed payload
[PASS] CORE-INV7-002: Budget exhausted - (N+1)-th action
...
============================================================
RESULTS: 33/33 passed
============================================================
```

### Verify Vector Integrity (CI Mode)

```bash
# Check that all baked vectors have correct hashes and signatures
python tools/bake_vector.py --check
```

### Run Clean-Room Implementations

```bash
# Go (stdlib only)
cd hacp-go && go build -o hacp-go .
python harness/harness.py --mode cli --binary-path hacp-go/hacp-go

# TypeScript (Node.js stdlib)
cd hacp-ts && npm install && npm run build
python harness/harness.py --mode cli --binary-path hacp-ts/dist/cli.js
```

## Reproducibility Guarantees

HACP conformance vectors are **byte-reproducible** across platforms and languages.

### Fixed Test Keypair

```
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
public_key = Ed25519_derive_public(seed)
```

The test keypair is committed to `harness/keys/`:

- `test-ed25519-001.pub` — Public key (verifier only)
- `test-ed25519-001.seed` — Private seed (baker only)
- `KEYS.md` — Documentation

**Security Notice:** These keys are published intentionally for reproducibility. They MUST NOT be used in production.

### Deterministic Baking

```bash
# Bake all golden vectors (compute hashes, sign payloads)
python tools/bake_vector.py
```

For each golden vector:

1. `action_hash = SHA-256(JCS(proposed_action))`
2. `signature = Ed25519(test_sk, JCS(token_without_signature))`
3. `draft_mode: false`
4. `policy_context.clock: explicit` (no `time.time()` in runner)

### Canonicalization

All hashing and signing uses strict JCS-like canonicalization (RFC 8785):

- Keys sorted lexicographically (UTF-8)
- Numbers without `.0`
- Strings with JSON escape rules
- No duplicate keys, no non-finite floats

Same logical payload → same canonical bytes → same hash on any platform.

## Conformance Testing Workflow

### Verified Implementations

The following implementations have passed the full conformance suite and are listed as reference clean-room proofs:

- **Go** (`hacp-go/`) — stdlib only, 20/20
- **TypeScript** (`hacp-ts/`) — Node.js stdlib, 20/20

Any new implementation can be verified the same way by exposing the HTTP or CLI interface and running the harness against it.

### For Clean-Room Implementations (Go, TypeScript, Rust)

1. **Clone repository:**

   ```bash
   git clone https://github.com/digital-humanism/hacp-spec.git
   ```

2. **Implement `evaluate()`** per `api/decision-api.md`:

   ```text
   function evaluate(
       envelope: IntentEnvelope,
       action: ProposedAction,
       context: PolicyContext
   ) -> AgencyDecision
   ```

3. **Expose HTTP endpoint** (`POST /evaluate`):

   ```bash
   ./your-impl conformance-server --port 8080
   ```

4. **Run harness against your implementation:**

   ```bash
   python harness/harness.py --mode http --target-url http://localhost:8080
   ```

5. **Verify results:**
   - Golden vectors → `ALLOW` with valid token
   - Negative vectors → `DENY` or `CHECKPOINT` with correct reason codes
   - All 20/20 passed = clean-room verification complete

### Alternative: CLI Mode

```bash
# Your binary accepts vector file path
./your-impl evaluate --vector vectors/core_inv3_001_golden.json
# stdout: {"decision": "ALLOW", "decision_token": {...}}

# Test with harness
python harness/harness.py --mode cli --binary-path ./your-impl
```

## Test Coverage

### Invariants Covered

| Invariant | Description | Vectors |
|-----------|-------------|---------|
| **INV-1** | Human Final Decision | 4 vectors |
| **INV-2** | Boundary Re-Authorization | 8 vectors |
| **INV-3** | Token Binding | 4 vectors |
| **INV-4** | Traceability | 5 vector |
| **INV-5** | Cryptographic Integrity | 8 vectors |
| **INV-7** | Bounded Autonomy | 4 vectors |

**Total:** 33 vectors

### Test Scenarios

**Human Final Decision (INV-1):**
- Human principal with human-required action
- System principal attempting human-required action (CHECKPOINT)
- Expired delegation envelope
- Checkpoint timeout

**Boundary Re-Authorization (INV-2):**
- All attributes within scope (golden)
- Audience boundary crossing (internal → external)
- Reversibility boundary crossing (reversible → irreversible)
- Quantity scope exceeded (max 100, proposed 500)
- Data class boundary crossing (internal → confidential)

**Token Binding (INV-3):**
- Valid token with correct action_hash (golden)
- Token with modified action field (hash mismatch)
- Token presented after expires_at
- Cross-envelope token replay (envelope A token for envelope B action)

**Traceability (INV-4):**
- Valid signed EVALUATED provenance event resolves by id (golden)
- Token revoked after issuance (REVOKED provenance event)
- Tampered provenance payload_hash → DENY
- Decision without provenance event → DENY
- Broken prev_event_hash linkage → DENY

**Cryptographic Integrity (INV-5):**
- Valid Ed25519 signature (golden)
- Tampered signature
- Unknown signing key
- Wrong algorithm (HMAC vs Ed25519)
- JCS canonicalization with reordered keys (golden)
- Duplicate JSON keys rejected
- Non-canonical (pretty-printed) serialization hash mismatch

**Bounded Autonomy (INV-7):**
- System principal within budget (golden)
- Budget exhausted (N+1)-th action
- Envelope revoked mid-budget

## Repository Structure

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
│   ├── keys/                        # Fixed test keypair (TEST ONLY)
│   │   ├── KEYS.md                  # Key registry and documentation
│   │   ├── test-ed25519-001.pub     # Public key (32 bytes, hex)
│   │   └── test-ed25519-001.seed    # Private seed (32 bytes, hex)
│   └── README.md                    # Harness usage documentation
│
├── tools/                           # Offline vector generation tools
│   ├── gen_test_keys.py             # Generate deterministic test keypair
│   └── bake_vector.py               # Bake vectors with hashes and signatures
│
├── hacp-go/                         # Clean-room Go implementation (stdlib only)
│   ├── go.mod
│   ├── main.go                      # CLI entry (evaluate --vector)
│   ├── canonical.go                 # JCS canonicalization
│   ├── crypto.go                    # Ed25519 + SHA-256
│   └── evaluate.go                  # Policy logic
│
└── hacp-ts/                         # Clean-room TypeScript implementation (Node stdlib)
    ├── package.json
    ├── tsconfig.json
    └── src/
        ├── cli.ts                   # CLI entry
        ├── canonical.ts             # JCS canonicalization
        ├── crypto.ts                # Ed25519 + SHA-256
        └── evaluate.ts              # Policy logic
```

## API Contract

See [`api/decision-api.md`](api/decision-api.md) for the complete language-agnostic interface:

- **Section 1:** Core interface (`evaluate`, `issue_token`, `revoke`, `explain`)
- **Section 2:** Error handling and fail-closed mandate
- **Section 3:** Conformance testing interface (HTTP and CLI targets)

### HTTP Interface (Enterprise)

```http
POST /evaluate
Content-Type: application/json

{
  "test_id": "CORE-INV3-001",
  "type": "golden",
  "inputs": { ... },
  "policy_context": { "clock": 1786000100 },
  "expected": { "outcome": "ALLOW" }
}
```

### CLI Interface (Development)

```bash
./hacp-impl evaluate --vector vectors/core_inv3_001_golden.json
```

## Implementation Status

### Reference Implementation (Python)

**Repository:** [`humanist-core`](https://github.com/digital-humanism/humanist-core)

- **Status:** v0.5.0-alpha (Phase 1-5 complete)
- **Coverage:** 100% test coverage (816 statements, 122 tests)
- **License:** AGPLv3 + Commercial Dual Licensing

### Clean-Room Implementations — Phase 2 Complete ✅

Two independent clean-room implementations pass the full conformance suite (20/20), proving the specification is implementable without reading the reference code.

| Language | Directory | Dependencies | Conformance |
|----------|-----------|--------------|-------------|
| Go | `hacp-go/` | stdlib only | 33/33 ✅ |
| TypeScript | `hacp-ts/` | Node.js stdlib | 33/33 ✅ |

Both were written from the published specification alone and communicate with the harness exclusively via JSON.

## Philosophy

**Digital Humanism** — Human agency as a first-class architectural concern.

HACP enforces transparency through:

- Open standard (CC BY 4.0)
- Dual licensing (AGPLv3 + Commercial)
- No telemetry, no hidden compromises
- Cryptographic honesty as foundation of trust

## Roadmap

### Phase 1: Specification ✅ (Complete)

- [x] Normative specification (v0.9.0-draft)
- [x] JSON schemas (6 core objects)
- [x] Conformance suite (33 vectors, 33/33 passing)
- [x] Reproducible test keypair
- [x] Cross-language harness (local/http/cli)

### Phase 2: Clean-Room Verification ✅ (Complete)

- [x] Go implementation (20/20)
- [x] TypeScript implementation (20/20)
- [x] Independent verification reports
- [ ] Rust implementation (optional, future)

### Phase 3: Production Readiness

- [ ] `humanist-core` synchronization with spec
- [ ] LangChain v2 integration
- [ ] Enterprise documentation
- [ ] Security audit

### Phase 4: Ecosystem

- [ ] Public conformance registry
- [ ] Certification program
- [ ] Commercial support

## Contributing

### Adding Test Vectors

1. Create vector JSON in `vectors/` following `INVARIANTS.md`
2. For golden vectors, set `signature: "PLACEHOLDER"` and `draft_mode: true`
3. Run `python tools/bake_vector.py` to compute hashes and signatures
4. Run `python tools/bake_vector.py --check` to verify integrity
5. Run `python harness/harness.py --mode local` to validate

### Reporting Issues

Open an issue with:

- Test ID (e.g., `CORE-INV3-001`)
- Expected vs actual behavior
- Relevant vector JSON

## References

- [RFC 8785 — JSON Canonicalization Scheme (JCS)](https://tools.ietf.org/html/rfc8785)
- [RFC 8032 — Edwards-Curve Digital Signature Algorithm (Ed25519)](https://tools.ietf.org/html/rfc8032)
- [OAuth 2.0 Conformance Testing](https://oauth.net/2/conformance/)
- [C2PA Content Authenticity](https://c2pa.org/)

## License

**Specification:** [CC BY 4.0](LICENSE)  
**Reference Implementation:** AGPLv3 + Commercial Dual Licensing

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)