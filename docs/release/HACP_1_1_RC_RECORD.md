# HACP 1.1.0 — Release Candidate Record

**Status:** Release Candidate Record
**Release line:** HACP 1.1.0
**Roadmap stage:** R8 — RC Record
**Contract:** HACP 1.0.0 Variant A inherited floor + HACP-Enforcement revision 2 Active + HC2-55 advertised under explicit revision-2 identity

## 1. Purpose

This record fixes the exact repository commit set that constitutes the HACP 1.1.0 release candidate for the R9 clean-clone verification stage.

R8 does not reopen the R7 final verification matrix.

R8 does not create the stable `v1.1.0` release.

No production, profile, wire, vector, or conformance semantics are changed by this record.

## 2. Candidate repository identities

### hacp-spec

```text
commit:
328c50cecc0fefdae4fc481b08b83f743b6e239a

commit subject:
docs: record R7 publication closure

signature:
Good ED25519 signature

remote state:
HEAD == origin/main

working tree:
clean
```

### hacp-sidecar

```text
commit:
1bc10acbe79620a165cee573c5b260cd8639280f

commit subject:
test: cover representative reason branches

signature:
Good ED25519 signature

remote state:
HEAD == origin/main

working tree:
clean
```

### humanist-core

```text
commit:
6d9ae82ed7fefe47b395e4ef68d0a18807866473

commit subject:
docs: fix HACP sidecar integration links

signature:
Good ED25519 signature

remote state:
HEAD == origin/main

working tree:
clean
```

These exact commit identities form the R8 candidate source commit set.
The hacp-spec commit that records this R8 artifact is release-lineage
evidence and is not self-recorded as one of the source revisions above.

## 3. Inherited R7 verification evidence

The R7 final verification matrix is closed and remains the verification basis for this candidate.

Canonical R7 artifact:

```text
docs/release/HACP_1_1_FINAL_VERIFICATION_MATRIX.md
```

R7 substantive matrix commit:

```text
e0cf58077baafbf822c3d4be90f7f4af05729364
```

R7 publication-closure commit:

```text
328c50cecc0fefdae4fc481b08b83f743b6e239a
```

R7 status:

```text
CLOSED / PASS / COMMITTED / SIGNED / PUBLISHED / CLEAN
```

Established R7 results:

```text
HACP-Core decision-level canonical conformance:
38/38 PASS

HC2 fixture integrity:
55/55 PASS

HC2 black-box:
55/55 PASS

hacp-sidecar full repository tests:
PASS

hacp-sidecar native build:
PASS

hacp-conformance-runner build:
PASS

Python ↔ Go external E2E:
5/5 PASS

signed repository heads:
PASS
```

No R7 verification suite is reopened by R8.

## 4. HC2-55 evidence identity

The advertised revision-2 evidence set remains:

```text
Profile:
HACP-Enforcement

Revision:
2

Evidence set:
HC2-55

Total vectors:
55

Canonicalization:
JCS-RFC8785

Digest:
sha256:fcf2b2ee93bf2623c0e088d8b02527713f517220bb63cb0f77b08ed3d2c3ba8a
```

The digest identity is inherited from the closed R2/R3 evidence and the R7 verification record.

R8 does not invent or introduce a new digest-recomputation mechanism.

## 5. Version-domain identity

The release candidate keeps the version domains distinct:

```text
release candidate:
HACP 1.1.0 candidate

HACP release version:
1.1.0 target

wire/object family:
hacp_version = "0.9"

profile:
HACP-Enforcement

profile revision:
2

profile status:
Active

conformance evidence:
HC2-55

Runner Protocol:
v1

Humanist Core 2.0:
NOT claimed
```

HACP 1.1.0 does not imply a wire/object version migration.

HACP 1.1.0 is not Humanist Core 2.0.

## 6. Stable HACP 1.0.0 inheritance

The immutable HACP 1.0.0 baseline remains unchanged:

```text
stable tag:
v1.0.0

tag object:
009cb86e2adada511ebcb3b2a6f343be8c968971

peeled release commit:
c468c9bb0427448e564bcf3e7d9c8a3a004b8513
```

The HACP 1.0.0 Variant A contract remains the inherited compatibility floor.

No stable 1.0.0 tag, commit, release evidence, or historical record is modified by R8.

## 7. R8 scope and non-actions

R8 performed only release-candidate identity capture and documentation.

```text
fresh conformance rerun:
NO

fresh HC2 rerun:
NO

fresh external E2E rerun:
NO

production changes:
0

profile semantic changes:
0

wire/object changes:
0

canonical vector changes:
0

new HC2 cases:
0

historical rewrites:
0

RC tag created:
NO

stable v1.1.0 tag created:
NO

R9 clean-clone verification:
NOT STARTED

R10 GO / release notes / stable release:
NOT STARTED
```

The controlling engineering rule remains:

> NO PRODUCTION CHANGE WITHOUT NORMATIVE BASIS AND PROVEN RED.

## 8. R8 exit

The exact HACP 1.1.0 candidate source commit set is now recorded:

```text
hacp-spec:
328c50cecc0fefdae4fc481b08b83f743b6e239a

hacp-sidecar:
1bc10acbe79620a165cee573c5b260cd8639280f

humanist-core:
6d9ae82ed7fefe47b395e4ef68d0a18807866473
```

All three candidate heads were verified as:

```text
signed
published
HEAD == origin/main
working tree clean
```

Therefore:

```text
R8:
READY FOR DOCUMENTATION CLOSURE

R9 ENTRY:
AUTHORIZED ONLY AFTER THIS R8 RECORD IS COMMITTED,
SIGNED, PUBLISHED, AND THE hacp-spec WORKING TREE IS CLEAN
```
