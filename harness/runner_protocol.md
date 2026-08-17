# HACP Conformance Runner Protocol

**Protocol version:** 1  
**HACP baseline:** HACP-Core v0.9.2  
**Status:** Implemented and validated  
**License:** CC BY 4.0

This document defines the language-neutral black-box protocol between the
HACP conformance harness and implementation runners.

Protocol version `1` is the runner transport/interaction contract. It is
distinct from the HACP specification version (`0.9.2`).

---

## 1. Purpose

The runner protocol allows the canonical HACP conformance harness to verify
implementations written in different languages without importing or linking
their implementation code.

```text
hacp-spec
   │
   ├── normative specification
   ├── canonical vectors
   ├── conformance manifest
   └── language-neutral harness
            │
            │ stdin/stdout JSON Lines
            ▼
        black-box runner
            │
      ┌─────┼─────┬─────────┐
      ▼     ▼     ▼         ▼
   Python   Go    TS      Sidecar
```

> **Conformance validates observable protocol behavior, not implementation structure.**

The harness MUST treat the runner as an external process.

The harness MUST NOT import implementation code directly when operating in
Runner Protocol mode.

---

## 2. Current Verified Baseline

The current HACP-Core v0.9.2 conformance baseline is:

```text
Spec:          HACP-Core v0.9.2
Vector set:    core-0.9.2
Vectors:       38
Manifest:      verified
Vector digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

The Go `hacp-sidecar` runner has been validated through this protocol with:

```text
Protocol version: 1
RESULTS: 38/38 passed
Failures: 0
Skipped normative vectors: 0
```

The same canonical vector set also converges in the current Python and
TypeScript implementations.

---

## 3. Transport

Runner Protocol v1 uses line-delimited JSON over standard process streams.

### 3.1 stdin

The harness writes one complete JSON request per line to the runner's stdin.

### 3.2 stdout

The runner writes exactly one complete JSON response per request to stdout.

### 3.3 stderr

All diagnostics, logs, traces, and debugging output MUST be written to stderr.

A runner MUST NOT write human-readable diagnostic text to stdout.

### 3.4 Process lifetime

A runner MAY remain alive for multiple vector evaluations.

It MUST:

1. continue reading requests until stdin is closed;
2. emit one response for each accepted request;
3. flush stdout after each response;
4. exit cleanly after stdin closes unless a fatal runner error occurs.

---

## 4. Protocol Version

All requests and responses MUST contain:

```json
{
  "protocol_version": "1"
}
```

Runner Protocol v1 currently uses the literal string:

```text
"1"
```

The harness MUST reject a response whose `protocol_version` is missing or
unsupported.

Because the current protocol identifier is a single major-version string,
there is no minor-version negotiation in v1.

Future protocol revisions MAY define an explicit compatibility policy, but
implementations MUST NOT infer compatibility with unknown protocol values.

A protocol-version mismatch is a runner/protocol error and maps to harness
exit code `3`.

---

## 5. Request Message

### 5.1 General form

The harness sends:

```json
{
  "protocol_version": "1",
  "operation": "evaluate",
  "vector_id": "CORE-INV5-001",
  "input": {
    "intent_envelope": {},
    "proposed_action": {},
    "decision_token": {},
    "policy_context": {},
    "checkpoint": {}
  }
}
```

Required fields:

| Field | Type | Description |
|---|---|---|
| `protocol_version` | string | Runner protocol version; currently `"1"` |
| `operation` | string | Operation requested by the harness |
| `vector_id` | string | Canonical vector identifier, e.g. `CORE-INV5-001` |
| `input` | object | Operation-specific input data |

### 5.2 `input`

For `evaluate`, the input object MAY contain:

| Field | Type | Description |
|---|---|---|
| `intent_envelope` | object | HACP IntentEnvelope |
| `proposed_action` | object | ProposedAction under evaluation |
| `decision_token` | object/null | DecisionToken when the vector exercises token behavior |
| `policy_context` | object | Explicit deterministic policy/runtime context |
| `checkpoint` | object | Runtime checkpoint data where applicable |
| `checkpoint_state` | object | Additional checkpoint lifecycle data where applicable |
| `provenance_event` | object | Provenance event where applicable |
| `prior_provenance_event` | object | Previous provenance event where applicable |
| `omit_provenance` | boolean | Negative-test control where applicable |

The exact input fields used by a vector are defined by the canonical vector
itself.

---

## 6. Operations

### 6.1 `evaluate` — REQUIRED in Protocol v1

`evaluate` is the normative operation required for HACP-Core v0.9.2
conformance.

Request:

```json
{
  "protocol_version": "1",
  "operation": "evaluate",
  "vector_id": "CORE-INV3-001",
  "input": {
    "intent_envelope": {},
    "proposed_action": {},
    "decision_token": {},
    "policy_context": {}
  }
}
```

Response:

```json
{
  "protocol_version": "1",
  "decision": "ALLOW",
  "reason_codes": [],
  "action_hash": "5dd0154322d58283f02c6c623e29b66c77b74ea659d1b6a43aaf064d3555a69e",
  "metrics": {
    "latency_ns": 12345
  }
}
```

### 6.2 Other operations

Earlier drafts described `revoke` and `explain` as runner operations.

They are **not required for the current HACP-Core v0.9.2 Protocol v1
conformance run**.

Implementations MUST NOT be marked non-conformant merely because they do not
implement those non-required operations.

If future vector profiles require additional operations, they should be added
to a later protocol revision or explicitly profiled extension.

---

## 7. Response Message

### 7.1 Required fields for `evaluate`

| Field | Type | Description |
|---|---|---|
| `protocol_version` | string | MUST equal `"1"` |
| `decision` | string | `ALLOW`, `DENY`, or `CHECKPOINT` |
| `reason_codes` | array[string] | Normative reason codes; empty for successful `ALLOW` |
| `action_hash` | string | SHA-256 hex of canonicalized ProposedAction |

Example:

```json
{
  "protocol_version": "1",
  "decision": "DENY",
  "reason_codes": ["HASH_MISMATCH"],
  "action_hash": "5dd0154322d58283f02c6c623e29b66c77b74ea659d1b6a43aaf064d3555a69e"
}
```

### 7.2 Optional fields

| Field | Type | Description |
|---|---|---|
| `metrics` | object | Non-normative execution metrics |
| `metrics.latency_ns` | integer | Runner processing latency in nanoseconds |
| `error_message` | string | Human-readable runner error detail for protocol/infrastructure errors |

Additional implementation-specific fields MAY be returned if they do not
change the normative meaning of required fields and do not break harness
parsing.

---

## 8. Action Hash

For Protocol v1 conformance:

```text
action_hash = SHA256(JCS(proposed_action))
```

Where:

- JCS is the HACP canonicalization profile based on RFC 8785;
- SHA-256 output is represented as lowercase hexadecimal;
- the hash is computed over the canonical UTF-8 bytes of the ProposedAction.

Equivalent logical objects with different source key ordering MUST produce
the same `action_hash`.

Security-relevant semantic changes MUST produce a different `action_hash`.

---

## 9. Output Constraints

The runner MUST obey all of the following:

1. exactly one JSON response per request;
2. no non-JSON content on stdout;
3. diagnostics only on stderr;
4. no partial JSON records;
5. one JSON record per line;
6. stdout flushed after each response;
7. no extra banner, prompt, or startup text on stdout.

Violating these rules is a runner/protocol error, not a normative vector
failure.

---

## 10. Timeout Semantics

The harness MAY enforce a per-vector timeout.

Current default:

```text
runner_timeout_ms = 5000
```

If a runner exceeds the configured timeout:

1. the harness MAY terminate the runner process;
2. the run MUST be classified as a runner execution/protocol error;
3. the harness MUST use exit code `3`;
4. the timeout MUST NOT be reported as an ordinary conformance-vector failure.

---

## 11. Runner Errors

A runner-level failure is different from a valid HACP decision of `DENY`.

For a recoverable internal runner error, a runner MAY emit:

```json
{
  "protocol_version": "1",
  "decision": "ERROR",
  "reason_codes": ["INTERNAL_ERROR"],
  "error_message": "description of the runner failure"
}
```

`ERROR` is not a normative HACP decision.

The harness MUST treat this as runner/protocol failure and exit with code `3`,
rather than comparing it as a normal conformance decision.

A runner crash, malformed response, broken pipe, or unsupported protocol
version is handled the same way.

---

## 12. Malformed Responses

A response is malformed if, for example:

- stdout contains invalid JSON;
- a response line contains non-JSON text;
- required response fields are missing;
- `protocol_version` is unsupported;
- more than one response is emitted for one request;
- no response arrives before timeout.

Malformed runner output MUST result in harness exit code `3`.

The harness MUST NOT reinterpret malformed runner output as a normative HACP
`DENY`.

---

## 13. Conformance Manifest

Runner-mode conformance SHOULD be executed with:

```text
harness/conformance_manifest.json
```

Current manifest:

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

Required manifest fields:

| Field | Type | Description |
|---|---|---|
| `spec_version` | string | HACP specification version |
| `profile` | string | Conformance profile |
| `vector_set` | string | Canonical vector-set identifier |
| `canonicalization` | string | Canonicalization profile used by the manifest |
| `digest_algorithm` | string | Digest algorithm |
| `vector_digest` | string | Digest of the canonical vector set |
| `total_vectors` | integer | Number of vectors expected |

The harness MUST verify the vector digest before executing vectors when a
manifest is supplied for the conformance run.

A digest mismatch is a harness/configuration error and MUST map to exit
code `2`.

The digest gate MUST NOT be silently bypassed to make a modified vector set
appear conformant.

---

## 14. Harness Comparison Rules

For each vector, the harness compares the runner response with the vector's
expected observable result.

Normative comparison includes, where the vector specifies them:

```text
decision / outcome
reason_codes
action_hash
```

Other fields may be compared when required by a future profile.

Performance metrics are non-normative for Protocol v1 conformance unless a
separate benchmark profile explicitly says otherwise.

---

## 15. Exit Codes

| Code | Meaning | Description |
|---|---|---|
| `0` | Conformant | All canonical vectors passed |
| `1` | Conformance failure | One or more vectors produced the wrong normative result |
| `2` | Harness/configuration error | Manifest, vector loading, paths, or harness setup failed |
| `3` | Runner execution/protocol error | Runner crash, malformed JSON, timeout, version mismatch, etc. |

The distinction between `1`, `2`, and `3` is normative for automation:

```text
1 = implementation behavior disagreed with vectors
2 = harness could not establish a valid test set/configuration
3 = black-box runner transport/execution failed
```

CI SHOULD fail on any non-zero code.

---

## 16. Harness Output Formats

The current runner harness supports:

```text
console
json
```

### 16.1 Console

Example:

```text
============================================================
HACP Conformance Harness v0.9.2 - Runner Mode
Protocol version: 1
Spec: 0.9.2 (HACP-Core)
============================================================

