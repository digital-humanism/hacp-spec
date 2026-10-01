//! RevocationStore — local revocation state with evaluator-context projection helpers.
//!
//! Snapshot replacement atomically swaps the local revocation state.

use std::sync::RwLock;

use super::state::ControlState;

/// Local revocation store used by evaluate.
/// Stores revoked keys, tokens, and envelopes.
pub struct RevocationStore {
    inner: RwLock<StoreInner>,
    pub control_state: ControlState,
}

struct StoreInner {
    revoked_keys: std::collections::HashSet<String>,
    revoked_tokens: std::collections::HashSet<String>,
    revoked_envelopes: std::collections::HashSet<String>,
}

impl RevocationStore {
    pub fn new(max_staleness: std::time::Duration) -> Self {
        Self {
            inner: RwLock::new(StoreInner {
                revoked_keys: std::collections::HashSet::new(),
                revoked_tokens: std::collections::HashSet::new(),
                revoked_envelopes: std::collections::HashSet::new(),
            }),
            control_state: ControlState::new(max_staleness),
        }
    }

    pub fn is_key_revoked(&self, id: &str) -> bool {
        self.inner.read().unwrap().revoked_keys.contains(id)
    }

    pub fn is_token_revoked(&self, id: &str) -> bool {
        self.inner.read().unwrap().revoked_tokens.contains(id)
    }

    pub fn is_envelope_revoked(&self, id: &str) -> bool {
        self.inner.read().unwrap().revoked_envelopes.contains(id)
    }

    pub fn revoke_key(&self, id: &str) {
        self.inner
            .write()
            .unwrap()
            .revoked_keys
            .insert(id.to_string());
    }

    pub fn revoke_token(&self, id: &str) {
        self.inner
            .write()
            .unwrap()
            .revoked_tokens
            .insert(id.to_string());
    }

    pub fn revoke_envelope(&self, id: &str) {
        self.inner
            .write()
            .unwrap()
            .revoked_envelopes
            .insert(id.to_string());
    }

    /// Atomically replace all revocations from a snapshot.
    pub fn replace_revocations(&self, keys: &[String], tokens: &[String], envelopes: &[String]) {
        let mut inner = self.inner.write().unwrap();
        inner.revoked_keys = keys.iter().cloned().collect();
        inner.revoked_tokens = tokens.iter().cloned().collect();
        inner.revoked_envelopes = envelopes.iter().cloned().collect();
    }

    /// Inject current revocations into a policy_context JSON value.
    /// This bridges the control plane with the existing evaluate function.
    pub fn inject_into_context(&self, context: &mut serde_json::Value) {
        let inner = self.inner.read().unwrap();

        if !inner.revoked_keys.is_empty() {
            let keys: Vec<serde_json::Value> = inner
                .revoked_keys
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect();
            context["revoked_keys"] = serde_json::Value::Array(keys);
        }

        if !inner.revoked_tokens.is_empty() {
            let tokens: Vec<serde_json::Value> = inner
                .revoked_tokens
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect();
            context["revoked_tokens"] = serde_json::Value::Array(tokens);
        }

        if !inner.revoked_envelopes.is_empty() {
            let envelopes: Vec<serde_json::Value> = inner
                .revoked_envelopes
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect();
            context["revoked_envelopes"] = serde_json::Value::Array(envelopes);
        }
    }
}
