# HACP 1.0.0

**Status:** Stable
**Release line:** HACP 1.0.0
**Contract boundary:** Variant A

## Release Scope

HACP 1.0.0 establishes the stable public HACP-Core contract with a reproducible canonical decision-level conformance baseline.

The release includes:

- the stable HACP-Core specification contract;
- the retained canonical HACP-Core executable baseline;
- Protocol v1 runner and strict reason-code verification tooling;
- reproducible decision-level canonical conformance across the verified implementations;
- the current sidecar implementation of the active Enforcement profile;
- the release-engineering evidence required for the HACP 1.0.0 stable release.

## Verification Summary

The final HACP 1.0.0 verification baseline records:

```text
hacp-go canonical conformance:            38/38 PASS
hacp-ts canonical conformance:            38/38 PASS
hacp-sidecar decision outcomes:           38/38 CORRECT
hacp-sidecar exact-reason baseline:       15/38 PASS
classified exact-reason mismatches:       23/38
humanist-core canonical conformance:      38/38 PASS
humanist-core local regression:           319 PASS / 5 expected SKIP / 0 FAIL
humanist-core statement coverage:         100%
humanist-core branch coverage:            100%
Python ↔ Go external E2E:                 5/5 PASS
clean-clone validation:                   PASS
unresolved HACP 1.0.0 release blockers:  0
```

All classified sidecar strict reason-code mismatches retain the correct decision outcome.

Exact reason-code 38/38 correspondence is not part of the HACP 1.0.0 Variant A release requirement.

## Version Domains

HACP 1.0.0 preserves distinct version domains rather than forcing them into a single version number:

```text
Specification release:       1.0.0
Canonical HACP-Core:         0.9.2
Canonical vector set:        core-0.9.2
Wire/object version:         0.9
Runner Protocol:             1
humanist-core package:       0.5.0
```

The HACP 1.0.0 specification release does not automatically migrate the HACP wire/object version or rename the retained canonical vector set.

## Enforcement Revision 2 and HC2

Enforcement revision 2 remains a draft successor surface.

It is not activated by HACP 1.0.0.

HC2-55 remains draft successor conformance evidence and is not promoted to the active HACP 1.0 Enforcement conformance contract.

Accordingly, HACP 1.0.0 does not claim:

- Enforcement revision 2 activation;
- HC2-55 as active HACP 1.0 Enforcement conformance;
- exact reason-code 38/38 correspondence;
- automatic migration of signed HACP objects to wire/object version 1.0.

## Release Lineage

The stable release descends from the approved HACP 1.0.0 release-candidate lineage.

```text
v1.0.0-rc.1 target:
71c0e01018edb282614f7b047737ea3a425e2542
docs: record HACP 1.0 release candidate

R8 clean-clone verification:
18d2a7a12f0122a18bf2d62aaa25f2a6ab66b3a9
docs: record HACP 1.0 clean-clone verification

R9 final GO decision:
361a28ceb24b0fa9949c7595efb79f27207eea83
docs: record HACP 1.0 final go decision
```

The release candidate remained immutable throughout R8 and R9 verification.

R9 concluded:

```text
DECISION: GO

CONTRACT BOUNDARY:                 PRESERVED
MANDATORY VERIFICATION:            PASS
CLEAN-CLONE REPRODUCIBILITY:       PASS
UNRESOLVED RELEASE BLOCKERS:       0
ENFORCEMENT REVISION 2:            DRAFT — NOT ACTIVE
HC2-55:                            NOT PROMOTED
EXACT REASON 38/38:                NOT REQUIRED / NOT CLAIMED
```

## Deferred Work

Work intentionally outside the HACP 1.0.0 Variant A release boundary remains eligible for later HACP 1.0.n or successor-profile work, including:

- exact reason-code correspondence hardening;
- normative adjudication of deferred HOLD items;
- Enforcement revision 2 activation;
- HC2-55 promotion after separate activation approval;
- additional post-1.0 conformance and implementation hardening.

These deferred items do not alter the HACP 1.0.0 stable contract.
