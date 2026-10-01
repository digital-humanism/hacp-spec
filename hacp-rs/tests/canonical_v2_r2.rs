use hacp_rs::jcs_v2_r2::canonicalize;
use serde_json::{json, Number, Value};

fn canonical_string(value: &Value) -> String {
    String::from_utf8(
        canonicalize(value)
            .expect("canonical-v2 r2 must succeed"),
    )
    .expect("canonical-v2 r2 output must be UTF-8")
}

#[test]
fn canonical_v2_r2_preserves_original_contract() {
    let cases = [
        (
            json!({"constraints":{"x":1.5}}),
            r#"{"constraints":{"x":1.5}}"#,
        ),
        (
            json!({"constraints":{"x":null}}),
            r#"{"constraints":{}}"#,
        ),
        (
            json!({"constraints":{"a":1,"b":null,"c":1.5}}),
            r#"{"constraints":{"a":1,"c":1.5}}"#,
        ),
        (
            json!({"constraints":{"x":[1,null,2]}}),
            r#"{"constraints":{"x":[1,null,2]}}"#,
        ),
        (
            json!({"a":null,"b":1,"c":2}),
            r#"{"b":1,"c":2}"#,
        ),
        (
            json!({"a":1,"b":2,"c":null}),
            r#"{"a":1,"b":2}"#,
        ),
        (
            json!({"constraints":{"outer":{"keep":1,"omit":null}}}),
            r#"{"constraints":{"outer":{"keep":1}}}"#,
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(canonical_string(&input), expected);
    }
}

#[test]
fn canonical_v2_r2_rfc8785_appendix_b_numeric_corpus() {
    let cases = [
        ("0000000000000000", "0"),
        ("8000000000000000", "0"),
        ("0000000000000001", "5e-324"),
        ("8000000000000001", "-5e-324"),
        ("7fefffffffffffff", "1.7976931348623157e+308"),
        ("ffefffffffffffff", "-1.7976931348623157e+308"),
        ("4340000000000000", "9007199254740992"),
        ("c340000000000000", "-9007199254740992"),
        ("4430000000000000", "295147905179352830000"),
        ("44b52d02c7e14af5", "9.999999999999997e+22"),
        ("44b52d02c7e14af6", "1e+23"),
        ("44b52d02c7e14af7", "1.0000000000000001e+23"),
        ("444b1ae4d6e2ef4e", "999999999999999700000"),
        ("444b1ae4d6e2ef4f", "999999999999999900000"),
        ("444b1ae4d6e2ef50", "1e+21"),
        ("3eb0c6f7a0b5ed8c", "9.999999999999997e-7"),
        ("3eb0c6f7a0b5ed8d", "0.000001"),
        ("41b3de4355555553", "333333333.3333332"),
        ("41b3de4355555554", "333333333.33333325"),
        ("41b3de4355555555", "333333333.3333333"),
        ("41b3de4355555556", "333333333.3333334"),
        ("41b3de4355555557", "333333333.33333343"),
        ("becbf647612f3696", "-0.0000033333333333333333"),
        ("43143ff3c1cb0959", "1424953923781206.2"),
    ];

    for (hex, expected) in cases {
        let bits =
            u64::from_str_radix(hex, 16)
                .expect("valid IEEE-754 fixture bits");

        let value = f64::from_bits(bits);

        let number =
            Number::from_f64(value)
                .expect("RFC 8785 fixture must be finite");

        let actual =
            canonical_string(&Value::Number(number));

        assert_eq!(
            actual,
            expected,
            "RFC 8785 fixture mismatch for bits {hex}"
        );
    }
}

#[test]
fn canonical_v2_r2_rejects_non_finite_binary64_construction() {
    assert!(Number::from_f64(f64::NAN).is_none());
    assert!(Number::from_f64(f64::INFINITY).is_none());
    assert!(Number::from_f64(f64::NEG_INFINITY).is_none());
}
