# HACP 1.0.0 Version Alignment and Release Metadata Assessment

**Stage:** R4 — Version Alignment and Release Metadata
**Repository:** `hacp-spec`
**Target release:** HACP `1.0.0`
**R1 status:** COMPLETE
**R2 status:** COMPLETE
**R3 status:** COMPLETE
**Production changes authorized by this document:** NONE

---

## 1. Purpose

This document records the R4 pre-tag version-alignment and release-metadata work for HACP `1.0.0`.

R4 was performed under the release-scope boundary established by R3.

Its purpose was to align the public HACP `1.0.0` release surface without collapsing independent version domains, changing wire/object semantics, migrating canonical conformance identities, activating an Enforcement revision, or expanding the frozen release scope.

The governing release boundary is:

```text
HACP 1.0.0 =
  stable public HACP-Core contract
+ reproducible decision-level canonical conformance
+ Protocol v1 runner / strict verifier as tooling
+ honestly documented historical / exact-reason scope
+ sidecar as current implementation of the applicable Enforcement lineage
```

R4 does not establish:

```text
HACP 1.0.0 = Enforcement revision 2 active
HACP 1.0.0 = HC2 advertised as active Enforcement conformance
HACP 1.0.0 = exact-reason canonical 38/38
HACP 1.0.0 = hacp_version 1.0
```

---

## 2. Governing Version Domains

R4 preserves the distinction among the following independently owned version domains.

| Domain | HACP 1.0.0 disposition |
|---|---|
| Specification release version | `1.0.0` |
| Human-readable claim compatibility line | `1.0` |
| HACP wire/object version | `"0.9"` |
| Canonical HACP-Core executable baseline | `HACP-Core v0.9.2` |
| Canonical vector-set identity | `core-0.9.2` |
| Runner Protocol | `1` |
| Harness/tool version | Retained where historically/tool-owned |
| Implementation/package version | Independently owned |
| Enforcement profile revision | Independently owned |
| Historical engineering milestone | Preserved when historically accurate |

No automatic migration between these domains is authorized.

---

## 3. Public Specification Release Metadata

Release-facing specification documents previously carrying development-era metadata such as:

```text
Version: 0.9.3
Status: Draft for public review
```

were aligned to:

```text
Version: 1.0.0
Status: Stable
```

where the status represented public specification release maturity.

Affected release-facing documents include:

```text
HACP-SPEC-0.9-draft.md
INVARIANTS.md
NON-GOALS.md
PROFILES.md
README.md
api/decision-api.md
boundary-matrix.md
canonicalization.md
error-model.md
threat-model.md
versioning.md
wire/crypto-profile.md
wire/encoding.md
```

`boundary-matrix.md` retained its existing normative status.

The historical filename:

```text
HACP-SPEC-0.9-draft.md
```

was not renamed. The filename is repository lineage and path identity; renaming it was not required for release-version alignment and would create unnecessary reference churn.

---

## 4. Claim Compatibility Line

Repository history established that human-readable HACP profile claims use a `major.minor` compatibility-line representation rather than the complete SemVer release string.

For example, the original `0.9.x` specification lineage used:

```text
HACP 0.9-Core
HACP 0.9-Runtime
HACP 0.9-Enforcement
```

even when the specification release carried a patch or draft suffix.

R4 therefore aligns the HACP `1.0.0` claim surface to:

```text
HACP 1.0-Core
HACP 1.0-Runtime
```

For Enforcement, the profile name alone is insufficient when materially distinct Enforcement revisions exist.

The release-facing Enforcement claim identity is therefore represented as:

```text
HACP 1.0-Enforcement
+ applicable Enforcement revision
```

as governed by `profiles/enforcement-identity.md`.

The wording formerly requiring the “exact spec version” was clarified to refer to the:

```text
specification compatibility line
```

so that normative prose matches the established claim grammar.

No new `HACP 1.0.0-Core` claim grammar was introduced.

---

## 5. Enforcement Revision Identity and Lifecycle

R4 did not activate any Enforcement revision.

### Revision 1

`profiles/enforcement.md` remains:

```text
Status: Draft
```

Its specification metadata was aligned to HACP `1.0.0`, but its lifecycle status was not changed.

R4 does not establish that revision 1 became active.

### Revision 2

`profiles/enforcement-v2-draft.md` remains:

```text
Status: Draft — not yet active
Profile revision: 2-draft
Phase: HC2 normative development
```

Its HACP specification metadata was aligned to:

