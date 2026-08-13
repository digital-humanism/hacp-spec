# HACP Core Conformance Suite 0.9

**Version:** 0.9.2
**Status:** Gate A closed (2026-08-14)
**License:** CC BY 4.0

This document declares the HACP-Core conformance suite, maps every testable
invariant from `INVARIANTS.md` to a vector (or an explicit deferral), and
defines the patch policy for the 0.9 line.

An implementation may claim **HACP-Core Compatible** only after passing the
full suite declared here, using the public harness and the published test
public key, without access to the reference implementation.

## 1. Suite Declaration

Core Conformance Suite 0.9 currently executes *38 vectors**:

- **27** map to Core Test IDs in `INVARIANTS.md`
- **1** (`CORE-INV1-006`) is a Runtime-preview vector (checkpoint timeout),
  counted for execution but owned by Phase 3 (Runtime profile)

**Gate A: reached.** All critical negatives present; harness green in local
and CLI modes across Python, Go, and TypeScript.

All vectors are reproducible: golden vectors carry real `action_hash` and
Ed25519 `signature` baked offline (`tools/bake_vector.py`), `draft_mode:
false`, and an explicit `policy_context.clock`. The harness verifies only;
it never signs at runtime. The harness verifies only; it never signs at runtime.

### Verification Methods

Two harness engines are available:

| Engine | File | Recommended for |
|--------|------|-----------------|
| **Runner Protocol** ⭐ | `harness/harness_runner.py` | External implementations (language-neutral, black-box) |
| Legacy | `harness/harness.py` | In-tree implementations (local/HTTP/CLI) |

The runner protocol communicates via stdin/stdout JSON (protocol version "1"),
treating implementations as black boxes. This enables verification without
importing implementation code.

Full runner protocol specification: [`harness/runner_protocol.md`](harness/runner_protocol.md)

### Conformance Manifest

The vector set is pinned via SHA-256 digest in [`harness/conformance_manifest.json`](harness/conformance_manifest.json).
The harness verifies this digest before running any vector, ensuring CI runs
against a known-good vector set.

## 2. Patch Policy (0.9.x)

1. A patch release **MAY add** negative vectors and new Test IDs.
2. A patch release **MUST NOT** change the expected outcome of an existing
   golden vector, nor alter canonicalization, hashing, or signature rules.
3. Adding a negative that an already-certified implementation fails is a
   **minor** change requiring re-certification.
4. Breaking golden semantics is a **major** change (1.0), not a patch.

## 3. Coverage Table

Status legend:
- ✅ in suite — vector present and passing
- ⚠️ to add — critical for Gate A, must be added before freeze
- ⏸ deferred — out of scope for 0.9, documented reason

### INV-1 — Human Final Decision

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV1-001 | golden | ✅ | core_inv1_001_golden.json | |
| CORE-INV1-002 | negative | ✅ | core_inv1_002_negative.json | |
| CORE-INV1-003 | negative | ⏸ | — | Self-asserted principal_kind; partially covered by INV-5-003 |
| CORE-INV1-004 | golden | ⏸ | — | Delegation chain model (Phase 3) |
| CORE-INV1-005 | negative | ✅ | core_inv1_005_negative.json | |
| CORE-INV1-006 | negative | ✅ | core_inv1_006_negative.json | Runtime preview (checkpoint timeout) |

### INV-2 — Boundary Re-Authorization

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV2-001 | golden | ✅ | core_inv2_001_golden.json | |
| CORE-INV2-002 | negative | ✅ | core_inv2_002_negative.json | audience |
| CORE-INV2-003 | negative | ✅ | core_inv2_003_negative.json | reversibility |
| CORE-INV2-004 | negative | ✅ | core_inv2_004_negative.json | externality |
| CORE-INV2-005 | negative | ✅ | core_inv2_005_negative.json | quantity |
| CORE-INV2-006 | negative | ✅ | core_inv2_006_negative.json | destination |
| CORE-INV2-007 | negative | ✅ | core_inv2_007_negative.json | data_class |
| CORE-INV2-008 | negative | ✅ | core_inv2_008_negative.json | absent optional |

### INV-3 — Token Binding

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV3-001 | golden | ✅ | core_inv3_001_golden.json | |
| CORE-INV3-002 | negative | ✅ | core_inv3_002_negative.json | |
| CORE-INV3-003 | negative |✅ | — | core_inv3_003_negative.json |
| CORE-INV3-004 | negative | ✅ | core_inv3_004_negative.json | |
| CORE-INV3-005 | negative | ⏸ | — | Truncated/re-padded signature |

### INV-4 — Traceability

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV4-001 | golden | ✅ | — | core_inv4_001_golden.json |
| CORE-INV4-002 | golden | ✅ | core_inv4_002_golden.json | |
| CORE-INV4-003 | negative | ✅ | — | core_inv4_003_negative.json |
| CORE-INV4-004 | negative | ✅ | — | core_inv4_004_negative.json |
| CORE-INV4-005 | negative | ✅ | — | core_inv4_005_negative.json |

