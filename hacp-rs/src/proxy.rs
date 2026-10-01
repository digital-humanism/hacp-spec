//! HTTP_PROXY evaluation adapter for HACP enforcement.
//!
//! This module decodes HACP headers, synthesizes evaluation inputs from
//! already extracted HTTP request components, and delegates authorization
//! to the shared `evaluate` implementation.
//!
//! The HTTP server binary is responsible for request-target extraction and
//! HTTP response/status construction.
//!
//! Headers:
//! - `X-HACP-Intent-Envelope`: base64url-encoded JSON IntentEnvelope
//! - `X-HACP-Decision-Token`: base64url-encoded JSON DecisionToken

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde_json::Value;

use crate::evaluate;
use crate::sha256::sha256_hex;

const HEADER_ENVELOPE: &str = "x-hacp-intent-envelope";
const HEADER_TOKEN: &str = "x-hacp-decision-token";
const MAX_HEADER_SIZE: usize = 8 * 1024; // 8 KB per wire/encoding.md

/// Build the evaluate input JSON from HTTP request components.
///
/// Synthesizes proposed_action from HTTP method + envelope scope,
/// and http_request for HC2 binding.
pub fn build_evaluate_input(
    envelope: &Value,
    token: &Value,
    method: &str,
    request_target: &str,
    body_hash: &str,
    clock: i64,
) -> (Value, Value) {
    // Synthesize proposed_action from HTTP request + envelope scope
    let verb = method_to_verb(method);
    let empty_scope = Value::Object(serde_json::Map::new());
    let scope = envelope.get("scope").unwrap_or(&empty_scope);
    let first = |arr: Option<&Value>| -> String {
        arr.and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };

    let proposed_action = serde_json::json!({
        "hacp_version": "0.9",
        "verb": verb,
        "resource_class": first(scope.get("resource_classes")),
        "resource_id": request_target,
        "audience": first(scope.get("audiences")),
        "reversibility": first(scope.get("reversibility")),
        "externality": first(scope.get("externality")),
        "data_class": first(scope.get("data_classes")),
        "payload_hash": body_hash,
    });

    // http_request for HC2 binding
    let http_request = serde_json::json!({
        "method": method,
        "request_target": request_target,
    });

    let inputs = serde_json::json!({
        "intent_envelope": envelope,
        "proposed_action": proposed_action,
        "decision_token": token,
        "http_request": http_request,
    });

    let context = serde_json::json!({
        "clock": clock,
    });

    (inputs, context)
}

fn method_to_verb(method: &str) -> &'static str {
    match method.to_uppercase().as_str() {
        "GET" | "HEAD" => "read",
        "POST" | "PUT" | "PATCH" => "write",
        "DELETE" => "delete",
        _ => "unknown",
    }
}

/// Extract and decode a HACP header (base64url → JSON Value).
pub fn extract_hacp_header(headers: &[(String, String)], name: &str) -> Result<Value, String> {
    let name_lower = name.to_lowercase();
    let header_val = headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == name_lower)
        .map(|(_, v)| v.as_str())
        .ok_or_else(|| format!("missing {} header", name))?;

    if header_val.len() > MAX_HEADER_SIZE {
        return Err(format!(
            "{} header too large: {} > {}",
            name,
            header_val.len(),
            MAX_HEADER_SIZE
        ));
    }

    let bytes = URL_SAFE_NO_PAD
        .decode(header_val.trim())
        .map_err(|e| format!("{} base64url decode error: {}", name, e))?;

    serde_json::from_slice(&bytes).map_err(|e| format!("{} JSON parse error: {}", name, e))
}

/// Build a 403 DENY response body.
pub fn deny_response_body(reason: &str, request_id: &str) -> String {
    serde_json::json!({
        "decision": "DENY",
        "reason": reason,
        "request_id": request_id,
    })
    .to_string()
}

/// Build a 200 ALLOW response body.
pub fn allow_response_body(request_id: &str) -> String {
    serde_json::json!({
        "decision": "ALLOW",
        "request_id": request_id,
    })
    .to_string()
}

/// Run the HTTP proxy evaluation on a request.
///
/// Returns (decision, reason_codes, action_hash).
/// Fail-closed: any error → DENY.
pub fn proxy_evaluate(
    headers: &[(String, String)],
    method: &str,
    request_target: &str,
    body: &[u8],
    pubkey_hex: &str,
    clock: i64,
) -> (String, Vec<String>, String) {
    // 1. Extract HACP headers
    let envelope = match extract_hacp_header(headers, HEADER_ENVELOPE) {
        Ok(v) => v,
        Err(_) => {
            return (
                "DENY".into(),
                vec!["INVALID_ENVELOPE".into()],
                String::new(),
            );
        }
    };

    let token = match extract_hacp_header(headers, HEADER_TOKEN) {
        Ok(v) => v,
        Err(_) => {
            return (
                "DENY".into(),
                vec!["SIGNATURE_FAILURE".into()],
                String::new(),
            );
        }
    };

    // 2. Body hash
    let body_hash = sha256_hex(body);

    // 3. Build evaluate input
    let (inputs, context) =
        build_evaluate_input(&envelope, &token, method, request_target, &body_hash, clock);

    // 4. Call existing evaluate (SAME function as runner)
    let result = evaluate::evaluate(&inputs, &context, pubkey_hex);

    (result.decision, result.reason_codes, result.action_hash)
}
