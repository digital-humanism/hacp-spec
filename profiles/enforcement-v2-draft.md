# HACP Enforcement Profile — Version 2 Draft

Status: Draft — not yet active
Specification version: 1.0.0
Profile revision: 2-draft
Based on: profiles/enforcement.md
Release lineage: successor candidate to the enforcement profile used by the 0.5.0 release line
Phase: HC2 normative development
Normative references:
- HACP-SPEC-0.9-draft.md
- INVARIANTS.md
- PROFILES.md
- canonicalization.md
- wire/encoding.md
- wire/crypto-profile.md
- api/decision-api.md
- error-model.md
- checkpoint-protocol.md

## 1. Scope

This profile defines normative behavior for an enforcement point that prevents execution of an agent action unless the request carries a valid HACP ALLOW decision token.

This profile defines the Enforcement scope described in this document.

Requires: HACP-Runtime (and therefore HACP-Core).

## 2. Enforcement point

The enforcement point MUST be positioned between the agent and every enforceable tool transport.

For this profile, enforceable transports are:

1. MCP tool calls carried through the sidecar.
2. HTTP requests sent through an explicit HTTP_PROXY to the sidecar.

All other transports are out of scope for this profile and MUST be treated as non-enforced unless deployment-level isolation prevents their use.

The enforcement point MUST be fail-closed.

## 3. Core invariant

A request MUST NOT be forwarded unless the enforcement point has verified all
required conditions defined by this profile, including:

- Required HACP headers are present.
- The intent envelope parses and is canonically valid.
- The decision token parses and is canonically valid.
- The token decision is ALLOW.
- The signer key is not revoked.
- The token signature is valid.
- The envelope signature is valid.
- The envelope is not revoked.
- The token is not revoked.
- The envelope is not expired.
- The token is not expired.
- The token `action_hash` matches the envelope `action_hash`.
- The token is bound to the current request.
- The scope check passes.
- The budget check passes.
- Revocation state is fresh.
- A provenance record can be appended.

This list defines required forwarding conditions and does not define their
relative verification order. Verification precedence is defined by Section 4.

If any required condition fails, the enforcement point MUST deny the request
and MUST NOT forward any part of the request payload upstream.

## 4. Verification precedence (normative)

The enforcement point MUST preserve the verification dependencies and
side-effect barriers defined by this section.

This profile does not require a single implementation-wide total order for
checks whose relative order is not otherwise specified.

### 4.1 Minimum structural validation

An enforcement point MAY perform the minimum structural validation necessary
to safely identify and evaluate subsequent enforcement prerequisites.

Structural parsing or inspection does not, by itself, make an envelope,
DecisionToken, or any claim carried by them authoritative.

### 4.2 Control-state prerequisite

When distributed control state participates in enforcement, its usability and
freshness MUST be established before authorization checks that depend on that
state.

Stale or otherwise unsafe required control state MUST prevent mutable replay or
authorization-budget consumption and MUST prevent forwarding.

### 4.3 Authentication before trust

Claims carried by an IntentEnvelope MUST NOT be treated as authoritative until
the envelope has been authenticated.

Claims carried by a DecisionToken MUST NOT be treated as authoritative until
the token has been authenticated.

This profile does not require DecisionToken signature verification and
IntentEnvelope signature verification to occur in a fixed relative order unless
another normative dependency requires such ordering.

### 4.4 DecisionToken decision authority

A DecisionToken decision MUST NOT be treated as authoritative until the token
has been authenticated and established as applicable to the relevant
authorization context.

An unauthenticated token decision MUST NOT be reported or acted upon as an
authoritative evaluator decision.

### 4.5 Checkpoint semantics

Existing checkpoint state, determination that a new checkpoint is required,
and resumption following human approval are distinct enforcement operations.

An existing checkpoint that blocks authorization MUST prevent progression to
execution authorization and MUST prevent subsequent mutable replay or
authorization-budget consumption.

`RESOLVED_ALLOW` MUST NOT itself authorize execution. Resumption after human
approval MUST follow the credential requirements defined by
`checkpoint-protocol.md`.

