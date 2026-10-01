//! ControlState — freshness tracker for distributed control state.
//!
//! Reports whether locally held distributed control state can be trusted.
//!
//! Rules:
//! - explicit unsafe → false
//! - no valid snapshot/event/heartbeat ever observed → false
//! - age <= maxStaleness → true
//! - age > maxStaleness → false
//!
//! Connectivity is deliberately not part of this decision.

use std::sync::RwLock;
use std::time::{Duration, Instant};

pub struct ControlState {
    inner: RwLock<Inner>,
    max_staleness: Duration,
}

struct Inner {
    last_seen_revision: u64,
    last_update: Option<Instant>,
    connected: bool,
    unsafe_flag: bool,
}

impl ControlState {
    pub fn new(max_staleness: Duration) -> Self {
        let max_staleness = if max_staleness.is_zero() {
            Duration::from_secs(5)
        } else {
            max_staleness
        };

        Self {
            inner: RwLock::new(Inner {
                last_seen_revision: 0,
                last_update: None,
                connected: false,
                unsafe_flag: false,
            }),
            max_staleness,
        }
    }

    pub fn last_seen_revision(&self) -> u64 {
        self.inner.read().unwrap().last_seen_revision
    }

    pub fn connected(&self) -> bool {
        self.inner.read().unwrap().connected
    }

    pub fn unsafe_flag(&self) -> bool {
        self.inner.read().unwrap().unsafe_flag
    }

    /// Record transport connectivity.
    /// Connectivity alone does NOT refresh freshness.
    pub fn mark_connected(&self) {
        self.inner.write().unwrap().connected = true;
    }

    pub fn mark_disconnected(&self) {
        self.inner.write().unwrap().connected = false;
    }

    /// Record successful atomic materialization of a complete snapshot.
    pub fn mark_snapshot(&self, revision: u64) {
        let mut inner = self.inner.write().unwrap();
        inner.last_seen_revision = revision;
        inner.last_update = Some(Instant::now());
        inner.unsafe_flag = false;
    }

    /// Record successful materialization of one ordered revision.
    pub fn mark_event(&self, revision: u64) {
        let mut inner = self.inner.write().unwrap();
        inner.last_seen_revision = revision;
        inner.last_update = Some(Instant::now());
        inner.unsafe_flag = false;
    }

    /// Record heartbeat evidence.
    /// A heartbeat MUST describe exactly the highest revision already
    /// materialized locally. Returns Err if mismatch.
    pub fn mark_heartbeat(&self, current_revision: u64) -> Result<(), String> {
        let mut inner = self.inner.write().unwrap();
        if current_revision != inner.last_seen_revision {
            inner.unsafe_flag = true;
            return Err(format!(
                "heartbeat revision mismatch: heartbeat={} last_seen={}",
                current_revision, inner.last_seen_revision
            ));
        }
        inner.last_update = Some(Instant::now());
        inner.unsafe_flag = false;
        Ok(())
    }

    /// Immediately invalidate locally held distributed control state.
    pub fn mark_unsafe(&self) {
        self.inner.write().unwrap().unsafe_flag = true;
    }

    /// Report whether local distributed control state may currently be trusted.
    pub fn is_fresh(&self) -> bool {
        let inner = self.inner.read().unwrap();

        if inner.unsafe_flag {
            return false;
        }

        let last_update = match inner.last_update {
            Some(t) => t,
            None => return false,
        };

        last_update.elapsed() <= self.max_staleness
    }
}
