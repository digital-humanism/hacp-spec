//! Protocol v1 runner — stdin/stdout JSON lines.
//!
//! Reads JSON requests from stdin, writes JSON responses to stdout.
//! Diagnostics only on stderr.

use std::io::{self, BufRead, Write};

use serde::Serialize;
use serde_json::Value;

use crate::{evaluate, json_ingress_v2::{self, IngressError}};

const PROTOCOL_VERSION: &str = "1";

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct Request {
    protocol_version: String,
    operation: String,
    vector_id: String,
    input: Value,
    #[serde(default)]
    policy_context: Option<Value>,
}

#[derive(Serialize)]
struct Response {
    protocol_version: String,
    decision: String,
    reason_codes: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    action_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    provenance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metrics: Option<Value>,
}

/// Run the Protocol v1 runner loop.
pub fn run(pubkey_hex: &str) -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("stdin read: {}", e))?;
        if line.trim().is_empty() {
            continue;
        }

        match json_ingress_v2::parse_strict_value(&line) {
            Ok(_) => {}
            Err(IngressError::DuplicateMember { path })
                if path == "/input/proposed_action"
                    || path.starts_with("/input/proposed_action/") =>
            {
                let resp = Response {
                    protocol_version: PROTOCOL_VERSION.to_string(),
                    decision: "DENY".to_string(),
                    reason_codes: vec!["INVALID_ACTION".to_string()],
                    action_hash: String::new(),
                    provenance_id: None,
                    metrics: None,
                };

                let json = serde_json::to_string(&resp)
                    .map_err(|e| format!("response serialize: {}", e))?;
                writeln!(out, "{}", json).map_err(|e| format!("stdout write: {}", e))?;
                out.flush().map_err(|e| format!("stdout flush: {}", e))?;
                continue;
            }
            Err(IngressError::DuplicateMember { .. }) | Err(IngressError::Syntax(_)) => {
                // Outside the proven B-PARSE-001 ProposedAction condition,
                // preserve the historical Protocol v1 parsing path below.
            }
        }

        let req: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{{\"error\":\"PARSE_ERROR\",\"message\":\"{}\"}}", e);
                continue;
            }
        };

        if req.protocol_version != PROTOCOL_VERSION {
            eprintln!(
                "{{\"error\":\"VERSION_MISMATCH\",\"message\":\"expected {}, got {}\"}}",
                PROTOCOL_VERSION, req.protocol_version
            );
            continue;
        }

        match req.operation.as_str() {
            "evaluate" => {
                let resp = handle_evaluate(&req, pubkey_hex);
                let json = serde_json::to_string(&resp)
                    .map_err(|e| format!("response serialize: {}", e))?;
                writeln!(out, "{}", json).map_err(|e| format!("stdout write: {}", e))?;
                out.flush().map_err(|e| format!("stdout flush: {}", e))?;
            }
            _ => {
                eprintln!(
                    "{{\"error\":\"UNKNOWN_OP\",\"message\":\"{}\"}}",
                    req.operation
                );
            }
        }
    }

    Ok(())
}

fn handle_evaluate(req: &Request, pubkey_hex: &str) -> Response {
    let context = req
        .policy_context
        .as_ref()
        .or_else(|| req.input.get("policy_context"))
        .cloned()
        .unwrap_or(Value::Object(serde_json::Map::new()));

    let result = evaluate::evaluate(&req.input, &context, pubkey_hex);

    // Extract provenance_id if present
    let provenance_id = req
        .input
        .get("provenance_event")
        .and_then(|p| p.get("event_id"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Response {
        protocol_version: PROTOCOL_VERSION.to_string(),
        decision: result.decision,
        reason_codes: result.reason_codes,
        action_hash: result.action_hash,
        provenance_id,
        metrics: None,
    }
}
