# HC2-F Trailing Empty Path Segment Preservation — Normative Assessment

Status: Intermediate normative assessment
Profile: HACP Enforcement v2 Draft
Scope: HTTP request binding
Production changes: None
Conformance vectors: Not yet created

## 1. Purpose

This document records the read-only normative assessment performed after completion of HC2-E.

Its purpose is to determine whether preservation of a trailing empty HTTP path segment is suitable for a new, minimal HACP Enforcement v2 request-binding conformance block.

This assessment does not activate new profile semantics, define executable vectors, or establish implementation conformance.

## 2. Candidate Boundary

The candidate representation boundary is:

```text
/a/
```

versus:

```text
/a
```

The trailing `/` in `/a/` creates a zero-length path segment following a non-empty path segment.

The normative question is whether that trailing empty segment remains representation-significant for HACP HTTP request binding.

## 3. Standards Basis

RFC 3986 defines a path as a sequence of path segments separated by `/`.

Its grammar defines:

```text
segment = *pchar
```

which permits a segment of zero length.

Accordingly, `/a/` and `/a` are syntactically distinguishable path representations: the former contains a trailing empty segment and the latter does not.

RFC 3986 does not establish a general rule requiring a trailing empty segment following a non-empty segment to be removed.

RFC 9110 likewise does not require a single universal method for determining HTTP URI equivalence. HTTP components may apply different comparison or normalization strategies.

The HTTP normalization rule treating an empty path component as equivalent to `/` concerns the URI path component being empty as a whole. It does not establish equivalence between `/a/` and `/a`.

Therefore, the HTTP and URI specifications do not require HACP request binding to treat these two representations as equivalent.

## 4. HACP Security Relevance

HACP HTTP request binding is an authorization boundary between an authorized action representation and an observed HTTP request target.

Introducing an implicit transformation such as:

```text
/a/ -> /a
```

or:

```text
/a -> /a/
```

would expand the set of request-target representations accepted under the same authorization.

Such expansion requires an explicit normative basis.

Where enforcement, proxy, routing, or application layers interpret trailing path delimiters differently, implicit trailing-slash normalization can create a normalization disagreement across the authorization boundary.

The HACP safety model therefore favors preserving the distinction unless equivalence is explicitly defined.

Loss of representation certainty must not silently broaden authorization.

## 5. Proposed Normative Direction

The assessment supports the following narrow semantic direction:

```text
A trailing empty path segment following a non-empty path segment
is representation-significant for HACP HTTP request binding.
```

Consequently:

```text
/a/ == /a/
```

and:

```text
/a/ != /a
/a  != /a/
```

for the specific boundary under consideration.

This is a request-binding representation rule, not a claim that all HTTP applications necessarily map the two paths to different resources.

## 6. Deliberate Scope Boundary

This assessment does not define semantics for:

```text
the empty path component as a whole
the root path /
leading empty path segments
multiple consecutive empty path segments
internal empty path segments beyond the already defined HC2-E boundary
dot segments
percent-encoded slash
percent-decoded path comparison
application router redirects
filesystem path normalization
URI reference resolution
query normalization
fragment handling
scheme or authority normalization
general URI normalization
```

In particular, it does not determine the semantics of:

```text
/
//a
/a//
/a///b
/a/./b
/a/x/../b
/a%2F
```

Any such boundary requires separate normative analysis.

## 7. Relationship to Existing HC2 Blocks

The existing verified request-binding blocks remain unchanged:

```text
HC2
exact representation and valid percent-triplet hex-case equivalence

HC2-B
invalid and boundary percent-triplet semantics

HC2-C
encoded delimiter preservation

HC2-D
empty query delimiter preservation

HC2-E
internal empty path segment preservation
```

The proposed HC2-F boundary addresses a distinct structural condition:

```text
absence of a trailing empty path segment
!=
presence of a trailing empty path segment
```

It does not modify or broaden the semantics established by HC2 through HC2-E.

## 8. Minimal Conformance Shape

If the normative direction is accepted into the Enforcement v2 draft profile, the smallest symmetric conformance matrix is expected to be:

```text
/a/ == /a/
ALLOW

/a/ != /a
DENY / SCOPE_EXCEEDED

/a != /a/
DENY / SCOPE_EXCEEDED
```

These are assessment-level candidate cases only.

No vectors should be created until the normative requirement is committed to the draft profile.

## 9. Production Decision

No production defect has been established.

No production implementation has been tested against this proposed invariant.

No production change is justified at this stage.

The required process remains:

```text
normative requirement
-> executable vectors
-> black-box evaluation
-> RED only if a defect exists
-> minimal production fix
-> GREEN
-> regression
```

If the existing implementation passes the eventual HC2-F vectors, production code must remain unchanged.

## 10. Assessment Conclusion

Trailing empty path segment preservation is suitable for a minimal, independently testable HACP Enforcement v2 request-binding conformance block.

The boundary has:

* clear URI-syntax grounding;
* direct authorization relevance;
* low normative ambiguity;
* low scope-expansion risk when restricted to a trailing empty segment following a non-empty segment;
* no dependency on a general URI-normalization policy.

The recommended next step is to review a narrowly scoped HC2-F normative subsection before modifying `profiles/enforcement-v2-draft.md`.

No profile, vector, harness, baker, or production implementation change is authorized by this assessment alone.
