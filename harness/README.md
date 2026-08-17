# HACP Conformance Harness

Cross-language conformance testing tool for HACP implementations.

The harness ships with **two engines**:

| Engine | File | When to use |
|--------|------|-------------|
| **Runner Protocol** ⭐ | `harness_runner.py` | Any new implementation. Language-neutral, black-box, CI-friendly. |
| Legacy (local / HTTP / CLI) | `harness.py` | Existing in-tree implementations (`hacp-go`, `hacp-ts`) and historical reference. |

For new implementations, the runner protocol is the recommended path.
It validates observable protocol behavior without importing implementation code.

## Current Conformance Baseline

The current canonical HACP-Core v0.9.2 vector set contains **38 normative vectors** and is pinned by the conformance manifest.

```text
Spec:          HACP-Core v0.9.2
Vector set:    core-0.9.2
Vectors:       38
Manifest:      verified
Vector digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

Current verified implementations against this baseline:

| Implementation | Verification path | Result |
|----------------|-------------------|--------|
| `humanist-core` | Python conformance suite against canonical vectors | 38/38 ✅ |
| `hacp-ts` | TypeScript conformance suite against canonical vectors | 38/38 ✅ |
| `hacp-go` | In-tree clean-room verification | 38/38 ✅ |
| [`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar) | Runner Protocol / black-box harness | 38/38 ✅ |

Additional verification completed on the same baseline:

```text
TypeScript total suite:         44/44 PASS
Python full regression:        324/324 PASS
Python statement coverage:        100%
Python branch coverage:           100%
Python ↔ Go real sidecar E2E:      5/5 PASS
```

Detailed TypeScript and Go verification record:

- [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md)

## Installation

```bash
pip install -r requirements.txt
```

## Runner Protocol (Recommended)

Language-neutral verification via stdin/stdout JSON. The implementation
provides a **runner** that speaks the protocol; the harness treats it
as a black box.

### Principle

> **Conformance validates observable protocol behavior, not implementation structure.**

### Quick Start

```bash
python harness_runner.py \
  --runner "./path/to/implementation-runner" \
  --vectors-dir ../vectors \
  --manifest conformance_manifest.json \
  --implementation-name my-impl \
  --implementation-version 0.9.2
```

### Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--runner` | ✅ | Command to launch the runner (e.g., `./hacp-sidecar-runner`) |
| `--manifest` | — | Path to `conformance_manifest.json` (recommended for CI) |
| `--vectors-dir` | — | Path to vectors (default: `../vectors`) |
| `--implementation-name` | — | Name for JSON output |
| `--implementation-version` | — | Version for JSON output |
| `--output` | — | `console` (default) or `json` |
| `--runner-timeout` | — | Per-vector timeout in ms (default: 5000) |
| `--verbose` | — | Forward runner stderr to console |

### Runner Contract

A conformant runner must:

1. Read JSON requests from **stdin** (one per line)
2. Write JSON responses to **stdout** (one per line)
3. Write diagnostics to **stderr** only (never stdout)
4. Exit cleanly when stdin closes
5. Support protocol version `"1"`

#### Request (harness → runner)

```json
{
  "protocol_version": "1",
  "operation": "evaluate",
  "vector_id": "CORE-INV5-001",
  "input": {
    "intent_envelope": { ... },
    "proposed_action": { ... },
    "decision_token": { ... },
    "policy_context": { ... },
    "checkpoint": { ... }
  }
}
```

#### Response (runner → harness)

```json
{
  "protocol_version": "1",
  "decision": "ALLOW",
  "reason_codes": [],
  "action_hash": "sha256...",
  "metrics": {
    "latency_ns": 12345
  }
}
```

Full specification: [`runner_protocol.md`](runner_protocol.md)

### Known Conformant Runners

