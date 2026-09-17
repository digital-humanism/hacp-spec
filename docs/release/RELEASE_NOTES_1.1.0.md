# HACP 1.1.0 Release Notes

**Release:** HACP 1.1.0
**Date:** 2026-09-17
**Status:** Approved for stable release

## Overview

HACP 1.1.0 promotes HACP-Enforcement revision 2 to the active Enforcement profile while preserving the stable HACP 1.0.0 Variant A core baseline.

The release adds a fully recertified HC2-55 Enforcement evidence set and completes clean-clone reproducibility verification across the specification, Go sidecar, and Python SDK repositories.

HACP 1.1.0 does not change the HACP wire/object version, which remains `0.9`.

## What Is New

HACP 1.1.0 includes:

* HACP-Enforcement revision 2 as the active Enforcement revision;
* HC2-55 as the bounded conformance evidence set for revision 2;
* recertified HTTP request-binding behavior under the revision-2 profile;
* final compatibility verification against the inherited HACP 1.0.0 Variant A baseline;
* clean-clone reproduction of the complete HACP 1.1 release verification matrix.

## Enforcement Revision 2

The active Enforcement evidence identity is:

```text
Profile: HACP-Enforcement
Revision: 2
Evidence set: HC2-55
Total vectors: 55
Canonicalization: JCS-RFC8785
Digest algorithm: SHA-256
Vector-set digest:
sha256:fcf2b2ee93bf2623c0e088d8b02527713f517220bb63cb0f77b08ed3d2c3ba8a
```

HC2-55 verifies the documented revision-2 HTTP request-binding surface.

The evidence includes request-binding cases covering encoded delimiters, empty query delimiters, internal and leading empty path segments, percent-encoded unreserved representation, and query empty-value delimiters.

## Compatibility

HACP 1.1.0 preserves the HACP 1.0.0 Variant A inherited floor.

The canonical HACP-Core baseline remains:

```text
38/38 PASS
```

The HACP-Core manifest is unchanged by HC2-55.

HC2-55 is a separate Enforcement revision 2 evidence set and does not replace the canonical HACP-Core vectors.

The HACP wire/object version remains:

```text
0.9
```

Release version, wire/object version, HACP-Core vector identity, Enforcement revision, Runner Protocol version, and implementation package versions remain distinct identities.

## Verification

The HACP 1.1 release verification completed with:

| Verification item                    | Result                               |
| ------------------------------------ | ------------------------------------ |
| HACP-Core canonical vectors          | **38/38 PASS**                       |
| HC2-55 fixture integrity             | **55/55 PASS**                       |
| HC2-55 black-box runner              | **55/55 PASS**                       |
| `hacp-conformance-runner` build      | **PASS**                             |
| `hacp-sidecar` full repository tests | **PASS**                             |
| Native `hacp-sidecar` build          | **PASS**                             |
| `humanist-core` default suite        | **319 passed / 5 conditional skips** |
| Python ↔ Go external E2E             | **5/5 PASS**                         |
| Clean-clone reproducibility          | **PASS**                             |

No production source change was required during clean-clone verification.

## Release Candidate Source Set

The verified candidate source set is:

| Repository      | Commit                                     |
| --------------- | ------------------------------------------ |
| `hacp-spec`     | `328c50cecc0fefdae4fc481b08b83f743b6e239a` |
| `hacp-sidecar`  | `1bc10acbe79620a165cee573c5b260cd8639280f` |
| `humanist-core` | `6d9ae82ed7fefe47b395e4ef68d0a18807866473` |

Subsequent `hacp-spec` release-lineage commits contain release documentation and publication records and do not alter the verified candidate protocol, profile, conformance-vector, or implementation source set.

## Claim Boundary

HACP 1.1.0 does not claim:

* general URI normalization beyond the HACP-Enforcement revision 2 request-binding rules;
* dot-segment normalization;
* AuthorityRoot;
* DelegationGrant;
* Semantic Checkpoint 2.0;
* Humanist Core 2.0;
* control-plane persistence;
* transport authentication;
* exact HACP-Core reason-code correspondence for all 38 canonical vectors;
* additional HC2 behavior beyond HC2-55.

These areas remain outside the HACP 1.1.0 release claim.

## Release Decision

The HACP 1.1.0 release decision is **GO**.

The release is based on the completed HACP 1.1 verification, compatibility, claim-audit, release-candidate, and clean-clone stages.

The stable release identity is:

```text
HACP 1.1.0
=
HACP 1.0.0 Variant A inherited floor
+
HACP-Enforcement revision 2 Active
+
HC2-55 bounded revision-2 evidence
```

The approved stable release tag is `v1.1.0`. The tag is created separately after publication of these release notes.