Manifest verified: 0.9.2 (HACP-Core)
Vector set: core-0.9.2
Digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58

[PASS] CORE-INV1-001
...
[PASS] CORE-RUNTIME-005

============================================================
RESULTS: 38/38 passed
============================================================
```

### 16.2 JSON

Machine-readable output SHOULD include enough metadata to identify:

```text
spec version
profile
implementation name
implementation version
runner protocol version
vector-set digest
total/passed/failed vectors
overall result
failure details
```

Illustrative shape:

```json
{
  "spec_version": "0.9.2",
  "profile": "HACP-Core",
  "implementation": "hacp-sidecar",
  "implementation_version": "0.3.0",
  "protocol_version": "1",
  "vector_set_digest": "sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58",
  "vectors": {
    "total": 38,
    "passed": 38,
    "failed": 0
  },
  "result": "conformant",
  "failures": []
}
```

Fields not emitted by the current harness implementation should not be treated
as required merely because they appear in this illustrative example.

---

## 17. Runner Implementation Requirements

A Protocol v1 `evaluate` runner MUST:

1. accept line-delimited JSON requests on stdin;
2. support `protocol_version: "1"`;
3. support `operation: "evaluate"`;
4. emit one JSON response per request;
5. return normative `decision`, `reason_codes`, and `action_hash`;
6. write diagnostics to stderr only;
7. flush stdout after every response;
8. exit cleanly when stdin closes;
9. fail closed on malformed protocol input;
10. preserve canonical vector semantics rather than silently repairing malformed test input.

For malformed raw JSON vectors such as duplicate-key negative tests, an
implementation MAY need a pre-parse validation layer because ordinary language
JSON parsers can otherwise normalize away the malformed condition.

---

## 18. Harness Implementation Requirements

The Runner Protocol harness MUST:

1. load vectors from the configured vectors directory;
2. optionally load and verify the conformance manifest;
3. verify the manifest digest before vector execution when manifest verification is enabled;
4. start the runner as an external process;
5. send one request per vector;
6. read exactly one response per request;
7. enforce the configured runner timeout;
8. compare normative response fields with vector expectations;
9. support console and JSON result output;
10. return the exit codes defined in Section 15.

Current CLI naming uses:

```text
--runner
--vectors-dir
--manifest
--implementation-name
--implementation-version
--output
--runner-timeout
--verbose
```

---

## 19. Reproducing the Current Go Runner Verification

Build the runner:

```powershell
cd ...\GitHub\Dev\hacp-sidecar
go build -o hacp-conformance-runner.exe .\cmd\hacp-conformance-runner
```

Run the manifest-verified harness:

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

---

## 20. Security and Assurance Notes

The runner protocol is intentionally small.

The harness boundary is designed so that an implementation cannot gain
conformance merely by sharing internal implementation objects with the
harness.

The current result demonstrates:

```text
black-box behavioral verification
manifest-pinned vector execution
language-neutral transport
reproducible protocol decisions
```

A successful run does not by itself constitute:

```text
a formal proof of protocol correctness
a complete security audit
a fuzzing result
a production deployment certification
```

Those assurance layers are separate from Protocol v1 conformance.

---

## 21. References

- [`../README.md`](../README.md) — HACP specification repository overview
- [`README.md`](README.md) — conformance harness usage
- [`../vectors/README.md`](../vectors/README.md) — canonical vector set
- [`conformance_manifest.json`](conformance_manifest.json) — pinned current vector set
- [`../error-model.md`](../error-model.md) — reason-code model
- [`../canonicalization.md`](../canonicalization.md) — canonicalization rules
- [`../wire/crypto-profile.md`](../wire/crypto-profile.md) — cryptographic profile
- [`../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md`](../docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md) — current TypeScript/Go verification record

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)
