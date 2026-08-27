# HC2-K Multiple Trailing Empty Path Segment Normative Assessment

## Status

Normative assessment for Enforcement v2 draft request-target binding.

This assessment is intentionally narrow.

It defines only whether a difference in the number of trailing empty path segments is significant for HACP request binding.

It does not define general slash normalization, leading repeated slash semantics, root versus empty-path semantics, dot-segment processing, router path cleaning, or general URI normalization.

---

## Question

Should the following request-target representations be considered binding-equivalent?

```text
/a//
/a/
```

The only difference is the number of trailing literal `/` delimiters after the same non-empty path segment.

---

## Normative Result

No.

For HACP request binding:

```text
/a//
```

and:

```text
/a/
```

are distinct request-target representations.

Therefore:

```text
/a// != /a/
```

The distinction is symmetric.

A request-target constraint bound to:

```text
/a//
```

MUST NOT authorize:

```text
/a/
```

solely because an HTTP framework, router, proxy, middleware component, filesystem abstraction, or downstream application may collapse repeated trailing path delimiters.

Likewise, a constraint bound to:

```text
/a/
```

MUST NOT authorize:

```text
/a//
```

without an explicit HACP equivalence rule defining such normalization.

No such equivalence is defined by this assessment.

---

## Representation Model

Within the HC2-K canonical boundary, trailing literal `/` delimiters remain part of the bound request-target representation.

For:

```text
/a/
```

there is one trailing empty path segment after `a`.

For:

```text
/a//
```

there are two trailing empty path segments after the same non-empty segment.

HC2-K treats that difference as representation-significant.

This assessment does not require HACP to construct a general parsed path-segment model.

The segment terminology is used only to describe the selected representation boundary.

---

## Rationale

HACP authorization must not silently widen a request-target constraint by applying trailing-slash normalization that has not been explicitly defined by the profile.

Collapsing:

```text
/a//
```

to:

```text
/a/
```

would remove one literal path delimiter and one corresponding trailing empty path-segment position.

That transformation is normalization, not simple representation comparison.

Different HTTP stacks may:

- preserve repeated trailing delimiters;
- collapse them;
- redirect them;
- normalize them before routing;
- treat the resulting resources differently.

HACP MUST NOT derive authorization equivalence from those implementation-specific behaviors unless an explicit HACP rule defines the equivalence.

The conservative rule is therefore to preserve the multiplicity of trailing empty path segments.

---

## Relationship to HC2-F

HC2-F established that:

```text
/a/ != /a
```

for request binding.

That boundary verifies preservation of one trailing empty path segment.

HC2-K addresses the next separate question:

```text
/a// ? /a/
```

HC2-K establishes that the multiplicity of trailing empty path segments is also significant.

Accordingly:

```text
/a// != /a/
```

No broader trailing-slash normalization rule follows from either HC2-F or HC2-K.

---

## Relationship to HC2-J

HC2-J established:

```text
/a///b != /a//b
```

for consecutive internal empty path segments.

HC2-K does not extend that rule generically to all slash multiplicity.

It addresses only trailing empty path segments in the canonical form:

```text
/a// ? /a/
```

Internal and trailing empty-segment multiplicity remain separate audited boundaries.

---

## Canonical Boundary

The canonical HC2-K comparison is:

```text
constraint:     /a//
request_target: /a/
```

Expected result:

```text
DENY
SCOPE_EXCEEDED
```

The reverse direction is independently significant:

```text
constraint:     /a/
request_target: /a//
```

Expected result:

```text
DENY
SCOPE_EXCEEDED
```

Exact representation remains binding-equivalent to itself:

```text
/a// == /a//
```

No broader equivalence follows from these statements.

---

## Scope

HC2-K defines only preservation of the number of trailing empty path segments after an otherwise identical non-empty path segment.

Specifically, it answers only:

```text
/a// ? /a/
```

and establishes:

```text
/a// != /a/
```

symmetrically.

---

## Explicitly Out of Scope

This assessment does not define semantics for the following classes.

### More than two trailing empty path segments

For example:

```text
/a///
/a//
```

HC2-K does not define whether these representations are equivalent.

### Leading repeated slashes

For example:

```text
//a
///a
```

HC2-K does not define their request-binding relationship.

### Root and empty-path representation

For example:

