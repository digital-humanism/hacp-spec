# HACP Phase B — Consolidated Cross-Language Licensing Amendment Package

**Date:** 2026-10-09
**Status:** PROPOSED — DOCUMENTARY PACKAGE ONLY; NOT APPROVED FOR APPLICATION
**Basis:** Published policy record `docs/licensing/HACP_CROSS_LANGUAGE_LICENSING_POLICY_DECISION_2026-10-09.md`, commit `f6b392cc354d51beaa8f608600f09db1abecfc77`, branch `docs/cross-language-licensing-policy` in `hacp-spec`.
**Evidence:** Phase B Slices 6–8; Slice 8 transcript `HACP_PHASE_B_SLICE8_CONSOLIDATED_LICENSE_AUDIT_2026-10-09.txt` (2026-10-09).
**Authority:** A planning proposal, **not** a license grant, relicensing approval, legal opinion, production-change authorization, merge, or release.

## 1. Stage transition and objective

The documentary policy/publication milestone is complete. The next stage is a **single consolidated licensing amendment**, not four independent language migrations. The objective is consistent licensing of **cleared, project-owned executable implementation code** in Python, TypeScript, Go, and Rust under **AGPLv3 plus a separate commercial license**, while maintaining HACP normative and IETF-related rights. The Go enforcement sidecar remains separately licensed under its existing model; Rust is a peer **target** enforcement runtime, not automatically a proven runtime-parity replacement.

## 2. Immutable baseline and evidence

| Object | Captured value | Status |
|---|---|---|
| `hacp-spec` policy branch | `docs/cross-language-licensing-policy` | Published; no merge to `main` |
| `hacp-spec` policy HEAD | `f6b392cc354d51beaa8f608600f09db1abecfc77` | Signed, local/remote HEAD matched, clean (user-provided verification) |
| Policy parent | `5c7f61a6af402f21ad6374f9f896fa9f44784d09` | Earlier admission baseline |
| `hacp-sidecar` evidence HEAD | `9f7619d3086fc119e0223a9091cf837d70a8d5a3` | Audit identity only; current state must be rechecked before any modification |
| `hacp-spec/LICENSE` | SHA-256 `D40BDCDE2EE08937B721FF2C15BA1BE8A547595F2813AA3F93989CC5F2A08F89` | CC BY 4.0 normative coverage; do not replace wholesale |
| `hacp-rs/Cargo.toml` | SHA-256 `379398A445ED3ABBB0E0BCE456E27E75E0854430F66EF78E07CABB96315E2B0D` | Currently `license = "CC-BY-4.0"`, version `0.1.1` |
| `hacp-sidecar/LICENSE.md` | SHA-256 `625EB2035DFF4076A779AFD95E601BA8131488A9B7680B8A55E73EE0C120E59A` | AGPLv3 plus commercial model, exact SPDX variant unspecified |

Slice 8 inventoried 420 tracked paths and scanned 168 source files across two repositories. The structural categories are not themselves a legally cleared file allowlist.

## 3. Proposed CURRENT → TARGET matrix

| Surface | Current declaration / boundary | Proposed treatment | Disposition |
|---|---|---|---|
| HACP normative documents, specification, schemas, `proto/**`, `vectors/**`, conformance materials | CC BY 4.0 and any applicable IETF rights | Preserve all existing grants and notices | **EXCLUDE from licensing amendment** |
| `hacp-go/` executable implementation source | No independently identified implementation-specific license | Additional AGPLv3 license plus separate commercial offering, **only for cleared project-owned files** | PROPOSE; exact file list pending |
| `hacp-ts/` executable implementation source | `package.json` has no license field; `private: true` | Same cleared-implementation model | PROPOSE; exact file list pending |
| `harness/**/*.py`, `tools/**/*.py` | Mixed conformance implementation, runners, tests, vector baking and verification | Apply common policy only to specifically adjudicated executable files, without restricting the normative conformance suite | **SPLIT FILE-BY-FILE** |
| `hacp-rs/` original Rust implementation | Explicit `CC-BY-4.0` in Cargo; historical grants must survive | Add AGPLv3 licensing prospectively where authorized plus separate commercial offering; accurate Cargo metadata only after approval | PROPOSE; historical rights preserved |
| `hacp-sidecar` original Go runtime | Existing AGPLv3 + commercial dual | Leave unchanged | **NO CHANGE** |
| Generated Go protobuf and Rust generated bindings | From normative `proto/hacp/control/v1/control_plane.proto` and tools | Retain upstream and generator obligations; do not assert blanket sublicensing | **EXCLUDE from automatic grant** |
| `humanist-core` | Frozen independently licensed Python SDK | No license or production change | **NO CHANGE** |

