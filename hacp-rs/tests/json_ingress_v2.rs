use std::io::Write;
use std::process::{Command, Stdio};

use hacp_rs::json_ingress_v2::{parse_strict_value, IngressError};

#[test]
fn duplicate_proposed_action_path_is_preserved_before_value_materialization() {
    let raw = r#"{"input":{"proposed_action":{"verb":"read","verb":"delete"}}}"#;

    let error = parse_strict_value(raw).expect_err("duplicate member must be rejected");

    assert_eq!(
        error,
        IngressError::DuplicateMember {
            path: "/input/proposed_action".to_string(),
        }
    );
}

#[test]
fn nested_duplicate_path_is_preserved() {
    let raw =
        r#"{"input":{"proposed_action":{"details":{"mode":"read","mode":"delete"}}}}"#;

    let error = parse_strict_value(raw).expect_err("nested duplicate must be rejected");

    assert_eq!(
        error,
        IngressError::DuplicateMember {
            path: "/input/proposed_action/details".to_string(),
        }
    );
}

#[test]
fn valid_json_is_preserved_as_normal_value() {
    let raw = r#"{"input":{"proposed_action":{"verb":"read"}}}"#;

    let value = parse_strict_value(raw).expect("valid JSON must parse");

    assert_eq!(
        value
            .get("input")
            .and_then(|v| v.get("proposed_action"))
            .and_then(|v| v.get("verb"))
            .and_then(|v| v.as_str()),
        Some("read")
    );
}

#[test]
fn runner_maps_duplicate_proposed_action_to_invalid_action_without_action_hash() {
    let runner = env!("CARGO_BIN_EXE_hacp-rs-runner");

    let request = r#"{"protocol_version":"1","operation":"evaluate","vector_id":"JSON-INGRESS-V2-001","input":{"intent_envelope":{"hacp_version":"0.9","envelope_id":"22222222-2222-2222-2222-222222222222","principal":"human_admin_01","principal_kind":"human","intent_statement":"Duplicate keys","scope":{"verbs":["read"],"resource_classes":["customer_record"],"audiences":["internal"],"reversibility":["reversible"],"externality":["internal"],"data_classes":["internal"]},"issued_at":1786000000,"expires_at":1786003600,"signer_key_id":"key-ed25519-test-001","signature":"PLACEHOLDER"},"proposed_action":{"hacp_version":"0.9","action_id":"11111111-1111-1111-1111-111111111111","envelope_id":"22222222-2222-2222-2222-222222222222","verb":"read","verb":"delete","resource_class":"customer_record","resource_id":"crm://acct/4411","audience":"internal","reversibility":"reversible","externality":"internal","data_class":"internal","proposed_at":1786000100},"policy_context":{"clock":1786000100,"current_action_count":0}}}"#;

    let mut child = Command::new(runner)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn hacp-rs-runner");

    {
        let stdin = child.stdin.as_mut().expect("runner stdin");
        stdin
            .write_all(request.as_bytes())
            .expect("write raw request");
        stdin.write_all(b"\n").expect("write newline");
    }

    drop(child.stdin.take());

    let output = child.wait_with_output().expect("wait for runner");

    assert!(
        output.status.success(),
        "runner process failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout UTF-8");
    let response_line = stdout.lines().next().expect("HACP response");

    let response: serde_json::Value =
        serde_json::from_str(response_line).expect("JSON response");

    assert_eq!(
        response.get("decision").and_then(|v| v.as_str()),
        Some("DENY")
    );

    let reasons = response
        .get("reason_codes")
        .and_then(|v| v.as_array())
        .expect("reason_codes array");

    assert!(
        reasons
            .iter()
            .any(|value| value.as_str() == Some("INVALID_ACTION"))
    );

    assert!(
        response.get("action_hash").is_none(),
        "malformed duplicate ProposedAction must not produce action_hash"
    );
}