```text
/
```

versus an empty path representation.

HC2-K does not define that boundary.

### Internal repeated slashes

HC2-J remains authoritative for its already-defined canonical internal boundary:

```text
/a///b != /a//b
```

HC2-K does not generalize that rule further.

### Percent-encoded slash representation

For example:

```text
/a/%2F
/a//
```

HC2-K does not define percent-decoding or encoded-delimiter equivalence.

### Dot-segment processing

For example:

```text
/a/./
/a/
```

or:

```text
/a/x/../
/a/
```

HC2-K does not define dot-segment normalization.

### Router or framework path cleaning

HC2-K does not adopt the normalization behavior of any specific:

- HTTP server;
- reverse proxy;
- router;
- framework;
- middleware stack;
- filesystem;
- application runtime.

### General trailing-slash normalization

No rule such as:

```text
multiple trailing "/" → single trailing "/"
```

is introduced.

### General slash normalization

No generic slash-collapsing rule is introduced.

### Scheme or authority processing

HC2-K applies only to the selected request-target path representation boundary.

It does not define URI parsing outside that boundary.

### General URI normalization

No general URI normalization claim follows from HC2-K.

---

## Security Considerations

Repeated trailing delimiters may be handled differently across HTTP stacks.

If HACP were to normalize:

```text
/a//
```

to:

```text
/a/
```

during authorization while a downstream component preserved the original representation, authorization could be evaluated against a different resource representation from the one actually processed.

The opposite mismatch is also possible when a downstream component normalizes a representation that an intermediary preserves.

HACP therefore MUST NOT introduce implicit trailing-delimiter normalization at the authorization boundary unless that behavior is explicitly standardized by the applicable HACP profile.

Preserving representation multiplicity reduces the risk of authorization widening caused by parser or router disagreement.

---

## Compatibility With Existing Enforcement v2 Rules

HC2-K does not alter any existing explicit equivalence.

In particular:

- percent-triplet hexadecimal digit case equivalence remains unchanged;
- encoded-delimiter preservation remains unchanged;
- internal empty path segment preservation remains unchanged;
- trailing empty path segment preservation remains unchanged;
- multiple consecutive internal empty path segment preservation remains unchanged;
- percent-encoded unreserved representation preservation remains unchanged;
- query-component ordering preservation remains unchanged;
- query empty-value delimiter preservation remains unchanged.

HC2-K adds one narrow rule:

> A difference in the number of trailing empty path segments in the HC2-K canonical boundary is significant for request binding.

No production implementation behavior is assumed by this assessment.

Implementation conformance must be established independently through executable vectors and black-box evaluation.

---

## Expected Conformance Shape

The minimum executable conformance matrix contains:

```text
ENF-HC2-K-001

constraint:     /a//
request_target: /a//
expected:       ALLOW
```

```text
ENF-HC2-K-002

constraint:     /a//
request_target: /a/
expected:       DENY / SCOPE_EXCEEDED
```

```text
ENF-HC2-K-003

constraint:     /a/
request_target: /a//
expected:       DENY / SCOPE_EXCEEDED
```

This matrix is intentionally limited to the HC2-K canonical boundary.

Additional trailing-slash forms require separate assessment.

---

## Implementation Discipline

This normative assessment does not justify a production-code change by itself.

The required workflow remains:

```text
normative invariant
→ profile rule
→ executable vectors
→ vector integrity validation
→ black-box evaluation
→ RED only if defect exists
→ minimal production fix only if required
→ GREEN
→ regression
```

If the existing implementation passes the HC2-K vectors without modification:

```text
PASS
→ production code not changed
```

A failing golden vector MUST first be investigated for:

- fixture integrity;
- cryptographic validity;
- harness behavior;
- runner behavior;
- request-context construction;

before being treated as evidence of a request-binding implementation defect.

Signed fixture content MUST NOT be modified without preserving the originally signed content or producing a corresponding valid signature.

---

## Normative Conclusion

For Enforcement v2 request-target binding:

```text
/a// != /a/
```

The distinction is symmetric.

The number of trailing empty path segments is representation-significant within the HC2-K canonical boundary.

No general trailing-slash normalization is introduced.

No general slash normalization is introduced.

No dot-segment processing semantics are introduced.

No general URI normalization semantics are introduced.
