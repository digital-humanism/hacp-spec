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

## Draft Vector Inventory

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

All vectors in this directory remain draft vectors until their
cryptographic artifacts are deterministically baked and the draft
Enforcement v2 conformance path executes them successfully.