```text
Specification version: 1.0.0
```

without altering its revision lifecycle.

R4 does not activate revision 2 and does not promote HC2 vectors into an active HACP-Enforcement conformance claim.

---

## 6. Legacy Phase / Gate Terminology

R3 required individual classification of development-era terminology such as:

```text
Phase 4
Gate D
Gate E
Phase 4 MVP
Normative for Gate D
```

R4 did not mechanically replace such terms.

Historical engineering statements remain valid where they describe completed project milestones, including roadmap, ADR, and historical conformance material.

Current normative Enforcement wording was reviewed separately.

Development-era `Phase 4` / `Gate D` terminology embedded in current Enforcement normative scope was replaced by profile-owned terminology without changing the underlying requirements.

Examples include:

```text
For Phase 4 MVP
→
For this profile
```

```text
out of scope for Gate D
→
out of scope for this profile
```

```text
Normative for Gate D
→
Normative for this profile
```

```text
A conformant Phase 4 deployment
→
A conformant deployment under this profile
```

```text
Minimum MVP requirements
→
Minimum deployment requirements
```

Kernel-level enforcement remains outside the required conformance scope.

The enforceable transport scope remains unchanged.

The control-channel requirements remain unchanged.

The deployment-isolation requirements remain unchanged.

---

## 7. Runtime Status

`checkpoint-protocol.md` is aligned to specification release version:

```text
Version: 1.0.0
```

while retaining its Runtime-specific lifecycle qualification:

```text
Status: Draft (Phase 3, does not block Core 1.0.0)
```

`PROFILES.md` continues to describe Runtime conformance as in progress.

These statements are not HACP-Core release-status labels.

They represent the current Runtime profile/conformance state and were therefore not mechanically promoted to Stable or complete.

No Runtime lifecycle transition is established by R4.

---

## 8. Canonical HACP-Core Conformance Baseline

R4 preserves the existing canonical executable baseline:

```text
HACP-Core v0.9.2
vector_set: core-0.9.2
```

Relevant manifest, workflow, harness, vector, and documentation references remain intentionally unchanged.

The HACP `1.0.0` specification release does not rename this retained executable evidence.

`versioning.md` now explicitly records that the conformance suite is independently versioned from the specification release version.

The retained canonical baseline for HACP `1.0.0` is:

```text
HACP-Core v0.9.2
core-0.9.2
```

Retention of this baseline does not change:

```text
specification release version = 1.0.0
claim compatibility line      = 1.0
wire/object version            = 0.9
runner protocol                = 1
```

Passing a retained or historical suite does not independently redefine the specification release represented by a conformance claim.

---

## 9. Wire / Object Version

R4 preserves:

```text
hacp_version = "0.9"
```

across the normative object model, schemas, canonical vectors, and examples.

The HACP `1.0.0` specification release and the HACP wire/object version are separate version domains.

No schema migration was authorized.

No signed-object version migration was performed.

No canonical-vector migration was performed.

A future change to `hacp_version` requires separate normative basis.

---

## 10. Runner Protocol

Runner Protocol remains:

```text
protocol_version = "1"
```

R4 did not modify Runner Protocol semantics.

The Protocol v1 runner and strict verifier remain tooling around the retained canonical and historical conformance surfaces.

No runner migration was required for HACP `1.0.0`.

---

## 11. Tool and Implementation Versions

Harness, tool, implementation, and package version strings were not mechanically rewritten as specification release metadata.

Examples include:

```text
HACP Conformance Harness v0.9.2
implementation-version
package version
```

These belong to independently owned version domains.

They remain unchanged unless separately owned by a release-hygiene task.

---

## 12. Historical Evidence

Historical references were intentionally preserved, including:

```text
0.9.3
HACP 0.9-Core
HACP 0.9-Runtime
HACP 0.9-Enforcement
Phase / Gate milestones
```

when they occur in historical records such as:

```text
CHANGELOG.md
docs/conformance/*
historical assessments
```

R4 does not rewrite prior evidence to make historical documents appear as if they had originally been authored against HACP `1.0.0`.

---

## 13. Machine-Readable Metadata Classification

R4 verified the major machine-readable version surfaces.

The following were classified as retained canonical-suite identity:

```text
spec_version = 0.9.2
vector_set = core-0.9.2
```

The following was classified as Runner Protocol identity:

```text
protocol_version = 1
```

The following was classified as HACP wire/object identity:

```text
hacp_version = 0.9
```