This profile does not otherwise define a universal relative order between an
existing checkpoint failure and every possible credential failure, nor between
a newly required checkpoint and every scope or boundary failure.

### 4.6 Mutable enforcement state

Replay and authorization-budget state MUST NOT be consumed across a prerequisite
failure that this section requires to be resolved first.

Additional ordering requirements for specific budget and replay mechanisms are
defined by Section 9.

### 4.7 Provenance and forwarding

Required provenance acceptance MUST occur before the request is forwarded.

The request MUST NOT be forwarded until all applicable required enforcement
conditions have succeeded.

### 4.8 Failure precedence

Where this profile defines one verification as a prerequisite for another,
failure of the prerequisite MUST prevent evaluation from progressing across
that dependency.

For checks whose relative order is not specified by this profile, an
implementation MAY choose its internal execution order provided that it
preserves all required fail-closed behavior, reason-code semantics,
side-effect barriers, and authorization dependencies.

## 5. Enforcement modes

| Mode | Behavior | Conformance |
|---|---|---|
| enforce | Fail-closed verification and forwarding. | Normative for this profile. |
| shadow | Logs verification results but does not deny. | Non-conformant. |
| disabled | Bypasses verification. | Non-conformant. |

The default mode MUST be `enforce`.

A conformant deployment MUST NOT silently downgrade to `shadow` or `disabled`.

## 6. Fail modes

The enforcement point MUST deny on any failure.

The following table defines normative HACP reason codes per `error-model.md`.

| Condition | Reason code |
|---|---|
| Missing required HACP header. | `INVALID_ENVELOPE` |
| Header value cannot be decoded. | `INVALID_ENVELOPE` |
| Envelope or token JSON cannot be parsed. | `INVALID_ENVELOPE` or `INVALID_ACTION` |
| Required claim is missing. | `INVALID_ENVELOPE` or `INVALID_ACTION` |
| Unsupported protocol version. | `INVALID_ENVELOPE` |
| Envelope signature invalid. | `SIGNATURE_FAILURE` |
| Token signature invalid. | `SIGNATURE_FAILURE` |
| Signer key cannot be resolved. | `SIGNATURE_FAILURE` |
| Signer key revoked. | `KEY_REVOKED` |
| Envelope expired. | `ENVELOPE_EXPIRED` |
| Token expired. | `TOKEN_EXPIRED` |
| Envelope revoked. | `ENVELOPE_REVOKED` |
| Token revoked. | `TOKEN_REVOKED` |
| Token already consumed. | `TOKEN_REVOKED` |
| Token action_hash mismatch. | `SIGNATURE_FAILURE` |
| Request binding mismatch. | `SCOPE_EXCEEDED` |
| Request method, path, or tool_name outside token scope. | `SCOPE_EXCEEDED` |
| Unknown scope attribute. | `UNKNOWN_ATTRIBUTE` |
| Request crosses declared boundary. | `BOUNDARY_CROSSING` |
| Budget exhausted. | `BUDGET_EXHAUSTED` |
| Budget ledger unavailable. | `BUDGET_EXHAUSTED` |
| Decision token is DENY. | Use token-supplied reason if present; otherwise `POLICY_DENIED`. |
| Decision token is CHECKPOINT and unresolved. | `HUMAN_REQUIRED` |
| Checkpoint not resolved before expiry. | `CHECKPOINT_TIMEOUT` |
| Provenance record cannot be appended. | `TRACEABILITY_FAILURE` |
| Revocation state is stale. | `CONTROL_STATE_STALE` |
| Control channel is unavailable beyond allowed staleness. | `CONTROL_STATE_STALE` |
| Provenance chain integrity broken. | `TRACEABILITY_FAILURE` |

The enforcement point MUST NOT invent success semantics when a failure occurs.

## 7. Token binding

An ALLOW decision token MUST be cryptographically bound to the exact proposed action via `action_hash`.

The token MAY include additional binding via the `constraints` object for request-level narrow binding.

Minimum required binding:

1. `action_hash`: SHA-256(JCS(proposed_action)) — MUST match envelope.
2. `envelope_id`: MUST match the envelope header.

