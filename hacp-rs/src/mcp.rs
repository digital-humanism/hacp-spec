//! MCP transport adapter for HACP enforcement.
//!
//! JSON-RPC 2.0 over stdio. Same `evaluate` as runner and proxy.
//!
//! Supported methods:
//! - `initialize` → server capabilities
//! - `tools/list` → empty (enforcement-only)
//! - `tools/call` → HACP enforcement → ALLOW/DENY
//!
//! HACP parameters in `tools/call` params:
//! - `hacp_intent_envelope`: base64url-encoded JSON (or raw JSON object)
//! - `hacp_decision_token`: base64url-encoded JSON (or raw JSON object)
//!
//! Fail-closed: missing/invalid → error response.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde_json::Value;

use crate::evaluate;

/// Extract a HACP object from MCP request params.
///
/// Accepts either base64url-encoded string or raw JSON object.
fn extract_hacp_param(params: &Value, key: &str) -> Result<Value, String> {
    let val = match params.get(key) {
        Some(v) => v,
        None => return Err(format!("missing {} param", key)),
    };

    // Raw JSON object
    if val.is_object() {
        return Ok(val.clone());
    }

    // Base64url-encoded string
    if let Some(s) = val.as_str() {
        let bytes = URL_SAFE_NO_PAD
            .decode(s.trim())
            .map_err(|e| format!("{} base64url decode: {}", key, e))?;
        return serde_json::from_slice(&bytes).map_err(|e| format!("{} JSON parse: {}", key, e));
    }

    Err(format!("{} must be object or base64url string", key))
}

/// Build evaluate input from MCP tool call params.
fn build_mcp_evaluate_input(
    envelope: &Value,
    token: &Value,
    tool_name: &str,
    _arguments: &Value,
    clock: i64,
) -> (Value, Value) {
    let scope = envelope.get("scope").unwrap_or(&Value::Null);
    let first = |arr: Option<&Value>| -> String {
        arr.and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };

    // Synthesize proposed_action from MCP tool call
    let proposed_action = serde_json::json!({
        "hacp_version": "0.9",
        "verb": "call",
        "resource_class": first(scope.get("resource_classes")),
        "resource_id": tool_name,
        "audience": first(scope.get("audiences")),
        "reversibility": first(scope.get("reversibility")),
        "externality": first(scope.get("externality")),
        "data_class": first(scope.get("data_classes")),
        "proposed_at": clock,
        "tool_name": tool_name,
    });

    let inputs = serde_json::json!({
        "intent_envelope": envelope,
        "proposed_action": proposed_action,
        "decision_token": token,
    });

    let context = serde_json::json!({
        "clock": clock,
    });

    (inputs, context)
}

/// Process a single MCP JSON-RPC request.
///
/// Returns the JSON-RPC response.
pub fn handle_mcp_request(request: &Value, pubkey_hex: &str, clock: i64) -> Value {
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
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "hacp-rs-mcp",
                        "version": "0.1.0"
                    }
                }
            })
        }

        "notifications/initialized" => {
            // Notification, no response needed
            Value::Null
        }

        "tools/list" => {
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "tools": []
                }
            })
        }

        "tools/call" => {
            let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let _arguments = params.get("arguments").unwrap_or(&Value::Null);

            // Extract HACP parameters
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

            // Build evaluate input and call evaluate
            let (inputs, context) =
                build_mcp_evaluate_input(&envelope, &token, tool_name, _arguments, clock);

            let result = evaluate::evaluate(&inputs, &context, pubkey_hex);

            if result.decision == "ALLOW" {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "content": [{
                            "type": "text",
                            "text": "HACP ALLOW"
                        }]
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

/// Run the MCP server loop (JSON-RPC over stdin/stdout).
pub fn run(pubkey_hex: &str) -> Result<(), String> {
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let clock = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

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
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    }
                });
                let json =
                    serde_json::to_string(&err_resp).map_err(|e| format!("serialize: {}", e))?;
                writeln!(out, "{}", json).map_err(|e| format!("write: {}", e))?;
                out.flush().map_err(|e| format!("flush: {}", e))?;
                continue;
            }
        };

        let response = handle_mcp_request(&request, pubkey_hex, clock);

        // Skip notifications (null response)
        if response.is_null() {
            continue;
        }

        let json = serde_json::to_string(&response).map_err(|e| format!("serialize: {}", e))?;
        writeln!(out, "{}", json).map_err(|e| format!("write: {}", e))?;
        out.flush().map_err(|e| format!("flush: {}", e))?;
    }

    Ok(())
}
