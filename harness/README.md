# HACP Conformance Harness

Cross-language conformance testing tool for HACP implementations.

The harness ships with **two engines**:

| Engine | File | When to use |
|--------|------|-------------|
| **Runner Protocol** ⭐ | `harness_runner.py` | Any new implementation. Language-neutral, black-box, CI-friendly. |
| Legacy (local / HTTP / CLI) | `harness.py` | Existing in-tree implementations (`hacp-go`, `hacp-ts`) and historical reference. |

For new implementations, the runner protocol is the recommended path.
It validates observable protocol behavior without importing implementation code.

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

## Legacy Engine (Local / HTTP / CLI)

The original `harness.py` supports three target modes and is still used
for in-tree clean-room implementations.

### Local Mode (Spec Validation)

Emulates HACP-Core logic locally:

```bash
python harness.py --mode local
```

### HTTP Target (Clean-Room Server)

Tests HTTP server implementations:

```bash
python harness.py --mode http --target-url http://localhost:8080
```

### CLI Target (Clean-Room Binary)

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
  "vector_digest": "sha256:9c0557dd...",
  "total_vectors": 38
}
```

The harness verifies this digest before running any vector. This
ensures CI runs against a known-good vector set and prevents silent
divergence when vectors are updated.

### Regenerating the Manifest

After adding or modifying vectors:

```bash
python generate_manifest.py
```

Then commit the updated `conformance_manifest.json` together with the
new vectors so downstream implementations pin against the same digest.

## Exit Codes

### Runner Protocol (`harness_runner.py`)

| Code | Meaning |
|------|---------|
| `0` | Conformant — all vectors pass |
| `1` | Conformance failure — one or more vectors failed |
| `2` | Harness/configuration error (bad manifest, missing vectors, etc.) |
| `3` | Runner execution/protocol error (crash, malformed JSON, timeout) |

Exit code `3` is distinct from `1`: it means the *infrastructure* of
verification failed, not that the implementation failed a vector.
CI pipelines should typically fail on any non-zero exit.

### Legacy Engine (`harness.py`)

| Code | Meaning |
|------|---------|
| `0` | All tests passed |
| `1` | One or more tests failed |

## Troubleshooting

### Runner hangs after a few vectors (without `--verbose`)

**Symptom:** harness prints 2–3 `[PASS]` lines then stops indefinitely.

**Root cause:** The runner writes many diagnostic lines to `stderr`
(e.g., `log.Printf` in Go). In non-verbose mode the harness redirects
`stderr` to `DEVNULL` so diagnostics are discarded. If an older harness
version redirected `stderr` to `PIPE` without draining it, the OS pipe
buffer fills and the runner blocks on the next write — while the parent
simultaneously blocks on `stdout.readline()` waiting for the JSON
response. Classic pipe deadlock.

**Fix:** Upgrade `harness_runner.py` to the current version (which uses
`subprocess.DEVNULL` in non-verbose mode), or run with `--verbose`
during development.

**Prevention for runner authors:** minimize stderr output in
production mode; keep detailed diagnostics behind a flag.

### "Vector digest mismatch"

The manifest's `vector_digest` does not match the current contents of
the vectors directory. Either:

- Vectors were modified without regenerating the manifest → run
  `python generate_manifest.py` and commit the result, or
- You are running against the wrong vectors directory → check
  `--vectors-dir`.

### "Unsupported protocol version"

The runner returned a `protocol_version` the harness does not
understand. Currently the harness supports only `"1"`. Update either
the runner or the harness to a compatible version.

## Repository Layout

```
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

## References

- Runner protocol specification: [`runner_protocol.md`](runner_protocol.md)
- Conformance concept: [`../README.md`](../README.md) → "Conformance Testing"
- Target interface contract: [`../api/decision-api.md`](../api/decision-api.md)
- Vector format: [`../vectors/README.md`](../vectors/README.md)
