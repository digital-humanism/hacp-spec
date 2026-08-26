# HC2-G — Percent-Encoded Unreserved Representation Preservation — Normative Assessment

## Status

Proposed normative assessment for the HACP Enforcement v2 draft.

This assessment defines a narrow HTTP request-target binding boundary following completion of HC2-F.

It does not activate a profile rule by itself and does not establish general URI normalization semantics.

## Context

The HC2 through HC2-F request-binding blocks established a representation-sensitive authorization model for HTTP request targets.

The verified surface preserves distinctions including:

```text
/a/b != /a%2Fb

/x != /x?

/a//b != /a/b

/a/ != /a
```

while permitting explicitly defined binding equivalence for hexadecimal letter case inside a syntactically valid percent-encoding triplet:

```text
%2F == %2f
```

HC2-G examines a different boundary:

```text
literal RFC 3986 unreserved character
vs
its corresponding single percent-encoded US-ASCII octet representation
```

Canonical probe:

```text
/a/~b
vs
/a/%7Eb
```

## Standards basis

RFC 3986 §2.3 defines the unreserved character set as:

```text
ALPHA / DIGIT / "-" / "." / "_" / "~"
```

RFC 3986 states that URIs differing only by replacement of an unreserved character with its corresponding percent-encoded US-ASCII octet are equivalent at the URI resource-equivalence level.

RFC 3986 §6.2.2.2 further defines decoding percent-encoded unreserved octets as a percent-encoding normalization operation.

RFC 9110 §4.2.3 preserves this model for HTTP URI normalization and comparison and gives forms such as `~smith` and `%7Esmith` as equivalent HTTP URI representations.

These standards define URI and HTTP resource-equivalence semantics.

They do not require every HTTP component, and do not require HACP authorization binding, to treat every URI-equivalent representation as authorization-binding-equivalent.

## Normative distinction

HACP request-target binding serves a different purpose from URI resource comparison.

URI comparison can ask:

```text
Do these URI representations identify an equivalent resource?
```

HACP authorization request-target binding asks:

```text
Is the observed request-target representation
binding-equivalent to the representation authorized
by the HACP scope under the explicitly defined
HACP request-binding rules?
```

These are distinct relations.

Therefore:

```text
URI resource equivalence
does not automatically establish
HACP authorization-binding equivalence.
```

## HC2-G invariant

For HACP HTTP request-target binding, a literal RFC 3986 unreserved character and its corresponding single percent-encoded US-ASCII octet representation are distinct unless an HACP rule explicitly defines an applicable binding equivalence.

Canonical case:

```text
/a/~b != /a/%7Eb
```

The distinction is symmetric.

Therefore:

```text
authorized: /a/~b
observed:   /a/%7Eb

→ DENY / SCOPE_EXCEEDED
```

and:

```text
authorized: /a/%7Eb
observed:   /a/~b

→ DENY / SCOPE_EXCEEDED
```

## Security rationale

Treating literal and percent-encoded unreserved representations as binding-equivalent would introduce a new transformation class into the HACP request-binding comparator.

Existing HC2 percent-encoding equivalence permits hexadecimal letter case differences inside the same syntactically valid percent-triplet:

```text
%7E == %7e
```

That rule preserves the percent-encoded representation itself.

By contrast:

```text
%7E
→
~
```

requires percent-decoding normalization and changes the request-target representation.

HC2-G does not authorize that transformation.

Where a representation difference is not covered by an explicitly defined HACP binding-equivalence rule, the difference remains authorization-significant.

The resulting failure mode is deliberately fail-closed:

```text
representation difference
not covered by an explicit HACP binding-equivalence rule
→ DENY
```

rather than:

```text
representation difference
→ inferred normalization
→ possible authorization widening
```

## Relationship to RFC unreserved semantics

HC2-G does not dispute RFC 3986 URI resource equivalence.

The following statements can both be true:

