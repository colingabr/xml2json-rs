use crate::{
  error::{Error, ErrorKind},
  utils
};

use serde_json::{json, Value as JsonValue};

use quick_xml::{events::*, Reader};
use regex::{NoExpand, Regex};

lazy_static! {
  static ref WHITESPACE_RE: Regex = Regex::new(r"^\s*$").unwrap();
  static ref TWO_OR_MORE_WHITESPACE_RE: Regex = Regex::new(r"\s{2,}").unwrap();
}

/// Configuration options for JsonBuilder
#[derive(Default)]
pub struct JsonConfig {
  charkey:          Option<String>,
  attrkey:          Option<String>,
  empty_tag:        Option<String>,
  explicit_root:    Option<bool>,
  trim:             Option<bool>,
  ignore_attrs:     Option<bool>,
  merge_attrs:      Option<bool>,
  normalize_text:   Option<bool>,
  lowercase_tags:   Option<bool>,
  explicit_array:   Option<bool>,
  explicit_charkey: Option<bool>
}

/// JsonBuilder configuration options
impl JsonConfig {
  /// Initialze a new JsonConfig instance.
  ///
  /// This uses the builder pattern. All options are initialized to `None` and can be set using
  /// `self`s methods. Any options not set will use their defaults upon call to `finalize`.
  pub fn new() -> JsonConfig {
    JsonConfig {
      charkey:          None,
      attrkey:          None,
      empty_tag:        None,
      explicit_root:    None,
      trim:             None,
      ignore_attrs:     None,
      merge_attrs:      None,
      normalize_text:   None,
      lowercase_tags:   None,
      explicit_array:   None,
      explicit_charkey: None
    }
  }

  /// Key to store character content under.
  ///
  /// (`"_"` by default)
  pub fn charkey<T: Into<String>>(&mut self, key: T) -> &mut JsonConfig {
    self.charkey = Some(key.into());
    self
  }

  /// Key to outer object containing tag attributes.
  ///
  /// (`"$"` by default)
  pub fn attrkey<T: Into<String>>(&mut self, key: T) -> &mut JsonConfig {
    self.attrkey = Some(key.into());
    self
  }

  /// The value of empty nodes.
  ///
  /// Can be used if you want to specify a value other than `""` for empty nodes.
  /// (`""` by default)
  pub fn empty_tag<T: Into<String>>(&mut self, key: T) -> &mut JsonConfig {
    self.empty_tag = Some(key.into());
    self
  }

  /// Sets the root node inside the resulting object.
  ///
  /// (`true` by default)
  pub fn explicit_root(&mut self, flag: bool) -> &mut JsonConfig {
    self.explicit_root = Some(flag);
    self
  }

  /// Trim whitespace at the beginning and end of text nodes.
  ///
  /// (`false` by default)
  pub fn trim(&mut self, flag: bool) -> &mut JsonConfig {
    self.trim = Some(flag);
    self
  }

  /// Ingore attributes.
  ///
  /// Setting this to true will skip adding element attributes and create text nodes.
  ///
  /// (`false` by default)
  pub fn ignore_attrs(&mut self, flag: bool) -> &mut JsonConfig {
    self.ignore_attrs = Some(flag);
    self
  }

  /// Merge attributes.
  ///
  /// Merge all XML attributes and child elements as properties of the parent, instead of keying
  /// attributes off of the child attribute object. This option will be ignored if `ignore_attrs`
  /// is set.
  ///
  /// (`false` by default)
  pub fn merge_attrs(&mut self, flag: bool) -> &mut JsonConfig {
    self.merge_attrs = Some(flag);
    self
  }

  /// Removes whitespace character data in text nodes.
  ///
  /// This option will result in behavior that is a superset of [`trim`]. Whitespace at the
  /// beginning and end of text nodes will be trimmed. In addition, blank space (`/s`) between
  /// other text data will be converted to a single space (`" "`). Corresponds to the `normalize`
  /// option in node-xmlj2.
  ///
  /// (`false` by default)
  ///
  /// [`trim`]: struct.JsonConfig.html#method.trim
  pub fn normalize_text(&mut self, flag: bool) -> &mut JsonConfig {
    self.normalize_text = Some(flag);
    self
  }

