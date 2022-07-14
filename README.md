# XML2JSON

![Tests](https://github.com/novcn/xml2json-rs/actions/workflows/test.yml/badge.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A rust library for converting to and from XML and JSON.

# Install

``` bash
λ › cargo add xml2json
```

# Usage

## JSON to XML

### XmlBuilder
`XmlBuilder` builds XML from JSON. 
- `build_from_json` builds an XML `String` from a `serde_json::Value`.
- `build_from_json_string` builds an XML `String` from a serialized JSON `String`.

#### Example
```rust
use xml2json::XmlBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut xml_builder = XmlBuilder::default();
  let xml= xml_builder.build_from_json_string(r#"{"book":{"$":{"category":"fantasy"},"title":{"_":"The Name of the Wind","$":{"lang":"en"}},"author":"Patrick Rothfuss","year":"2007"}}"#)?;
  assert_eq!(xml, r#"<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title><author>Patrick Rothfuss</author><year>2007</year></book>"#);
  Ok(())
}
```

### XmlConfig
`XmlConfig` Uses the Builder pattern to set configuration options and then `finalize` to
build an `XmlBuilder`.

#### Example

```rust
use xml2json::XmlConfig;
use xml2json::{ Indentation, Declaration, Version, Encoding };
use std::error::Error;
use indoc::indoc;

fn main() -> Result<(), Box<dyn Error>> {
  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .attr_key("^")
    .root_name("store")
    .finalize();

  let xml = xml_builder.build_from_json_string(r#"{"book":{"^":{"category":"fantasy"},"title":{"_":"The Name of the Wind","^":{"lang":"en"}},"author":"Patrick Rothfuss","year":"2007"}}"#)?;
  assert_eq!(xml, indoc!(r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <store>
    <book category="fantasy">
      <title lang="en">The Name of the Wind</title>
      <author>Patrick Rothfuss</author>
      <year>2007</year>
    </book>
  </store>"#));
  Ok(())
}

```
## XML to JSON

### JsonBuilder
`JsonBuilder` builds JSON from XML. 
- `build_from_xml` build a `serde_json::Value` from an XML `String`.
- `build_string_from_xml` build a JSON serialized `String` from an XML `String`.
- `build_pretty_string_from_xml` build a pretty-printed JSON serialized `String` from an XML `String`.

### Example

```rust
use xml2json::JsonBuilder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let json_builder = JsonBuilder::default();
  let json = json_builder.build_string_from_xml(r#"<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title><author>Patrick Rothfuss</author><year>2007</year></book>"#)?;
  assert_eq!(json, r#"{"book":{"$":{"category":"fantasy"},"title":{"$":{"lang":"en"},"_":"The Name of the Wind"},"author":"Patrick Rothfuss","year":"2007"}}"#);
  Ok(())
}
```

#### JsonConfig
`JsonConfig` Uses the Builder pattern to set configuration options and then `finalize` to build an `JsonBuilder`.

### Example

```rust
use xml2json::JsonConfig;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let json_builder = JsonConfig::new()
    .ignore_attrs(true)
    .explicit_array(false)
    .finalize();
  let json = json_builder.build_string_from_xml(r#"<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title><author>Patrick Rothfuss</author><year>2007</year></book>"#)?;
  assert_eq!(json, r#"{"book":{"title":"The Name of the Wind","author":"Patrick Rothfuss","year":"2007"}}"#);
  Ok(())
}
```

# Objective

This library was inspired by [node-xml2json](https://github.com/buglabs/node-xml2json) and has a primary objective of maintaining parity with its `0.4.20` version. 

# Tests

Integration tests are generated via scripts using `node-xml2json` in order to verify parity.

### Generating

``` bash
λ › cd tests/generator
λ › yarn
λ › ./generate
```

### Running

``` bash
λ › cargo test
```
