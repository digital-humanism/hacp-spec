# HACP Boundary Matrix (INV-2)

**Version:** 1.0.0
**Status:** Normative (referenced by SPEC §6)
**License:** CC BY 4.0

This document is the normative decision table for INV-2 (Boundary
Re-Authorization). Every security-relevant attribute of a `ProposedAction`
is checked against the granted `ScopeGrant`. An implementation MUST produce
exactly the outcome and reason code specified here. There is no discretion.

## 1. Security-Relevant Attributes

| Attribute | Type | Granted form | Violation reason |
|-----------|------|--------------|------------------|
| `verb` | set | `string[]` | `SCOPE_EXCEEDED` |
| `resource_class` | set | `string[]` | `SCOPE_EXCEEDED` |
| `audience` | ordered set | subset of `{internal, external, public}` | `BOUNDARY_CROSSING` |
| `reversibility` | ordered set | subset of `{reversible, irreversible}` | `BOUNDARY_CROSSING` |
| `externality` | ordered set | subset of `{internal, external}` | `BOUNDARY_CROSSING` |
| `data_class` | ordered set | subset of `{public, internal, confidential, restricted}` | `BOUNDARY_CROSSING` |
| `quantity` | ceiling | `max_quantity: int` | `SCOPE_EXCEEDED` |
| `destination` | allowlist | `string[]` | `BOUNDARY_CROSSING` |
| `tool_name` | allowlist | `string[]` | `BOUNDARY_CROSSING` |

## 2. Decision Rule

For **set / ordered-set / allowlist** attributes:

```
proposed ∈ granted  →  in-scope (continue)
proposed ∉ granted  →  DENY(<violation reason>)
```

For **ceiling** (`quantity`):

```
proposed ≤ max_quantity  →  in-scope (continue)
proposed >  max_quantity →  DENY(SCOPE_EXCEEDED)
```

For **absent optional** attributes (`quantity`, `destination`, `tool_name`):

```
attribute absent AND policy defaults it   →  treat defaulted value as proposed
attribute absent AND policy does NOT default →  DENY(UNKNOWN_ATTRIBUTE)
```

The final decision is `ALLOW` only if every attribute is in-scope. The first
violating attribute determines the reason code.

## 3. Sensitivity Lattice (documentation)

Ordered attributes have a sensitivity ordering. Proposing a value strictly
more sensitive/permissive than every granted value is a boundary crossing.

```
audience:      internal < external < public
data_class:    public < internal < confidential < restricted
reversibility: reversible < irreversible
externality:   internal < external
```

Note: the lattice is informational. The enforceable rule is set membership
(§2). A proposed value is allowed iff it is a member of the granted set,
regardless of position in the lattice.

## 4. Per-Attribute Matrices

### audience (granted × proposed)

| granted \ proposed | internal | external | public |
|--------------------|----------|----------|--------|
| {internal} | in-scope | BOUNDARY_CROSSING | BOUNDARY_CROSSING |
| {internal, external} | in-scope | in-scope | BOUNDARY_CROSSING |
| {internal, external, public} | in-scope | in-scope | in-scope |

### data_class (granted × proposed)

| granted \ proposed | public | internal | confidential | restricted |
|--------------------|--------|----------|--------------|------------|
| {public, internal} | in-scope | in-scope | BOUNDARY_CROSSING | BOUNDARY_CROSSING |
| {public, internal, confidential} | in-scope | in-scope | in-scope | BOUNDARY_CROSSING |
| {public, internal, confidential, restricted} | in-scope | in-scope | in-scope | in-scope |

### reversibility (granted × proposed)

| granted \ proposed | reversible | irreversible |
|--------------------|------------|--------------|
| {reversible} | in-scope | BOUNDARY_CROSSING |
| {reversible, irreversible} | in-scope | in-scope |

### externality (granted × proposed)

| granted \ proposed | internal | external |
|--------------------|----------|----------|
| {internal} | in-scope | BOUNDARY_CROSSING |
| {internal, external} | in-scope | in-scope |

### quantity (ceiling)

| max_quantity | proposed | outcome |
|--------------|----------|---------|
| 100 | 100 | in-scope |
| 100 | 101 | SCOPE_EXCEEDED |
| absent | any | UNKNOWN_ATTRIBUTE (if policy does not default) |

### destination / tool_name (allowlist)

| allowlist | proposed | outcome |
|-----------|----------|---------|
| {a, b} | a | in-scope |
| {a, b} | c | BOUNDARY_CROSSING |
| absent | any | UNKNOWN_ATTRIBUTE (if policy does not default) |

## 5. Implementation Mandate

The conformance harness `evaluate_logic` MUST implement exactly this matrix.
Ad-hoc conditionals not derivable from this table are non-conforming. Each
INV-2 conformance vector corresponds to one cell of this matrix.