  /// Normalize all tags by converting them to lowercase.
  ///
  /// Corresponds to the `normalizeTags` option in node-xml2js.
  ///
  /// (`false` by default)
  pub fn lowercase_tags(&mut self, flag: bool) -> &mut JsonConfig {
    self.lowercase_tags = Some(flag);
    self
  }

  /// Always put the child nodes in an array, otherwise an array is only created if there is more
  /// than one child.
  ///
  /// (`true` by default)
  pub fn explicit_array(&mut self, flag: bool) -> &mut JsonConfig {
    self.explicit_array = Some(flag);
    self
  }

  /// Always store character data under `charkey` even if there are are no other text elements
  /// stored inside the tag.
  ///
  /// (`false` by default)
  pub fn explicit_charkey(&mut self, flag: bool) -> &mut JsonConfig {
    self.explicit_charkey = Some(flag);
    self
  }

  /// Finalize configuration options and build a JsonBuilder instance
  pub fn finalize(&self) -> JsonBuilder {
    JsonBuilder {
      charkey:          self.charkey.clone().unwrap_or_else(|| "_".to_owned()),
      attrkey:          self.attrkey.clone().unwrap_or_else(|| "$".to_owned()),
      empty_tag:        self.empty_tag.clone().unwrap_or_else(|| "".to_owned()),
      explicit_root:    self.explicit_root.clone().unwrap_or(true),
      trim:             self.trim.clone().unwrap_or(false),
      ignore_attrs:     self.ignore_attrs.clone().unwrap_or(false),
      merge_attrs:      self.merge_attrs.clone().unwrap_or(false),
      normalize_text:   self.normalize_text.clone().unwrap_or(false),
      lowercase_tags:   self.lowercase_tags.clone().unwrap_or(false),
      explicit_array:   self.explicit_array.clone().unwrap_or(true),
      explicit_charkey: self.explicit_charkey.clone().unwrap_or(false)
    }
  }
}

// Text storage with state to distingiush between text in elements and text in CDATA sections
// CDATA (literal) text will be added to JSON even when it is whitespace.
struct Text {
  data:    String,
  literal: bool
}

impl Default for Text {
  fn default() -> Text {
    Text {
      data:    "".to_owned(),
      literal: false
    }
  }
}

// Stores state for the current and previous levels in the XML tree.
struct Node {
  value: JsonValue,
  text:  Text
}

impl Node {
  fn new() -> Node {
    Node {
      value: json!({}),
      text:  Text::default()
    }
  }
}

/// JSON builder struct for building JSON from XML
pub struct JsonBuilder {
  charkey:          String,
  attrkey:          String,
  empty_tag:        String,
  explicit_root:    bool,
  trim:             bool,
  ignore_attrs:     bool,
  merge_attrs:      bool,
  normalize_text:   bool,
  lowercase_tags:   bool,
  explicit_array:   bool,
  explicit_charkey: bool
}

impl Default for JsonBuilder {
  fn default() -> JsonBuilder {
    JsonBuilder {
      charkey:          "_".to_owned(),
      attrkey:          "$".to_owned(),
      empty_tag:        "".to_owned(),
      explicit_root:    true,
      trim:             false,
      ignore_attrs:     false,
      merge_attrs:      false,
      normalize_text:   false,
      lowercase_tags:   false,
      explicit_array:   true,
      explicit_charkey: false
    }
  }
}

impl JsonBuilder {
  // If text matches only newlines, spaces and tabs
  fn is_whitespace(&self, value: &str) -> bool {
    WHITESPACE_RE.is_match(value)
  }

  // This function is used to build out the JSON object.
  // the behavior depends on the `explicit_array` setting. When this value is
  // - true: an array will be created at `key` if it doesn't exist and new values will be pushed
  // - false: `value` is assigned at `key` and converted into an array if there are multiple values
  // at that key
  fn assign_or_push(&self, object: &mut JsonValue, key: &str, value: JsonValue) {
    if object.get(key).is_none() {
      if self.explicit_array {
        object[key] = json!([value]);
      } else {
        object[key] = value;
      }
    } else {
      // Wrap object[key] in an array if it isn't one already
      if !object[key].is_array() {
        let current = object[key].take();
        object[key] = json!([current]);
      }
      if let Some(array) = object[key].as_array_mut() {
        array.push(value);
      }
    }
  }

