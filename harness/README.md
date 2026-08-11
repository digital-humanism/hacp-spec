# HACP Conformance Harness

Cross-language conformance testing tool for HACP implementations.

## Installation

```bash
pip install -r requirements.txt
```

## Usage

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

## Target Interface

See `../api/decision-api.md` Section 3 for complete conformance testing contract.

### HTTP Interface

```http
POST /evaluate
Content-Type: application/json

{
  "test_id": "CORE-INV3-001",
  "type": "golden",
  "inputs": { ... },
  "policy_context": { ... },
  "expected": { "outcome": "ALLOW" }
}
```

### CLI Interface

```bash
./hacp-impl evaluate --vector vectors/core_inv3_001_golden.json
```

## Exit Codes

- `0`: All tests passed
- `1`: One or more tests failed