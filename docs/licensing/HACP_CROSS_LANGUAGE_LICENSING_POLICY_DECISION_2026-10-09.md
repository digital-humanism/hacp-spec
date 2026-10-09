# HACP Cross-Language Licensing Policy Decision

**Date:** 2026-10-09
**Status:** ARCHITECTURAL POLICY ACCEPTED — IMPLEMENTATION NOT YET EFFECTIVE
**Scope:** `hacp-spec`, `hacp-sidecar`; `humanist-core` referenced only
**Nature:** Documentary decision; not a license grant, relicensing instrument, or release authorization.

## 1. Decision

HACP retains a vendor-neutral open specification. The intended licensing model for **project-owned executable implementations** in Python, TypeScript, Go, and Rust is **AGPLv3 plus a separately negotiated commercial license**, aligned with the established `hacp-sidecar` model. Go and Rust are recognized as peer **target** enforcement-runtime options; equivalent runtime capability, conformance, and deployment readiness remain subject to independent verification.

This is an **implementation licensing policy**, not evidence that every file is already AGPL-licensed. The public AGPL SPDX variant (`AGPL-3.0-only` vs `AGPL-3.0-or-later`) and commercial grantor authority have not yet been finalized.

## 2. Scope and boundaries

| Surface | Current evidence as of captured Git baseline | Accepted policy / preserved boundary |
|---|---|---|
| Normative HACP documents, protocol, `proto/**`, `schemas/**`, `vectors/**`, conformance materials | Root `hacp-spec/LICENSE` expressly declares CC BY 4.0 for the specification, schemas, conformance suites, and architectural texts | **Preserve CC BY 4.0** and applicable IETF submission/contribution rights |
| `hacp-go/` | No separate explicit Go implementation license identified | AGPLv3 + separately contracted commercial license **target**, subject to rights/scope clearance |
| `hacp-ts/` | `package.json` has no `license` field; `private: true` does not constitute a license | Same implementation licensing target |
| Python executable sources within `harness/` and `tools/` | Mixed conformance, tooling, and implementation surface; no comprehensive explicit code-level AGPL grant established | Same target **only for explicitly identified, cleared code files**; preserve conformance/normative content |
| `hacp-rs/` | `Cargo.toml` currently states `license = "CC-BY-4.0"` (`0.1.1`) | Same implementation licensing target; retain historical CC BY 4.0 grants |
| `hacp-sidecar` original Go runtime | `LICENSE.md` states AGPLv3 and commercial dual licensing | **Preserve existing licensing**, without assuming `-only` or `-or-later` variant |
| Generated Go/Rust bindings | Two generated Go protobuf files traced to shared `control_plane.proto`; Rust build uses that schema | Separate generated-source, upstream attribution, and distribution obligations |
| `humanist-core` Python SDK | Existing separately licensed, frozen historical baseline | **No changes** here; later additive, version-pinned composition metadata only |

The mapping above is a **target model** for implementation code. It does not automatically relicense directories, tests, normative data, standards text, generated files, or third-party dependencies.

## 3. Standards and historical-rights preservation

1. Do not modify the normative standard, its open CC BY 4.0 licensing, or IETF contribution/submission rights as part of implementation licensing work.
2. No retroactive cancellation of existing CC BY 4.0 grants. Any prospective additional licensing must be supported by the rights held by the actual licensor.
3. No exclusivity claims over third-party crates, packages, generated tool output, shared protobuf material, or independent HACP implementations.
4. Commercial contracts may cover only the rights actually controlled by the licensor; they do not waive third-party obligations.
5. No runtime behavior change, semantic version change, release, or deployment is implied by this decision.

## 4. Captured evidence

**Phase B / Slices 6–8 — read-only evidence, 2026-10-09.**

- `hacp-spec`: `admit/hacp-1.1.1-pre-release`, HEAD `5c7f61a6af402f21ad6374f9f896fa9f44784d09`, clean.
- `hacp-sidecar`: `admit/hacp-1.1.1-pre-release`, HEAD `9f7619d3086fc119e0223a9091cf837d70a8d5a3`, clean.
- Slice 8 audited **420 tracked paths**, scanned **168 source files**, and verified **two** generated Go protobuf files; final Git invariants passed.
- Slice 8 structural counts: normative proto/schemas/vectors **105**; Python `harness/` **20**; Python tools `*.py` **5**; TypeScript **14**; Go reference **11**; Rust **44**; sidecar Go runtime/tests **84**; generated Go **2**. These are classifications of tracked paths, not a legal ownership determination or a complete implementation allowlist.
- Evidence record: `HACP_PHASE_B_SLICE8_CONSOLIDATED_LICENSE_AUDIT_2026-10-09.txt` (internal read-only transcript). Git identity, repository metadata, and absence of detected copyright markers do not establish exclusive legal title.

## 5. Gates required before implementation

The accepted policy is **not an authorization to apply licensing changes**. Before any license/manifest/README amendment or redistribution under the new model:

1. Adjudicate an exact file-by-file implementation-only allowlist, especially Python harness/tests, and exclude normative/conformance assets as appropriate.
2. Verify copyright ownership, contributor authority, AI-assisted/copied-source concerns, and rights to grant **both** public and commercial licenses.
3. Decide the exact AGPL SPDX variant consistently and identify the commercial licensor/contract terms.
4. Preserve historical CC BY rights and IETF-related notices without suggesting withdrawal of previous grants.
5. Reconcile third-party license and attribution notices for the actual distribution artifact, including Cargo crates, other language dependencies, shared proto, and generated outputs.
6. Produce and review an exact **CURRENT → PROPOSED** file-scoped diff, establish normative basis and proven RED where a production change is in scope, and obtain explicit approval of the patch.
7. Verify branch/HEAD/clean tree before change; explicitly stage files; sign commits; verify signature and working-tree state; **no push or release without separate authorization**.

## 6. Implementation state

```text
ARCHITECTURAL_LICENSING_POLICY=ACCEPTED
STANDARD_NORMATIVE_CC_BY_4_0=PRESERVE
CROSS_LANGUAGE_IMPLEMENTATION_MODEL=AGPLv3_PLUS_COMMERCIAL_TARGET
GO_RUST_ENFORCEMENT_PEER_MODEL=ACCEPTED_TARGET
GO_RUST_RUNTIME_PARITY=NOT_ESTABLISHED_BY_THIS_RECORD
EXACT_AGPL_SPDX=OPEN
FILE_LEVEL_RIGHTS=OPEN
DISTRIBUTION_GATE=OPEN
LICENSE_CHANGE=NOT_AUTHORIZED_BY_THIS_RECORD
DOCUMENTATION_RECORD=AUTHORIZED
SOURCE_MUTATION=NO
RELEASE=NO
```

This record is a project policy decision, not a legal opinion or a substitute for an executable license grant.

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
