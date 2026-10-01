//! canonical-v2 deterministic JSON canonicalization.
//!
//! This module is an additive behavioral successor to historical `jcs`.
//! The historical `jcs` module remains immutable.
//!
//! E5C bounded behavioral deltas:
//! - schema-reachable finite JSON numbers follow the existing JCS/RFC 8785
//!   number path used by the Rust implementation;
//! - null-valued object members are omitted;
//! - null values inside arrays remain literal `null`.

use serde_json::Value;
use std::fmt::Write;

/// Canonicalize a parsed JSON value according to canonical-v2.
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
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            buf.push('{');

            let mut first = true;

            for key in keys {
                let value = map
                    .get(key)
                    .ok_or_else(|| "object key disappeared during canonicalization".to_string())?;

                if value.is_null() {
                    continue;
                }

                if !first {
                    buf.push(',');
                }

                first = false;

                write_canonical_string(key, buf);
                buf.push(':');
                write_canonical(value, buf)?;
            }

            buf.push('}');
        }
    }

    Ok(())
}

fn write_canonical_number(
    n: &serde_json::Number,
    buf: &mut String,
) -> Result<(), String> {
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
