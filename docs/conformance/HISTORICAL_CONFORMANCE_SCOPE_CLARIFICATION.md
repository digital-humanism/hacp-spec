# Historical Conformance Scope Clarification

This note clarifies how the original HACP-Core v0.9.2 release verification results should be interpreted in light of later Protocol v1 reason-code re-certification work.

It does not change the HACP-Core normative requirements or invalidate previously published conformance results.

## Original published verification results

The HACP specification repository explicitly recorded two separate verification surfaces in the root `README.md`, under **Cross-Language Conformance Baseline**.

Canonical HACP-Core conformance:

```text
Canonical vectors: 38

Go clean-room implementation          38/38 PASS
TypeScript clean-room implementation  38/38 PASS
Python reference implementation       38/38 PASS
Go enforcement sidecar                38/38 PASS
```

Additional regression evidence was recorded separately, including:

```text
Python ↔ Go real sidecar E2E: 5/5 PASS
```

The same section explicitly characterized these results as a reproducible interoperability and regression milestone and stated that they were not presented as a formal security proof.

Reference:

* root `README.md`
* section: `Cross-Language Conformance Baseline`
* subsection / block: `Additional regression evidence`

These results were therefore not presented as interchangeable claims.

## Later Protocol v1 reason-code re-certification

Later engineering work established that the historical Protocol v1 runner-mode verification path validated the expected decision outcome but did not independently compare exact returned `reason_codes` against the canonical expected reason codes.

A historical Gate-A reconstruction reproduced the original `38/38` decision-level result while also demonstrating that exact reason-code correspondence was not independently observed by that historical verification path.

This finding should be interpreted as a limitation of historical semantic observability, not as a disclosure, test-inventory, or release-integrity failure.

The original published results remain valid for the verification surface exercised and reported at the time.

## Current interpretation

The later strict Protocol v1 verifier adds exact reason-code correspondence as an independently observed conformance dimension.

Accordingly:

```text
historical canonical result:
38/38 within the historical verification surface

additional real-sidecar E2E evidence:
5/5, reported separately

later strict re-certification:
adds exact reason-code observation
```

The newer verification work strengthens the conformance proof; it does not rewrite or invalidate the original release record.

## Current workflow

The current repository states in `README.md`, under **Conformance Testing Workflow**, that a conformant implementation is expected to:

> return the expected outcome and reason semantics for all 38 vectors.

This reflects the stronger current verification surface.

## References

* `README.md` — `Cross-Language Conformance Baseline`
* `README.md` — `Additional regression evidence`
* `README.md` — `Conformance Testing Workflow`
* `harness/runner_protocol.md`
