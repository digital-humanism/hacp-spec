# HACP IETF materials

This directory preserves the public submission artifacts and provenance records for the initial `-00` Internet-Drafts of the Human Agency Continuity Protocol (HACP).

The drafts were published on 2026-09-13 as individual submissions.

## Published Internet-Drafts

- [Human Agency Continuity Protocol (HACP) Architecture](https://datatracker.ietf.org/doc/draft-cassandres-hacp-agency-arch/) — intended status: **Informational**
- [Human Agency Continuity Protocol (HACP) Core](https://datatracker.ietf.org/doc/draft-cassandres-hacp-agency-core/) — intended status: **Experimental**

These documents are Internet-Drafts. Publication does not make them IETF Standards and does not imply IETF consensus or adoption.

## Repository baseline

The `-00` drafts restate the frozen HACP Specification v1.0.0 baseline:

- tag: `v1.0.0`
- commit: `c468c9bb0427448e564bcf3e7d9c8a3a004b8513`
- core conformance set: `core-0.9.2`
- wire `hacp_version`: `0.9`

Later HACP development is not part of the `-00` contract unless explicitly incorporated by a subsequent Internet-Draft revision.

## Files

- `draft-cassandres-hacp-agency-arch-00.xml` — submitted RFCXML source for the Architecture draft
- `draft-cassandres-hacp-agency-arch-00.txt` — published text form of the Architecture draft
- `draft-cassandres-hacp-agency-core-00.xml` — submitted RFCXML source for the Core draft
- `draft-cassandres-hacp-agency-core-00.txt` — published text form of the Core draft
- `DOCUMENT-CHARTER-SPLIT.md` — mapping from the frozen specification to the initial and later I-D boundaries
- `P1-CLAIM-CHECK.md` — normative provenance check for Core claims against the frozen v1.0.0 specification
- `SUBMISSION-RECORD-00.md` — publication metadata, validation results, hashes, and Datatracker references

## Integrity

The XML and TXT artifacts in this directory were verified byte-for-byte against the corresponding files in the IETF archive.

The normative provenance check is intentionally separate from the Internet-Drafts themselves. It documents the relationship between the IETF-facing restatement and the frozen HACP specification without creating additional protocol requirements.
