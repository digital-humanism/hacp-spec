//! Journal — in-memory revocation journal.
//!
//! Properties:
//! - revision is globally monotonic
//! - revocation state is monotonic (entries added, not removed)
//! - duplicate revocation of same (kind, subject_id) is idempotent
//! - events contains full history for replay

use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

/// Revocation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevocationKind {
    Key = 1,
    Token = 2,
    Envelope = 3,
    ParentEnvelope = 4,
}

impl RevocationKind {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            1 => Some(Self::Key),
            2 => Some(Self::Token),
            3 => Some(Self::Envelope),
            4 => Some(Self::ParentEnvelope),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RevocationEvent {
    pub revision: u64,
    pub event_id: String,
    pub kind: i32,
    pub subject_id: String,
    pub issued_at_ms: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RevocationEntry {
    pub kind: i32,
    pub subject_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RevocationSnapshot {
    pub revision: u64,
    pub entries: Vec<RevocationEntry>,
    pub generated_at_ms: i64,
}

impl Default for Journal {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Journal {
    inner: RwLock<JournalInner>,
}

struct JournalInner {
    revision: u64,
    events: Vec<RevocationEvent>,
    revoked: HashMap<RevocationKind, HashSet<String>>,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(JournalInner {
                revision: 0,
                events: Vec::new(),
                revoked: HashMap::new(),
            }),
        }
    }

    pub fn revision(&self) -> u64 {
        self.inner.read().unwrap().revision
    }

    /// Commit a revocation. Returns (event, true) for new revision,
    /// (None, false) for idempotent duplicate.
    pub fn revoke(
        &self,
        kind: RevocationKind,
        subject_id: &str,
    ) -> Result<Option<RevocationEvent>, String> {
        if subject_id.is_empty() {
            return Err("empty revocation subject id".into());
        }

        let mut inner = self.inner.write().unwrap();

        // Check duplicate before borrowing set
        let is_dup = inner
            .revoked
            .get(&kind)
            .map(|s| s.contains(subject_id))
            .unwrap_or(false);
        if is_dup {
            return Ok(None);
        }

        inner.revision += 1;
        let revision = inner.revision;

        let event = RevocationEvent {
            revision,
            event_id: format!("revocation-{:020}", revision),
            kind: kind as i32,
            subject_id: subject_id.to_string(),
            issued_at_ms: chrono_ms(),
        };

        inner
            .revoked
            .entry(kind)
            .or_default()
            .insert(subject_id.to_string());
        inner.events.push(event.clone());

        Ok(Some(event))
    }

    /// Returns complete revocation state at one atomic revision.
    pub fn snapshot(&self) -> RevocationSnapshot {
        let inner = self.inner.read().unwrap();

        let mut entries = Vec::new();
        for (kind, subjects) in &inner.revoked {
            for subject_id in subjects {
                entries.push(RevocationEntry {
                    kind: *kind as i32,
                    subject_id: subject_id.clone(),
                });
            }
        }

        entries.sort_by(|a, b| {
            a.kind
                .cmp(&b.kind)
                .then_with(|| a.subject_id.cmp(&b.subject_id))
        });

        RevocationSnapshot {
            revision: inner.revision,
            entries,
            generated_at_ms: chrono_ms(),
        }
    }

    /// Returns events with revision > after_revision.
    pub fn events_after(&self, after_revision: u64) -> Result<Vec<RevocationEvent>, String> {
        let inner = self.inner.read().unwrap();

        if after_revision > inner.revision {
            return Err(format!(
                "requested revision is ahead: after={} current={}",
                after_revision, inner.revision
            ));
        }

        if !inner.events.is_empty() {
            let oldest = inner.events[0].revision;
            if oldest > 0 && after_revision < oldest - 1 {
                return Err(format!(
                    "replay unavailable: after={} oldest_available={}",
                    after_revision, oldest
                ));
            }
        }

        let events: Vec<_> = inner
            .events
            .iter()
            .filter(|e| e.revision > after_revision)
            .cloned()
            .collect();

        Ok(events)
    }

    pub fn oldest_available_revision(&self) -> u64 {
        let inner = self.inner.read().unwrap();
        inner
            .events
            .first()
            .map(|e| e.revision)
            .unwrap_or(inner.revision + 1)
    }

    /// Remove replay history through the supplied revision.
    pub fn compact_through(&self, revision: u64) {
        let mut inner = self.inner.write().unwrap();
        let idx = inner
            .events
            .iter()
            .position(|e| e.revision > revision)
            .unwrap_or(inner.events.len());
        inner.events.drain(..idx);
    }
}

fn chrono_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
