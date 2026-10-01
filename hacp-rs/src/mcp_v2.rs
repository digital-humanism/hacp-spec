//! MCP Adapter v2 — Model B: Deterministic Transport-Derived ProposedAction.
//!
//! This is a **behavioral successor** to mcp-adapter-v1 (mcp.rs).
//! v1 is preserved unchanged.
//!
//! The adapter deterministically derives the hash-relevant ProposedAction
//! from transport-observed MCP facts plus the authenticated envelope scope.
//!
//! Synthesis rule (deterministic, no clock):
//!
//! ```text
//! hacp_version   = "0.9"
//! verb           = "call"
//! resource_class = first envelope.scope.resource_classes
//! resource_id    = MCP params.name
//! audience       = first envelope.scope.audiences
//! reversibility  = first envelope.scope.reversibility
//! externality    = first envelope.scope.externality
//! data_class     = first envelope.scope.data_classes
//! tool_name      = MCP params.name
//! ```
//!
//! No `proposed_at`, `action_id`, `envelope_id`, `payload_hash`, or
//! any time-dependent value enters the synthesized action.
//!
//! The evaluation clock is captured fresh per `tools/call` and enters
//! `policy_context.clock` only — NOT the ProposedAction.
//!
//! Known gaps:
//! - MCP arguments are NOT included in the action hash (no args_hash)
//! - evaluate() does not check token.decision (authorization-level gap)

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde_json::Value;

use crate::evaluate;

// ============================================================
// Param extraction
// ============================================================

fn extract_hacp_param(params: &Value, key: &str) -> Result<Value, String> {
    let val = match params.get(key) {
        Some(v) => v,
        None => return Err(format!("missing {} param", key)),
    };
    if val.is_object() {
        return Ok(val.clone());
    }
    if let Some(s) = val.as_str() {
        let bytes = URL_SAFE_NO_PAD
            .decode(s.trim())
            .map_err(|e| format!("{} base64url decode: {}", key, e))?;
        return serde_json::from_slice(&bytes).map_err(|e| format!("{} JSON parse: {}", key, e));
    }
    Err(format!("{} must be object or base64url string", key))
}

// ============================================================
// Deterministic ProposedAction synthesis (no clock)
// ============================================================

/// Extract the first string element from a JSON array, or empty string.
fn first_scope_value(scope: &Value, key: &str) -> String {
    scope
        .get(key)
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

/// Deterministically synthesize ProposedAction from transport facts
/// and envelope scope.
///
/// This function is PURE: no clock, no randomness, no side effects.
/// Same inputs → byte-equivalent output regardless of execution time.
pub fn synthesize_proposed_action(envelope: &Value, mcp_tool_name: &str) -> Value {
    let scope = envelope.get("scope").unwrap_or(&Value::Null);

    serde_json::json!({
        "hacp_version": "0.9",
        "verb": "call",
        "resource_class": first_scope_value(scope, "resource_classes"),
        "resource_id": mcp_tool_name,
        "audience": first_scope_value(scope, "audiences"),
        "reversibility": first_scope_value(scope, "reversibility"),
        "externality": first_scope_value(scope, "externality"),
        "data_class": first_scope_value(scope, "data_classes"),
        "tool_name": mcp_tool_name,
    })
}

// ============================================================
// tools/call handler (Model B)
// ============================================================

/// Handle a `tools/call` request with deterministic transport synthesis.
///
/// 1. Extract envelope and token from MCP params
/// 2. Synthesize ProposedAction from transport facts (deterministic, no clock)
/// 3. Capture fresh evaluation clock
/// 4. Call shared evaluate::evaluate()
pub fn handle_tools_call_v2(id: &Value, params: &Value, pubkey_hex: &str) -> Value {
    let mcp_tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");

    // Fail-closed: missing MCP tool name
    if mcp_tool_name.is_empty() {
        return serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32000,
                "message": "HACP DENY: INVALID_ACTION: missing MCP params.name"
            }
        });
    }

    // Extract envelope
    let envelope = match extract_hacp_param(params, "hacp_intent_envelope") {
        Ok(v) => v,
        Err(e) => {
            return serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32000,
                    "message": format!("HACP DENY: INVALID_ENVELOPE: {}", e)
                }
            });
        }
    };

    // Extract token
    let token = match extract_hacp_param(params, "hacp_decision_token") {
        Ok(v) => v,
        Err(e) => {
            return serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32000,
                    "message": format!("HACP DENY: SIGNATURE_FAILURE: {}", e)
                }
            });
        }
    };

    // Deterministic synthesis (no clock, no time)
    let proposed_action = synthesize_proposed_action(&envelope, mcp_tool_name);

    // Fresh evaluation clock — captured per call, NOT in proposed_action
    let eval_clock = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let context = serde_json::json!({ "clock": eval_clock });
    let inputs = serde_json::json!({
        "intent_envelope": envelope,
        "proposed_action": proposed_action,
        "decision_token": token,
    });

    // Shared evaluate — unchanged path
    let result = evaluate::evaluate(&inputs, &context, pubkey_hex);

    if result.decision == "ALLOW" {
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "content": [{ "type": "text", "text": "HACP ALLOW" }]
            }
        })
    } else {
        let reason = result
            .reason_codes
            .first()
            .map(|s| s.as_str())
            .unwrap_or("DENIED");
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32000,
                "message": format!("HACP DENY: {}", reason)
            }
        })
    }
}

// ============================================================
// MCP request dispatcher (v2)
// ============================================================

pub fn handle_mcp_request_v2(request: &Value, pubkey_hex: &str) -> Value {
    let id = request.get("id").cloned().unwrap_or(Value::Null);
    let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
    let params = request.get("params").unwrap_or(&Value::Null);

    match method {
        "initialize" => {
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": {
                        "name": "hacp-rs-mcp-v2",
                        "version": "0.2.0"
                    }
                }
            })
        }
        "notifications/initialized" => Value::Null,
        "tools/list" => {
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": { "tools": [] }
            })
        }
        "tools/call" => handle_tools_call_v2(&id, params, pubkey_hex),
        _ => {
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                }
            })
        }
    }
}

// ============================================================
// Server loop
// ============================================================

pub fn run_v2(pubkey_hex: &str) -> Result<(), String> {
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("stdin read: {}", e))?;
        if line.trim().is_empty() {
            continue;
        }

        let request: Value = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let err_resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": { "code": -32700, "message": format!("Parse error: {}", e) }
                });
                writeln!(out, "{}", serde_json::to_string(&err_resp).unwrap())
                    .map_err(|e| format!("write: {}", e))?;
                out.flush().map_err(|e| format!("flush: {}", e))?;
                continue;
            }
        };

        let response = handle_mcp_request_v2(&request, pubkey_hex);
        if response.is_null() {
            continue;
        }

        writeln!(out, "{}", serde_json::to_string(&response).unwrap())
            .map_err(|e| format!("write: {}", e))?;
        out.flush().map_err(|e| format!("flush: {}", e))?;
    }

    Ok(())
}