| Implementation | Runner command | Profile | Vectors |
|----------------|----------------|---------|---------|
| [`hacp-sidecar`](https://github.com/digital-humanism/hacp-sidecar) | `hacp-conformance-runner` | HACP-Core 0.9.2 | 38/38 ✅ |

The current runner-mode verification result for `hacp-sidecar` is:

```text
Manifest verified: 0.9.2 (HACP-Core)
Vector set: core-0.9.2
Digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
RESULTS: 38/38 passed
```

## Legacy Engine (Local / HTTP / CLI)

The original `harness.py` supports three target modes and remains useful
for in-tree implementations and historical compatibility.

### Local Mode (Spec Validation)

Emulates HACP-Core logic locally:

```bash
python harness.py --mode local
```

### HTTP Target

Tests HTTP server implementations:

```bash
python harness.py --mode http --target-url http://localhost:8080
```

### CLI Target

Tests CLI implementations:

```bash
python harness.py --mode cli --binary-path ./hacp-go
```

Target interface contract: [`../api/decision-api.md`](../api/decision-api.md) Section 3.

## Conformance Manifest

The vector set is pinned via a SHA-256 digest in
[`conformance_manifest.json`](conformance_manifest.json):

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

The harness verifies this digest before running any vector. This
ensures CI runs against a known vector set and prevents silent
divergence when vectors are updated.

### Regenerating the Manifest

After adding or modifying vectors:

```bash
python generate_manifest.py
```

Then commit the updated `conformance_manifest.json` together with the
new vectors so downstream implementations pin against the same digest.

When a vector is intentionally changed, a manifest mismatch is expected
until the digest is regenerated. Do not bypass the manifest gate merely
to make a conformance run proceed.

## Reproducing the Current Go Runner Verification

Build the current Go runner:

```powershell
cd ...\GitHub\Dev\hacp-sidecar
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
```

Run the canonical harness:

```powershell
cd ...\GitHub\Dev\hacp-spec\harness

python harness_runner.py `
  --runner "...\GitHub\Dev\hacp-sidecar\hacp-conformance-runner.exe" `
  --vectors-dir "...\GitHub\Dev\hacp-spec\vectors" `
  --manifest conformance_manifest.json `
  --implementation-name hacp-sidecar `
  --implementation-version 0.3.0 `
  --output console `
  --verbose
```

Expected result:

```text
Manifest verified: 0.9.2 (HACP-Core)
Vector set: core-0.9.2
Digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58

RESULTS: 38/38 passed
```

## Exit Codes

### Runner Protocol (`harness_runner.py`)

| Code | Meaning |
|------|---------|
| `0` | Conformant — all vectors pass |
| `1` | Conformance failure — one or more vectors failed |
| `2` | Harness/configuration error (bad manifest, missing vectors, etc.) |
| `3` | Runner execution/protocol error (crash, malformed JSON, timeout) |

Exit code `3` is distinct from `1`: it means the verification
infrastructure failed, not that the implementation failed a vector.
CI pipelines should fail on any non-zero exit.

### Legacy Engine (`harness.py`)

| Code | Meaning |
|------|---------|
| `0` | All tests passed |
| `1` | One or more tests failed |

## Troubleshooting

### Runner hangs after a few vectors (without `--verbose`)

**Symptom:** harness prints 2–3 `[PASS]` lines then stops indefinitely.

**Root cause:** The runner writes many diagnostic lines to `stderr`
(e.g., `log.Printf` in Go). In non-verbose mode the current harness
redirects `stderr` to `DEVNULL` so diagnostics are discarded. Older
harness versions that redirected `stderr` to `PIPE` without draining it
could deadlock once the OS pipe buffer filled.

**Fix:** Upgrade `harness_runner.py` to the current version, or run with
`--verbose` during development.

**Prevention for runner authors:** minimize stderr output in production
mode and keep detailed diagnostics behind a flag.

### "Vector digest mismatch"

The manifest's `vector_digest` does not match the current contents of
the vectors directory. Either:

- vectors were modified without regenerating the manifest → run
  `python generate_manifest.py` and commit the result, or
- you are running against the wrong vectors directory → check
  `--vectors-dir`.

Do not disable manifest verification to work around this error.

### "Unsupported protocol version"

The runner returned a `protocol_version` the harness does not
understand. The current harness supports protocol version `"1"`.
Update either the runner or the harness to a compatible version.

## Repository Layout

```text
harness/
├── harness.py                    # Legacy engine (local / http / cli)
├── harness_runner.py             # Runner protocol engine (recommended)
├── runner_protocol.md            # Normative runner protocol spec
├── conformance_manifest.json     # Pinned vector set digest
├── generate_manifest.py          # Regenerate manifest
├── requirements.txt              # Python dependencies
├── keys/                         # Fixed test keypair (TEST ONLY)
│   ├── KEYS.md
│   ├── test-ed25519-001.pub
│   └── test-ed25519-001.seed
└── README.md                     # This file
```

## Assurance Boundary

A successful conformance run establishes agreement with the canonical
HACP-Core vector baseline. It is strong regression and interoperability
evidence, but it is not by itself:

- a formal proof of protocol correctness;
- a complete security proof;
- a substitute for property-based testing;
- a substitute for fuzzing;
- a substitute for adversarial production testing.

The manifest-verified 38-vector baseline should therefore remain a
release gate while additional assurance layers are added.

## References

- Runner protocol specification: [`runner_protocol.md`](runner_protocol.md)
- Conformance concept: [`../README.md`](../README.md) → "Conformance Testing"
- TypeScript and Go verification report: [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md)
- Target interface contract: [`../api/decision-api.md`](../api/decision-api.md)
- Vector format: [`../vectors/README.md`](../vectors/README.md)

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)