Optional binding in `constraints` for enforcement:

1. `method`
2. `path`
3. `tool_name`
4. `payload_hash`

The enforcement point MUST recompute the request payload_hash over the exact request body.

For requests without a body, the payload_hash MUST be the SHA-256 hash of the empty byte string:

```text
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

### 7.1 HTTP path binding representation

When `constraints.path` is present for an HTTP request, the enforcement
point MUST compare it against the path-and-query request-binding
representation derived from the HTTP request target observed at the
enforcement boundary.

The request-binding representation MUST contain the path component and,
when a query component is present, the `?` delimiter followed by that
query component. Scheme and authority MUST NOT form part of this
representation.

Percent-encoded octets MUST remain percent-encoded during request-binding
evaluation unless this profile explicitly defines otherwise.

Hexadecimal digits within a valid percent-encoded triplet MUST compare
case-insensitively.

For request-binding comparison, a valid percent-encoded triplet is a
three-character sequence consisting of `%` followed by exactly two ASCII
hexadecimal digits (`0-9`, `A-F`, or `a-f`).

Only the two hexadecimal digits within such a valid triplet receive the
case-insensitive comparison defined above.

A `%` character that does not begin a valid percent-encoded triplet has no
special equivalence for request-binding comparison. Such a `%` character is
compared literally and case-sensitively. Characters following it remain
subject to these comparison rules independently.

The presence of an invalid or incomplete percent sequence does not, by
itself, define a separate request-binding validation failure in this profile.
If the resulting request-binding representations do not compare equal, the
request MUST be denied as a request binding mismatch with `SCOPE_EXCEEDED`.

These comparison rules apply equally to the path and query portions of the
request-binding representation. No percent-decoding, repair, or additional
URI normalization is implied.

The enforcement point MUST NOT recursively percent-decode the
request-binding representation.

An implementation MUST NOT derive HTTP path binding solely from a decoded
URI-path representation when doing so would cause distinct
execution-boundary request targets to compare equal.

No other URI normalization or equivalence is implied by these rules.

### 7.1.1 Encoded question-mark delimiter preservation

For HTTP request-binding comparison, a percent-encoded question mark in
the path remains percent-encoded request-target data.

A valid percent-encoded triplet representing `?` MUST NOT compare equal
to the literal `?` delimiter that separates the path from the query
portion of the request target.

Percent-triplet hexadecimal digits remain case-insensitive according to
the existing percent-triplet comparison rules.

Therefore:

```text
/x%3Fy == /x%3fy

/x%3Fy != /x?y
```

This rule does not perform percent-decoding and does not imply any
broader URI normalization or reserved-character equivalence.

### 7.1.2 Empty query delimiter preservation

For HTTP request-binding comparison, the presence of the literal `?`
delimiter is significant even when the query component following it is
empty.

A request-binding representation with no query component MUST NOT
compare equal to a representation containing a literal `?` delimiter
followed by an empty query component.

Therefore:

```text
/x != /x?
```

This rule does not define any broader query normalization or
equivalence semantics.

### 7.1.3 Internal empty path segment preservation

For HTTP request binding, an internal empty path segment between two non-empty path segments is representation-significant.

An implementation MUST NOT treat a request target containing such an empty path segment as equivalent to the otherwise identical request target with that segment removed.

Therefore:

```text
/a//b != /a/b
```

and the comparison is symmetric.

This rule does not define normalization or equivalence semantics for leading or trailing empty path segments, multiple consecutive empty path segments, dot-segments, percent-encoded delimiters, or any other URI normalization.

### 7.1.4 Trailing empty path segment preservation

For HTTP request binding, a trailing empty path segment following a non-empty path segment is representation-significant.

An implementation MUST NOT treat a request target containing such a trailing empty path segment as equivalent to the otherwise identical request target with that segment removed.

Therefore:

```text
/a/ != /a
```

and the comparison is symmetric.

This rule does not define normalization or equivalence semantics for the empty path component as a whole, the root path, leading empty path segments, multiple consecutive empty path segments, dot-segments, percent-encoded delimiters, or any other URI normalization.

### 7.1.5 Percent-encoded unreserved representation preservation

For HTTP request-target binding, a literal RFC 3986 unreserved character
and its corresponding single percent-encoded US-ASCII octet
representation MUST remain distinct unless this profile explicitly
defines an applicable binding equivalence.

URI resource equivalence does not, by itself, establish HACP
authorization-binding equivalence.

Therefore:

```text
/a/~b != /a/%7Eb

