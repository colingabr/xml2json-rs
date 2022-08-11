use serde_json::Value as JsonValue;

// Check if a JSON value is empty
pub fn json_is_empty(node: &JsonValue) -> bool {
  match node {
    JsonValue::Null => true,
    JsonValue::Bool(_) => false,
    JsonValue::Number(_) => false,
    JsonValue::String(ref v) => v.is_empty(),
    JsonValue::Array(ref v) => v.is_empty(),
    JsonValue::Object(ref v) => v.is_empty()
  }
}

// Check the number of keys in a JsonValue object. If it is another type return 0
pub fn json_object_key_len(node: &JsonValue) -> usize {
  match node.as_object() {
    Some(o) => o.keys().len(),
    None => 0
  }
}

// serde's to_string will wrap strings in quotes, use as_str for Strings,
// to_string for everything else
pub fn to_string_raw(node: &JsonValue) -> String {
  if node.is_string() {
    node.as_str().unwrap_or("").to_owned()
  } else {
    node.to_string()
  }
}
