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

## HC2 Vector Inventory

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

## HC2-B — Invalid and Boundary Percent-Triplet Semantics

HC2-B defines the comparison boundary for invalid or incomplete percent
sequences without introducing additional URI normalization.

Only valid percent-encoded triplets (`%HH`, where both `H` characters are
ASCII hexadecimal digits) receive hexadecimal case equivalence. Invalid or
incomplete percent sequences compare literally and case-sensitively, while
subsequent characters remain subject to the comparison rules independently.

These rules apply to both the path and query portions of the request-binding
representation.

### HC2-B Vector Inventory

| Vector          | Type     | Constraint | Request target | Expected                |
| --------------- | -------- | ---------- | -------------- | ----------------------- |
| `ENF-HC2-B-001` | golden   | `/a%`      | `/a%`          | `ALLOW`                 |
| `ENF-HC2-B-002` | golden   | `/a%2`     | `/a%2`         | `ALLOW`                 |
| `ENF-HC2-B-003` | negative | `/a%2F`    | `/a%2`         | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-B-004` | negative | `/a%2G`    | `/a%2g`        | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-B-005` | golden   | `/a%2G`    | `/a%2G`        | `ALLOW`                 |
| `ENF-HC2-B-006` | golden   | `/x?q=%2F` | `/x?q=%2f`     | `ALLOW`                 |
| `ENF-HC2-B-007` | negative | `/x?q=%2G` | `/x?q=%2g`     | `DENY / SCOPE_EXCEEDED` |
| `ENF-HC2-B-008` | golden   | `/x?q=%`   | `/x?q=%`       | `ALLOW`                 |
| `ENF-HC2-B-009` | golden   | `/a%2G%2F` | `/a%2G%2f`     | `ALLOW`                 |

### HC2-C encoded delimiter preservation

Verified black-box result:

```text
ENF-HC2-C-001 PASS
ENF-HC2-C-002 PASS
ENF-HC2-C-003 PASS

RESULTS: 3/3 passed
```

HC2-C verifies that a percent-encoded question mark in the path remains
encoded request-target data and does not compare equal to the literal
path/query delimiter.

These results do not imply general URI normalization or broader
reserved-character equivalence conformance.

### HC2-D empty query delimiter preservation

Verified black-box result:

```text
ENF-HC2-D-001 PASS
ENF-HC2-D-002 PASS
ENF-HC2-D-003 PASS

RESULTS: 3/3 passed
```

HC2-D verifies that the absence of a query component does not compare
equal to a request-target containing a literal `?` delimiter followed
by an empty query component.

These results do not define broader query normalization or equivalence
semantics.

### HC2-E internal empty path segment preservation

The following cases verify that an internal empty path segment is representation-significant for HACP HTTP request binding.

```text
ENF-HC2-E-001 PASS
ENF-HC2-E-002 PASS
ENF-HC2-E-003 PASS

RESULTS: 3/3 passed
```

The existing implementation conforms to the HC2-E request-binding invariant without production changes.

These results do not define normalization or equivalence semantics for leading or trailing empty path segments, multiple consecutive empty path segments, dot-segments, percent-encoded delimiters, or general URI normalization.

### HC2-F trailing empty path segment preservation

The following cases verify that a trailing empty path segment following a non-empty path segment is representation-significant for HACP HTTP request binding.

```text
ENF-HC2-F-001 PASS
ENF-HC2-F-002 PASS
ENF-HC2-F-003 PASS

RESULTS: 3/3 passed
```

The existing implementation conforms to the HC2-F request-binding invariant without production changes.

These results do not define normalization or equivalence semantics for the empty path component as a whole, the root path, leading empty path segments, multiple consecutive empty path segments, dot-segments, percent-encoded delimiters, or general URI normalization.

### HC2-G percent-encoded unreserved representation preservation

The following cases verify that RFC 3986 unreserved characters remain
representation-significant for HACP HTTP request-target binding when
compared with their corresponding single percent-encoded US-ASCII octet
representations.

```text
ENF-HC2-G-001 PASS
ENF-HC2-G-002 PASS
ENF-HC2-G-003 PASS
ENF-HC2-G-004 PASS
ENF-HC2-G-005 PASS
ENF-HC2-G-006 PASS
ENF-HC2-G-007 PASS
ENF-HC2-G-008 PASS

RESULTS: 8/8 passed
```

