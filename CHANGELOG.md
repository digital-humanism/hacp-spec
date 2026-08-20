# Changelog

All notable changes to the HACP specification, canonical conformance vectors,
language-neutral harness, and in-repository verification tooling are documented
in this file.

This project follows a Keep a Changelog-style structure. Until a formal stable
release policy is established, version entries record specification and
conformance milestones rather than claiming production certification.

---

## [Unreleased]

### Added

- None yet.

### Changed

- None yet.

### Fixed

- None yet.

### Security

- None yet.

---

## [0.9.3-rc.1] - 2026-08-20

### Added

- Release-candidate documentation alignment for the HACP 0.9.3 specification corpus.
- Distributed control-plane specification artifacts for Gate E, including revocation streaming, snapshot recovery, revision continuity, replay, and stale-control-state semantics.
- `CONTROL_STATE_STALE` in the normative error model.

### Changed

- Aligned human-readable specification document headers to version `0.9.3`.
- Preserved the frozen HACP-Core `v0.9.2` conformance baseline and `core-0.9.2` vector set.
- Preserved Runner Protocol `1` and wire `hacp_version` `0.9`.
- Updated enforcement-profile documentation to reflect the published normative document.
- Updated canonicalization documentation to describe the already published conformance coverage.

### Fixed

- Corrected minor conformance documentation formatting.
- Removed obsolete release-readiness/debug artifacts.

### Security

- No cryptographic, vector, key, or wire-format semantics were changed in this release candidate.

---

## [0.9.2] - 2026-08-17

### Cross-language conformance baseline

HACP-Core v0.9.2 reached a reproducible cross-language conformance milestone
across Python, TypeScript, and Go against the same canonical vector set.

Final normative status:

```text
Python       38/38 PASS
TypeScript   38/38 PASS
Go           38/38 PASS

Normative failures:        0
Skipped normative vectors: 0
```

The canonical vector set is pinned by:

```text
Spec:          HACP-Core v0.9.2
Vector set:    core-0.9.2
Vectors:       38
Manifest:      verified
Vector digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

### Added

#### Canonical conformance baseline

- Established a complete 38-vector HACP-Core v0.9.2 executable baseline.
- Added manifest-pinned vector integrity verification.
- Standardized deterministic cross-language verification against the same
  canonical vector set.
- Preserved explicit timestamps and deterministic test cryptographic material
  for reproducibility.

#### Runner Protocol v1

- Added and validated the language-neutral black-box conformance runner
  protocol over JSON Lines on stdin/stdout.
- Defined ProtocolVersion `1`.
- Added clear separation between:
  - normative conformance failures;
  - harness/configuration failures;
  - runner execution/protocol failures.
- Defined exit codes:
  - `0` — conformant;
  - `1` — conformance failure;
  - `2` — harness/configuration error;
  - `3` — runner execution/protocol error.
- Established stderr-only diagnostics and stdout-only JSON responses for
  black-box runners.
- Added per-vector runner timeout handling.

#### TypeScript conformance layer

- Added a dedicated TypeScript conformance evaluator:
  - `hacp-ts/src/conformance.ts`
- Added canonical vector tests:
  - `hacp-ts/tests/conformance.test.ts`
- Added action-hash invariant tests:
  - `hacp-ts/tests/action-hash.test.ts`
- Added Node built-in test runner integration.
- Added strict TypeScript build/test workflow.
- Added duplicate-key raw JSON detection for malformed negative vectors.

Final TypeScript result:

```text
38/38 normative vectors PASS
44/44 total TypeScript tests PASS
0 failed
0 skipped
```

#### Go / sidecar conformance verification

- Rebuilt and validated:
  - `cmd/hacp-conformance-runner`
  - `cmd/sidecar`
- Verified Go repository build/test state with `go test ./...`.
- Executed the Go conformance runner through the manifest-verified
  `harness_runner.py` black-box path.
- Confirmed ProtocolVersion `1` interoperability.

Final Go conformance result:

```text
RESULTS: 38/38 passed
```

#### Cross-language documentation

Added or updated documentation for the conformance milestone:

```text
README.md
harness/README.md
harness/runner_protocol.md
vectors/README.md
tools/README.md
docs/conformance/HACP_TYPESCRIPT_GO_CONFORMANCE_REPORT.md
```

These documents now record:

- the current canonical vector digest;
- the 38-vector inventory;
- cross-language implementation status;
- runner protocol behavior;
- deterministic baking rules;
- manifest discipline;
- reproducibility commands;
- assurance boundaries.

### Changed

#### Canonical vector finalization

Two previously draft/placeholder cases were converted into fully baked,
deterministic normative vectors.

##### `CORE-INV3-002`

Final semantics:

```text
valid token
bound to original ProposedAction
presented ProposedAction changed afterward
→ HASH_MISMATCH
```

Expected result:

```text
DENY / HASH_MISMATCH
```

The vector now verifies that token binding changes when a
security-relevant action field is modified.

##### `CORE-INV5-002`

Final semantics:

```text
valid signed token
signed payload modified after signing
→ Ed25519 verification failure
```

Expected result:

```text
DENY / SIGNATURE_FAILURE
```

The vector now explicitly tests tampering of a signed payload rather than a
generic malformed signature fixture.

#### Conformance manifest

- Regenerated the canonical vector-set digest after finalizing the vector set.
- Replaced the previous stale digest with:

```text
sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

