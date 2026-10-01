use hacp_rs::jcs_v2_r2::canonicalize;
use serde::Deserialize;
use serde_json::{Number, Value};
use sha2::{Digest, Sha256};
use std::{env, fs};

const EXPECTED_LEN: usize = 1642;
const EXPECTED_SHA256: &str =
    "45c486b9ad1cb7a7a958f9876eb41c474cb62f144ee2ef947872c13a4ea91bb7";

#[derive(Debug, Deserialize)]
struct Row {
    id: String,
    bits: String,
    expected: String,
}

fn load_manifest() -> Vec<Row> {
    let path = env::var("HACP_E13_CORPUS")
        .expect("HACP_E13_CORPUS must be set to the absolute E13 manifest path");

    let bytes = fs::read(&path).expect("E13 corpus manifest must be readable");

    assert_eq!(
        bytes.len(),
        EXPECTED_LEN,
        "E13 corpus manifest byte length mismatch"
    );

    let digest = hex::encode(Sha256::digest(&bytes));

    assert_eq!(
        digest,
        EXPECTED_SHA256,
        "E13 corpus manifest SHA-256 mismatch"
    );

    let rows: Vec<Row> =
        serde_json::from_slice(&bytes).expect("E13 corpus manifest must be valid JSON");

    assert_eq!(rows.len(), 24, "E13 corpus manifest must contain 24 rows");

    for (index, row) in rows.iter().enumerate() {
        assert_eq!(
            row.id,
            format!("R{:02}", index + 1),
            "E13 corpus row identifier mismatch"
        );
    }

    rows
}

#[test]
fn e13_canonical_v2_r01_r24_matches_shared_manifest() {
    let rows = load_manifest();

    for row in rows {
        assert_eq!(
            row.bits.len(),
            16,
            "{} must contain exactly 16 hexadecimal digits",
            row.id
        );

        let bits =
            u64::from_str_radix(&row.bits, 16).expect("E13 bits must be valid hexadecimal");

        let value = f64::from_bits(bits);

        assert!(value.is_finite(), "{} must represent finite binary64", row.id);

        let number =
            Number::from_f64(value).expect("finite E13 binary64 must map to serde_json Number");

        let actual = canonicalize(&Value::Number(number))
            .expect("canonical-v2 r2 must canonicalize E13 row");

        assert_eq!(
            actual,
            row.expected.as_bytes(),
            "{} canonical-v2 mismatch for bits {}",
            row.id,
            row.bits
        );
    }
}