The canonical tilde cases verify exact-match behavior and symmetric
literal-versus-percent-encoded mismatch behavior. The remaining cases
provide representative coverage across the RFC 3986 unreserved
categories: alphabetic characters, digits, hyphen, underscore, and
period.

The existing implementation conforms to the HC2-G request-binding
invariant without production changes.

These results provide representative category coverage and do not
constitute exhaustive testing of every unreserved character. They do not
define dot-segment processing, general percent-decoding equivalence, or
general URI normalization conformance.

### HC2-H query-component ordering preservation

The following cases verify that query-component ordering remains
representation-significant for HACP HTTP request-target binding unless an
explicit equivalence rule applies.

```text
ENF-HC2-H-001 PASS
ENF-HC2-H-002 PASS
ENF-HC2-H-003 PASS

RESULTS: 3/3 passed
```

The exact-ordering case verifies exact representation equality. The remaining
cases verify symmetric rejection of reordered query-component representations.

The existing implementation conforms to the HC2-H request-binding invariant
without production changes.

Initial verification exposed an invalid conformance-vector construction in
which signed IntentEnvelope content had been modified without a corresponding
signature update. The vectors were corrected by restoring the signed envelope
content, after which all HC2-H cases passed against the unchanged
implementation.

These results do not define query parameter parsing, duplicate-parameter
semantics, first-value or last-value selection, form encoding, query sorting,
percent-decoding equivalence, or general query normalization.

### HC2-I query empty-value delimiter preservation

The following cases verify that the presence or absence of the literal `=`
delimiter in an otherwise identical query-component representation is
significant for HACP HTTP request binding.

```text
ENF-HC2-I-001 PASS
ENF-HC2-I-002 PASS
ENF-HC2-I-003 PASS

RESULTS: 3/3 passed
```

The existing implementation conforms to the HC2-I request-binding invariant
without production changes.

HC2-I verifies only the representation distinction between `/x?a` and
`/x?a=`. It does not define query-parameter parsing, empty query-name
semantics, duplicate-field semantics, `+` versus `%20`, form-encoding
semantics, query percent-decoding, query canonicalization, or general URI
normalization.

### HC2-J multiple consecutive empty path segment preservation

The following cases verify that the multiplicity of consecutive internal empty
path segments is significant for HACP HTTP request binding.

```text
ENF-HC2-J-001 PASS
ENF-HC2-J-002 PASS
ENF-HC2-J-003 PASS

RESULTS: 3/3 passed
```

The existing implementation conforms to the HC2-J request-binding invariant
without production changes.

HC2-J verifies only the representation distinction between `/a///b` and
`/a//b`. It does not define multiple trailing empty path segment semantics,
leading repeated slash semantics, percent-encoded slash equivalence,
dot-segment processing, router or framework path cleaning, general slash
normalization, or general URI normalization.

## Verification Status

The HC2, HC2-B, HC2-C, HC2-D, HC2-E, HC2-F, HC2-G, HC2-H, HC2-I, and HC2-J vectors in this directory have been deterministically baked
and verified through the Enforcement v2 conformance path.

Current verified results:

* HC2 baseline: `8/8 passed`
* HC2-B boundary semantics: `9/9 passed`
* HC2-C encoded delimiter preservation: `3/3 passed`
* HC2-D empty query delimiter preservation: `3/3 passed`
* HC2-E internal empty path segment preservation: `3/3 passed`
* HC2-F trailing empty path segment preservation: `3/3 passed`
* HC2-G percent-encoded unreserved representation preservation: `8/8 passed`
* HC2-H query-component ordering preservation: `3/3 passed`
* HC2-I query empty-value delimiter preservation: `3/3 passed`
* HC2-J multiple consecutive empty path segment preservation: `3/3 passed`

Total verified request-binding cases: `46`

These results demonstrate only the request-binding semantics explicitly
defined by the current Enforcement v2 draft. They do not imply general URI
normalization conformance.

The Enforcement v2 profile and all vectors in this directory remain draft and
MUST NOT be included in claims against the current HACP-Core canonical
conformance set or its published manifest.
