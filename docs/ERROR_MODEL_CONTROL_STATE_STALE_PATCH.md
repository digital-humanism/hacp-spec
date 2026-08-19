# Error model patch — CONTROL_STATE_STALE

Add the following normative reason code to `hacp-spec/error-model.md`.

```markdown
### `CONTROL_STATE_STALE`

The sidecar does not have sufficiently fresh distributed control-plane state to safely authorize execution.

This condition is fail-closed.

Typical causes include:

- control-plane connectivity has been unavailable beyond the configured maximum staleness interval;
- a revision gap was observed;
- an invalid or inconsistent heartbeat was received;
- the subscriber detected unsafe distributed state and has not yet completed snapshot recovery.

A temporary network disconnect does not immediately imply `CONTROL_STATE_STALE`.

The sidecar may continue using its last fully materialized control state while that state remains within the configured freshness interval.

Once that interval is exceeded, authorization MUST fail closed until fresh control state is established.

Recovery from this state requires one of:

- a valid revocation event continuing the expected revision sequence;
- a valid heartbeat consistent with the fully materialized revision;
- successful snapshot recovery following `ResetRequired`.
```