No machine-readable occurrence was changed merely because the public specification release is `1.0.0`.

---

## 14. Surfaces Explicitly Not Changed

R4 made no semantic changes to:

```text
schemas/
vectors/
harness/
```

No production implementation changes were required.

No canonical expected outcome was changed.

No canonical vector identity was migrated.

No runner/protocol semantics were modified.

No wire/object identifier was migrated.

No Enforcement revision was activated.

---

## 15. Verification Evidence

R4 verification established:

```text
current release-facing 0.9.3 occurrences:
NONE
```

Remaining `0.9.3` occurrences are historical and appear in release history or prior conformance assessments.

Current profile claims are aligned to:

```text
HACP 1.0-Core
HACP 1.0-Runtime
HACP 1.0-Enforcement + applicable revision
```

Historical `HACP 0.9-*` claim strings remain in prior assessment evidence.

Current Enforcement normative files contain no remaining:

```text
Phase 4
Gate D
Phase 4 MVP
Normative for Gate D
```

development-era scope labels.

Lifecycle verification confirms:

```text
profiles/enforcement.md
Status: Draft

profiles/enforcement-v2-draft.md
Status: Draft — not yet active
Profile revision: 2-draft
Phase: HC2 normative development

wire-headers.md
Status: Draft
```

The following diff was empty:

```text
schemas
vectors
harness
```

confirming no R4 semantic modification to those surfaces.

---

## 16. R4 Disposition

| Item | Disposition |
|---|---|
| Public `0.9.3` specification release labels | ALIGNED TO `1.0.0` |
| Public Core draft-review status | ALIGNED TO `Stable` |
| README release metadata | ALIGNED |
| Claim compatibility line | ALIGNED TO `1.0` |
| Enforcement claim revision identity | PRESERVED / MADE EXPLICIT |
| Runtime lifecycle | PRESERVED |
| Enforcement revision 1 lifecycle | PRESERVED — Draft |
| Enforcement revision 2 lifecycle | PRESERVED — Draft / not active |
| HC2 activation | NOT PERFORMED |
| Phase/Gate wording in current Enforcement scope | ALIGNED |
| Historical Phase/Gate wording | PRESERVED WHERE HISTORICAL |
| Canonical HACP-Core suite identity | PRESERVED |
| Canonical vector-set identity | PRESERVED |
| Runner Protocol | PRESERVED |
| HACP wire/object version | PRESERVED |
| Schemas | UNCHANGED |
| Canonical vectors | UNCHANGED |
| Harness/runner semantics | UNCHANGED |
| Production implementation | UNCHANGED |

---

## 17. Release Boundary After R4

After completion of R4, the HACP `1.0.0` release boundary remains:

```text
HACP 1.0.0 =
  stable HACP-Core specification release
+ claim compatibility line 1.0
+ retained canonical HACP-Core v0.9.2 executable baseline
+ retained core-0.9.2 vector-set identity
+ retained Runner Protocol v1
+ retained hacp_version 0.9
+ honest Runtime and Enforcement lifecycle status
```

It explicitly does not imply:

```text
wire/object migration to 1.0
canonical suite rename to 1.0.0
new canonical vector set
Enforcement revision 1 activation
Enforcement revision 2 activation
HC2 activation
strict exact-reason 38/38 claim
```

---

## 18. Final R4 Result

```text
R4 — VERSION ALIGNMENT AND RELEASE METADATA

TECHNICAL EDIT PHASE:
COMPLETE

PUBLIC RELEASE VERSION:
1.0.0

PUBLIC CORE STATUS:
STABLE

CLAIM COMPATIBILITY LINE:
1.0

CANONICAL EXECUTABLE BASELINE:
HACP-Core v0.9.2
core-0.9.2

RUNNER PROTOCOL:
1

WIRE / OBJECT VERSION:
0.9

RUNTIME LIFECYCLE:
UNCHANGED

ENFORCEMENT REVISION 1:
DRAFT
NOT ACTIVATED

ENFORCEMENT REVISION 2:
DRAFT — NOT YET ACTIVE
NOT ACTIVATED

HC2:
NOT PROMOTED

PRODUCTION CHANGES:
NONE

SCHEMA CHANGES:
NONE

CANONICAL VECTOR CHANGES:
NONE

RUNNER SEMANTIC CHANGES:
NONE

R4 RESULT:
COMPLETE
```

The repository is ready to proceed to the next bounded HACP `1.0.0` release-engineering stage after review and signed commit of the R4 alignment set.