/a/%7Eb != /a/~b
```

A representation difference not covered by an explicit HACP
binding-equivalence rule MUST result in denial as a request binding
mismatch with `SCOPE_EXCEEDED`.

This rule applies to the period character as a member of the RFC 3986
unreserved set. Therefore:

```text
/a/%2E/b != /a/./b
```

This rule does not define dot-segment processing. In particular, it does
not establish any equivalence between:

```text
/a/./b
and
/a/b
```

or between:

```text
/a/x/../b
and
/a/b
```

Hexadecimal letter case inside a valid percent-encoded triplet remains
subject to the existing case-insensitive percent-triplet comparison
rule. Therefore:

```text
/a/%7Eb == /a/%7eb
```

No general percent-decoding, dot-segment removal, recursive decoding,
or broader URI normalization is implied by this rule.

### 7.1.6 Query-component ordering preservation

For HTTP request-target binding, ordering within the query component
MUST remain representation-significant unless this profile explicitly
defines an applicable binding equivalence.

HACP MUST NOT infer unordered parameter-map equivalence from
application-looking query syntax.

Therefore:

```text
/x?a=1&b=2 != /x?b=2&a=1
```

The distinction is symmetric.

A query-ordering difference not covered by an explicit HACP
binding-equivalence rule MUST result in denial as a request binding
mismatch with `SCOPE_EXCEEDED`.

Exact representation remains binding-equivalent:

```text
/x?a=1&b=2 == /x?a=1&b=2
```

This rule does not define query-parameter parsing, duplicate-parameter
semantics, first-wins or last-wins behavior, form-encoding semantics,
query sorting or canonicalization, percent-decoding inside query data,
or general query normalization.

### 7.1.7 Query empty-value delimiter preservation

The presence or absence of the literal `=` character in an otherwise identical query-component representation is significant for request binding.

An implementation MUST NOT infer equivalence between:

```text
/x?a
```

and:

```text
/x?a=
```

solely because an application, framework, router, query parser, or downstream component may interpret both forms as representing an empty parameter value.

Therefore:

```text
/x?a != /x?a=
```

and the distinction is symmetric.

A request-target constraint bound to:

```text
/x?a
```

MUST NOT match:

```text
/x?a=
```

unless an explicit HACP equivalence rule defines such normalization.

Likewise, a constraint bound to:

```text
/x?a=
```

MUST NOT match:

```text
/x?a
```

without such an explicit rule.

A mismatch caused solely by this representation difference MUST be treated as a request-binding mismatch and mapped to:

```text
SCOPE_EXCEEDED
```

Exact representation remains equivalent to itself.

This rule does not define query-parameter parsing or application-level query semantics.

In particular, it does not define:

* empty query-name semantics;
* duplicate query-field semantics;
* first-value or last-value behavior;
* equivalence involving additional empty fields or delimiters;
* `+` as a representation of space;
* `application/x-www-form-urlencoded` semantics;
* percent-decoding of the query component;
* query sorting or parameter-map canonicalization;
* application-specific query normalization;
* general URI normalization.

No broader query equivalence follows from this rule.

### 7.1.8 Multiple consecutive empty path segment preservation

The number of consecutive internal empty path segments is significant for request binding.

An implementation MUST NOT infer equivalence between:

```text
/a///b
```

and:

```text
/a//b
```

solely because an HTTP framework, router, proxy, middleware component, or downstream application may collapse repeated path delimiters.

Therefore:

```text
/a///b != /a//b
```

and the distinction is symmetric.

A request-target constraint bound to:

```text
/a///b
```

MUST NOT match:

```text
/a//b
```

unless an explicit HACP equivalence rule defines such normalization.

Likewise, a constraint bound to:

```text
/a//b
```

MUST NOT match:

```text
/a///b
```

without such an explicit rule.

A mismatch caused solely by this representation difference MUST be treated as a request-binding mismatch and mapped to:

```text
SCOPE_EXCEEDED
```

Exact representation remains equivalent to itself.

This rule applies only to the multiplicity of consecutive internal empty path segments between otherwise identical non-empty path segments.

It does not define:

* multiple trailing empty path segment semantics;
* leading repeated slash semantics;
* root versus empty-path representation;
* percent-decoding or percent-encoded slash equivalence;
* dot-segment processing;
* router or framework path cleaning;
* general slash normalization;
* scheme or authority processing;
* general URI normalization.

No broader path equivalence follows from this rule.

### 7.1.9 Multiple trailing empty path segment preservation

The number of trailing empty path segments is significant for request binding within the HC2-K canonical boundary.

An implementation MUST NOT infer equivalence between:

```text
/a//
```

and:

```text
/a/
```

solely because an HTTP framework, router, proxy, middleware component, or downstream application may collapse repeated trailing path delimiters.

Therefore:

```text
/a// != /a/
```

and the distinction is symmetric.

A request-target constraint bound to:

```text
/a//
```

MUST NOT match:

```text
/a/
```

unless an explicit HACP equivalence rule defines such normalization.

Likewise, a constraint bound to:

```text
/a/
```

MUST NOT match:

```text
/a//
```

without such an explicit rule.

A mismatch caused solely by this representation difference MUST be treated as a request-binding mismatch and mapped to:

```text
SCOPE_EXCEEDED
```

Exact representation remains equivalent to itself.

This rule applies only to the HC2-K canonical distinction between one and two trailing empty path segments after the same non-empty path segment.

It does not define:

* semantics for more than two trailing empty path segments;
* leading repeated slash semantics;
* root versus empty-path representation;
* additional internal repeated-slash semantics beyond already defined rules;
* percent-decoding or percent-encoded slash equivalence;
* dot-segment processing;
* router or framework path cleaning;
* general trailing-slash normalization;
* general slash normalization;
* scheme or authority processing;
* general URI normalization.

No broader path equivalence follows from this rule.

### 7.1.10 Leading empty path segment preservation

For HTTP request binding, a leading empty path segment preceding a non-empty path segment is representation-significant.

An implementation MUST NOT treat a request target containing such a leading empty path segment as equivalent to the otherwise identical request target with that segment removed.

Therefore:

```text
//a != /a
```

and the distinction is symmetric.

A request-target constraint bound to:

```text
//a
```

MUST NOT match:

```text
/a
```

unless an explicit HACP equivalence rule defines such normalization.

Likewise, a constraint bound to:

```text
/a
```

MUST NOT match:

```text
//a
```

without such an explicit rule.

A mismatch caused solely by this representation difference MUST be treated as a request-binding mismatch and mapped to:

```text
SCOPE_EXCEEDED
```

Exact representation remains equivalent to itself.

This rule applies only to the HC2-L canonical distinction between a leading empty path segment and the otherwise identical path with that segment removed.

It does not define:

* semantics for more than one leading empty path segment;
* `///a` versus `//a`;
* generic URI-reference network-path semantics;
* authority interpretation or reconstruction;
* absolute-form request-target semantics;
* root versus empty-path representation;
* percent-decoding or percent-encoded slash equivalence;
* dot-segment processing;
* proxy, intermediary, router, framework, or middleware path normalization;
* general leading-slash normalization;
* general slash normalization;
* scheme or authority processing;
* general URI normalization.