**Critical distinction:** This is a target *additional licensing model*, not a retroactive withdrawal of CC BY 4.0. The rights holder cannot revoke earlier irrevocable CC BY licenses merely by changing a repository file.

## 4. Candidate file scopes — not yet approved allowlists

- **Go:** `hacp-go/*.go` as candidate original implementation and associated tests; `hacp-go/go.mod` as prospective metadata/documentation scope only.
- **TypeScript:** `hacp-ts/src/**/*.ts` as candidate original implementation; `hacp-ts/tests/**/*.ts` subject to conformance-material boundary decision; `hacp-ts/package.json` metadata only. Do **not** alter `private: true` merely to add licensing metadata.
- **Rust:** `hacp-rs/src/**/*.rs`, `hacp-rs/src/bin/**/*.rs`, and `hacp-rs/build.rs` as candidate original implementation; `hacp-rs/tests/**/*.rs` requires test/conformance classification; `hacp-rs/Cargo.toml` is metadata only. `Cargo.lock` and third-party crates do not become project-owned source via this amendment.
- **Python:** `harness/*.py`, `harness/**/*_tests/*.py` / `harness/**/test_*.py`, `tools/*.py`, `tools/**/test_*.py` are *review candidates only*. Each file must be classified as executable implementation, conformance harness, normative test asset, or mixed before any license notice is applied. The policy does **not** create a separate `hacp-py` SDK directory.
- **Excluded:** `proto/**`, `schemas/**`, `vectors/**`, normative docs, IETF materials, conformance data/fixtures, `.seed` and public test-key fixture assets, generated bindings, all unowned dependencies. Do not treat `harness/` and `tools/` directories as blanket implementation grants.

## 5. Proposed minimal amendment shape (not an executable patch)

| Candidate path | CURRENT | PROPOSED only after gates pass |
|---|---|---|
| `docs/licensing/IMPLEMENTATION_LICENSE_SCOPE.md` **(new)** | Absent | Human-reviewed explicit per-file allowlist, excluded materials, provenance, grantor and license-expression explanation; normative CC BY boundary |
| `docs/licensing/COMMERCIAL_LICENSING_NOTICE.md` **(new)** | Absent | Commercial inquiries and scope statement; expressly no entitlement to relicense third-party work |
| `hacp-rs/Cargo.toml` | `license = "CC-BY-4.0"` | **Do not select a replacement yet**. Evaluate truthful dual/multiple-license Cargo SPDX expression and `license-file` handling under historical CC BY rights and actual public grant; do not equate a separately negotiated commercial agreement with SPDX dual license selector |
| `hacp-ts/package.json` | No `license` key, `private: true` | Add accurate SPDX metadata **only after** public grant selection and confirmed package scope; otherwise leave unspecified |
| `hacp-go/` | No subdirectory license identified | Add carefully scoped notice/license file or per-file markers after ownership and conformance boundary clearance |
| Python `harness/`, `tools/` | Conformance and executable code mixed | Explicit file-level notices where authorized; avoid directory-wide substitution of normative grants |
| Root `hacp-spec/LICENSE` | CC BY 4.0 covering named normative surfaces | Preserve existing text. Any clarifying cross-reference requires separate reviewed non-subtractive wording; not an authorization to edit root LICENSE now |
| `hacp-sidecar/LICENSE.md`, `humanist-core/**` | Existing historical licensing | **No change** |

Do not introduce a concrete SPDX expression (`AGPL-3.0-only` or `AGPL-3.0-or-later`) or new source-header grants until the grantor and scope have been signed off. Commercial licensing is a separate contract, **not** automatically an SPDX license choice available to every recipient.

## 6. Single acceptance gate for the whole amendment

All conditions below must be resolved **before** writing an effective licensing patch:

