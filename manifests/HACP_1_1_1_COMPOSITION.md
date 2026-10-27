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

## Release Train Identity

```text
HACP_RELEASE=1.1.1
HACP_RELEASE_CANDIDATE=1.1.1-rc.1
FINAL_RELEASE_TAG=v1.1.1
```

The HACP 1.1.1 release candidate is a mutable working candidate.

```text
RC=MUTABLE
FINAL_RELEASE_TAG_STATE=IMMUTABLE
```

Corrective commits, documentation corrections, CI or reproduction corrections,
and other bounded release-readiness changes may be applied while the active
release candidate is being verified.

The active release candidate is not an immutable final Git tag.

The exact Git representation of the mutable release candidate is defined
separately at the release-candidate representation checkpoint.

Raw Git commit SHA values remain factual Git identities and evidence.
They are not manually maintained cross-repository release-composition bindings.

Floating references such as `main` or `latest` are not final release identities.

## Repository Ownership Boundary

This manifest is owned by `hacp-spec`.

The corresponding `hacp-sidecar` repository owns its own
`HACP_1_1_1_COMPOSITION.md` with:

```text
COMPOSITION_ROLE=SIDECAR_RUNTIME
```

The two repository-owned manifests are not required to be byte-identical.

They must converge on the HACP 1.1.1 release-train semantics:

```text
HACP_RELEASE
HACP_RELEASE_CANDIDATE
FINAL_RELEASE_TAG
RC=MUTABLE
FINAL_RELEASE_TAG_STATE=IMMUTABLE
```

Repository commit SHA values may be recorded as factual evidence but are not
manually maintained peer bindings for the active release candidate.

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
