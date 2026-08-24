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