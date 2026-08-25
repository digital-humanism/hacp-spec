# HACP Enforcement Profile v2 Draft Conformance Vectors

Status: Draft — not part of the current canonical HACP-Core conformance set.

These vectors exercise candidate normative requirements defined by
`profiles/enforcement-v2-draft.md`.

The vectors in this directory MUST NOT be included in claims against the
current HACP-Core canonical vector set or its published manifest.

## HC2 — HTTP Path Binding Representation

This draft vector line covers the request-binding semantics of
`DecisionToken.constraints.path`, including path-and-query binding and
the explicitly defined percent-encoding comparison rules.

Deferred URI normalization cases are out of scope unless explicitly
added by a later normative revision.

## HC2 Vector Inventory

| Vector | Type | Constraint | Request target | Expected |
|---|---|---|---|---|
| `ENF-HC2-001` | golden | `/a/b` | `/a/b` | `ALLOW` |
| `ENF-HC2-002` | negative | `/a/b` | `/a%2Fb` | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-003` | golden | `/a%2Fb` | `/a%2Fb` | `ALLOW` |
| `ENF-HC2-004` | golden | `/a%2Fb` | `/a%2fb` | `ALLOW` |
| `ENF-HC2-005` | negative | `/a%2Fb` | `/a%252Fb` | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-006` | golden | `/a%252Fb` | `/a%252Fb` | `ALLOW` |
| `ENF-HC2-007` | negative | `/transfer?account=A` | `/transfer?account=B` | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-008` | golden | `/transfer?account=A` | `/transfer?account=A` | `ALLOW` |

## HC2-B — Invalid and Boundary Percent-Triplet Semantics

HC2-B defines the comparison boundary for invalid or incomplete percent
sequences without introducing additional URI normalization.

Only valid percent-encoded triplets (`%HH`, where both `H` characters are
ASCII hexadecimal digits) receive hexadecimal case equivalence. Invalid or
incomplete percent sequences compare literally and case-sensitively, while
subsequent characters remain subject to the comparison rules independently.

These rules apply to both the path and query portions of the request-binding
representation.

### HC2-B Vector Inventory

| Vector          | Type     | Constraint | Request target | Expected                |
| --------------- | -------- | ---------- | -------------- | ----------------------- |
| `ENF-HC2-B-001` | golden   | `/a%`      | `/a%`          | `ALLOW`                 |
| `ENF-HC2-B-002` | golden   | `/a%2`     | `/a%2`         | `ALLOW`                 |
| `ENF-HC2-B-003` | negative | `/a%2F`    | `/a%2`         | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-B-004` | negative | `/a%2G`    | `/a%2g`        | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-B-005` | golden   | `/a%2G`    | `/a%2G`        | `ALLOW`                 |
| `ENF-HC2-B-006` | golden   | `/x?q=%2F` | `/x?q=%2f`     | `ALLOW`                 |
| `ENF-HC2-B-007` | negative | `/x?q=%2G` | `/x?q=%2g`     | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-B-008` | golden   | `/x?q=%`   | `/x?q=%`       | `ALLOW`                 |
| `ENF-HC2-B-009` | golden   | `/a%2G%2F` | `/a%2G%2f`     | `ALLOW`                 |

## Verification Status

The HC2 and HC2-B vectors in this directory have been deterministically baked
and verified through the Enforcement v2 conformance path.

Current verified results:

* HC2 baseline: `8/8 passed`
* HC2-B boundary semantics: `9/9 passed`

These results demonstrate only the request-binding semantics explicitly
defined by the current Enforcement v2 draft. They do not imply general URI
normalization conformance.

The Enforcement v2 profile and all vectors in this directory remain draft and
MUST NOT be included in claims against the current HACP-Core canonical
conformance set or its published manifest.
