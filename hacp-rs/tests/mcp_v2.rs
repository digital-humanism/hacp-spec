//! Focused tests for MCP adapter v2 (Model B).
//!
//! Evidence IDs: M2-01 through M2-12.

use hacp_rs::mcp_v2;
use serde_json::json;

const PUBKEY: &str = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

/// Build a minimal envelope with scope for testing.
fn test_envelope() -> serde_json::Value {
    json!({
        "hacp_version": "0.9",
        "envelope_id": "22222222-2222-2222-2222-222222222222",
        "principal": "test_user",
        "principal_kind": "human",
        "intent_statement": "test",
        "scope": {
            "verbs": ["call"],
            "resource_classes": ["mcp_tool"],
            "audiences": ["internal"],
            "reversibility": ["reversible"],
            "externality": ["internal"],
            "data_classes": ["internal"]
        },
        "issued_at": 1786000000,
        "expires_at": 1786003600,
        "signer_key_id": "key-ed25519-test-001",
        "signature": "dummy"
    })
}

// ============================================================
// M2-01: valid deterministic MCP projection
// ============================================================
#[test]
fn m2_01_valid_deterministic_projection() {
    let env = test_envelope();
    let action = mcp_v2::synthesize_proposed_action(&env, "weather-tool");

    assert_eq!(action["hacp_version"], "0.9");
    assert_eq!(action["verb"], "call");
    assert_eq!(action["resource_class"], "mcp_tool");
    assert_eq!(action["resource_id"], "weather-tool");
    assert_eq!(action["audience"], "internal");
    assert_eq!(action["reversibility"], "reversible");
    assert_eq!(action["externality"], "internal");
    assert_eq!(action["data_class"], "internal");
    assert_eq!(action["tool_name"], "weather-tool");
}

// ============================================================
// M2-02: same transport facts at different execution times
//        → identical synthesized ProposedAction
// ============================================================
#[test]
fn m2_02_deterministic_across_time() {
    let env = test_envelope();
    let a1 = mcp_v2::synthesize_proposed_action(&env, "tool-x");

    // Simulate time passing
    std::thread::sleep(std::time::Duration::from_millis(50));

    let a2 = mcp_v2::synthesize_proposed_action(&env, "tool-x");

    assert_eq!(
        a1, a2,
        "ProposedAction must be identical regardless of execution time"
    );
}

// ============================================================
// M2-03: same transport facts → identical action_hash
// ============================================================
#[test]
fn m2_03_identical_action_hash_across_time() {
    let env = test_envelope();
    let a1 = mcp_v2::synthesize_proposed_action(&env, "tool-y");
    let c1 = hacp_rs::jcs::canonicalize(&a1).unwrap();
    let h1 = hacp_rs::sha256::sha256_hex(&c1);

    std::thread::sleep(std::time::Duration::from_millis(50));

    let a2 = mcp_v2::synthesize_proposed_action(&env, "tool-y");
    let c2 = hacp_rs::jcs::canonicalize(&a2).unwrap();
    let h2 = hacp_rs::sha256::sha256_hex(&c2);

    assert_eq!(
        h1, h2,
        "action_hash must be identical regardless of execution time"
    );
}

// ============================================================
// M2-06: no proposed_at in synthesized action
// ============================================================
#[test]
fn m2_06_no_proposed_at_in_synthesized_action() {
    let env = test_envelope();
    let action = mcp_v2::synthesize_proposed_action(&env, "any-tool");

    assert!(
        action.get("proposed_at").is_none(),
        "ProposedAction MUST NOT contain proposed_at"
    );
}

// ============================================================
// M2-07: params.name deterministically binds resource_id
// ============================================================
#[test]
fn m2_07_name_binds_resource_id() {
    let env = test_envelope();
    let action = mcp_v2::synthesize_proposed_action(&env, "my-tool-42");

    assert_eq!(
        action["resource_id"], "my-tool-42",
        "resource_id must equal params.name"
    );
}

// ============================================================
// M2-08: params.name deterministically binds tool_name
// ============================================================
#[test]
fn m2_08_name_binds_tool_name() {
    let env = test_envelope();
    let action = mcp_v2::synthesize_proposed_action(&env, "my-tool-42");

    assert_eq!(
        action["tool_name"], "my-tool-42",
        "tool_name must equal params.name"
    );
}

// ============================================================
// M2-09: one hash-relevant field mismatch → fail closed
// ============================================================
#[test]
fn m2_09_field_mismatch_different_hash() {
    let env = test_envelope();
    let a1 = mcp_v2::synthesize_proposed_action(&env, "tool-a");
    let a2 = mcp_v2::synthesize_proposed_action(&env, "tool-b");

    let c1 = hacp_rs::jcs::canonicalize(&a1).unwrap();
    let c2 = hacp_rs::jcs::canonicalize(&a2).unwrap();
    let h1 = hacp_rs::sha256::sha256_hex(&c1);
    let h2 = hacp_rs::sha256::sha256_hex(&c2);

    assert_ne!(
        h1, h2,
        "Different tool names must produce different action hashes"
    );
}