- Confirmed the harness verifies this digest before running the Go black-box
  conformance suite.

#### TypeScript verification architecture

- Preserved the runtime evaluator instead of making runtime code
  vector-specific.
- Added a separate clean-room conformance evaluator operating on raw wire
  dictionaries.
- Standardized structured conformance results including:
  - decision;
  - reason codes;
  - action hash;
  - canonical payload representations;
  - signature verification status;
  - provenance status where applicable.

#### Runner protocol documentation

- Clarified that ProtocolVersion `1` and HACP spec version `0.9.2` are separate
  version domains.
- Made `evaluate` the required operation for the current HACP-Core v0.9.2
  runner conformance path.
- Removed the earlier implication that `revoke` and `explain` were mandatory
  for the current Protocol v1 conformance run.

### Fixed

#### Duplicate JSON keys

- Fixed cross-language handling for `CORE-INV5-006`.
- JavaScript's default `JSON.parse()` behavior would otherwise silently keep
  the last duplicate key.
- Added a pre-parse malformed-JSON check so duplicate-key input is rejected
  rather than normalized.

Expected fail-closed behavior:

```text
DENY / INVALID_ACTION
```

#### TypeScript strict-mode parsing

- Fixed strict-nullability handling in the custom JSON number parser by using
  an explicit `throw` for invalid-number parsing paths.
- Preserved strict TypeScript compilation.

#### Harness stderr deadlock behavior

- Documented and hardened the runner harness behavior so non-verbose runner
  stderr does not fill an unread pipe and deadlock the subprocess.
- Current non-verbose runner execution discards diagnostic stderr rather than
  leaving it blocked in an unread `PIPE`.

### Security

#### Deterministic conformance test key

The public TEST ONLY Ed25519 identity is:

```text
seed phrase:
hacp-conformance-v0.9-key-001

derived seed:
4f656d4e80b0ae758c8035ece5fd076f443497f714a134c481ed72f58ed34017

raw public key:
9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3

key id:
key-ed25519-test-001
```

This material is intentionally deterministic and public for conformance
reproducibility.

It MUST NOT be used for production trust, signing, authentication, or
authorization.

#### Fail-closed behavior

The conformance baseline verifies fail-closed behavior across:

- malformed input;
- unknown or revoked keys;
- expired tokens/envelopes;
- token replay/binding mismatch;
- scope and boundary violations;
- exhausted autonomy budget;
- broken provenance;
- unresolved or invalid checkpoint state.

#### Manifest integrity gate

- Canonical vector drift is detected before conformance execution.
- Manifest mismatch is treated as a harness/configuration failure.
- Public conformance claims must not bypass the manifest gate.

### Verification evidence

The current milestone is supported by the following completed verification
runs.

#### TypeScript

```text
44 tests
44 pass
0 fail
0 skipped
```

Including:

```text
38 normative HACP-Core vectors
5 action-hash invariants
1 vector inventory test
```

#### Go

```text
go test ./...            PASS
conformance runner       38/38 PASS
manifest verification    PASS
```

#### Python interoperability and regression evidence

Although Python implementation details live in `humanist-core`, the same
canonical vector set has also been independently validated there:

```text
Python HACP-Core conformance: 38/38 PASS
Python full regression:      324/324 PASS
Statement coverage:             100%
Branch coverage:                100%
Python ↔ Go real sidecar E2E:    5/5 PASS
```

### Assurance boundary

This milestone establishes:

```text
normative executable conformance
manifest-pinned reproducibility
cross-language model convergence
black-box Go runner verification
real Python ↔ Go sidecar interoperability evidence
```

It does not by itself constitute:

```text
a formal proof of protocol correctness
a complete security audit
a fuzzing campaign
a property-based verification result
a production deployment certification
```

Those are separate assurance layers planned beyond the current conformance
baseline.

---

## Earlier history

Earlier specification and implementation work predates this changelog and is
not reconstructed here from incomplete commit history.

Future entries should be added prospectively so that protocol-significant
changes, vector-set changes, manifest digest updates, compatibility changes,
and security-relevant behavior remain traceable.

---

**Contact:** [digital.humanism.collective@protonmail.com](mailto:digital.humanism.collective@protonmail.com)
