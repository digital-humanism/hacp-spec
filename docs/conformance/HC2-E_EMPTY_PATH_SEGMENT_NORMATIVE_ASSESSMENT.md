# HC2-E Empty Path Segment Preservation — Normative Assessment

Status: Intermediate normative assessment
Profile: HACP Enforcement v2 Draft
Scope: HTTP request binding
Production changes: None
Conformance vectors: Not yet created

## 1. Purpose

This document records the read-only normative assessment performed after completion of HC2-D.

Its purpose is to determine whether preservation of an internal empty HTTP path segment is suitable for a new, minimal HACP Enforcement v2 request-binding conformance block.

This assessment does not activate new profile semantics, define executable vectors, or establish implementation conformance.

## 2. Candidate Boundary

The candidate representation boundary is:

```text
/a//b
```

versus:

```text
/a/b
```

The additional `/` in `/a//b` creates an internal zero-length path segment between two non-empty path segments.

The normative question is whether that empty segment remains representation-significant for HACP HTTP request binding.

## 3. Standards Basis

RFC 3986 defines a path as a sequence of path segments separated by `/`.

Its grammar defines:

```text
segment = *pchar
```

which permits a segment of zero length.

Accordingly, `/a//b` and `/a/b` are syntactically distinguishable path representations: the former contains an internal empty segment and the latter does not.

RFC 3986 does not establish a general rule requiring adjacent path delimiters to be collapsed.

RFC 9110 likewise does not require a single universal method for determining HTTP URI equivalence. HTTP components may apply different comparison or normalization strategies.

The HTTP normalization rule treating an empty path component as equivalent to `/` concerns the URI path component being empty as a whole. It does not establish equivalence between an internal empty segment in `/a//b` and its absence in `/a/b`.

Therefore, the HTTP and URI specifications do not require HACP request binding to treat these two representations as equivalent.

## 4. HACP Security Relevance

HACP HTTP request binding is an authorization boundary between an authorized action representation and an observed HTTP request target.

Introducing an implicit transformation:

```text
/a//b -> /a/b
```

would expand the set of request-target representations accepted under the same authorization.

Such expansion requires an explicit normative basis.

Where different enforcement, proxy, routing, or application layers interpret or normalize path representations differently, implicit slash collapsing can create a normalization disagreement across the authorization boundary.

The HACP safety model therefore favors preserving the distinction unless equivalence is explicitly defined.

Loss of representation certainty must not silently broaden authorization.

## 5. Proposed Normative Direction

The assessment supports the following narrow semantic direction:

```text
An internal empty path segment is representation-significant
for HACP HTTP request binding.
```

Consequently:

```text
/a//b == /a//b
```

and:

```text
/a//b != /a/b
/a/b  != /a//b
```

for the specific boundary under consideration.

This is a request-binding representation rule, not a claim that all HTTP applications necessarily map the two paths to different resources.

## 6. Deliberate Scope Boundary

This assessment does not define semantics for:

```text
leading double slash
trailing empty path segments
more than one consecutive empty path segment
dot segments
percent-encoded slash
percent-decoded path comparison
filesystem path normalization
application router normalization
URI reference resolution
query normalization
fragment handling
scheme or authority normalization
general URI normalization
```

In particular, it does not determine the semantics of:

```text
//a
/a/
/a//
/a///b
/a/./b
/a/x/../b
/a%2Fb
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
```

The proposed HC2-E boundary addresses a distinct structural condition:

```text
absence of an internal empty path segment
!=
presence of an internal empty path segment
```

It does not modify or broaden the semantics established by HC2 through HC2-D.

## 8. Minimal Conformance Shape

If the normative direction is accepted into the Enforcement v2 draft profile, the smallest symmetric conformance matrix is expected to be:

```text
/a//b == /a//b
ALLOW

/a//b != /a/b
DENY / SCOPE_EXCEEDED

/a/b != /a//b
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

If the existing implementation passes the eventual HC2-E vectors, production code must remain unchanged.

## 10. Assessment Conclusion

Internal empty path segment preservation is suitable for a minimal, independently testable HACP Enforcement v2 request-binding conformance block.

The boundary has:

* clear URI-syntax grounding;
* direct authorization relevance;
* low normative ambiguity;
* low scope-expansion risk when restricted to an internal empty segment;
* no dependency on a general URI-normalization policy.

The recommended next step is to review a narrowly scoped HC2-E normative subsection before modifying `profiles/enforcement-v2-draft.md`.

No profile, vector, harness, baker, or production implementation change is authorized by this assessment alone.