```text
RFC URI semantics:
    /a/~b and /a/%7Eb are resource-equivalent.

HACP authorization-binding semantics:
    /a/~b and /a/%7Eb are distinct.
```

HACP therefore maintains an authorization-binding equivalence relation that can be stricter than URI resource equivalence.

## Relationship to percent-triplet hexadecimal case

HC2-G does not alter the previously established rule that hexadecimal letters inside a valid percent-triplet are case-insensitive for HACP request-target binding.

For example:

```text
/a/%7Eb == /a/%7eb
```

can remain binding-equivalent under the existing percent-triplet hexadecimal-case rule.

This does not imply:

```text
/a/%7Eb == /a/~b
```

Hexadecimal spelling equivalence and percent-decoding equivalence are separate operations.

## Period character and dot-segment boundary

RFC 3986 defines the period character as part of the unreserved set.

Therefore, HC2-G applies the same representation-preservation rule to `.` as to other RFC 3986 unreserved characters.

Accordingly:

```text
/a/%2E/b != /a/./b
```

for HACP request-target binding.

This result defines only the representation relationship between encoded and literal period forms.

It does not define dot-segment processing.

In particular, HC2-G does not establish:

```text
/a/./b ? /a/b
```

or:

```text
/a/x/../b ? /a/b
```

and it does not define whether any future dot-segment processing operates on percent-encoded dot representations.

Dot-segment semantics remain separately deferred.

## Explicit non-claims

HC2-G does not establish:

```text
general URI normalization
```

It does not establish:

```text
general percent-decoding equivalence
```

It does not establish:

```text
reserved-character decoding equivalence
```

It does not modify the existing distinction:

```text
/a/b != /a%2Fb
```

It does not define:

```text
dot-segment removal
encoded dot-segment processing
recursive percent decoding
UTF-8 percent-encoding equivalence
Unicode normalization
query parameter ordering
query value normalization
duplicate-parameter semantics
empty parameter-name semantics
empty parameter-value semantics
+ vs %20
scheme normalization
authority normalization
host normalization
port normalization
router-specific normalization
framework-specific normalization
general URI normalization conformance
```

## Scope boundary

HC2-G defines only an HACP authorization request-target binding boundary.

It does not require HACP to reproduce the normalization behavior of an HTTP client, reverse proxy, router, framework, gateway, application server, or downstream resource resolver.

No broader URI-normalization conformance claim follows from HC2-G.

## Minimal conformance surface

A minimal canonical executable matrix for this invariant is:

```text
/a/~b == /a/~b
ALLOW

/a/~b != /a/%7Eb
DENY / SCOPE_EXCEEDED

/a/%7Eb != /a/~b
DENY / SCOPE_EXCEEDED
```

This matrix intentionally does not duplicate existing hexadecimal-case coverage.

It is a canonical probe for the HC2-G invariant and, by itself, does not establish exhaustive executable coverage for every member of the RFC 3986 unreserved character class.

Any broader representative or category-complete vector coverage is a separate conformance-design decision.

## Architectural result

HC2-G establishes an explicit boundary between:

```text
Internet URI resource-equivalence semantics
```

and:

```text
HACP authorization request-target binding equivalence
```

The governing principle is:

```text
URI equivalence alone is insufficient
to establish HACP authorization-binding equivalence.
```

Accordingly, when two request-target representations are URI-equivalent but are not explicitly defined as HACP binding-equivalent, the distinction remains security-relevant.

## Assessment

The proposed HC2-G invariant is:

```text
/a/~b != /a/%7Eb
```

with symmetric mismatch behavior:

```text
DENY / SCOPE_EXCEEDED
```

The same representation-preservation rule applies to the period character:

```text
/a/%2E/b != /a/./b
```

without defining dot-segment processing.

This decision preserves the existing representation-sensitive HACP request-binding model, avoids implicitly introducing percent-decoding normalization into the authorization comparator, and keeps dot-segment processing and general URI-normalization semantics outside the scope of this block.
