# HC2-H — Query-Component Ordering Preservation — Normative Assessment

## Status

Proposed normative assessment for the HACP Enforcement v2 draft.

This assessment defines a narrow HTTP request-target binding boundary
following completion of HC2-G.

It does not activate a profile rule by itself, does not define a
universal query-parameter model, and does not establish general URI
normalization semantics.

## Context

The HC2 through HC2-G request-binding blocks establish a
representation-sensitive authorization model for HTTP request targets.

The verified surface preserves distinctions including:

```text
/a/b != /a%2Fb

/x != /x?

/a//b != /a/b

/a/ != /a

/a/~b != /a/%7Eb
```

while permitting only explicitly defined HACP binding equivalences,
including hexadecimal letter case inside a syntactically valid
percent-encoding triplet:

```text
%2F == %2f
```

HC2-H examines a different boundary:

```text
ordering within the query-component representation
```

Canonical probe:

```text
/x?a=1&b=2
vs
/x?b=2&a=1
```

The apparent use of `&` and `=` resembles common application-level
query-parameter syntax.

The question for HC2-H is not whether a particular downstream
application interprets the two forms as semantically equivalent.

The question is:

```text
Does HACP infer unordered parameter-map equivalence
from those query-component representations?
```

## Standards basis

RFC 3986 §3.4 defines the URI query component as:

```text
query = *( pchar / "/" / "?" )
```

The generic URI syntax defines the query as a component of the URI.

Although query data is commonly used by applications to encode
name/value information, RFC 3986 does not define `&` and `=` as a
universal unordered parameter-map model for all URI queries.

RFC 9110 adopts the RFC 3986 `query` production for HTTP URI syntax.

RFC 9110 does not establish a general HTTP rule under which reordering
application-looking query data creates an equivalent HTTP
request-target representation.

RFC 9421 provides a useful security-protocol distinction between the
entire query component and application-level query parameters.

Its `@query` derived component represents the query component as a
whole and applies URI simple-string comparison semantics.

Its separate `@query-param` mechanism applies only when the query uses
the specific HTML form parameter format and introduces additional
parameter-processing rules.

This distinction is informative for HACP:

```text
query-component representation
```

and:

```text
application-level parsed parameters
```

are different abstraction layers.

These standards do not require HACP authorization binding to infer
application-level parameter equivalence from the representation of the
query component.

## Normative distinction

HACP request-target binding operates on the authorized and observed
request-target representation under explicitly defined HACP
binding-equivalence rules.

It does not implicitly reinterpret application-looking query syntax as
an unordered parameter map.

Therefore:

```text
application-level query interpretation
does not automatically establish
HACP authorization-binding equivalence.
```

This follows the same architectural discipline established by HC2-G:

```text
URI resource equivalence
does not automatically establish
HACP authorization-binding equivalence.
```

HC2-H applies that discipline to a different abstraction boundary.

## HC2-H invariant

For HACP HTTP request-target binding, ordering within the query-component
representation is authorization-significant unless an HACP rule
explicitly defines an applicable binding equivalence.

HACP does not infer unordered parameter-map equivalence from
application-looking query syntax.

Canonical case:

```text
/x?a=1&b=2 != /x?b=2&a=1
```

The distinction is symmetric.

Therefore:

```text
authorized: /x?a=1&b=2
observed:   /x?b=2&a=1

→ DENY / SCOPE_EXCEEDED
```

and:

```text
authorized: /x?b=2&a=1
observed:   /x?a=1&b=2

→ DENY / SCOPE_EXCEEDED
```

Exact representation remains binding-equivalent:

```text
authorized: /x?a=1&b=2
observed:   /x?a=1&b=2

→ ALLOW
```

subject to all other applicable HACP checks.

## Security rationale

Treating reordered application-looking query data as automatically
binding-equivalent would require HACP to introduce a query
interpretation model that is not defined by the generic URI or HTTP
syntax alone.

For example, an unordered comparison would first need to infer that:

```text
&
```

acts as a parameter separator and:

```text
=
```

acts as a name/value separator.

It would then need to establish that ordering is semantically
irrelevant.

That immediately creates further questions such as:

```text
?a=1&a=2
vs
?a=2&a=1
```

and:

```text
?a
vs
?a=
```

as well as questions about empty names, empty values, percent-decoding,
`+` handling, duplicate names, and first-wins or last-wins behavior.

Those semantics are application-level concerns and are not implied by
the generic query-component representation.

HC2-H therefore does not introduce such parsing or normalization.

