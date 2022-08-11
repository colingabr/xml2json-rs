use quick_xml::{events::*, Writer};

use std::{cell::RefCell, convert::TryFrom, io::Cursor, ops::DerefMut, rc::Rc};

use crate::{
  error::{Error, ErrorKind},
  utils
};

use serde_json::{Map as JsonMap, Value as JsonValue};

#[derive(Clone)]
/// XML [Declaration] encoding.
///
/// For now only UTF-8 is supported
///
/// [Declaration]: struct.Declaration.html
pub enum Encoding {
  /// UTF-8
  UTF8 // see https://www.w3resource.com/xml/declarations.php
}

impl Encoding {
  /// Serialize `Encoding` as a `&' static str`
  pub fn to_string(&self) -> &'static str {
    match *self {
      Encoding::UTF8 => "UTF-8"
    }
  }
}

impl TryFrom<&str> for Encoding {
  type Error = Error;

  /// Try to initialize an `Encoding` from a `&str`.
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    match s {
      "UTF-8" | "UTF8" => Ok(Encoding::UTF8),
      _ => Err(Error::new(ErrorKind::Encoding, format!("Cannot convert from {} to Encoding.", s)))
    }
  }
}

#[derive(Clone)]
/// XML [Declaration] version.
///
/// Setting this in a [Declaration] will not alter the output of the XML apart from writing the
/// version number to the declaration.
///
/// [Declaration]: struct.Declaration.html
pub enum Version {
  /// 1.0
  XML10,
  /// 1.1
  XML11
}

impl Version {
  /// Serialize `Version` as a `&' static str`
  pub fn to_string(&self) -> &'static str {
    match *self {
      Version::XML10 => "1.0",
      Version::XML11 => "1.1"
    }
  }
}

impl TryFrom<&str> for Version {
  type Error = Error;

  /// Try to initialize a `Version` from `&str`
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    match s {
      "1.0" => Ok(Version::XML10),
      "1.1" => Ok(Version::XML11),
      _ => Err(Error::new(ErrorKind::Unknown, format!("Cannot convert from {} to Version.", s)))
    }
  }
}

#[derive(Clone)]
/// XML Declaration
pub struct Declaration {
  version:    Version,
  encoding:   Option<Encoding>,
  standalone: Option<bool>
}

impl Default for Declaration {
  fn default() -> Declaration {
    Declaration {
      version:    Version::XML10,
      encoding:   None,
      standalone: None
    }
  }
}

impl Declaration {
  /// Initialize  a Declaration
  pub fn new(version: Version, encoding: Option<Encoding>, standalone: Option<bool>) -> Declaration {
    Declaration {
      version,
      encoding,
      standalone
    }
  }

  fn as_bytes_decl(&self) -> BytesDecl {
    let version = self.version.to_string().as_bytes();
    let encoding = self.encoding.as_ref().map(|v| v.to_string().as_bytes());
    let standalone = self
      .standalone
      .as_ref()
      .map(|v| if *v { "yes".as_bytes() } else { "no".as_bytes() });

    BytesDecl::new(version, encoding, standalone)
  }
}

#[derive(Clone, Debug)]
/// XML Indentation rendering options
pub struct Indentation {
  indent_char: u8,
  indent_size: usize
}

/// Optional indentation rendering
impl Indentation {
  /// Initialize Indentation instance.
  ///
  /// This can be passed as a `XmlConfig.rendering` option to output XML with line-breaks and
  /// indentations.
  pub fn new(indent_char: u8, indent_size: usize) -> Indentation {
    Indentation { indent_char, indent_size }
  }
}

impl Default for Indentation {
  fn default() -> Indentation {
    let indent_char = b' ';
    Indentation {
      indent_char,
      indent_size: 2
    }
  }
}

/// XmlBuilder configuration options
pub struct XmlConfig {
  attrkey:   Option<String>,
  charkey:   Option<String>,
  root_name: Option<String>,
  decl:      Option<Declaration>,
  rendering: Option<Indentation>
}

