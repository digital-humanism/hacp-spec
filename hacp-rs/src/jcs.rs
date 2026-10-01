//! Deterministic JSON canonicalization used by the HACP Rust implementation.
//!
//! Current properties implemented by this module:
//! - object keys sorted lexicographically
//! - no whitespace between tokens
//! - deterministic serialization of supported serde_json::Value numbers
//! - JSON string escaping
//! - non-finite floating-point values rejected

use serde_json::Value;
use std::fmt::Write;

/// Canonicalize a parsed JSON Value into deterministic bytes.
pub fn canonicalize(v: &Value) -> Result<Vec<u8>, String> {
    let mut buf = String::new();
    write_canonical(v, &mut buf)?;
    Ok(buf.into_bytes())
}

fn write_canonical(v: &Value, buf: &mut String) -> Result<(), String> {
    match v {
        Value::Null => buf.push_str("null"),
        Value::Bool(b) => buf.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => write_canonical_number(n, buf)?,
        Value::String(s) => write_canonical_string(s, buf),
        Value::Array(arr) => {
            buf.push('[');
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    buf.push(',');
                }
                write_canonical(item, buf)?;
            }
            buf.push(']');
        }
        Value::Object(map) => {
            // Collect and sort keys
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            buf.push('{');
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    buf.push(',');
                }
                write_canonical_string(key, buf);
                buf.push(':');
                write_canonical(map.get(*key).unwrap(), buf)?;
            }
            buf.push('}');
        }
    }
    Ok(())
}

fn write_canonical_number(n: &serde_json::Number, buf: &mut String) -> Result<(), String> {
    if let Some(i) = n.as_i64() {
        write!(buf, "{}", i).map_err(|e| e.to_string())?;
        return Ok(());
    }
    if let Some(u) = n.as_u64() {
        write!(buf, "{}", u).map_err(|e| e.to_string())?;
        return Ok(());
    }
    if let Some(f) = n.as_f64() {
        if !f.is_finite() {
            return Err("non-finite number".into());
        }
        // Use ryu for shortest representation (matches Go's 'g' format)
        let mut ryu_buf = ryu::Buffer::new();
        let s = ryu_buf.format(f);
        buf.push_str(s);
        return Ok(());
    }
    Err("unknown number type".into())
}

fn write_canonical_string(s: &str, buf: &mut String) {
    buf.push('"');
    for c in s.chars() {
        match c {
            '"' => buf.push_str("\\\""),
            '\\' => buf.push_str("\\\\"),
            '\u{08}' => buf.push_str("\\b"),
            '\u{0C}' => buf.push_str("\\f"),
            '\n' => buf.push_str("\\n"),
            '\r' => buf.push_str("\\r"),
            '\t' => buf.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                write!(buf, "\\u{:04x}", c as u32).unwrap();
            }
            c => buf.push(c),
        }
    }
    buf.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sorted_keys() {
        let v = json!({"b": 1, "a": 2});
        let canon = canonicalize(&v).unwrap();
        assert_eq!(String::from_utf8(canon).unwrap(), r#"{"a":2,"b":1}"#);
    }

    #[test]
    fn test_no_whitespace() {
        let v = json!({"x": [1, 2, 3]});
        let canon = canonicalize(&v).unwrap();
        assert_eq!(String::from_utf8(canon).unwrap(), r#"{"x":[1,2,3]}"#);
    }

    #[test]
    fn test_string_escaping() {
        let v = json!({"key": "val\"ue"});
        let canon = canonicalize(&v).unwrap();
        assert_eq!(String::from_utf8(canon).unwrap(), r#"{"key":"val\"ue"}"#);
    }
}
