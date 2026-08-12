# HACP Checkpoint Protocol (Runtime Profile)

**Version:** 0.9.0-draft
**Status:** Draft (Phase 3, does not block Core 1.0.0)
**License:** CC BY 4.0

A checkpoint is a *human-final-decision pause*, not "a human sitting at a
console for 300 seconds". This document defines the normative state machine,
messages, and storage policy for the Runtime profile.

## 1. State Machine

```
                 evaluate() -> CHECKPOINT
                          |
                          v
                       [ OPEN ] ---------------- clock > expires_at
                          |      \                          |
              human approve       \ human reject            v
              (valid signature)    \                  [ EXPIRED ]
                          |         \                       |
                          v          v                      v
                  [ RESOLVED_ALLOW ] [ RESOLVED_DENY ]   DENY
                          |                |          (CHECKPOINT_TIMEOUT)
                          v                v
                 issue DecisionToken    final DENY
                 resume action
```

Terminal states: `RESOLVED_ALLOW`, `RESOLVED_DENY`, `EXPIRED`.
An `OPEN` checkpoint never yields `ALLOW` on its own.

## 2. Checkpoint Record

| Field | Type | Note |
|-------|------|------|
| `checkpoint_id` | uuid | unique |
| `envelope_id` | uuid | originating envelope |
| `action_hash` | hex64 | SHA-256(JCS(proposed_action)) — no cleartext action |
| `created_at` | int | unix |
| `expires_at` | int | unix; timeout boundary |
| `state` | enum | OPEN / RESOLVED_ALLOW / RESOLVED_DENY / EXPIRED |
| `resolver_principal` | string | set on resolution; MUST be human |
| `resolution_signature` | b64url | human signature over resolution payload |

## 3. Messages

**CHECKPOINT response** (from `evaluate`):

```json
{
  "decision": "CHECKPOINT",
  "checkpoint_id": "88888888-...",
  "reauthorization_required": true,
  "expires_at": 1786000400
}
```

**Poll:** `GET /checkpoint/{id}` → `{ "state": "...", "expires_at": ... }`

**Webhook:** `POST {notify_url}` on every state transition, signed.

## 4. Resolution Rules (Normative)

1. Only a **human** principal with a valid signature over the resolution
   payload may transition `OPEN → RESOLVED_*`. A system principal attempting
   to resolve its own checkpoint MUST be denied.
2. `OPEN → RESOLVED_ALLOW` issues a `DecisionToken` bound to the checkpoint's
   `action_hash`. Resume is permitted only with a valid token whose
   `action_hash` equals the pending action's hash (INV-3).
3. `OPEN → RESOLVED_DENY` is final.
4. `clock > expires_at` while `OPEN` → `EXPIRED` → `DENY(CHECKPOINT_TIMEOUT)`.
   Expiry is fail-closed: it never yields `ALLOW`.
5. Every transition emits a signed provenance event.

## 5. State Storage Policy

- Store **hashes**, never cleartext actions or secrets.
- Checkpoint state is append-only; transitions are provenance events.
- Cleartext intent or payload in checkpoint storage is non-conforming.

## 6. Notification Payload (Optional)

```json
{
  "checkpoint_id": "88888888-...",
  "state": "OPEN",
  "summary": "Irreversible external action requires human approval",
  "expires_at": 1786000400,
  "approve_url": "https://.../approve/88888888-...",
  "deny_url": "https://.../deny/88888888-..."
}
```

`summary` MUST NOT contain cleartext confidential payload.