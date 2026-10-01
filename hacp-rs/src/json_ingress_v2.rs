//! Strict raw JSON ingress for behavioral component json-ingress-v2.
//!
//! The component observes object members before serde_json::Value
//! materialization so duplicate-member conditions cannot be normalized away.

use std::collections::HashSet;
use std::fmt;

use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};

const DUPLICATE_MARKER: &str = "HACP_JSON_INGRESS_V2_DUPLICATE:";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngressError {
    DuplicateMember { path: String },
    Syntax(String),
}

impl fmt::Display for IngressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateMember { path } => {
                write!(f, "duplicate JSON object member at {path}")
            }
            Self::Syntax(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for IngressError {}

/// Parse raw JSON while rejecting duplicate object members before
/// serde_json::Value materialization can discard the duplicate condition.
///
/// Duplicate locations use JSON Pointer-like paths. For example, a duplicate
/// member directly inside ProposedAction is reported as:
///
/// `/input/proposed_action`
pub fn parse_strict_value(raw: &str) -> Result<Value, IngressError> {
    let mut deserializer = serde_json::Deserializer::from_str(raw);

    let value = StrictValueSeed { path: Vec::new() }
        .deserialize(&mut deserializer)
        .map_err(classify_error)?;

    deserializer
        .end()
        .map_err(|error| IngressError::Syntax(error.to_string()))?;

    Ok(value)
}

fn classify_error(error: serde_json::Error) -> IngressError {
    let message = error.to_string();

    if let Some(index) = message.find(DUPLICATE_MARKER) {
        let rest = &message[index + DUPLICATE_MARKER.len()..];
        let path = rest
            .split(" at line ")
            .next()
            .unwrap_or(rest)
            .to_string();

        return IngressError::DuplicateMember { path };
    }

    IngressError::Syntax(message)
}

struct StrictValueSeed {
    path: Vec<String>,
}

impl<'de> DeserializeSeed<'de> for StrictValueSeed {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictValueVisitor { path: self.path })
    }
}

struct StrictValueVisitor {
    path: Vec<String>,
}

impl StrictValueVisitor {
    fn pointer(&self) -> String {
        if self.path.is_empty() {
            return "/".to_string();
        }

        let mut pointer = String::new();

        for segment in &self.path {
            pointer.push('/');
            pointer.push_str(
                &segment
                    .replace('~', "~0")
                    .replace('/', "~1"),
            );
        }

        pointer
    }
}

impl<'de> Visitor<'de> for StrictValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("valid JSON without duplicate object members")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        StrictValueSeed { path: self.path }.deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        let mut index = 0usize;

        loop {
            let mut child_path = self.path.clone();
            child_path.push(index.to_string());

            match sequence.next_element_seed(StrictValueSeed { path: child_path })? {
                Some(value) => {
                    values.push(value);
                    index += 1;
                }
                None => break,
            }
        }

        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut seen = HashSet::new();
        let mut object = Map::new();

        while let Some(key) = map_access.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(<A::Error as de::Error>::custom(format!(
                    "{DUPLICATE_MARKER}{}",
                    self.pointer()
                )));
            }

            let mut child_path = self.path.clone();
            child_path.push(key.clone());

            let value =
                map_access.next_value_seed(StrictValueSeed { path: child_path })?;

            object.insert(key, value);
        }

        Ok(Value::Object(object))
    }
}