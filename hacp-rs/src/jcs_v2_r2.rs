//! canonical-v2 deterministic JSON canonicalization — implementation revision r2.
//!
//! Behavioral identity remains `canonical-v2`.
//! This module corrects the pre-convergence numeric serialization defect in
//! `jcs_v2` without modifying that historical implementation identity.
//!
//! Numeric representation follows the RFC 8785 / ECMAScript binary64
//! presentation rules using the existing Ryu shortest-roundtrip primitive.
//!
//! Non-numeric canonical-v2 behavior remains unchanged:
//! - null-valued object members are omitted;
//! - null values inside arrays remain literal `null`.

use serde_json::Value;
use std::fmt::Write;

/// Canonicalize a parsed JSON value according to canonical-v2 implementation r2.
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
    let f = n
        .as_f64()
        .ok_or_else(|| "number is not representable as binary64".to_string())?;

    if !f.is_finite() {
        return Err("non-finite number".into());
    }

    let rendered = format_ecmascript_number(f)?;
    buf.push_str(&rendered);

    Ok(())
}

fn format_ecmascript_number(value: f64) -> Result<String, String> {
    if !value.is_finite() {
        return Err("non-finite number".into());
    }

    if value == 0.0 {
        return Ok("0".to_string());
    }

    let mut ryu_buf = ryu::Buffer::new();
    let raw = ryu_buf.format(value).to_ascii_lowercase();

    let (sign, unsigned) = match raw.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", raw.as_str()),
    };

    let Some(e_pos) = unsigned.find('e') else {
        let body = unsigned.strip_suffix(".0").unwrap_or(unsigned);
        return Ok(format!("{sign}{body}"));
    };

    let mantissa = &unsigned[..e_pos];

    let exponent: i32 = unsigned[e_pos + 1..]
        .parse()
        .map_err(|_| "invalid Ryu exponent".to_string())?;

    let decimal_before = mantissa.find('.').unwrap_or(mantissa.len()) as i32;

    let raw_digits = mantissa.replace('.', "");
    let trimmed = raw_digits.trim_start_matches('0');

    let digits = if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    };

    let k = exponent + decimal_before;
    let scientific_exponent = k - 1;

    if (-6..21).contains(&scientific_exponent) {
        let mut body = if k <= 0 {
            format!(
                "0.{}{}",
                "0".repeat((-k) as usize),
                digits
            )
        } else if k as usize >= digits.len() {
            format!(
                "{}{}",
                digits,
                "0".repeat(k as usize - digits.len())
            )
        } else {
            let split = k as usize;
            format!("{}.{}", &digits[..split], &digits[split..])
        };

        if body.contains('.') {
            while body.ends_with('0') {
                body.pop();
            }

            if body.ends_with('.') {
                body.pop();
            }
        }

        return Ok(format!("{sign}{body}"));
    }

    let mut body = digits[..1].to_string();

    if digits.len() > 1 {
        let tail = digits[1..].trim_end_matches('0');

        if !tail.is_empty() {
            body.push('.');
            body.push_str(tail);
        }
    }

    let exponent_sign = if scientific_exponent >= 0 {
        "+"
    } else {
        "-"
    };

    Ok(format!(
        "{sign}{body}e{exponent_sign}{}",
        scientific_exponent.abs()
    ))
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
