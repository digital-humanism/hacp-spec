use hacp_rs::jcs_v2::canonicalize;
use serde_json::json;

fn canonical_string(value: &serde_json::Value) -> String {
    String::from_utf8(canonicalize(value).expect("canonical-v2 must succeed"))
        .expect("canonical-v2 output must be UTF-8")
}

#[test]
fn canonical_v2_f1_finite_number() {
    let input = json!({
        "constraints": {
            "x": 1.5
        }
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"constraints":{"x":1.5}}"#
    );
}

#[test]
fn canonical_v2_f2_omits_null_object_member() {
    let input = json!({
        "constraints": {
            "x": null
        }
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"constraints":{}}"#
    );
}

#[test]
fn canonical_v2_f3_mixed_object() {
    let input = json!({
        "constraints": {
            "a": 1,
            "b": null,
            "c": 1.5
        }
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"constraints":{"a":1,"c":1.5}}"#
    );
}

#[test]
fn canonical_v2_f4_preserves_null_inside_array() {
    let input = json!({
        "constraints": {
            "x": [1, null, 2]
        }
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"constraints":{"x":[1,null,2]}}"#
    );
}

#[test]
fn canonical_v2_omits_leading_null_without_comma_artifact() {
    let input = json!({
        "a": null,
        "b": 1
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"b":1}"#
    );
}

#[test]
fn canonical_v2_omits_trailing_null_without_comma_artifact() {
    let input = json!({
        "a": 1,
        "b": null
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"a":1}"#
    );
}

#[test]
fn canonical_v2_omits_nested_null_object_members() {
    let input = json!({
        "constraints": {
            "outer": {
                "keep": 1,
                "omit": null
            }
        }
    });

    assert_eq!(
        canonical_string(&input),
        r#"{"constraints":{"outer":{"keep":1}}}"#
    );
}
