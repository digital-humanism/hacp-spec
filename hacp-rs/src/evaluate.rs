//! HACP evaluation logic exercised by Core 38 and HC2-55 request-binding evidence.
//!
//! Covers the evaluation paths exercised by the HACP-Core vectors.
//! HC2-55 provides bounded revision-2 HTTP request-binding evidence.

use serde_json::Value;

use crate::ed25519::load_public_key;
use crate::ed25519_v2::verify_signature;
use crate::sha256::sha256_hex;
use crate::jcs::canonicalize;

/// Evaluation result.
#[derive(Debug, Clone)]
pub struct EvalResult {
    pub decision: String,
    pub reason_codes: Vec<String>,
    pub action_hash: String,
}

/// Helper: get string field from JSON object.
fn get_str<'a>(obj: &'a Value, key: &str) -> &'a str {
    obj.get(key).and_then(|v| v.as_str()).unwrap_or("")
}

/// Helper: get integer field.
fn get_int(obj: &Value, key: &str) -> Option<i64> {
    obj.get(key).and_then(|v| v.as_i64())
}

/// Helper: copy a JSON object without a given key.
fn copy_without(obj: &Value, exclude_key: &str) -> Value {
    if let Some(map) = obj.as_object() {
        let new_map: serde_json::Map<String, Value> = map
            .iter()
            .filter(|(k, _)| *k != exclude_key)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        Value::Object(new_map)
    } else {
        obj.clone()
    }
}

/// Verify provenance event (INV-4).
fn verify_provenance(
    event: &Value,
    prior: Option<&Value>,
    pubkey: &ed25519_dalek::VerifyingKey,
) -> bool {
    let payload = match event.get("payload") {
        Some(p) => p,
        None => return false,
    };
    let pb = match canonicalize(payload) {
        Ok(b) => b,
        Err(_) => return false,
    };
    let computed_hash = sha256_hex(&pb);
    if get_str(event, "payload_hash") != computed_hash {
        return false;
    }

    let genesis = "0000000000000000000000000000000000000000000000000000000000000000";
    let expected_prev = match prior {
        Some(p) if !p.is_null() => {
            let prb = match canonicalize(p) {
                Ok(b) => b,
                Err(_) => return false,
            };
            sha256_hex(&prb)
        }
        _ => genesis.to_string(),
    };
    if get_str(event, "prev_event_hash") != expected_prev {
        return false;
    }

    let ev_no_sig = copy_without(event, "signature");
    let evb = match canonicalize(&ev_no_sig) {
        Ok(b) => b,
        Err(_) => return false,
    };
    verify_signature(pubkey, &evb, get_str(event, "signature"))
}