### INV-5 — Cryptographic Integrity

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV5-001 | golden | ✅ | core_inv5_001_golden.json | |
| CORE-INV5-002 | negative | ✅ | core_inv5_002_negative.json | |
| CORE-INV5-003 | negative | ✅ | core_inv5_003_negative.json | |
| CORE-INV5-004 | negative | ✅ | core_inv5_004_negative.json | |
| CORE-INV5-005 | golden | ✅ | core_inv5_005_golden.json | |
| CORE-INV5-006 | negative | ✅ | core_inv5_006_negative.json | duplicate keys |
| CORE-INV5-007 | negative | ✅ | core_inv5_007_negative.json | non-canonical |
| CORE-INV5-008 | negative | ✅ | core_inv5_008_negative.json | key revoke |

### INV-7 — Bounded Autonomy

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV7-001 | golden | ✅ | core_inv7_001_golden.json | |
| CORE-INV7-002 | negative | ✅ | core_inv7_002_negative.json | |
| CORE-INV7-003 | golden | ⏸ | — | Human path vs budget (Phase 3) |
| CORE-INV7-004 | negative | ⏸ | — | Forged budget state |
| CORE-INV7-005 | negative | ✅ | core_inv7_005_negative.json | |
| CORE-INV7-006 | negative | ✅ | core_inv7_006_negative.json | parent-envelope inheritance |

### Runtime Profile (Phase 3)

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-RUNTIME-001 | golden | ✅ | runtime_001_golden.json | resume after human approval |
| CORE-RUNTIME-002 | negative | ✅ | runtime_002_negative.json | timeout → EXPIRED |
| CORE-RUNTIME-003 | negative | ✅ | runtime_003_negative.json | token bound to wrong action_hash |
| CORE-RUNTIME-004 | negative | ✅ | runtime_004_negative.json | unresolved OPEN → CHECKPOINT |
| CORE-RUNTIME-005 | negative | ✅ | runtime_005_negative.json | system self-resolution |

## 4. Summary

- **In suite now:** 38 executed (32 Core-mapped + 6 Runtime profile)
- **Critical to add:** 0 — Gate A closed
- **Deferred (post-0.9):** 5 → INV-1-003/004, INV-3-005, INV-7-003/004

### Gate A Status

**Gate A was closed on 2026-08-14.** All 38 vectors pass via the runner
protocol against `hacp-sidecar`, the first enforcement-profile implementation.

Conformant implementations:

| Implementation | Type | Profile | Vectors | Date |
|----------------|------|---------|---------|------|
| `hacp-go` | Clean-room library | HACP-Core | 38/38 ✅ | 2026-01 (Phase 2) |
| `hacp-ts` | Clean-room library | HACP-Core | 38/38 ✅ | 2026-01 (Phase 2) |
| [`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar) | Enforcement proxy | HACP-Core | 38/38 ✅ | 2026-08-14 |

## 5. Patch Policy Application

Per Section 2 (Patch Policy):

- Adding new negative vectors (e.g., for boundary matrix in Gate B) is a **patch** (0.9.x)
- Changing golden vector outcomes is a **major** change (1.0)
- Adding new Test IDs (e.g., INV-6) is a **minor** change requiring re-certification

Gate B (boundary matrix) will add negative vectors under 0.9.x without
breaking existing conformant implementations. Gate B vectors will be
documented in `vectors/README.md` and added to this coverage table.

## 6. Running the Suite

### Verify Vector Integrity

```bash
python tools/bake_vector.py --check
```

### Runner Protocol (Recommended)

Test an external implementation via stdin/stdout JSON:

```bash
python harness/harness_runner.py \
  --runner "./path/to/implementation-runner" \
  --vectors-dir vectors \
  --manifest harness/conformance_manifest.json \
  --implementation-name my-impl \
  --implementation-version 0.9.2
```

Exit codes:
- `0` — Conformant (all vectors pass)
- `1` — Conformance failure
- `2` — Harness/configuration error
- `3` — Runner execution/protocol error

### Legacy Engine (Local / HTTP / CLI)

```bash
python harness/harness.py --mode local                                    # emulation
python harness/harness.py --mode http --target-url http://localhost:8080  # HTTP server
python harness/harness.py --mode cli --binary-path <impl>                 # CLI binary
```

### Test with hacp-sidecar

```bash
# Build the conformance runner
cd hacp-sidecar
go build -o hacp-conformance-runner ./cmd/hacp-conformance-runner

# Run conformance suite
cd ../hacp-spec
python harness/harness_runner.py \
  --runner "../hacp-sidecar/hacp-conformance-runner" \
  --vectors-dir vectors \
  --manifest harness/conformance_manifest.json \
  --implementation-name hacp-sidecar \
  --implementation-version 0.3.0
```

Expected output: `RESULTS: 38/38 passed`