1. **Normative/IETF exclusion:** verify no source/notice change withdraws rights attached to standards, normative vectors, protobuf schemas, or IETF submissions; assess any IETF code-component notices separately.
2. **Exact rights and provenance:** record authors/contributor rights for each included file, copied/AI-assisted material as applicable, and ability of identified licensor to grant commercial terms. Git author fields alone are insufficient.
3. **File-specific scope:** approve per-file inclusion/exclusion, particularly Python and tests. Files with mixed normative and executable content must be split by license notice or left out unless there is a defensible explicit rights basis; no structural redesign by default.
4. **Public license identity:** select `AGPL-3.0-only` **or** `AGPL-3.0-or-later`; reconcile with existing Go sidecar wording and Python SDK terms without retroactively reinterpreting them.
5. **Commercial model:** identify legal grantor and precise original owned code scope; third-party software and irrevocable historic permissions remain outside proprietary exclusivity.
6. **Dependencies & generated artifacts:** generate distribution-specific notices/SBOM and verify upstream terms (Cargo crates, npm, Go/Python dependencies, generated protobuf Go/Rust outputs).
7. **Exact diffs and behavior:** produce a reviewed file-scoped `CURRENT → PROPOSED` patch, no production redesign. Normative basis and demonstrated RED are mandatory if any change affects production behavior; licensing-only documentation does not imply a semantic version bump.
8. **Execution control:** separate explicit authorization; re-check branch, HEAD, clean tree, exact path allowlist; stage explicit files only; signed commit with verified signature; publish/merge/release only after independent approvals.

## 7. Proposed execution grouping (one amendment, one review)

**A — Documentary adjudication:** a single explicit implementation file matrix (every included path, owner, current grant, proposed public expression, commercial grantor, exclusion reason where applicable).
**B — Patch candidate:** one minimally scoped set of LICENSE/NOTICE/manifest/header changes **prepared only after** A is approved; textual diff verified against the matrix.
**C — Controlled commit:** one signed documentation/licensing commit or clearly separated atomic commits only if differing legal grants require it; **no automatic push, merge, release or runtime promotion**.

No recurring broad audits are required: any further fact-finding is a **specific unresolved rights question**, not another inventory phase.

## 8. FACT / INFERENCE / OPEN QUESTION / DECISION

- **FACT:** policy commit `f6b392cc354d51beaa8f608600f09db1abecfc77` was reported signed, published, and clean; Slice 8 classified the four language surfaces, while Rust Cargo still declares CC BY 4.0.
- **INFERENCE:** one reviewed cross-language amendment is feasible without modification of the normative specification, provided file-level rights are established.
- **OPEN QUESTION:** exact SPDX variant; licensor authorization; file-by-file Python/conformance treatment; generated-code and dependencies distribution notices.
- **DECISION:** prepare this amendment proposal. **No license-effective repository modifications authorized yet.**

```text
PHASE_B_DOCUMENTARY_POLICY=CLOSED_AND_PUBLISHED
PHASE_B_AMENDMENT_PACKAGE=PROPOSED
NORMATIVE_LICENSE=CC_BY_4_0_PRESERVE
IMPLEMENTATION_LICENSE_TARGET=AGPLv3_PLUS_SEPARATE_COMMERCIAL
EXACT_SPDX=UNRESOLVED
FILE_SCOPE_AND_RIGHTS=UNRESOLVED
EFFECTIVE_RELICENSING=NO
REPO_MUTATION=NO
COMMIT=NO
PUSH=NO
MERGE=NO
RELEASE=NO
```

## Rust Cargo Licensing Implementation — 2026-10-09

The historical Rust licensing baseline recorded in this document
identified `hacp-rs/Cargo.toml` as declaring `CC-BY-4.0`.

That baseline has since been superseded for current Cargo package
metadata by the following signed implementation commit:

`d738d893d1413f2dc7c701422c5df85b0c0a4415`

The exact metadata change was:

```diff
-license = "CC-BY-4.0"
+license = "AGPL-3.0-only"
```

This aligns the Rust package's declared AGPL version with the existing AGPLv3 licensing model of the Go enforcement sidecar.

The `hacp-rs` package version remains `0.1.1`. Rust source code, `Cargo.lock`, cryptographic behavior, and enforcement semantics were not modified by this commit.

Previously granted CC BY 4.0 rights are not revoked. Separate commercial licensing remains subject to applicable agreements and grants.

Historical SHA-256 values and original decision records in this document remain evidence of their respective earlier states, rather than hashes of the updated Cargo manifest.
