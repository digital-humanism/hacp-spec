# HC2-L Leading Empty Path Segment Preservation — Normative Assessment

## Status

Normative assessment for the Enforcement v2 draft request-target binding model.

This document evaluates whether a leading empty path segment in an HTTP
origin-form request-target is a distinct, representation-significant binding
boundary that warrants an explicit HC2 rule.

This assessment does not itself modify the Enforcement v2 draft profile,
conformance vectors, or production implementation.

## Decision

**Positive.**

The distinction:

```text
//a != /a
```

is normatively novel relative to the existing HC2 request-binding rules and can
be isolated cleanly at the HACP request-binding layer.

A narrow HC2 rule for leading empty path segment preservation is therefore
justified.

## 1. Assessment question

For HACP HTTP request binding, is a leading empty path segment in an observed
origin-form request-target representation significant?

Canonical pair:

```text
//a
/a
```

The narrow question is whether HACP may treat these two representations as
equivalent when evaluating a bound HTTP request target.

## 2. Existing HC2 coverage does not define this boundary

The existing Enforcement v2 draft request-binding taxonomy contains explicit
rules for other empty-segment positions, but it does not establish a
location-independent empty-segment normalization rule.

In particular, HC2-E defines internal empty path segment preservation and
deliberately limits its scope to an empty segment between non-empty path
segments. Existing trailing-empty-segment rules likewise do not define leading
empty path segment semantics.

Therefore:

```text
//a vs /a
```

is not merely another representative of an already established location-
independent invariant.

The leading case remains a distinct normative gap.

## 3. HTTP origin-form permits the representation without authority semantics

RFC 9112 defines the HTTP/1.1 origin-form request-target as:

```text
origin-form = absolute-path [ "?" query ]
```

RFC 9110 defines:

```text
absolute-path = 1*( "/" segment )
```

where `segment` is the URI segment production and can be empty.

Accordingly, an origin-form request-target can contain consecutive slash
delimiters such that the path begins with a leading empty segment before a
subsequent non-empty segment.

For the canonical case:

```text
//a
```

the assessment does not require interpreting the representation as a generic
URI-reference network-path reference and does not require deriving an authority
component from it.

The relevant HACP boundary is the origin-form HTTP request-target representation
observed for binding.

References:

- RFC 9112, Section 3.2.1, `origin-form`
- RFC 9110, Section 4.1, `absolute-path`

## 4. Request-binding significance is representation-local

The Enforcement v2 draft request-binding model operates on the path-and-query
representation derived from the HTTP request target observed at the enforcement
boundary.

The proposed distinction can therefore be evaluated directly as two different
observed representations:

```text
R1 = //a
R2 = /a
```

No target URI reconstruction is necessary to determine that the representations
differ.

No application-level resource model is necessary.

No router or framework interpretation is necessary.

No proxy or intermediary normalization behavior is necessary.

No generic URI authority interpretation is necessary.

The assessment asks only whether HACP itself defines an equivalence that removes
the leading empty segment during request-binding comparison.

No such equivalence is currently defined.

## 5. Candidate normative invariant

The resulting narrow invariant is:

> For HTTP request binding, a leading empty path segment present in an observed
> origin-form request-target representation is representation-significant.
> An implementation MUST NOT treat a request target containing that leading
> empty path segment as equivalent to the otherwise identical request target
> with that segment removed.

Canonical distinction:

```text
//a != /a
```

The distinction is symmetric for binding comparison.

## 6. Why this is a separate normative boundary

This assessment does not introduce the rule merely to complete positional
symmetry among internal, trailing, and leading empty path segments.

The candidate became eligible only after a read-only remaining-boundary
inventory established both of the required conditions:

```text
normatively novel
and
cleanly isolatable at the HACP request-binding layer
```

The first condition is satisfied because existing HC2 wording deliberately does
not define leading empty path segment semantics.

The second condition is satisfied because the distinction can be made directly
on the observed origin-form request-target representation without invoking a
broader URI, routing, proxy, or application semantics model.

## 7. Security relevance

Request-binding enforcement is sensitive to representation changes that occur
before or after authorization checks.

Collapsing:

```text
//a
```

to:

```text
/a
```

during HACP binding comparison would introduce an implicit normalization rule
that the profile does not otherwise define.

This assessment does not claim that any particular router, framework, proxy, or
origin server performs such a transformation.

The security property is narrower: HACP request binding must not silently erase
a representation distinction unless an explicit HACP binding-equivalence rule
authorizes that equivalence.

RFC 9112 also cautions against automatic correction of request-line
representations before processing because inconsistent interpretations along a
request chain can create security-filter bypass risks. That warning supports
the conservative representation-preserving direction, but the HC2-L rule does
not depend on any particular downstream normalization behavior.

## 8. Implementation observation

In the current Go HTTP implementation used by the HACP sidecar,
`http.Request.RequestURI` exposes the unmodified request-target of the HTTP
request line as sent by the client to the server.

This is implementation evidence that the relevant request-target representation
can be observed at the enforcement boundary.

It is not the normative basis of the rule.

The normative basis remains the HACP request-binding model together with the
HTTP origin-form and absolute-path syntax.

## 9. Explicit out of scope

This assessment does **not** define:

```text
///a ? //a
```

or any broader leading-slash multiplicity rule.

It also does not define:

- generic URI-reference network-path semantics;
- authority interpretation;
- absolute-form request-target semantics;
- target URI reconstruction;
- proxy or intermediary rewriting or normalization;
- framework, middleware, or router path cleaning;
- application-level routing equivalence;
- root versus empty-path equivalence;
- percent-encoded slash equivalence;
- dot-segment processing;
- recursive percent decoding;
- general slash normalization;
- general URI normalization.

No broader path equivalence rule follows from this assessment.

## 10. Assessment against the admission criteria

| Criterion | Result | Basis |
| --- | --- | --- |
| Normative novelty | PASS | Existing HC2 rules deliberately leave the leading empty-segment case undefined. |
| Representation-only purity | PASS | The distinction is visible directly in the observed origin-form request-target. |
| Application semantic independence | PASS | No parsed application resource or query model is required. |
| Transport feasibility | PASS | HTTP origin-form uses `absolute-path`, whose grammar permits the representation. |
| Clean HACP-layer isolation | PASS | No URI reconstruction, authority inference, router behavior, or proxy normalization is required. |
| Security relevance | PASS | Silent collapse would create an undeclared binding equivalence. |
| Narrow symmetric testability | PASS | One exact golden case and two symmetric mismatch cases are sufficient if vectors are later authorized. |
| Taxonomy inflation control | PASS | The rule exists because current wording leaves a real positional normative gap, not to add another representative. |

## 11. Assessment result

The leading-empty-path-segment boundary satisfies the admission condition:

```text
normatively novel
AND
cleanly isolatable at the HACP request-binding layer
```

The canonical distinction:

```text
//a != /a
```

is therefore eligible for an explicit Enforcement v2 draft normative rule.

For taxonomy purposes, the proposed class name is:

```text
HC2-L — Leading Empty Path Segment Preservation
```

This assessment authorizes discussion and drafting of that narrow profile rule.
It does not authorize unrelated normalization rules, additional multiplicity
classes, opportunistic refactoring, or production changes.

## 12. Change state at assessment completion

```text
production changes = 0
profile changes    = 0
vectors            = 0
```

The next allowed step is a narrow Enforcement v2 draft profile rule for HC2-L.
Vectors remain subsequent to the normative profile change, and production code
remains unchanged unless a later executable conformance test demonstrates a
real defect.
