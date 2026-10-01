use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use serde_json::Value;
use sha2::{Digest, Sha256};

use hacp_rs::evaluate;
use hacp_rs::jcs::canonicalize;

const TEST_PUBKEY_HEX: &str =
    "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

const CONFORMANCE_SEED_SOURCE: &[u8] =
    b"hacp-conformance-v0.9-key-001";

#[test]
fn authorization_v2_honors_authenticated_applicable_signed_deny_token() {
    let mut vector: Value =
        serde_json::from_str(include_str!("../../vectors/core_inv1_001_golden.json"))
            .expect("CORE-INV1-001 must parse");

    let context = vector
        .get("policy_context")
        .expect("golden vector must contain policy_context")
        .clone();

    let seed_digest = Sha256::digest(CONFORMANCE_SEED_SOURCE);
    let seed: [u8; 32] = seed_digest.into();
    let signing_key = SigningKey::from_bytes(&seed);

    assert_eq!(
        hex::encode(signing_key.verifying_key().as_bytes()),
        TEST_PUBKEY_HEX,
        "derived conformance signing identity must match key-ed25519-test-001"
    );

    {
        let inputs = vector
            .get_mut("inputs")
            .expect("golden vector must contain inputs");

        let token = inputs
            .get_mut("decision_token")
            .expect("golden inputs must contain decision_token");

        assert_eq!(
            token.get("decision").and_then(Value::as_str),
            Some("ALLOW"),
            "golden token must begin as ALLOW"
        );

        token["decision"] = Value::String("DENY".to_owned());

        let token_object = token
            .as_object_mut()
            .expect("DecisionToken must be an object");

        token_object.remove("signature");

        let payload = canonicalize(token)
            .expect("modified DecisionToken must canonicalize");

        let signature = signing_key.sign(&payload);
        token["signature"] =
            Value::String(URL_SAFE_NO_PAD.encode(signature.to_bytes()));
    }

    let inputs = vector
        .get("inputs")
        .expect("golden vector must contain inputs");

    let result = evaluate::evaluate(inputs, &context, TEST_PUBKEY_HEX);

    assert_eq!(
        result.decision,
        "DENY",
        "active Enforcement revision 2 requires an authenticated and applicable signed DENY DecisionToken to be authoritative"
    );

    assert_eq!(
        result.reason_codes,
        vec!["POLICY_DENIED".to_string()],
        "active Enforcement revision 2 requires POLICY_DENIED when a DENY token supplies no explicit reason"
    );
}