  // Process start tag
  fn process_start(&self, event: &BytesStart, stack: &mut Vec<Node>, reader: &mut Reader<&[u8]>) -> Result<(), Error> {
    let mut node = Node::new();

    // Add any attributes
    if !self.ignore_attrs {
      // Initialize attribute object
      if event.attributes().peekable().peek().is_some() && node.value.get(&self.attrkey).is_none() && !self.merge_attrs {
        node.value[&self.attrkey] = json!({});
      }

      for attr in event.attributes() {
        if let Ok(attr) = attr {
          let value = attr.unescape_and_decode_value(&reader)?;
          let key = std::str::from_utf8(attr.key)?;
          if self.merge_attrs {
            self.assign_or_push(&mut node.value, key, value.into());
          } else {
            node.value[&self.attrkey][key] = value.into();
          }
        }
      }
    }

    stack.push(node);
    Ok(())
  }

  // Process text
  fn process_text(&self, event: &BytesText, stack: &mut Vec<Node>, reader: &mut Reader<&[u8]>) -> Result<(), Error> {
    let cdata = event.unescape_and_decode(&reader)?;

    if let Some(last_node) = stack.last_mut() {
      let text = &mut last_node.text.data;
      // Setting reader.trim_text will remove all whitespaces in char data. To preserve
      // compatibility with node-xml2js two or more consecutive whitespace characters will be
      // replaced with a single space and then the resulting string will be trimmed
      if self.normalize_text && !text.is_empty() {
        let normalized = TWO_OR_MORE_WHITESPACE_RE.replace_all(text, NoExpand(" ")).into_owned();
        text.clear();
        text.push_str(&normalized);
        let _ = text.trim();
      }
      text.push_str(&cdata);
    }

    Ok(())
  }

  // Process end, takes a `tag` rather than an `event` since an Event::Empty(e) uses this function as
  // well
  fn process_end(&self, tag: &[u8], stack: &mut Vec<Node>) -> Result<Option<JsonValue>, Error> {
    let close_tag = if self.lowercase_tags {
      std::str::from_utf8(tag)?.to_lowercase()
    } else {
      std::str::from_utf8(tag)?.to_owned()
    };
    // The JSON value that which will be nested inside of `outer` (unless we are at EOF)
    let mut inner = match stack.pop() {
      Some(j) => j,
      None => return Err(Error::new(ErrorKind::Unknown, "Expected stack item at close tag."))
    };
    let stack_len = stack.len();
    let outer = stack.last_mut();

    // This can grow to contain other whitespace characters ('\s')
    let mut whitespace = "".to_owned();
    let mut text = inner.text.data.as_ref();

    if self.is_whitespace(text) && !inner.text.literal {
      whitespace.push_str(text);
    } else {
      if self.trim {
        text = text.trim();
      }

      // FIXME: warning for unused `normalized` can this be restructured in a better way?
      let mut _normalized = String::new();
      if self.normalize_text {
        _normalized = TWO_OR_MORE_WHITESPACE_RE.replace_all(text, NoExpand(" ")).into_owned();
        text = _normalized.trim().as_ref();
      }

      if utils::json_is_empty(&inner.value) && !self.explicit_charkey {
        inner.value = JsonValue::String(text.to_owned());
      } else {
        inner.value[&self.charkey] = text.into();
      }
    }

    if utils::json_is_empty(&inner.value) {
      if !self.empty_tag.is_empty() {
        inner.value = JsonValue::String(self.empty_tag.clone());
      } else {
        inner.value = JsonValue::String(whitespace);
      }
    }

    // Check if we have closed all open tags
    if stack_len > 0 {
      if let Some(outer) = outer {
        self.assign_or_push(&mut outer.value, &close_tag, inner.value);
      }
    } else {
      // At EOF - either wrap result in an explicit root or return inner's value
      let output = if self.explicit_root {
        let output = json!({
          close_tag: inner.value
        });
        output
      } else {
        inner.value
      };
      return Ok(Some(output));
    }
    Ok(None)
  }

