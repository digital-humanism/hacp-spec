# HC2-I Query Empty-Value Delimiter Normative Assessment

## Status

Normative assessment for Enforcement v2 draft request-target binding.

This assessment is intentionally narrow.

It does not define query-parameter parsing semantics, application-level query equivalence, form encoding, or general URI normalization.

---

## Question

Should the following request-target representations be considered binding-equivalent?

```text
/x?a
/x?a=
```

The only difference is the presence of the literal `=` character after the same query-component prefix.

---

## Normative Result

No.

For HACP request binding:

```text
/x?a
```

and:

```text
/x?a=
```

are distinct request-target representations.

Therefore:

```text
/x?a != /x?a=
```

The distinction is symmetric.

A constraint bound to:

```text
/x?a
```

MUST NOT authorize:

```text
/x?a=
```

solely because an application, framework, router, query parser, or downstream component may interpret both forms as representing an empty parameter value.

Likewise, a constraint bound to:

```text
/x?a=
```

MUST NOT authorize:

```text
/x?a
```

without an explicit HACP equivalence rule defining such normalization.

No such equivalence is defined by this assessment.

---

## Rationale

HACP request binding operates on the request-target representation rather than on an inferred application-level query-parameter model.

The representations:

```text
?a
```

and:

```text
?a=
```

are textually distinct.

Treating them as equivalent would require HACP to infer parameter semantics that are not currently part of the Enforcement v2 request-binding model.

Different HTTP frameworks, query parsers, application libraries, gateways, and downstream services may interpret query fields differently.

HACP MUST NOT derive authorization equivalence from those implementation-specific interpretations unless such equivalence is explicitly defined by the HACP profile.

This follows the same conservative request-binding principle already applied to other Enforcement v2 representation boundaries:

- encoded delimiters are not automatically treated as decoded delimiters;
- empty query delimiter presence is preserved;
- internal and trailing empty path segments are preserved;
- literal unreserved characters are not automatically equivalent to percent-encoded representations;
- query-field ordering is preserved rather than converted into unordered parameter-map equivalence.

Accordingly, the presence of the literal `=` delimiter is preserved as part of the bound request-target representation.

---

## Canonical Boundary

The canonical HC2-I comparison is:

```text
constraint:     /x?a
request_target: /x?a=
```

Expected result:

```text
DENY
SCOPE_EXCEEDED
```

The reverse direction is independently significant:

```text
constraint:     /x?a=
request_target: /x?a
```

Expected result:

```text
DENY
SCOPE_EXCEEDED
```

Exact representation remains binding-equivalent to itself:

```text
/x?a == /x?a
```

and, independently:

```text
/x?a= == /x?a=
```

No broader equivalence follows from these statements.

---

## Scope

HC2-I defines only preservation of the presence or absence of the literal `=` delimiter in an otherwise identical query-component representation.

Specifically, it answers only:

```text
/x?a ? /x?a=
```

It does not establish a general query grammar or query canonicalization algorithm.

---

## Explicitly Out of Scope

This assessment does not define semantics for:

### Empty query names

For example:

```text
/x?=a
/x?=
```

### Multiple query fields

For example:

```text
/x?a=&b
/x?a&b=
```

### Duplicate query fields

For example:

```text
/x?a=1&a=2
/x?a=2&a=1
```

No first-value, last-value, merge, list, or set semantics are defined.

### Additional empty-field representations

For example:

```text
/x?a=
/x?a=&
/x?a&&
```

No equivalence or inequality beyond the HC2-I canonical boundary is defined here.

### Plus-sign interpretation

For example:

```text
/x?q=a+b
/x?q=a%20b
```

HC2-I does not interpret `+` as space.

### `application/x-www-form-urlencoded`

No form-encoding semantics are introduced.

### Query percent-decoding

HC2-I does not define whether query characters represented literally and through percent encoding are equivalent.

Any such boundary requires separate normative assessment unless already covered by an explicit Enforcement v2 rule.

### Query sorting or application-level parameter equivalence

HC2-I does not convert a query component into a parameter map, multimap, dictionary, set, or ordered application data structure.

### General URI normalization

No general URI normalization claim follows from HC2-I.

### Dot-segment processing

Path dot-segment semantics remain independent and deferred.

Examples such as:

```text
/a/./b
/a/b
```

or:

```text
/a/x/../b
/a/b
```

are unaffected by this assessment.

---

## Security Considerations

Authorization comparison must not rely on assumptions about how a downstream component interprets superficially similar query representations.

If HACP were to collapse:

```text
?a
```

and:

```text
?a=
```

without an explicit normative rule, authorization could be evaluated against a representation different from the one actually processed by downstream software.

Even when two representations happen to be treated identically by one application stack, that behavior is not necessarily portable across:

- HTTP frameworks;
- reverse proxies;
- routers;
- middleware;
- query parsers;
- language runtimes;
- application-specific request processing.

Preserving the request-target representation avoids creating an implicit normalization boundary inside the authorization layer.

---

## Compatibility With Existing Enforcement v2 Rules

HC2-I does not alter existing explicit equivalences.

In particular, valid percent-triplet hexadecimal digit case equivalence remains unchanged.

HC2-I also does not modify previously established representation-preservation rules.

It adds one narrow rule:

> The presence or absence of the literal `=` delimiter in the HC2-I canonical query representation is significant for request binding.

No production implementation behavior is assumed by this assessment.

Implementation conformance must be established independently through executable vectors and black-box evaluation.

---

## Expected Conformance Shape

The minimum executable conformance matrix contains:

```text
ENF-HC2-I-001

constraint:     /x?a
request_target: /x?a
expected:       ALLOW
```

```text
ENF-HC2-I-002

constraint:     /x?a
request_target: /x?a=
expected:       DENY / SCOPE_EXCEEDED
```

```text
ENF-HC2-I-003

constraint:     /x?a=
request_target: /x?a
expected:       DENY / SCOPE_EXCEEDED
```

This matrix is intentionally limited to the normative boundary defined above.

Additional query representation questions require separate assessment.

---

## Implementation Discipline

This normative assessment does not justify a production-code change by itself.

The required workflow remains:

```text
normative invariant
→ executable vectors
→ black-box evaluation
→ RED only if defect exists
→ minimal production fix
→ GREEN
→ regression
```

If the existing implementation passes the HC2-I vectors without modification:

```text
PASS
→ production code not changed
```

A failing golden vector must first be investigated for vector integrity, cryptographic validity, harness behavior, and runner behavior before being treated as evidence of a request-binding implementation defect.

Signed fixture content MUST NOT be modified without either preserving the originally signed content or producing a corresponding valid signature.

---

## Normative Conclusion

For Enforcement v2 request-target binding:

```text
/x?a != /x?a=
```

The distinction is symmetric.

No application-level query-parameter equivalence is inferred.

No additional query normalization semantics are introduced.