No broader path equivalence follows from this rule.

If any binding claim does not match the current request, the request MUST be denied with `SCOPE_EXCEEDED`.

## 8. Scope guard

The enforcement point MUST verify that the current request is inside the token scope.

The scope check MUST include at least:

1. Transport type.
2. Method.
3. Path.
4. Tool name.
5. Boundary constraints, if present.

If the token scope contains an attribute unknown to the enforcement point, the request MUST be denied with `UNKNOWN_ATTRIBUTE`.

## 9. Budget and replay protection

A decision token MAY contain a budget in `constraints` or inherit from the envelope.

The enforcement point MUST maintain local replay state for consumed tokens and budget counters.

Budget rules:

1. A token MUST NOT be used more times than its budget permits.
2. A consumed single-use token MUST be denied on replay with `TOKEN_REVOKED`.
3. Budget counters MUST be checked atomically before forwarding.
4. If budget state is unavailable, the request MUST be denied fail-closed with `BUDGET_EXHAUSTED`.

## 10. Distributed control state

When enforcement relies on distributed control state, the enforcement point MUST establish that the required state is usable before relying on it for authorization.

Required distributed control state MUST have a finite maximum staleness threshold.

Default:

```text
max_revocation_staleness_ms = 5000
```

The default value is a profile parameter. An implementation MAY use a different finite threshold when explicitly configured by the applicable deployment or policy.

