# README patch — Gate E

Use this patch in the public `hacp-spec/README.md`.

## Replace status line

Replace:

```markdown
**Gate E** — Distributed management (gRPC control plane): ⏸ Pending
```

with:

```markdown
**Gate E** — Distributed management (gRPC control plane): ✅ Complete
```

## Add Gate E summary

```markdown
### Gate E — Distributed control plane

Gate E establishes the distributed revocation control plane for HACP sidecars.

Verified capabilities:

- gRPC control-plane contract;
- authoritative monotonic revision journal;
- atomic revocation snapshots;
- resumable server-streaming revocation feed;
- duplicate and old-event idempotency;
- revision-gap detection and fail-closed behavior;
- reconnect with bounded exponential backoff;
- replay from `last_seen_revision`;
- `ResetRequired` recovery through snapshot reload;
- heartbeat-based control-state freshness;
- stale control state fails closed with `CONTROL_STATE_STALE`;
- distributed revocation propagation into the real evaluation pipeline;
- deterministic convergence across multiple independent sidecars;
- CI regression coverage.

Recovery model:

```text
startup
  ↓
snapshot @ revision R
  ↓
WatchRevocations(after_revision=R)
  ↓
live events

disconnect
  ↓
reconnect(after_revision=last_seen_revision)
  ↓
replay missed events
  ↓
live events

replay unavailable
  ↓
ResetRequired
  ↓
fresh snapshot
  ↓
resume stream
```

`revision` is the durable global control-plane mutation order.

`sequence` is stream-local transport ordering and is not persisted.

`last_seen_revision` means the highest revision that has been fully materialized into local sidecar state.
```
