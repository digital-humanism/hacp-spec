# HC2-J Multiple Consecutive Empty Path Segment Normative Assessment

## Status

Normative assessment for Enforcement v2 draft request-target binding.

This assessment is intentionally narrow.

It defines only whether a difference in the number of consecutive internal empty path segments is significant for HACP request binding.

It does not define general slash normalization, URI-reference processing, path cleaning, dot-segment processing, router semantics, or general URI normalization.

---

## Question

Should the following request-target representations be considered binding-equivalent?

```text
/a///b
/a//b
```

The only difference is the number of consecutive literal `/` delimiters between the same non-empty path segments.

---

## Normative Result

No.

For HACP request binding:

```text
/a///b
```

and:

```text
/a//b
```

are distinct request-target representations.

Therefore:

```text
/a///b != /a//b
```

The distinction is symmetric.

A request-target constraint bound to:

```text
/a///b
```

MUST NOT authorize:

```text
/a//b
```

solely because an HTTP framework, router, proxy, middleware component, filesystem abstraction, or downstream application may collapse repeated path delimiters.

Likewise, a constraint bound to:

```text
/a//b
```

MUST NOT authorize:

```text
/a///b
```

without an explicit HACP equivalence rule defining such normalization.

No such equivalence is defined by this assessment.

---

## Representation Model

Within the HC2-J canonical boundary, each literal `/` path delimiter contributes to the bound request-target representation.

For:

```text
/a//b
```

there is one internal empty path segment between `a` and `b`.

For:

```text
/a///b
```

there are two consecutive internal empty path segments between the same non-empty segments.

HC2-J treats that difference as representation-significant.

This assessment does not require HACP to construct a general parsed path-segment model.

The segment terminology is used only to describe the representation difference being tested.

---

## Rationale

HACP authorization must not silently widen a request-target constraint by applying path normalization that has not been explicitly defined by the profile.

Collapsing:

```text
/a///b
```

to:

```text
/a//b
```

would remove one literal path delimiter and one corresponding empty path-segment position.

That transformation would constitute normalization rather than comparison of the representations as bound.

Different HTTP stacks may handle repeated path delimiters differently.

Some may preserve them.

Some may collapse them.

Some may normalize them before routing.

Some applications may assign different resources or behavior to the resulting paths.

HACP MUST NOT derive authorization equivalence from those implementation-specific behaviors unless an explicit HACP rule defines the equivalence.

The conservative rule is therefore to preserve the multiplicity of consecutive internal empty path segments.

---

## Relationship to HC2-E

HC2-E established that:

```text
/a//b != /a/b
```

for request binding.

That boundary verifies preservation of one internal empty path segment.

HC2-J addresses a separate question:

```text
/a///b ? /a//b
```

HC2-J establishes that the multiplicity of consecutive internal empty path segments is also significant.

Accordingly:

```text
/a///b != /a//b
```

No broader slash-normalization rule follows from either HC2-E or HC2-J.

---

## Relationship to HC2-F

HC2-F established preservation of a trailing empty path segment.

For example:

```text
/a/ != /a
```

HC2-J does not extend that rule to multiple trailing empty path segments.

Representations such as:

```text
/a//
/a///
```

remain outside the scope of this assessment.

They require separate normative treatment if later selected.

---

## Canonical Boundary

The canonical HC2-J comparison is:

```text
constraint:     /a///b
request_target: /a//b
```

Expected result:

```text
DENY
SCOPE_EXCEEDED
```

The reverse direction is independently significant:

```text
constraint:     /a//b
request_target: /a///b
```

Expected result:

```text
DENY
SCOPE_EXCEEDED
```

Exact representation remains binding-equivalent to itself:

```text
/a///b == /a///b
```

No broader equivalence follows from these statements.

---

## Scope

HC2-J defines only preservation of the number of consecutive internal empty path segments between otherwise identical non-empty path segments.

Specifically, it answers only:

```text
/a///b ? /a//b
```

and establishes:

```text
/a///b != /a//b
```

symmetrically.

---

## Explicitly Out of Scope

This assessment does not define semantics for the following classes.

### Multiple trailing empty path segments

For example:

```text
/a//
/a///
```

HC2-J does not define whether these representations are equivalent.

### Leading repeated slashes

For example:

```text
//a
///a
```

HC2-J does not define their request-binding relationship.

This exclusion is deliberate.

Leading repeated slashes may interact with URI-reference or authority-like interpretation in some contexts and therefore require separate assessment.

### Root and empty-path representation

For example:

```text
/
```

versus an empty path representation.

HC2-J does not define that boundary.

### Percent-encoded slash representation

For example:

```text
/a/%2Fb
/a//b
```

HC2-J does not define percent-decoding or encoded-delimiter equivalence.

Existing explicit Enforcement v2 rules remain authoritative for previously defined encoded-delimiter boundaries.

### Dot-segment processing

For example:

```text
/a/./b
/a/b
```

or:

```text
/a/x/../b
/a/b
```

HC2-J does not define dot-segment normalization.

### Dot segments combined with empty segments

For example:

```text
/a//./b
/a///b
/a//x/../b
```

No equivalence is defined.

### Router or framework path cleaning

HC2-J does not adopt the normalization behavior of any specific:

- HTTP server;
- reverse proxy;
- router;
- framework;
- middleware stack;
- filesystem;
- application runtime.

### Scheme or authority processing

HC2-J applies only to the selected request-target path representation boundary.

It does not define URI parsing outside that boundary.

### General slash normalization

No rule such as:

```text
multiple "/" → single "/"
```

is introduced.

### General URI normalization

No general URI normalization claim follows from HC2-J.

---

## Security Considerations

Repeated path delimiters are handled inconsistently across HTTP stacks.

If HACP were to collapse repeated delimiters during authorization while a downstream component preserved them, authorization could be evaluated against a different resource representation from the one actually processed.

The inverse mismatch is also undesirable.

A downstream component may normalize repeated delimiters while another intermediary preserves them, producing inconsistent interpretation across layers.

HACP therefore MUST NOT introduce implicit repeated-delimiter normalization at the authorization boundary unless that behavior is explicitly standardized by the applicable HACP profile.

Preserving representation multiplicity minimizes the risk of authorization widening caused by parser or router disagreement.

---

## Compatibility With Existing Enforcement v2 Rules

HC2-J does not alter any existing explicit equivalence.

In particular:

- percent-triplet hexadecimal digit case equivalence remains unchanged;
- encoded-delimiter preservation remains unchanged;
- internal empty path segment preservation remains unchanged;
- trailing empty path segment preservation remains unchanged;
- percent-encoded unreserved representation preservation remains unchanged;
- query-component ordering preservation remains unchanged;
- query empty-value delimiter preservation remains unchanged.

HC2-J adds one narrow rule:

> A difference in the number of consecutive internal empty path segments is significant for request binding.

No production implementation behavior is assumed by this assessment.

Implementation conformance must be established independently through executable vectors and black-box evaluation.

---

## Expected Conformance Shape

The minimum executable conformance matrix contains:

```text
ENF-HC2-J-001

constraint:     /a///b
request_target: /a///b
expected:       ALLOW
```

```text
ENF-HC2-J-002

constraint:     /a///b
request_target: /a//b
expected:       DENY / SCOPE_EXCEEDED
```

```text
ENF-HC2-J-003

constraint:     /a//b
request_target: /a///b
expected:       DENY / SCOPE_EXCEEDED
```

This matrix is intentionally limited to the HC2-J canonical boundary.

Additional repeated-slash or empty-segment forms require separate assessment.

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

If the existing implementation passes the HC2-J vectors without modification:

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
/a///b != /a//b
```

The distinction is symmetric.

The number of consecutive internal empty path segments is representation-significant.

No general slash normalization is introduced.

No dot-segment processing semantics are introduced.

No general URI normalization semantics are introduced.
