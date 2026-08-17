**# HACP — Human Agency Continuity Protocol**

![tests](https://github.com/digital-humanism/hacp-spec/actions/workflows/conformance.yml/badge.svg)

****Version:**** 0.9.2  
****Status:**** Phase 1–3 Complete · Phase 4 (Enforcement) Gate A Closed  
****License:**** CC BY 4.0

A language-agnostic protocol for preserving human agency in AI agent systems. HACP enforces pre-execution policy decisions through cryptographic tokens, preventing autonomous M2M loops and ensuring human oversight.

**## Core Principles**

1\. ****Pre-execution enforcement**** — Decisions made BEFORE action execution
2\. ****Deterministic hot path**** — No LLMs on the decision path
3\. ****Cryptographic binding**** — Tokens bound to exact action hashes (SHA-256)
4\. ****Fail-closed mandate**** — Internal errors → DENY, never ALLOW
5\. ****Scope containment**** — Actions must stay within envelope boundaries

**## Quick Start**

**### Run Conformance Tests (Local Mode)**

```bash
# Install dependencies
pip install -r harness/requirements.txt

# Run all 38 test vectors
python harness/harness.py --mode local
```

Expected output:

```
\============================================================
HACP Conformance Harness v0.9.2 - Mode: local
\============================================================

[PASS] CORE-INV1-001: Human principal with human-required consequence class
[PASS] CORE-INV2-001: All attributes within granted scope
[PASS] CORE-INV3-001: Token presented with the exact bound action
[PASS] CORE-INV5-002: Flip one byte in signed payload
[PASS] CORE-INV7-002: Budget exhausted - (N+1)-th action
...
\============================================================
RESULTS: 38/38 passed
\============================================================
```
**### Run Conformance Tests (Runner Protocol)**

For language-neutral verification via stdin/stdout JSON:

```bash
# Test an external implementation (e.g., hacp-sidecar)
python harness/harness_runner.py \\
  --runner "./path/to/implementation-runner" \\
  --vectors-dir vectors \\
  --manifest harness/conformance_manifest.json \\
  --implementation-name my-impl \\
  --implementation-version 0.9.2
```

Expected output:

```
\============================================================
HACP Conformance Harness v0.9.2 - Runner Mode
Protocol version: 1
Spec: 0.9.2 (HACP-Core)
\============================================================

Manifest verified: 0.9.2 (HACP-Core)
Vector set: core-0.9.2
Digest: sha256:1e167887...

[PASS] CORE-INV1-001: Human principal with human-required consequence class
...
[PASS] CORE-RUNTIME-005: System principal resolves its own checkpoint

\============================================================
RESULTS: 38/38 passed
\============================================================
```

Full runner protocol specification: [`harness/runner_protocol.md`](harness/runner_protocol.md)

### Verify Vector Integrity (CI Mode)

```bash
# Check that all baked vectors have correct hashes and signatures
python tools/bake_vector.py --check
```

**### Run Clean-Room Implementations**

```bash
# Go (stdlib only)
cd hacp-go && go build -o hacp-go .
python harness/harness.py --mode cli --binary-path hacp-go/hacp-go

# TypeScript
cd hacp-ts
npm ci
npm run build
npm test

# Optional CLI-mode harness integration
cd ..
python harness/harness.py --mode cli --binary-path hacp-ts/dist/src/cli.js
```

**## Cross-Language Conformance Baseline

The current HACP-Core v0.9.2 canonical vector set has converged across Python, TypeScript, and Go:

```text
Python       38/38 PASS
TypeScript   38/38 PASS
Go           38/38 PASS

Normative failures:        0
Skipped normative vectors: 0
Manifest verified:         YES
```

Additional verification completed around the same baseline:

```text
TypeScript total suite:         44/44 PASS
Python full regression:        324/324 PASS
Python statement coverage:        100%
Python branch coverage:           100%
Python ↔ Go real sidecar E2E:      5/5 PASS
```

This establishes a reproducible cross-language conformance baseline for the current model. It is a conformance and regression milestone, not a formal security proof.

See [`docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md) for the detailed TypeScript and Go verification record.

## Reproducibility Guarantees**

HACP conformance vectors are ****byte-reproducible**** across platforms and languages.

**### Fixed Test Keypair**

```
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
public_key = Ed25519_derive_public(seed)
```

The test keypair is committed to `harness/keys/`:

- `test-ed25519-001.pub` — Public key (verifier only)
- `test-ed25519-001.seed` — Private seed (baker only)
- `KEYS.md` — Documentation

****Security Notice:**** These keys are published intentionally for reproducibility. They MUST NOT be used in production.

**### Deterministic Baking**

```bash
# Bake all golden vectors (compute hashes, sign payloads)
python tools/bake_vector.py
```

For each golden vector:

1\. `action_hash = SHA-256(JCS(proposed_action))`
2\. `signature = Ed25519(test_sk, JCS(token_without_signature))`
3\. `draft_mode: false`
4\. `policy_context.clock: explicit` (no `time.time()` in runner)

**### Conformance Manifest**

The canonical vector set is pinned via a SHA-256 digest stored in [`harness/conformance_manifest.json`](harness/conformance_manifest.json):

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

The harness verifies this digest before running any vector. This ensures CI runs against a known-good vector set, preventing silent divergence when vectors are updated.

Regenerate after adding new vectors:

```bash
python harness/generate_manifest.py
```

**### Canonicalization**

All hashing and signing uses strict JCS-like canonicalization (RFC 8785):

- Keys sorted lexicographically (UTF-8)
- Numbers without `.0`
- Strings with JSON escape rules
- No duplicate keys, no non-finite floats

Same logical payload → same canonical bytes → same hash on any platform.

**## Conformance Testing Workflow**

**### Verified Implementations**

**### Verified Implementations**

The following implementations have passed the full conformance suite and are listed as conformant proofs:

| Implementation | Type | Language | Conformance | Vectors |
|----------------|------|----------|-------------|---------|
| `hacp-go` | Clean-room library | Go | ✅ Conformant | 38/38 (HACP-Core 0.9.2) |
| `hacp-ts` | Clean-room library | TypeScript | ✅ Conformant | 38/38 (HACP-Core 0.9.2) |
| [`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar) | Enforcement proxy | Go | ✅ Conformant | 38/38 (HACP-Core 0.9.2) |
| `humanist-core` | Reference impl | Python | ⏸ Pending | — |

Any new implementation can be verified by exposing a runner (stdin/stdout JSON per [`harness/runner_protocol.md`](harness/runner_protocol.md)) and running the harness against it.

**### For Clean-Room Implementations (Go, TypeScript, Rust)**

1\. ****Clone repository:****

   ```bash
   git clone https://github.com/digital-humanism/hacp-spec.git
   ```

2\. ****Implement `evaluate()`**** per `api/decision-api.md`:

   ```text
   function evaluate(
       envelope: IntentEnvelope,
       action: ProposedAction,
       context: PolicyContext
   ) -> AgencyDecision
   ```

3\. ****Expose HTTP endpoint**** (`POST /evaluate`):

   ```bash
   ./your-impl conformance-server --port 8080
   ```

4\. ****Run harness against your implementation:****

   ```bash
   python harness/harness.py --mode http --target-url http://localhost:8080
   ```

5\. ****Verify results:****
   - Golden vectors → `ALLOW` with valid token
   - Negative vectors → `DENY` or `CHECKPOINT` with correct reason codes
   - All 38/38 passed = clean-room verification complete

**### Alternative: CLI Mode**

```bash
# Your binary accepts vector file path
./your-impl evaluate --vector vectors/core_inv3_001_golden.json
# stdout: {"decision": "ALLOW", "decision_token": {...}}

# Test with harness
python harness/harness.py --mode cli --binary-path ./your-impl
```

**## Test Coverage**

**### Invariants Covered**

| Invariant | Description | Vectors |
|-----------|-------------|---------|
| ****INV-1**** | Human Final Decision | 4 vectors |
| ****INV-2**** | Boundary Re-Authorization | 8 vectors |
| ****INV-3**** | Token Binding | 4 vectors |
| ****INV-4**** | Traceability | 5 vector |
| ****INV-5**** | Cryptographic Integrity | 8 vectors |
| ****INV-7**** | Bounded Autonomy | 4 vectors |
| ****Runtime**** | Checkpoint state machine (Phase 3) | 5 vectors |

****Total:**** 38 vectors (8 golden + 30 negative)

**### Test Scenarios**

****Human Final Decision (INV-1):****
- Human principal with human-required action
- System principal attempting human-required action (CHECKPOINT)
- Expired delegation envelope
- Checkpoint timeout

****Boundary Re-Authorization (INV-2):****
- All attributes within scope (golden)
- Audience boundary crossing (internal → external)
- Reversibility boundary crossing (reversible → irreversible)
- Quantity scope exceeded (max 100, proposed 500)
- Data class boundary crossing (internal → confidential)

****Token Binding (INV-3):****
- Valid token with correct action_hash (golden)
- Token with modified action field (hash mismatch)
- Token presented after expires_at
- Cross-envelope token replay (envelope A token for envelope B action)

****Traceability (INV-4):****
- Valid signed EVALUATED provenance event resolves by id (golden)
- Token revoked after issuance (REVOKED provenance event)
- Tampered provenance payload_hash → DENY
- Decision without provenance event → DENY
- Broken prev_event_hash linkage → DENY

****Cryptographic Integrity (INV-5):****
- Valid Ed25519 signature (golden)
- Tampered signed payload / signature failure
- Unknown signing key
- Wrong algorithm (HMAC vs Ed25519)
- JCS canonicalization with reordered keys (golden)
- Duplicate JSON keys rejected
- Non-canonical (pretty-printed) serialization hash mismatch

****Bounded Autonomy (INV-7):****
- System principal within budget (golden)
- Budget exhausted (N+1)-th action
- Envelope revoked mid-budget

**## Repository Structure**

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
│   ├── harness_runner.py            # Runner protocol engine (stdin/stdout JSON)
│   ├── runner_protocol.md           # Normative runner protocol specification
│   ├── conformance_manifest.json    # Pinned vector set with SHA-256 digest
│   ├── generate_manifest.py         # Regenerate manifest after vector changes
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
└── hacp-ts/                         # TypeScript implementation
    ├── package.json
    ├── package-lock.json
    ├── tsconfig.json
    ├── src/
    │   ├── cli.ts                   # CLI entry
    │   ├── canonical.ts             # JCS canonicalization
    │   ├── crypto.ts                # Ed25519 + SHA-256
    │   ├── evaluate.ts              # Runtime policy logic
    │   └── conformance.ts           # HACP-Core v0.9.2 conformance evaluator
    └── tests/
        ├── action-hash.test.ts      # Canonical action-hash invariants
        └── conformance.test.ts      # 38-vector conformance suite
```

**## API Contract**

See [`api/decision-api.md`](api/decision-api.md) for the complete language-agnostic interface:

- ****Section 1:**** Core interface (`evaluate`, `issue_token`, `revoke`, `explain`)
- ****Section 2:**** Error handling and fail-closed mandate
- ****Section 3:**** Conformance testing interface (HTTP and CLI targets)

**### HTTP Interface (Enterprise)**

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

**### CLI Interface (Development)**

```bash
./hacp-impl evaluate --vector vectors/core_inv3_001_golden.json
```

**## Implementation Status**

**### Reference Implementation (Python)**

****Repository:**** [`humanist-core`](https://github.com/digital-humanism/humanist-core)

- ****Status:**** v0.5.0-alpha (Phase 1-5 complete)
- ****Coverage:**** 100% test coverage (816 statements, 122 tests)
- ****License:**** AGPLv3 + Commercial Dual Licensing

**### Clean-Room Implementations — Phase 2 Complete ✅**

The Go and TypeScript implementations pass the complete canonical HACP-Core v0.9.2 conformance suite (38/38) independently of the Python SDK runtime.

| Language | Directory | Dependencies | Conformance |
|----------|-----------|--------------|-------------|
| Go | `hacp-go/` | stdlib only | 38/38 ✅ |
| TypeScript | `hacp-ts/` | Node.js + TypeScript toolchain | 38/38 ✅ |

Both are validated against the canonical language-independent vector set; runner-based verification remains the preferred black-box interoperability boundary.

**## Philosophy**

****Digital Humanism**** — Human agency as a first-class architectural concern.

HACP enforces transparency through:

- Open standard (CC BY 4.0)
- Dual licensing (AGPLv3 + Commercial)
- No telemetry, no hidden compromises
- Cryptographic honesty as foundation of trust

**## Roadmap**

**### Phase 1: Specification ✅ (Complete)**

- [x] Normative specification baseline (HACP-Core v0.9.2)
- [x] JSON schemas (6 core objects)
- [x] Conformance suite (38 vectors, 38/38 passing)
- [x] Reproducible test keypair
- [x] Cross-language harness (local/http/cli)

**### Phase 2: Clean-Room Verification ✅ (Complete)**

- [x] Go implementation (38/38)
- [x] TypeScript implementation (38/38)
- [x] Independent verification reports
- [ ] Rust implementation (optional, future)

**### Phase 3: Production Readiness**

- [x] checkpoint-protocol.md + 5 runtime vectors
- [x] Language-neutral runner protocol + conformance manifest
- [x] [`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar) enforcement proxy (38/38 conformant)
- [x] `humanist-core` synchronization with HACP-Core v0.9.2 (38/38)
- [ ] LangChain v2 integration
- [ ] Enterprise documentation
- [ ] Security audit

