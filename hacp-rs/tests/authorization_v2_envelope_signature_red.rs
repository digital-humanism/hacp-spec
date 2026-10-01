use serde_json::Value;

use hacp_rs::evaluate;

const TEST_PUBKEY_HEX: &str =
    "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

#[test]
fn authorization_v2_rejects_invalid_intent_envelope_signature_before_trusting_claims() {
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

        let envelope = inputs
            .get_mut("intent_envelope")
            .expect("golden inputs must contain intent_envelope");

        assert!(
            envelope.get("signature").is_some(),
            "golden IntentEnvelope must contain signature"
        );

        // Preserve every envelope claim, token, action, and policy-context
        // value. Corrupt only the envelope signature.
        envelope["signature"] = Value::String("00".repeat(64));
    }

    let inputs = vector
        .get("inputs")
        .expect("golden vector must contain inputs");

    let result = evaluate::evaluate(inputs, &context, TEST_PUBKEY_HEX);

    assert_eq!(result.decision, "DENY");

    assert_eq!(
        result.reason_codes,
        vec!["SIGNATURE_FAILURE".to_string()],
        "active Enforcement revision 2 requires IntentEnvelope authentication before trusting envelope claims"
    );
}