impl Default for XmlConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl XmlConfig {
  /// Initialze a new XmlConfig instance.
  ///
  /// This uses the builder pattern. All options are initialized to `None` and can be set using
  /// `self`s methods. Any options not set will use their defaults upon call to `finalize`.
  pub fn new() -> XmlConfig {
    XmlConfig {
      root_name: None,
      attrkey:   None,
      charkey:   None,
      decl:      None,
      rendering: None
    }
  }

  /// Root key name to contain produced JSON object.
  ///
  /// When this is set to its default value "root", the output will not be wrapped in `root_name`'s
  /// value. This is to conform to match the behavior of node-xml2js.
  ///
  /// (`"root"` by default)
  pub fn root_name<T: Into<String>>(&mut self, key: T) -> &mut XmlConfig {
    self.root_name = Some(key.into());
    self
  }

  /// Attribute key
  ///
  /// The value of the JSON key used to store XML attributes under.
  ///
  /// (`"$"` by default)
  pub fn attrkey<T: Into<String>>(&mut self, key: T) -> &mut XmlConfig {
    self.attrkey = Some(key.into());
    self
  }

  /// Char data key
  ///
  /// The value of the JSON key used to store XML character data under.
  ///
  /// (`"_"` by default)
  pub fn charkey<T: Into<String>>(&mut self, key: T) -> &mut XmlConfig {
    self.charkey = Some(key.into());
    self
  }

  /// XML Declaration
  ///
  /// ([Declaration::default()] by default)
  /// [Declaration::default()]: Declaration::default
  pub fn decl(&mut self, decl: Declaration) -> &mut XmlConfig {
    self.decl = Some(decl);
    self
  }

  /// Rendering indentation options
  ///
  /// (`None` by default)
  pub fn rendering(&mut self, indentation: Indentation) -> &mut XmlConfig {
    self.rendering = Some(indentation);
    self
  }

  /// Finalize configuration options and build an XmlBuilder instance
  pub fn finalize(&self) -> XmlBuilder {
    let writer = if let Some(indentation) = &self.rendering {
      Writer::new_with_indent(Cursor::new(Vec::new()), indentation.indent_char, indentation.indent_size)
    } else {
      Writer::new(Cursor::new(Vec::new()))
    };

    let decl = self.decl.clone().unwrap_or_default();

    XmlBuilder {
      root_name: self.root_name.clone().unwrap_or_else(|| "root".to_owned()),
      attrkey: self.attrkey.clone().unwrap_or_else(|| "$".to_owned()),
      charkey: self.charkey.clone().unwrap_or_else(|| "_".to_owned()),
      decl,
      writer: Rc::new(RefCell::new(writer)),
      indent: self.rendering.clone()
    }
  }
}

/// XML builder
pub struct XmlBuilder {
  attrkey:   String,
  charkey:   String,
  root_name: String,
  decl:      Declaration,
  writer:    Rc<RefCell<Writer<Cursor<Vec<u8>>>>>,
  indent:    Option<Indentation>
}

impl Default for XmlBuilder {
  fn default() -> XmlBuilder {
    XmlBuilder {
      root_name: "root".to_owned(),
      attrkey:   "$".to_owned(),
      charkey:   "_".to_owned(),
      decl:      Declaration::default(),
      writer:    Rc::new(RefCell::new(Writer::new(Cursor::new(Vec::new())))),
      indent:    None
    }
  }
}

/// Tag attributes type. A vector of key, value tuples which are each byte arrays
type TagAttrs<'a> = Vec<(&'a [u8], &'a [u8])>;

impl XmlBuilder {
  // Check if key is an attribute key
  fn is_attrkey(&self, key: &str) -> bool {
    self.attrkey == *key
  }

  // Check if key is a character key
  fn is_charkey(&self, key: &str) -> bool {
    self.charkey == *key
  }

