//! JSON-RPC stdio server for control plane operations.
//!
//! Commands:
//! - `revoke` — commit a revocation
//! - `snapshot` — get complete revocation state
//! - `events_after` — get events after a revision
//! - `revision` — get current revision

use std::io::{self, BufRead, Write};
use std::sync::Arc;

use super::journal::{Journal, RevocationKind};

/// Run the control plane JSON-RPC server over stdio.
pub fn run(journal: Arc<Journal>) -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("stdin read: {}", e))?;
        if line.trim().is_empty() {
            continue;
        }

        let request: serde_json::Value = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = serde_json::json!({
                    "ok": false,
                    "error": format!("parse error: {}", e)
                });
                writeln!(out, "{}", serde_json::to_string(&resp).unwrap())
                    .map_err(|e| format!("write: {}", e))?;
                out.flush().map_err(|e| format!("flush: {}", e))?;
                continue;
            }
        };

        let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");
        let id = request
            .get("id")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let response = match method {
            "revoke" => {
                let kind = request.get("kind").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let subject_id = request
                    .get("subject_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                match RevocationKind::from_i32(kind) {
                    Some(k) => match journal.revoke(k, subject_id) {
                        Ok(Some(event)) => serde_json::json!({
                            "ok": true,
                            "event": serde_json::to_value(&event).unwrap()
                        }),
                        Ok(None) => serde_json::json!({
                            "ok": true,
                            "duplicate": true
                        }),
                        Err(e) => serde_json::json!({
                            "ok": false,
                            "error": e
                        }),
                    },
                    None => serde_json::json!({
                        "ok": false,
                        "error": format!("invalid revocation kind: {}", kind)
                    }),
                }
            }

            "snapshot" => {
                let snapshot = journal.snapshot();
                serde_json::json!({
                    "ok": true,
                    "snapshot": serde_json::to_value(&snapshot).unwrap()
                })
            }

            "events_after" => {
                let after = request
                    .get("after_revision")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                match journal.events_after(after) {
                    Ok(events) => serde_json::json!({
                        "ok": true,
                        "events": serde_json::to_value(&events).unwrap()
                    }),
                    Err(e) => serde_json::json!({
                        "ok": false,
                        "error": e
                    }),
                }
            }

            "revision" => {
                serde_json::json!({
                    "ok": true,
                    "revision": journal.revision()
                })
            }

            _ => {
                serde_json::json!({
                    "ok": false,
                    "error": format!("unknown method: {}", method)
                })
            }
        };

        // Add id to response
        let mut resp_map = response.as_object().unwrap().clone();
        resp_map.insert("id".to_string(), id);

        let json = serde_json::to_string(&serde_json::Value::Object(resp_map))
            .map_err(|e| format!("serialize: {}", e))?;
        writeln!(out, "{}", json).map_err(|e| format!("write: {}", e))?;
        out.flush().map_err(|e| format!("flush: {}", e))?;
    }

    Ok(())
}