**### Phase 4: Ecosystem**

****Gate A**** — Protocol correctness: ✅ 38/38 vectors pass  
****Gate B**** — Semantic completeness (boundary matrix): ⏸ Pending  
****Gate C**** — Deployability (docker-compose reference stack): ⏸ Pending  
****Gate D**** — Operational viability (p99/throughput benchmark): ⏸ Pending  
****Gate E**** — Distributed management (gRPC control plane): ⏸ Pending

- [ ] Public conformance registry
- [ ] Certification program
- [ ] Commercial support

**## Contributing**

**### Adding Test Vectors**

1\. Create vector JSON in `vectors/` following `INVARIANTS.md`
2\. For golden vectors, set `signature: "PLACEHOLDER"` and `draft_mode: true`
3\. Run `python tools/bake_vector.py` to compute hashes and signatures
4\. Run `python tools/bake_vector.py --check` to verify integrity
5\. Run `python harness/harness.py --mode local` to validate

**### Reporting Issues**

Open an issue with:

- Test ID (e.g., `CORE-INV3-001`)
- Expected vs actual behavior
- Relevant vector JSON

**## References**

- [RFC 8785 — JSON Canonicalization Scheme (JCS)](https://tools.ietf.org/html/rfc8785)
- [RFC 8032 — Edwards-Curve Digital Signature Algorithm (Ed25519)](https://tools.ietf.org/html/rfc8032)
- [OAuth 2.0 Conformance Testing](https://oauth.net/2/conformance/)
- [C2PA Content Authenticity](https://c2pa.org/)

**## License**

****Specification:**** [CC BY 4.0](LICENSE)  
****Reference Implementation:**** AGPLv3 + Commercial Dual Licensing

**---**

****Contact:**** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)