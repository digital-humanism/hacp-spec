use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn json_ingress_v1_must_reject_duplicate_proposed_action_key() {
    let runner = env!("CARGO_BIN_EXE_hacp-rs-runner");

    // Intentional malformed raw JSON.
    //
    // Mirrors the normative CORE-INV5-006 condition:
    // ProposedAction contains duplicate "verb" members.
    //
    // The raw form is essential. Parsing this fixture before it reaches the
    // runner would destroy the condition being tested.
    let request = r#"{"protocol_version":"1","operation":"evaluate","vector_id":"CORE-INV5-006-RED","input":{"intent_envelope":{"hacp_version":"0.9","envelope_id":"22222222-2222-2222-2222-222222222222","principal":"human_admin_01","principal_kind":"human","intent_statement":"Duplicate keys","scope":{"verbs":["read"],"resource_classes":["customer_record"],"audiences":["internal"],"reversibility":["reversible"],"externality":["internal"],"data_classes":["internal"]},"issued_at":1786000000,"expires_at":1786003600,"signer_key_id":"key-ed25519-test-001","signature":"PLACEHOLDER"},"proposed_action":{"hacp_version":"0.9","action_id":"11111111-1111-1111-1111-111111111111","envelope_id":"22222222-2222-2222-2222-222222222222","verb":"read","verb":"delete","resource_class":"customer_record","resource_id":"crm://acct/4411","audience":"internal","reversibility":"reversible","externality":"internal","data_class":"internal","proposed_at":1786000100},"policy_context":{"clock":1786000100,"current_action_count":0}}}"#;

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

    let stdout = String::from_utf8(output.stdout).expect("stdout UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr UTF-8");

    assert!(
        output.status.success(),
        "runner process failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let response_line = stdout
        .lines()
        .next()
        .unwrap_or_else(|| {
            panic!(
                "normative malformed duplicate-key input produced no HACP response\nstdout:\n{stdout}\nstderr:\n{stderr}"
            )
        });

    let response: serde_json::Value =
        serde_json::from_str(response_line).unwrap_or_else(|e| {
            panic!(
                "runner returned non-JSON response: {e}\nstdout:\n{stdout}\nstderr:\n{stderr}"
            )
        });

    assert_eq!(
        response.get("decision").and_then(|v| v.as_str()),
        Some("DENY"),
        "duplicate-key ProposedAction must fail closed as DENY\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let reasons = response
        .get("reason_codes")
        .and_then(|v| v.as_array())
        .expect("DENY response must contain reason_codes");

    assert!(
        reasons.iter().any(|v| v.as_str() == Some("INVALID_ACTION")),
        "CORE-INV5-006 duplicate-key condition must produce INVALID_ACTION; \
         current json-ingress-v1 must not silently normalize duplicate members\n\
         stdout:\n{stdout}\nstderr:\n{stderr}"
    );
}