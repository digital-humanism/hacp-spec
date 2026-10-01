use serde_json::Value;

use hacp_rs::evaluate;

const TEST_PUBKEY_HEX: &str =
    "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

#[test]
fn authorization_v2_action_hash_mismatch_uses_current_enforcement_reason() {
    let mut vector: Value =
        serde_json::from_str(include_str!("../../vectors/core_inv1_001_golden.json"))
            .expect("CORE-INV1-001 must parse");

    let context = vector
        .get("policy_context")
        .expect("golden vector must contain policy_context")
        .clone();

    {
        let inputs = vector
            .get_mut("inputs")
            .expect("golden vector must contain inputs");

        let original_resource_id = inputs["proposed_action"]["resource_id"]
            .as_str()
            .expect("golden proposed_action.resource_id must be a string")
            .to_owned();

        assert_eq!(original_resource_id, "db://prod/main");

        // Keep the already-signed envelope and DecisionToken semantically
        // unchanged. Change only the presented ProposedAction so
        // token.action_hash no longer matches SHA-256(JCS(proposed_action)).
        inputs["proposed_action"]["resource_id"] =
            Value::String("db://prod/changed-after-token-issuance".to_owned());
    }

    let inputs = vector
        .get("inputs")
        .expect("golden vector must contain inputs");

    let result = evaluate::evaluate(inputs, &context, TEST_PUBKEY_HEX);

    assert_eq!(result.decision, "DENY");

    assert_eq!(
        result.reason_codes,
        vec!["SIGNATURE_FAILURE".to_string()],
        "active Enforcement revision 2 requires token action_hash mismatch to map to SIGNATURE_FAILURE"
    );
}