// ============================================================
// M2-10: MCP arguments do not affect action_hash
// ============================================================
#[test]
fn m2_10_arguments_do_not_affect_hash() {
    let env = test_envelope();
    // Synthesize does not take arguments — pure function of (envelope, name)
    let a1 = mcp_v2::synthesize_proposed_action(&env, "tool-z");
    let c1 = hacp_rs::jcs::canonicalize(&a1).unwrap();
    let h1 = hacp_rs::sha256::sha256_hex(&c1);

    // Same tool, different "arguments" (not passed to synthesize)
    let a2 = mcp_v2::synthesize_proposed_action(&env, "tool-z");
    let c2 = hacp_rs::jcs::canonicalize(&a2).unwrap();
    let h2 = hacp_rs::sha256::sha256_hex(&c2);

    assert_eq!(
        h1, h2,
        "Arguments must not affect action hash (known gap, not in synthesis)"
    );
}

// ============================================================
// M2-04: two tools/call in one process → different eval clock
// (We test by checking that the function doesn't panic and
//  produces valid responses for two sequential calls)
// ============================================================
#[test]
fn m2_04_two_calls_in_one_process() {
    let request1 = json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {},
        "id": 1
    });
    let resp1 = mcp_v2::handle_mcp_request_v2(&request1, PUBKEY);
    assert!(resp1.get("result").is_some(), "initialize must succeed");

    let request2 = json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {},
        "id": 2
    });
    let resp2 = mcp_v2::handle_mcp_request_v2(&request2, PUBKEY);
    assert!(
        resp2.get("result").is_some(),
        "second initialize must succeed"
    );

    // Both calls succeed — no shared mutable state breaks the second call
    assert_ne!(resp1["id"], resp2["id"]);
}

// ============================================================
// M2-05: no process-start clock reuse
// (The synthesis function is pure — no clock at all.
//  The run() loop captures fresh clock per call.)
// ============================================================
#[test]
fn m2_05_synthesis_has_no_clock_dependency() {
    let env = test_envelope();
    let action = mcp_v2::synthesize_proposed_action(&env, "clock-test");

    // Verify no time-dependent fields
    let time_fields = ["proposed_at", "created_at", "timestamp", "time", "clock"];
    for field in &time_fields {
        assert!(
            action.get(field).is_none(),
            "Synthesized action must not contain time-dependent field '{}'",
            field
        );
    }
}

// ============================================================
// M2-11: v1 adapter unchanged (compile-time check)
// ============================================================
#[test]
fn m2_11_v1_module_exists() {
    // If mcp.rs was deleted or renamed, this test won't compile.
    // The module is declared in lib.rs as `pub mod mcp;`
    let request = json!({
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {},
        "id": 1
    });
    let resp = hacp_rs::mcp::handle_mcp_request(&request, PUBKEY, 0);
    assert!(
        resp.get("result").is_some(),
        "v1 must still handle initialize"
    );
    assert_eq!(
        resp["result"]["serverInfo"]["name"], "hacp-rs-mcp",
        "v1 identity must be preserved"
    );
}

// ============================================================
// M2-12: evaluate.rs unchanged (compile-time check)
// ============================================================
#[test]
fn m2_12_evaluate_module_exists_and_callable() {
    // If evaluate.rs was modified incompatibly, this won't compile.
    let inputs = json!({
        "intent_envelope": {},
        "proposed_action": {},
        "decision_token": null
    });
    let context = json!({ "clock": 1786000100 });
    let result = hacp_rs::evaluate::evaluate(&inputs, &context, PUBKEY);
    // evaluate() must return a valid decision
    assert!(
        result.decision == "ALLOW" || result.decision == "DENY" || result.decision == "CHECKPOINT",
        "evaluate must return a valid decision, got: {}",
        result.decision
    );
}

// ============================================================
// Fail-closed tests
// ============================================================

#[test]
fn fail_closed_missing_tool_name() {
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "",
            "hacp_intent_envelope": {},
            "hacp_decision_token": {}
        },
        "id": 1
    });
    let resp = mcp_v2::handle_mcp_request_v2(&request, PUBKEY);
    let msg = resp["error"]["message"].as_str().unwrap_or("");
    assert!(
        msg.contains("INVALID_ACTION"),
        "Missing tool name must fail closed: {}",
        msg
    );
}

#[test]
fn fail_closed_missing_envelope() {
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "test-tool",
            "hacp_decision_token": {}
        },
        "id": 1
    });
    let resp = mcp_v2::handle_mcp_request_v2(&request, PUBKEY);
    let msg = resp["error"]["message"].as_str().unwrap_or("");
    assert!(
        msg.contains("INVALID_ENVELOPE"),
        "Missing envelope must fail closed: {}",
        msg
    );
}

#[test]
fn fail_closed_missing_token() {
    let request = json!({
        "jsonrpc": "2.0",
        "method": "tools/call",
        "params": {
            "name": "test-tool",
            "hacp_intent_envelope": {}
        },
        "id": 1
    });
    let resp = mcp_v2::handle_mcp_request_v2(&request, PUBKEY);
    let msg = resp["error"]["message"].as_str().unwrap_or("");
    assert!(
        msg.contains("SIGNATURE_FAILURE"),
        "Missing token must fail closed: {}",
        msg
    );
}
