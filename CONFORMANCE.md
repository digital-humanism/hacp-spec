# HACP Core Conformance Suite 0.9

**Version:** 0.9.0-draft
**Status:** Gate A pending (critical negatives to add)
**License:** CC BY 4.0

This document declares the HACP-Core conformance suite, maps every testable
invariant from `INVARIANTS.md` to a vector (or an explicit deferral), and
defines the patch policy for the 0.9 line.

An implementation may claim **HACP-Core Compatible** only after passing the
full suite declared here, using the public harness and the published test
public key, without access to the reference implementation.

## 1. Suite Declaration

Core Conformance Suite 0.9 currently executes **28 vectors**:

- **27** map to Core Test IDs in `INVARIANTS.md`
- **1** (`CORE-INV1-006`) is a Runtime-preview vector (checkpoint timeout),
  counted for execution but owned by Phase 3 (Runtime profile)

**Gate A: reached.** All critical negatives present; harness green in local
and CLI modes across Python, Go, and TypeScript.

All vectors are reproducible: golden vectors carry real `action_hash` and
Ed25519 `signature` baked offline (`tools/bake_vector.py`), `draft_mode:
false`, and an explicit `policy_context.clock`. The harness verifies only;
it never signs at runtime.

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
| CORE-INV2-004 | negative | ⏸ | — | externality (Phase 2 matrix) |
| CORE-INV2-005 | negative | ✅ | core_inv2_005_negative.json | quantity |
| CORE-INV2-006 | negative | ⏸ | — | destination allowlist (Phase 2 matrix) |
| CORE-INV2-007 | negative | ✅ | core_inv2_007_negative.json | data_class |
| CORE-INV2-008 | negative | ⏸ | — | absent optional attribute (Phase 2 matrix) |

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
| CORE-INV5-001 | golden | ✅ | — | core_inv5_001_golden.json |
| CORE-INV5-002 | negative | ✅ | core_inv5_002_negative.json | |
| CORE-INV5-003 | negative | ✅ | core_inv5_003_negative.json | |
| CORE-INV5-004 | negative | ✅ | core_inv5_004_negative.json | |
| CORE-INV5-005 | golden | ✅ | core_inv5_005_golden.json | |
| CORE-INV5-006 | negative | ✅ | — | core_inv5_006_negative.json |
| CORE-INV5-007 | negative | ✅ | — | core_inv5_007_negative.json |

### INV-7 — Bounded Autonomy

| Test ID | Type | Status | Vector | Note |
|---------|------|--------|--------|------|
| CORE-INV7-001 | golden | ✅ | core_inv7_001_golden.json | |
| CORE-INV7-002 | negative | ✅ | core_inv7_002_negative.json | |
| CORE-INV7-003 | golden | ⏸ | — | Human path vs budget (Phase 3) |
| CORE-INV7-004 | negative | ⏸ | — | Forged budget state |
| CORE-INV7-005 | negative | ✅ | core_inv7_005_negative.json | |

## 4. Summary

- **In suite now:** 28 executed (27 Core-mapped + 1 Runtime preview)
- **Critical to add:** 0 — Gate A reached
- **Deferred (post-0.9):** 8 → INV-1-003/004, INV-2-004/006/008, INV-3-005, INV-7-003/004

Gate A is reached when all ⚠️ rows become ✅ and the harness reports green
in local, HTTP, and CLI modes.

## 5. Running the Suite

```bash
python tools/bake_vector.py --check     # integrity of committed vectors
python harness/harness.py --mode local  # emulation
python harness/harness.py --mode cli --binary-path <impl>  # clean-room