/// Normalize a percent-encoded path for HC2 binding comparison.
fn hc2_normalize_request_target(target: &str) -> String {
    let mut result = String::with_capacity(target.len());
    let bytes = target.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = bytes[i + 1];
            let lo = bytes[i + 2];
            if hi.is_ascii_hexdigit() && lo.is_ascii_hexdigit() {
                result.push('%');
                result.push(hi.to_ascii_uppercase() as char);
                result.push(lo.to_ascii_uppercase() as char);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

/// HC2-55 request binding check.
fn hc2_request_binding_matches(request_target: &str, constraint_path: &str) -> bool {
    hc2_normalize_request_target(request_target) == hc2_normalize_request_target(constraint_path)
}

/// Evaluate HACP inputs using the Rust implementation.
/// Includes the HC2-55 request-binding path exercised by the bounded evidence set.
pub fn evaluate(inputs: &Value, context: &Value, pubkey_hex: &str) -> EvalResult {
    let empty = Value::Object(serde_json::Map::new());
    let action = inputs.get("proposed_action").unwrap_or(&empty);
    let envelope = inputs.get("intent_envelope").unwrap_or(&empty);
    let token = inputs.get("decision_token").unwrap_or(&Value::Null);
    let checkpoint = inputs.get("checkpoint");
    let checkpoint_state = inputs.get("checkpoint_state");
    let has_token = !token.is_null();

    let pubkey = match load_public_key(pubkey_hex) {
        Ok(k) => k,
        Err(_) => {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["SIGNATURE_FAILURE".into()],
                action_hash: String::new(),
            }
        }
    };

    // Compute action_hash
    let action_hash = match canonicalize(action) {
        Ok(bytes) => sha256_hex(&bytes),
        Err(_) => String::new(),
    };

    // ============================================================
    // Step 1: Checkpoint pre-evaluation
    // ============================================================
    if let Some(cp) = checkpoint {
        if !cp.is_null() {
            let clock = get_int(context, "clock")
                .or_else(|| get_int(context, "current_time"))
                .unwrap_or(0);
            let mut state = get_str(cp, "state").to_string();
            if state.is_empty() {
                state = get_str(cp, "status").to_string();
            }
            if let Some(exp) = get_int(cp, "expires_at") {
                if state == "OPEN" && clock > exp {
                    state = "EXPIRED".into();
                }
            }
            match state.as_str() {
                "EXPIRED" | "RESOLVED_DENY" => {
                    return EvalResult {
                        decision: "DENY".into(),
                        reason_codes: vec!["CHECKPOINT_TIMEOUT".into()],
                        action_hash,
                    }
                }
                "OPEN" => {
                    return EvalResult {
                        decision: "CHECKPOINT".into(),
                        reason_codes: vec![],
                        action_hash,
                    }
                }
                "RESOLVED_ALLOW" => {
                    let kind = get_str(cp, "resolver_principal_kind");
                    let alt = get_str(cp, "resolved_by_kind");
                    let resolver = if !kind.is_empty() { kind } else { alt };
                    if resolver == "system" {
                        return EvalResult {
                            decision: "DENY".into(),
                            reason_codes: vec!["HUMAN_RESOLUTION_REQUIRED".into()],
                            action_hash,
                        };
                    }
                    // Human-approved checkpoint continues
                }
                _ => {
                    return EvalResult {
                        decision: "DENY".into(),
                        reason_codes: vec!["CHECKPOINT_TIMEOUT".into()],
                        action_hash,
                    }
                }
            }
        }
    }

    // ============================================================
    // Step 1b: Checkpoint state (from inputs.checkpoint_state)
    // ============================================================
    if let Some(cs) = checkpoint_state {
        if !cs.is_null() {
            let clock = get_int(context, "clock")
                .or_else(|| get_int(context, "current_time"))
                .unwrap_or(0);
            let created_at = get_int(cs, "created_at").unwrap_or(0);
            let timeout = get_int(cs, "timeout_seconds")
                .or_else(|| get_int(context, "checkpoint_timeout_seconds"))
                .unwrap_or(0);
            if timeout > 0 && clock > created_at + timeout {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["CHECKPOINT_TIMEOUT".into()],
                    action_hash,
                };
            }
        }
    }

    // ============================================================
    // Step 2: Clock and envelope expiry
    // ============================================================
    let current_time = get_int(context, "clock")
        .or_else(|| get_int(context, "current_time"))
        .or_else(|| get_int(envelope, "issued_at"))
        .unwrap_or(0);

    // Checkpoint timeout from policy_context (when checkpoint_state not sent)
    if checkpoint_state.is_none() || checkpoint_state.unwrap().is_null() {
        if let Some(timeout) = get_int(context, "checkpoint_timeout_seconds") {
            if timeout > 0 {
                let verb = get_str(action, "verb");
                let is_system = get_str(envelope, "principal_kind") == "system";
                let needs_human = context
                    .get("human_required_verbs")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().any(|r| r.as_str() == Some(verb)))
                    .unwrap_or(false);
                if is_system && needs_human {
                    let wait_start = get_int(action, "proposed_at")
                        .or_else(|| get_int(envelope, "issued_at"))
                        .unwrap_or(0);
                    if wait_start > 0 && current_time > wait_start + timeout {
                        return EvalResult {
                            decision: "DENY".into(),
                            reason_codes: vec!["CHECKPOINT_TIMEOUT".into()],
                            action_hash,
                        };
                    }
                }
            }
        }
    }

    // Token expiry (before envelope expiry when token present)
    if has_token {
        if let Some(tok_exp) = get_int(token, "expires_at") {
            if current_time > tok_exp {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["TOKEN_EXPIRED".into()],
                    action_hash,
                };
            }
        }
    }

    if let Some(exp) = get_int(envelope, "expires_at") {
        if current_time > exp {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["ENVELOPE_EXPIRED".into()],
                action_hash,
            };
        }
    }

    // ============================================================
    // Step 3: Envelope revocation
    // ============================================================
    if let Some(rev) = context.get("revoked_envelopes").and_then(|v| v.as_array()) {
        let eid = get_str(envelope, "envelope_id");
        if rev.iter().any(|r| r.as_str() == Some(eid)) {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["ENVELOPE_REVOKED".into()],
                action_hash,
            };
        }
        let parent = get_str(envelope, "parent_envelope_id");
        if !parent.is_empty() && rev.iter().any(|r| r.as_str() == Some(parent)) {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["ENVELOPE_REVOKED".into()],
                action_hash,
            };
        }
    }

    // ============================================================
    // Step 4: Key revocation
    // ============================================================
    if let Some(rk) = context.get("revoked_keys").and_then(|v| v.as_array()) {
        let env_key = get_str(envelope, "signer_key_id");
        if rk.iter().any(|r| r.as_str() == Some(env_key)) {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["KEY_REVOKED".into()],
                action_hash,
            };
        }
        if has_token {
            let tok_key = get_str(token, "signer_key_id");
            if rk.iter().any(|r| r.as_str() == Some(tok_key)) {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["KEY_REVOKED".into()],
                    action_hash,
                };
            }
        }
    }

    // ============================================================
    // Step 5: HMAC rejection
    // ============================================================
    let env_key_lower = get_str(envelope, "signer_key_id").to_lowercase();
    if env_key_lower.contains("hmac") {
        return EvalResult {
            decision: "DENY".into(),
            reason_codes: vec!["SIGNATURE_FAILURE".into()],
            action_hash,
        };
    }
    if has_token {
        let tok_key_lower = get_str(token, "signer_key_id").to_lowercase();
        if tok_key_lower.contains("hmac") {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["SIGNATURE_FAILURE".into()],
                action_hash,
            };
        }
    }

    // ============================================================
    // Step 6: Token revocation (expiry already checked in Step 2)
    // ============================================================
    if has_token {
        if let Some(rev) = context.get("revoked_tokens").and_then(|v| v.as_array()) {
            let tid = get_str(token, "token_id");
            if rev.iter().any(|r| r.as_str() == Some(tid)) {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["TOKEN_REVOKED".into()],
                    action_hash,
                };
            }
        }
    }

    // ============================================================
    // Step 7: Trusted keys
    // ============================================================
    if let Some(trusted) = context.get("trusted_keys").and_then(|v| v.as_array()) {
        let env_key = get_str(envelope, "signer_key_id");
        if !trusted.iter().any(|r| r.as_str() == Some(env_key)) {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["SIGNATURE_FAILURE".into()],
                action_hash,
            };
        }
        if has_token {
            let tok_key = get_str(token, "signer_key_id");
            if !trusted.iter().any(|r| r.as_str() == Some(tok_key)) {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["SIGNATURE_FAILURE".into()],
                    action_hash,
                };
            }
        }
    }

    // ============================================================
    // Step 8: Human final decision (INV-1)
    // ============================================================
    if let Some(human_verbs) = context
        .get("human_required_verbs")
        .and_then(|v| v.as_array())
    {
        let verb = get_str(action, "verb");
        if human_verbs.iter().any(|r| r.as_str() == Some(verb))
            && get_str(envelope, "principal_kind") == "system"
        {
            let parent = get_str(envelope, "parent_envelope_id");
            if parent.is_empty() {
                return EvalResult {
                    decision: "CHECKPOINT".into(),
                    reason_codes: vec!["HUMAN_REQUIRED".into()],
                    action_hash,
                };
            }
        }
    }

    // ============================================================
    // Step 9: Bounded autonomy (INV-7)
    // ============================================================
    let has_budget = if let Some(budget) = envelope.get("autonomy_budget") {
        if let Some(max_actions) = get_int(budget, "max_actions") {
            let current = get_int(context, "current_action_count").unwrap_or(0);
            if current >= max_actions {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["BUDGET_EXHAUSTED".into()],
                    action_hash,
                };
            }
            true
        } else {
            false
        }
    } else {
        false
    };

    // ============================================================
    // Step 10: Boundary re-authorization (INV-2)
    // ============================================================
    let scope = match envelope.get("scope") {
        Some(s) if s.is_object() => s,
        _ => {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["BOUNDARY_CROSSING".into()],
                action_hash,
            }
        }
    };

    let checks = [
        ("audience", "audiences"),
        ("reversibility", "reversibility"),
        ("externality", "externality"),
        ("data_class", "data_classes"),
        ("verb", "verbs"),
        ("resource_class", "resource_classes"),
    ];
    for (attr, key) in &checks {
        if let Some(allowed) = scope.get(key) {
            if let Some(arr) = allowed.as_array() {
                // Check if the attribute is present in the action
                let val = action.get(*attr);
                if val.is_none() || val.unwrap().as_str().is_none() {
                    // Attribute missing from action
                    return EvalResult {
                        decision: "DENY".into(),
                        reason_codes: vec!["UNKNOWN_ATTRIBUTE".into()],
                        action_hash,
                    };
                }
                let s = val.unwrap().as_str().unwrap();
                if !arr.iter().any(|r| r.as_str() == Some(s)) {
                    return EvalResult {
                        decision: "DENY".into(),
                        reason_codes: vec!["BOUNDARY_CROSSING".into()],
                        action_hash,
                    };
                }
            }
        }
    }

    // Quantity
    if let Some(q) = get_int(action, "quantity") {
        if let Some(max_q) = get_int(scope, "max_quantity") {
            if q > max_q {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["SCOPE_EXCEEDED".into()],
                    action_hash,
                };
            }
        }
    }

    // Destination allowlist
    if let Some(allowed) = scope.get("destinations").and_then(|v| v.as_array()) {
        if !allowed.is_empty() {
            let dest = get_str(action, "destination");
            if dest.is_empty() || !allowed.iter().any(|r| r.as_str() == Some(dest)) {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["BOUNDARY_CROSSING".into()],
                    action_hash,
                };
            }
        }
    }

    // Tool allowlist
    if let Some(allowed) = scope.get("tool_names").and_then(|v| v.as_array()) {
        if !allowed.is_empty() {
            if action.get("tool_name").is_none() {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["UNKNOWN_ATTRIBUTE".into()],
                    action_hash,
                };
            }
            let tool = get_str(action, "tool_name");
            if tool.is_empty() || !allowed.iter().any(|r| r.as_str() == Some(tool)) {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["BOUNDARY_CROSSING".into()],
                    action_hash,
                };
            }
        }
    }

    // ============================================================
    // Step 11: Token crypto verification (INV-3, INV-5)
    // ============================================================
    if has_token {
        // Token-envelope binding (BEFORE action hash)
        if get_str(token, "envelope_id") != get_str(envelope, "envelope_id") {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["TOKEN_ENVELOPE_MISMATCH".into()],
                action_hash,
            };
        }

        // Action hash binding
        let canonical_action = match canonicalize(action) {
            Ok(b) => b,
            Err(_) => {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["INVALID_ACTION".into()],
                    action_hash,
                }
            }
        };
        let computed = sha256_hex(&canonical_action);
        if get_str(token, "action_hash") != computed {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["HASH_MISMATCH".into()],
                action_hash: computed,
            };
        }

        // Token signature verification
        let token_no_sig = copy_without(token, "signature");
        let payload = match canonicalize(&token_no_sig) {
            Ok(b) => b,
            Err(_) => {
                return EvalResult {
                    decision: "DENY".into(),
                    reason_codes: vec!["SIGNATURE_FAILURE".into()],
                    action_hash,
                }
            }
        };
        if !verify_signature(&pubkey, &payload, get_str(token, "signature")) {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["SIGNATURE_FAILURE".into()],
                action_hash,
            };
        }
    }

    // ============================================================
    // Step 12: No token path — check for missing provenance
    // ============================================================
    if !has_token && !has_budget {
        let provenance = inputs.get("provenance_event");
        let has_prov = provenance.map(|p| !p.is_null()).unwrap_or(false);
        if !has_prov {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["TRACEABILITY_MISSING".into()],
                action_hash,
            };
        }
    }

    // ============================================================
    // Step 13: Provenance verification (INV-4)
    // ============================================================
    let provenance = inputs.get("provenance_event");
    let prior = inputs.get("prior_provenance_event");
    let omit = inputs
        .get("omit_provenance")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if omit {
        return EvalResult {
            decision: "DENY".into(),
            reason_codes: vec!["TRACEABILITY_MISSING".into()],
            action_hash,
        };
    }
    if let Some(prov) = provenance {
        if !prov.is_null() && !verify_provenance(prov, prior, &pubkey) {
            return EvalResult {
                decision: "DENY".into(),
                reason_codes: vec!["TRACEABILITY_FAILURE".into()],
                action_hash,
            };
        }
    }

    // ============================================================
    // Step 14: HC2-55 request binding
    // ============================================================
    if let Some(http_req) = inputs.get("http_request") {
        if !http_req.is_null() {
            let request_target = http_req
                .get("request_target")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if let Some(constraints) = token.get("constraints") {
                if let Some(constraint_path) = constraints.get("path").and_then(|v| v.as_str()) {
                    if !hc2_request_binding_matches(request_target, constraint_path) {
                        return EvalResult {
                            decision: "DENY".into(),
                            reason_codes: vec!["SCOPE_EXCEEDED".into()],
                            action_hash,
                        };
                    }
                }
            }
        }
    }

    // ============================================================
    // Final: ALLOW
    // ============================================================
    EvalResult {
        decision: "ALLOW".into(),
        reason_codes: vec![],
        action_hash,
    }
}