  // Process empty
  fn process_empty(&self, event: &BytesStart, stack: &mut Vec<Node>, reader: &mut Reader<&[u8]>) -> Result<Option<JsonValue>, Error> {
    self.process_start(event, stack, reader)?;
    self.process_end(event.name(), stack)
  }

  // Process XML CDATA
  fn process_cdata(&self, event: &BytesCData, stack: &mut Vec<Node>, reader: &mut Reader<&[u8]>) -> Result<(), Error> {
    self.process_text(&event.clone().escape(), stack, reader)?;

    if let Some(mut last_node) = stack.last_mut() {
      last_node.text.literal = true;
    }
    Ok(())
  }

  /// Build JSON from xml
  pub fn build_from_xml(&self, xml: &str) -> Result<JsonValue, Error> {
    let mut reader = Reader::from_str(xml);
    let mut buffer = Vec::new();
    let mut output = JsonValue::Null;
    let mut stack = Vec::new();

    loop {
      match reader.read_event(&mut buffer) {
        Ok(Event::Start(ref e)) => self.process_start(e, &mut stack, &mut reader)?,

        Ok(Event::Text(ref e)) => self.process_text(e, &mut stack, &mut reader)?,

        Ok(Event::End(ref e)) => {
          if let Some(o) = self.process_end(e.name(), &mut stack)? {
            output = o;
          }
        },

        Ok(Event::CData(ref e)) => self.process_cdata(e, &mut stack, &mut reader)?,

        Ok(Event::Empty(ref e)) => {
          if let Some(o) = self.process_empty(e, &mut stack, &mut reader)? {
            output = o;
          }
        },

        Ok(Event::Eof) => {
          break;
        },

        // Skip over everything else
        Ok(_) => (),

        Err(e) => {
          return Err(Error::new(
            ErrorKind::Syntax,
            format!("Error at position {}: {:?}", reader.buffer_position(), e)
          ))
        },
      }

      buffer.clear();
    }

    Ok(output)
  }

  /// Build JSON string from xml
  pub fn build_string_from_xml(&self, xml: &str) -> Result<String, Error> {
    let object = self.build_from_xml(xml)?;
    serde_json::to_string(&object).map_err(|e| e.into())
  }

  /// Build pretty JSON string from xml
  pub fn build_pretty_string_from_xml(&self, xml: &str) -> Result<String, Error> {
    let object = self.build_from_xml(xml)?;
    serde_json::to_string_pretty(&object).map_err(|e| e.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn invalid_xml() {
    let builder = JsonBuilder::default();
    let err = builder.build_from_xml("<foo>bar</baz>").unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Syntax)
  }

  #[test]
  fn is_whitespace1() {
    let builder = JsonBuilder::default();
    assert!(builder.is_whitespace(" \t \n "));
  }

  #[test]
  fn is_whitespace2() {
    let builder = JsonBuilder::default();
    assert!(!builder.is_whitespace(" \t A \n "));
  }

  #[test]
  fn assign_or_push1() {
    let builder = JsonBuilder::default();
    let mut actual = json!({});
    let _ = builder.assign_or_push(&mut actual, "A", "B".into());
    let _ = builder.assign_or_push(&mut actual, "C", "D".into());
    let _ = builder.assign_or_push(&mut actual, "C", "E".into());
    let expected: JsonValue = serde_json::from_str(r#"{"A":["B"],"C":["D","E"]}"#).unwrap();
    assert_eq!(actual, expected);
  }

  #[test]
  fn assign_or_push2() {
    let builder = JsonConfig::new().explicit_array(false).finalize();
    let mut actual = json!({});
    let _ = builder.assign_or_push(&mut actual, "A", "B".into());
    let _ = builder.assign_or_push(&mut actual, "C", "D".into());
    let _ = builder.assign_or_push(&mut actual, "C", "E".into());
    let expected: JsonValue = serde_json::from_str(r#"{"A":"B","C":["D","E"]}"#).unwrap();
    assert_eq!(actual, expected);
  }
}
