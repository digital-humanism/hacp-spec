# HACP 1.1 Clean-Clone Verification

**Stage:** R9
**Status:** CLOSED / PASS
**Date:** 2026-09-17

## 1. Purpose

This record documents clean-clone reproducibility verification for the HACP 1.1 release candidate source set fixed by the R8 release-candidate record.

R9 verifies that the mandatory HACP 1.1 verification surface can be reproduced from fresh isolated clones at the exact candidate source commits, without relying on uncommitted source files or developer-tree state.

This record does not redefine the HACP 1.1 release candidate and does not change any protocol, profile, vector, implementation, or wire semantics.

## 2. Candidate Source Set

The R8 candidate source set verified by R9 is:

| Repository      | Commit                                     |
| --------------- | ------------------------------------------ |
| `hacp-spec`     | `328c50cecc0fefdae4fc481b08b83f743b6e239a` |
| `hacp-sidecar`  | `1bc10acbe79620a165cee573c5b260cd8639280f` |
| `humanist-core` | `6d9ae82ed7fefe47b395e4ef68d0a18807866473` |

Each repository was freshly cloned into an isolated verification root and checked out at the exact commit above.

Final identity verification confirmed that all three repositories remained at the required candidate commits and had clean working trees.

## 3. Verification Environment

The verification was performed on Windows with:

* Python `3.13.14`
* pytest `9.1.1`
* Go `1.26.5 windows/amd64`

Python verification used an R9-local virtual environment created from:

`C:\Program Files\Python313\python.exe`

Accepted Python evidence was executed through the explicit R9-local interpreter rather than through a developer-tree virtual environment.

The Go sidecar and conformance runner were built directly from the fresh `hacp-sidecar` clone.

## 4. HACP-Core Verification

The canonical HACP-Core harness was executed from the fresh `hacp-spec` clone.

Result:

```text
RESULTS: 38/38 passed
```

Status: **PASS**

The canonical Core vector set was not modified.

## 5. Enforcement Revision 2 / HC2-55 Verification

### 5.1 Fixture integrity

The Enforcement revision 2 vector baker was executed in check mode against the explicit active vector directory:

```text
vectors/enforcement-v2
```

Result:

```text
CHECK RESULTS: 55/55 passed
```

Test identity:

```text
key-ed25519-test-001
```

Raw Ed25519 public key:

```text
9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3
```

Status: **PASS**

### 5.2 Black-box runner verification

A fresh `hacp-conformance-runner` binary was built from the exact `hacp-sidecar` candidate commit and executed through Runner Protocol v1 against:

```text
vectors/enforcement-v2
```

Result:

```text
RESULTS: 55/55 passed
```

Status: **PASS**

This verification remains bounded to the active HACP-Enforcement revision 2 request-binding evidence represented by HC2-55. It does not establish general URI normalization behavior beyond that defined surface.

## 6. hacp-sidecar Verification

### 6.1 Full repository test suite

Command surface:

```text
go test ./... -count=1
```

All tested packages passed.

Status: **PASS**

### 6.2 Native sidecar build

A native sidecar binary was built from the exact candidate commit.

Status: **PASS**

### 6.3 Conformance runner build

A native `hacp-conformance-runner` binary was built from the exact candidate commit.

Status: **PASS**

Both build operations left the source working tree clean.

## 7. humanist-core Verification

### 7.1 Default Python suite

The default test suite was executed through the isolated R9-local Python environment.

Result:

```text
319 passed, 5 skipped
```

The five skipped tests were the externally executed real-sidecar integration tests, which are conditional in the default suite.

Status: **PASS**

### 7.2 Python ↔ Go external E2E

The deterministic HACP conformance Ed25519 identity was independently reproduced from:

```text
SHA-256("hacp-conformance-v0.9-key-001")
```

The derived raw public key matched:

```text
9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3
```

The private key was exported as an R9-local PKCS8 PEM for test execution only.

The external integration used:

* the fresh-built `hacp-sidecar` binary;
* explicit test trust mode;
* an isolated reference upstream;
* the R9-local Python interpreter;
* the reproduced `key-ed25519-test-001` identity.

Result:

```text
5 passed
0 failed
0 skipped
```

The verified cases included fail-closed behavior, HTTP action-hash interoperability, Python envelope/token signature consistency, real-sidecar ALLOW, and `SidecarClient` ALLOW.

Status: **PASS**

## 8. Reproducibility Findings

R9 established the following:

* the required source identities are reproducible from fresh clones;
* the mandatory Core and Enforcement verification surfaces reproduce successfully;
* the Go sidecar and conformance runner build successfully from the clean candidate source;
* the Python SDK default suite reproduces in an isolated local environment;
* the Python ↔ Go external E2E path reproduces using fresh candidate artifacts;
* accepted R9 evidence does not depend on the normal developer working trees;
* no accepted result depends on uncommitted source files;
* all candidate repositories remained clean after verification.

No production source change was required.

## 9. Invocation and Environment Observations

Several setup and invocation issues were encountered during execution and were classified before accepted evidence was recorded.

They included:

* accidental use of a developer-tree Python virtual environment during an initial setup attempt;
* an initial vector-baker invocation against the default Core vector directory instead of the explicit Enforcement revision 2 directory;
* an already occupied reference-upstream port;
* omission of `HACP_SIDECAR_BIN` during an initial external-integration invocation.

These observations were classified as test/invocation environment issues.

Affected runs were excluded from accepted R9 evidence and were repeated using explicit isolated configuration.

They did not establish:

* a production defect;
* a conformance defect;
* a reproducibility defect;
* a production RED.

No production changes were made in response to these observations.

## 10. Final R9 Matrix

| Verification item                      | Result                               |
| -------------------------------------- | ------------------------------------ |
| HACP-Core canonical vectors            | **38/38 PASS**                       |
| HC2-55 fixture integrity               | **55/55 PASS**                       |
| HC2-55 black-box runner                | **55/55 PASS**                       |
| `hacp-conformance-runner` build        | **PASS**                             |
| `hacp-sidecar` full repository tests   | **PASS**                             |
| Native `hacp-sidecar` build            | **PASS**                             |
| `humanist-core` default suite          | **319 passed / 5 conditional skips** |
| Python ↔ Go external E2E               | **5/5 PASS**                         |
| Final candidate repository cleanliness | **PASS**                             |

## 11. R9 Conclusion

R9 clean-clone verification is **CLOSED / PASS**.

The HACP 1.1 R8 candidate source set reproduced the required verification matrix from fresh isolated clones without production changes.

No production RED was established during R9.

## 12. Release Boundary

R9 is a reproducibility and clean-clone verification stage only.

This record:

* does not constitute the R10 GO decision;
* does not create or approve HACP 1.1 release notes;
* does not create, move, or approve the `v1.1.0` tag;
* does not modify HACP wire version `0.9`;
* does not change HACP-Core vector identity;
* does not change HACP-Enforcement revision 2;
* does not expand HC2-55 beyond its documented claim boundary;
* does not reopen completed R7 or R8 verification.

R10 remains a separate release decision and publication stage.
