# hacp-rs — HACP Clean-Room Implementation (Rust)

**Language:** Rust (stable)  
**Profile:** Ed25519 + SHA-256 + JCS RFC 8785 (canonical 1.0.0/1.1.0)  
**Conformance:** Core 38/38 + HC2-55 55/55

## Prerequisites

- Rust stable (tested with rustc 1.98.1)

## Build

```bash
cd hacp-rs
cargo build --release --bin hacp-rs-runner
```

## Test

```bash
cargo test
```

## Harness Runner — Core 38/38

```bash
cd ..
python harness/harness_runner.py \
  --runner "hacp-rs/target/release/hacp-rs-runner" \
  --vectors-dir vectors \
  --manifest harness/conformance_manifest.json \
  --implementation-name hacp-rs \
  --implementation-version 0.1.0-1.1.0 \
  --output console
```

Expected:

```text
RESULTS: 38/38 passed
```

## Harness Runner — HC2-55 55/55

```bash
python harness/enforcement_v2_runner.py \
  --runner "hacp-rs/target/release/hacp-rs-runner" \
  --vectors-dir vectors/enforcement-v2 \
  --implementation-name hacp-rs \
  --implementation-version 0.1.0-1.1.0 \
  --output console
```

Expected:

```text
RESULTS: 55/55 passed
```

## Crypto Profile

| Component | Algorithm | Standard |
|---|---|---|
| Hashing | SHA-256 | FIPS 180-4 |
| Signatures | Ed25519 | RFC 8032 |
| Canonicalization | JCS | RFC 8785 |

Test keypair:

```text
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
public_key = Ed25519_derive_public(seed)
```

See [`wire/crypto-profile.md`](../wire/crypto-profile.md) for the full profile specification.

## Non-Goals

- Not a replacement for the Go enforcement sidecar
- Not a GOST/SM2 crypto suite implementation
- Not a general URI normalization library
- Not Gate E / gRPC distributed control plane
- Not claiming "exact-reason 38/38" beyond decision + reason_codes

## HTTP Proxy (B1)

```bash
cargo build --release --bin hacp-rs-proxy
HACP_SIDECAR_PORT=8080 ./target/release/hacp-rs-proxy
```

Headers: `X-HACP-Intent-Envelope`, `X-HACP-Decision-Token` (base64url-encoded JSON).
Fail-closed: missing/invalid headers → 403 DENY.
Same `evaluate` function as runner — no forked logic.

## MCP Server (B2)

```bash
cargo build --release --bin hacp-rs-mcp
echo '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}' | ./target/release/hacp-rs-mcp
```

JSON-RPC 2.0 over stdio. Methods: `initialize`, `tools/list`, `tools/call`.
HACP in `tools/call` params: `hacp_intent_envelope`, `hacp_decision_token` (base64url or raw JSON).
Fail-closed: missing/invalid HACP → error response.
Same `evaluate` function as runner and proxy — no forked logic.

## Gate E Control Plane (B3)

```bash
cargo build --release --bin hacp-rs-controlplane
echo '{"method":"revoke","kind":1,"subject_id":"key-001","id":1}' | ./target/release/hacp-rs-controlplane
```

JSON-RPC over stdio. Commands: `revoke`, `snapshot`, `events_after`, `revision`.
Components:
- **Journal** — in-memory revocation journal (monotonic revision, idempotent, replay)
- **ControlState** — freshness tracker (`is_fresh`, `mark_snapshot/event/heartbeat/unsafe`)
- **RevocationStore** — bridges control plane with evaluate (`inject_into_context`)

No tonic/protoc required. Same evaluate function — revocations injected via policy_context.

## Dependencies

- `ed25519-dalek` 2.x — Ed25519 signatures
- `sha2` 0.10 — SHA-256
- `serde` + `serde_json` — JSON serialization
- `base64` 0.22 — base64url encoding
- `ryu` 1.x — shortest float representation (JCS)
- `hyper` 1.x — HTTP server (proxy binary)
- `tokio` 1.x — async runtime (proxy binary)