  // Get all a attributes at node. If successful, returns a vector of (name, value) attributes
  fn tag_attributes<'a>(&self, node: &'a JsonValue) -> Result<TagAttrs<'a>, Error> {
    // Node should either be an object {} or a wrapped object [{}]
    // If it's an array, unwrap it and call self recursively
    if let Some(array) = node.as_array() {
      if array.len() == 1 {
        let child = array.iter().next().unwrap_or(&JsonValue::Null);
        self.tag_attributes(child)
      } else {
        Ok(Vec::new())
      }
    } else {
      let mut attrs = Vec::new();
      if let Some(attrs_value) = node.get(&self.attrkey) {
        if let Some(object) = attrs_value.as_object() {
          for (name, value) in object {
            let attr = value
              .as_str()
              .ok_or_else(|| Error::new(ErrorKind::Syntax, "Expected attribute to be a string."))?;
            attrs.push((name.as_bytes(), attr.as_bytes()));
          }
        }
      }
      Ok(attrs)
    }
  }

  // Self closing tags are elements that don't contain text or other elements
  fn is_empty_tag(&self, node: &JsonValue) -> bool {
    // Check if child is an empty string, if not recursively check it's child
    if let Some(array) = node.as_array() {
      if let Some(child) = array.iter().next() {
        return child.is_string() && utils::json_is_empty(child) || self.is_empty_tag(child);
      }
    } else if let Some(object) = node.as_object() {
      for (k, v) in object.iter() {
        if !self.is_attrkey(k) && !utils::json_is_empty(v) {
          return false;
        }
      }
      return true;
    }
    utils::json_is_empty(node)
  }

  // Write XML declaration
  fn write_xml_decl(&mut self) -> Result<(), Error> {
    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();
    writer.write_event(Event::Decl(self.decl.as_bytes_decl())).map_err(|e| e.into())
  }

  // Write element's start tag including any attributes
  fn write_start_tag(&mut self, key: &str, node: &JsonValue) -> Result<(), Error> {
    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();

    // Initialize the tag with key value
    let mut tag = BytesStart::owned(key.to_owned(), key.len());

    // Write any attributes
    let attributes = self.tag_attributes(node)?;
    for attr in attributes {
      tag.push_attribute(attr);
    }

    // Write the tag as either empty / self-closing (<element />) or as a start tag (<element>)
    let tag_is_empty = self.is_empty_tag(node);
    if tag_is_empty {
      writer.write_event(Event::Empty(tag)).map_err(|e| e.into())
    } else {
      writer.write_event(Event::Start(tag)).map_err(|e| e.into())
    }
  }

  // Write text
  fn write_text(&mut self, text: &str) -> Result<(), Error> {
    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();
    let text_content = BytesText::from_plain_str(text);
    writer.write_event(Event::Text(text_content)).map_err(|e| e.into())
  }

  // Write element's end tag if the element wasn't self-closing
  fn write_end_tag(&mut self, key: &str, node: &JsonValue) -> Result<(), Error> {
    // If the tag was self-closing, do not write an end tag
    if self.is_empty_tag(node) {
      return Ok(());
    }

    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();
    let tag_end = BytesEnd::owned(key.as_bytes().into());
    writer.write_event(Event::End(tag_end)).map_err(|e| e.into())
  }

  // Write a string without triggering any indentation heuristics
  fn write_raw(&mut self, value: &str) -> Result<(), Error> {
    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();
    writer.write(value.as_bytes()).map_err(|e| e.into())
  }

  // Write an indentation. Used when quick-xml's indentation heuristic doesn't
  // have the context to properly indent
  fn write_indent(&mut self) -> Result<(), Error> {
    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();
    writer.write_indent().map_err(|e| e.into())
  }

  /// Write end of file
  fn write_eof(&mut self) -> Result<(), Error> {
    let mut writer_ref = self.writer.borrow_mut();
    let writer = writer_ref.deref_mut();
    writer.write_event(Event::Eof).map_err(|e| e.into())
  }

  // A leaf node is an object that contains no keys apart from attrkey or charkey
  fn is_leaf_node(&self, node: &JsonMap<String, JsonValue>) -> bool {
    let normal_keys: Vec<&str> = node
      .iter()
      .filter(|&(k, _)| !self.is_charkey(k) && !self.is_attrkey(k))
      .map(|(k, _)| k.as_ref())
      .collect();
    if normal_keys.is_empty() {
      return true;
    }
    false
  }

  // Recursively traverse JSON while writing XML
  fn traverse(&mut self, node: &JsonValue, parent_key: Option<String>) -> Result<(), Error> {
    if let Some(object) = node.as_object() {
      // Iterate over child object elements
      for (key, child) in object {
        // Traverse if the parent is not an attribute and not a character key
        let pk = parent_key.clone().unwrap_or_else(|| "".to_owned());
        if !self.is_attrkey(&pk) && !self.is_charkey(&pk) {
          if self.is_charkey(&key) {
            if self.indent.is_some() && !self.is_leaf_node(object) {
              if let Some(s) = child.as_str() {
                // Write indentation for a case quick-xml's auto-indent heuristic doesn't cover
                self.write_indent()?;
                self.write_raw(s)?;
              }
            } else {
              self.traverse(child, Some(key.to_owned()))?;
            }
          }
          // If we're not at an attribute and child is an object, write start tag, traverse and continue
          else if !self.is_attrkey(&key) {
            if !child.is_array() {
              self.write_start_tag(key, child)?;
              self.traverse(child, None)?;
              self.write_end_tag(key, child)?;
            } else {
              self.traverse(child, Some(key.to_owned()))?;
            }
            continue;
          } else {
            self.traverse(child, Some(key.to_owned()))?;
          }
        }
      }
    } else if let Some(array) = node.as_array() {
      // Iterate over child array elements
      for child in array {
        if let Some(ref pk) = parent_key.as_ref() {
          self.write_start_tag(pk, child)?;
          self.traverse(child, None)?;
          self.write_end_tag(pk, child)?;
        } else {
          self.traverse(child, None)?;
        }
      }
    } else {
      let node_s = utils::to_string_raw(node);
      if !node_s.is_empty() {
        self.write_text(&node_s)?;
      }
    }

    Ok(())
  }

  /// Build XML from a JSON value
  pub fn build_from_json(&mut self, root: &JsonValue) -> Result<String, Error> {
    // As per node-xml2js - if the root name "root" is used, then it is not added to the produced xml
    // document. It's unclear if this is a bug or not. Keeping this behavior for now for parity reasons
    let explicit_root = self.root_name != *"root" || utils::json_object_key_len(root) > 1;
    let root_name = self.root_name.clone();

    self.write_xml_decl()?;

    // If an explicit root is set, write that before the root defined in JSON
    if explicit_root {
      self.write_start_tag(&root_name, root)?;
    }


    self.traverse(root, Some(root_name.clone()))?;

    if explicit_root {
      self.write_end_tag(&root_name, root)?;
    }

    self.write_eof()?;

    let mut writer_guard = self.writer.borrow_mut();
    let writer_ref = writer_guard.deref_mut();
    let result = writer_ref.inner().get_ref();
    String::from_utf8(result.to_vec()).map_err(|e| e.into())
  }

  /// Build XML from a JSON string
  pub fn build_from_json_string(&mut self, json_s: &str) -> Result<String, Error> {
    let root = serde_json::from_str(json_s)?;
    self.build_from_json(&root)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[test]
  fn build_simple() {
    let mut builder = XmlBuilder::default();
    let xml = builder.build_from_json_string(r#"{"foo":"bar"}"#).unwrap();
    assert_eq!(xml, r#"<?xml version="1.0"?><foo>bar</foo>"#);
  }

  #[test]
  fn leaf_node1() {
    let builder = XmlBuilder::default();
    let node = json!({});
    let is_leaf = builder.is_leaf_node(&node.as_object().unwrap());
    assert!(is_leaf);
  }

  #[test]
  fn leaf_node2() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"{"$": {}, "_": {}}"#).unwrap();
    let is_leaf = builder.is_leaf_node(&node);
    assert!(is_leaf);
  }

  #[test]
  fn leaf_node3() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"{"a": {}}"#).unwrap();
    let is_leaf = builder.is_leaf_node(&node);
    assert!(!is_leaf);
  }

  #[test]
  fn leaf_node4() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"{"$": {}, "_": {}, "a": "b"}"#).unwrap();
    let is_leaf = builder.is_leaf_node(&node);
    assert!(!is_leaf);
  }

  #[test]
  fn empty_tag1() {
    let builder = XmlBuilder::default();
    let node = json!({});
    let is_empty = builder.is_empty_tag(&node);
    assert!(is_empty);
  }

  #[test]
  fn empty_tag2() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"[{"$":{"desc":"nodata"}}]"#).unwrap();
    let is_empty = builder.is_empty_tag(&node);
    assert!(is_empty);
  }

  #[test]
  fn empty_tag3() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"{"$":{"desc":"nodata"},"_":""}"#).unwrap();
    let is_empty = builder.is_empty_tag(&node);
    assert!(is_empty);
  }

  #[test]
  fn empty_tag4() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"[{"$":{"desc":"nodata"},"_":"A"}]"#).unwrap();
    let is_empty = builder.is_empty_tag(&node);
    assert!(!is_empty);
  }

  #[test]
  fn empty_tag5() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"[{"$":{"desc":"nodata"},"A":{"B":"C"}}]"#).unwrap();
    let is_empty = builder.is_empty_tag(&node);
    assert!(!is_empty);
  }

  #[test]
  fn tag_attributes1() {
    let builder = XmlBuilder::default();
    let node = json!({});
    let attrs = builder.tag_attributes(&node).unwrap();
    assert!(attrs.is_empty());
  }

  #[test]
  fn tag_attributes2() {
    let builder = XmlBuilder::default();
    let node = serde_json::from_str(r#"{"$":{"A":"B","C":"D"}}"#).unwrap();
    let attrs = builder.tag_attributes(&node).unwrap();
    assert!(!attrs.is_empty());
    let mut expected = vec![("A", "B"), ("C", "D")];
    expected.reverse(); // lazy alternative to a veqdeque
    for attr in attrs {
      let (k, v) = attr;
      let a_key = std::str::from_utf8(k).unwrap();
      let a_val = std::str::from_utf8(v).unwrap();
      let (e_key, e_val) = expected.pop().unwrap();
      assert_eq!(e_key, a_key);
      assert_eq!(e_val, a_val);
    }
  }

  #[test]
  fn tag_attributes3() {
    let builder = XmlConfig::new().attrkey("^").finalize();
    let node = serde_json::from_str(r#"{"^":{"A":"B","C":"D"}}"#).unwrap();
    let attrs = builder.tag_attributes(&node).unwrap();
    assert!(!attrs.is_empty());
    let mut expected = vec![("A", "B"), ("C", "D")];
    expected.reverse(); // lazy alternative to a veqdeque
    for attr in attrs {
      let (k, v) = attr;
      let a_key = std::str::from_utf8(k).unwrap();
      let a_val = std::str::from_utf8(v).unwrap();
      let (e_key, e_val) = expected.pop().unwrap();
      assert_eq!(e_key, a_key);
      assert_eq!(e_val, a_val);
    }
  }

  #[test]
  fn attrkey1() {
    let builder = XmlBuilder::default();
    let is_key = builder.is_attrkey(&"$".to_owned());
    assert!(is_key);
  }

  #[test]
  fn attrkey2() {
    let builder = XmlConfig::new().attrkey("^").finalize();
    assert!(builder.is_attrkey(&"^".to_owned()));
    assert!(!builder.is_attrkey(&"$".to_owned()));
  }

  #[test]
  fn charkey1() {
    let builder = XmlBuilder::default();
    let is_key = builder.is_charkey(&"_".to_owned());
    assert!(is_key);
  }

  #[test]
  fn charkey2() {
    let builder = XmlConfig::new().charkey("^").finalize();
    assert!(builder.is_charkey(&"^".to_owned()));
    assert!(!builder.is_charkey(&"_".to_owned()));
  }
}