Required distributed control state MUST be treated as unusable when any of the following applies:

1. No sufficiently trustworthy control state has yet been established.
2. The established state is older than the applicable maximum staleness threshold.
3. The enforcement point has evidence that the state may be incomplete, inconsistent, corrupted, or otherwise unsafe.

When required distributed control state is unusable, the enforcement point MUST deny fail-closed with `CONTROL_STATE_STALE`.

Transport connectivity alone MUST NOT establish that distributed control state is fresh, synchronized, complete, or trustworthy.

A temporary loss of transport connectivity does not by itself require previously established control state to become unusable while that state remains within the applicable freshness bound and is otherwise trustworthy.

Freshness evidence MAY be established without a mutation to the distributed authorization state.

Previously unusable distributed control state MAY become usable again only after sufficient trustworthy synchronization evidence establishes that the local state is complete, consistent, and sufficiently current.

When distributed control state participates in enforcement, its usability and freshness MUST be established before authorization processing that depends on that state and before mutable replay or authorization-budget consumption.

The specific mechanism used to transport, synchronize, refresh, reconcile, or recover distributed control state is implementation-defined.

Distributed revocation state MUST support at least the following revocation targets:

1. Token identifiers.
2. Envelope identifiers.
3. Signing key identifiers.

Distributed control-state mechanisms MUST NOT independently grant an `ALLOW` decision.

Distributed control-state mechanisms MUST NOT independently override a `DENY` decision.

Implementations MAY use mechanisms including streaming delivery, signed control events, monotonic revisions, snapshots, replay, heartbeats, or local persistence. These mechanisms are not required by this profile unless separately specified by another applicable protocol or profile.

A CHECKPOINT resolution received through distributed control-state mechanisms MUST still result in a new evaluator-signed DecisionToken before the approved action is authorized.

## 11. Provenance

The enforcement point MUST maintain a provenance ring buffer.

Each record MUST include at least:

1. Timestamp (Unix seconds).
2. Request identifier.
3. Envelope identifier.
4. Token identifier.
5. Action hash.
6. Decision.
7. Reason code.
8. Enforcement latency.

The flush MAY be asynchronous, but record acceptance MUST happen before the request is forwarded.

If the ring buffer cannot accept a record, the request MUST be denied with `TRACEABILITY_FAILURE`.

Provenance records MUST NOT include the full request payload unless explicitly required by a declared audit policy.

Payload hashes and identifiers are sufficient by default.

## 12. Failure isolation

The enforcement point MUST NOT forward a partial request before verification is complete.

If the enforcement process crashes, the deployment MUST fail closed by preventing direct agent egress.

The enforcement point MUST NOT expose upstream services to the agent without a valid ALLOW decision.

## 13. Deployment requirements for conformance

A conformant deployment under this profile MUST ensure that the agent cannot bypass the enforcement point for enforceable transports.

Minimum deployment requirements:

1. The agent MUST be configured with an explicit HTTP_PROXY or MCP endpoint pointing to the sidecar.
2. The agent container or process MUST NOT have unrestricted direct egress.
3. Direct upstream access MUST be blocked by network policy, container policy, or equivalent isolation.
4. Sidecar failure MUST result in action denial for enforceable transports.

Kernel-level enforcement, such as eBPF, is outside the scope of this profile and is not required for conformance.
