# HACP 1.1.1 Composition

Status: PRE-RELEASE CANDIDATE

Repository:

```text
hacp-spec
```

Composition role:

```text
COMPOSITION_ROLE=SPEC_AUTHORITY
```

This repository owns the normative, conformance, and reference view of the
HACP 1.1.1 release composition.

## Exact Immutable Binding

```text
HACP_RELEASE=1.1.1
HACP_COMPOSITION_ID=HACP-1.1.1-spec-3ea4adc-sidecar-1bc10ac
HACP_SPEC_COMMIT=3ea4adcbf6ef2548e41d340709945326026a73b6
SIDECAR_IMPLEMENTATION_COMMIT=1bc10acbe79620a165cee573c5b260cd8639280f
```

The peer identities above are immutable release-composition bindings.

Floating references such as `main`, `latest`, or an unpinned branch name are
not composition identities.

## Repository Ownership Boundary

This manifest is owned by `hacp-spec`.

The corresponding `hacp-sidecar` repository owns its own
`HACP_1_1_1_COMPOSITION.md` with:

```text
COMPOSITION_ROLE=SIDECAR_RUNTIME
```

The two repository-owned manifests are not required to be byte-identical.

They must converge exactly on:

```text
HACP_RELEASE
HACP_COMPOSITION_ID
HACP_SPEC_COMMIT
SIDECAR_IMPLEMENTATION_COMMIT
```

## Release Boundary

This pre-release composition record does not itself authorize:

```text
main integration
tag creation
stable release
production source mutation
humanist-core mutation
```

Historical identities and evidence remain immutable.