Where the authorized and observed query-component representations
differ in ordering and no explicit HACP binding-equivalence rule
applies, the difference remains authorization-significant.

The resulting failure mode is deliberately fail-closed:

```text
query representation difference
not covered by an explicit HACP binding-equivalence rule
→ DENY
```

rather than:

```text
query representation difference
→ inferred application semantics
→ inferred equivalence
→ possible authorization widening
```

## Relationship to downstream application semantics

HC2-H does not claim that query ordering is semantically significant to
every downstream application.

A particular application may interpret:

```text
?a=1&b=2
```

and:

```text
?b=2&a=1
```

as equivalent.

That application-level equivalence does not automatically establish
HACP authorization-binding equivalence.

Both statements can therefore be true:

```text
downstream application semantics:
    ?a=1&b=2 and ?b=2&a=1 may be equivalent

HACP authorization-binding semantics:
    ?a=1&b=2 and ?b=2&a=1 are distinct
```

unless HACP explicitly defines an applicable binding equivalence.

## Relationship to earlier query binding

Earlier HC2 request-binding cases establish that the query component is
part of the request-target binding surface and that differing query
representations can produce a request binding mismatch.

HC2-D separately establishes preservation of the empty query delimiter:

```text
/x != /x?
```

HC2-H does not redefine those results.

It addresses only whether HACP may infer equivalence from reordering
data within a present query component.

## Explicit non-claims

HC2-H does not establish:

```text
general query normalization
```

It does not establish:

```text
query parameter parsing
```

It does not assign universal application semantics to:

```text
&
=
```

It does not define:

```text
duplicate-parameter semantics
first-wins behavior
last-wins behavior
empty parameter-name semantics
empty parameter-value semantics
?a vs ?a=
+ vs %20
percent-decoding inside query data
application/x-www-form-urlencoded semantics
query sorting or canonicalization
application-specific query normalization
general URI normalization conformance
```

HC2-H does not claim that reordered query data necessarily identifies a
different resource.

It does not claim that query ordering is semantically significant to
every downstream application.

It establishes only that HACP does not infer authorization-binding
equivalence from query reordering unless an explicit HACP
binding-equivalence rule says otherwise.

## Dot-segment boundary

HC2-H does not alter the separately deferred dot-segment question.

In particular, it does not establish:

```text
/a/./b ? /a/b
```

or:

```text
/a/x/../b ? /a/b
```

Dot-segment processing remains outside the scope of HC2-H.

## Scope boundary

HC2-H defines only an HACP authorization request-target binding
boundary.

It does not require HACP to reproduce the query parsing, parameter
ordering, normalization, or routing behavior of an HTTP client, reverse
proxy, router, framework, gateway, application server, or downstream
resource resolver.

No general query-normalization or URI-normalization conformance claim
follows from HC2-H.

## Minimal conformance surface

A minimal canonical executable matrix for this invariant is:

```text
/x?a=1&b=2 == /x?a=1&b=2
ALLOW

/x?a=1&b=2 != /x?b=2&a=1
DENY / SCOPE_EXCEEDED

/x?b=2&a=1 != /x?a=1&b=2
DENY / SCOPE_EXCEEDED
```

This matrix verifies exact representation and symmetric mismatch
behavior for the canonical ordering probe.

It intentionally does not introduce duplicate parameter names, empty
parameter names, empty parameter values, alternate separators,
form-encoding semantics, percent-decoding semantics, or application
query parsing.

Any such behavior requires a separate normative assessment.

## Architectural result

HC2-H establishes an explicit boundary between:

```text
query-component representation
```

and:

```text
application-level query interpretation
```

The governing principle is:

```text
application-level query equivalence alone is insufficient
to establish HACP authorization-binding equivalence.
```

Accordingly, when two query-component representations differ only by an
application-interpretable reordering, but no HACP rule explicitly
defines them as binding-equivalent, the distinction remains
security-relevant.

## Assessment

The proposed HC2-H invariant is:

```text
/x?a=1&b=2 != /x?b=2&a=1
```

with symmetric mismatch behavior:

```text
DENY / SCOPE_EXCEEDED
```

and exact-representation behavior:

```text
/x?a=1&b=2 == /x?a=1&b=2
```

HC2-H binds the query-component representation and does not infer an
unordered application-level parameter map from application-looking
query syntax.

This decision preserves the existing representation-sensitive HACP
request-binding model, avoids implicitly introducing application-level
query parsing into the authorization comparator, and keeps duplicate
parameter semantics, form encoding, query canonicalization,
application-specific query normalization, dot-segment processing, and
general URI-normalization semantics outside the scope of this block.