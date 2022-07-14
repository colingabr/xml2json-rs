//! # XML2JSON
//!
//! A library for converting to and from XML and JSON.
//!
//! # JSON to XML
//!
//! ## XmlBuilder
//! [`XmlBuilder`] builds a XML from JSON.
//! - [`build_from_json`] builds an XML `String` from a [`serde_json::Value`]
//! - [`build_from_json_string`] builds an XML `String` from a serialized JSON `String`.
//!
//! ### Example
//! ```rust
//! use xml2json_rs::XmlBuilder;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   let mut xml_builder = XmlBuilder::default();
//!   let xml= xml_builder.build_from_json_string(r#"{"book":{"$":{"category":"fantasy"},"title":[{"_":"The Name of the Wind","$":{"lang":"en"}}],"author":["Patrick Rothfuss"],"year":["2007"]}}"#)?;
//!   assert_eq!(xml, r#"<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title><author>Patrick Rothfuss</author><year>2007</year></book>"#);
//!   Ok(())
//! }
//! ```
//!
//! ## XmlConfig
//! [`XmlConfig`] Uses the [Builder] pattern to set configuration options and then `finalize` to
//! build an [`XmlBuilder`]
//!
//! ### Example
//!
//! ```rust
//! use xml2json_rs::XmlConfig;
//! use xml2json_rs::{ Indentation, Declaration, Version, Encoding };
//! use std::error::Error;
//! use indoc::indoc;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   let mut xml_builder = XmlConfig::new()
//!     .rendering(Indentation::new(b' ', 2))
//!     .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
//!     .attrkey("^")
//!     .root_name("store")
//!     .finalize();
//!
//!   let xml = xml_builder.build_from_json_string(r#"{"book":{"^":{"category":"fantasy"},"title":[{"_":"The Name of the Wind","^":{"lang":"en"}}],"author":["Patrick Rothfuss"],"year":["2007"]}}"#)?;
//!   assert_eq!(xml, indoc!(r#"
//!   <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
//!   <store>
//!     <book category="fantasy">
//!       <title lang="en">The Name of the Wind</title>
//!       <author>Patrick Rothfuss</author>
//!       <year>2007</year>
//!     </book>
//!   </store>"#));
//!   Ok(())
//! }
//! ```
//! ## XML to JSON
//!
//! ### JsonBuilder
//! [`JsonBuilder`] builds JSON from XML.
//! - [`build_from_xml`] build a [`serde_json::Value`] from an XML `String`.
//! - [`build_string_from_xml`] build a JSON serialized `String` from an XML `String`.
//! - [`build_pretty_string_from_xml`] build a pretty-printed JSON serialized `String` from an XML
//! `String`
//!
//! ## Example
//!
//! ```rust
//! use xml2json_rs::JsonBuilder;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   let json_builder = JsonBuilder::default();
//!   let json = json_builder.build_string_from_xml(r#"<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title><author>Patrick Rothfuss</author><year>2007</year></book>"#)?;
//!   assert_eq!(json, r#"{"book":{"$":{"category":"fantasy"},"title":[{"$":{"lang":"en"},"_":"The Name of the Wind"}],"author":["Patrick Rothfuss"],"year":["2007"]}}"#);
//!   Ok(())
//! }
//! ```
//!
//! ### JsonConfig
//! [`JsonConfig`] Uses the [Builder] pattern to set configuration options and then `finalize` to
//! build an [`JsonBuilder`]
//!
//! ## Example
//!
//! ```rust
//! use xml2json_rs::JsonConfig;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!   let json_builder = JsonConfig::new()
//!     .ignore_attrs(true)
//!     .explicit_array(false)
//!     .finalize();
//!   let json = json_builder.build_string_from_xml(r#"<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title><author>Patrick Rothfuss</author><year>2007</year></book>"#)?;
//!   assert_eq!(json, r#"{"book":{"title":"The Name of the Wind","author":"Patrick Rothfuss","year":"2007"}}"#);
//!   Ok(())
//! }
//! ```
//!
//! [`XmlBuilder`]: struct.XmlBuilder.html
//! [`XmlConfig`]: struct.XmlConfig.html
//! [`serde_json::Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
//! [`build_pretty_string_from_xml`]: struct.JsonBuilder.html#method.build_pretty_string_from_xml
//! [`build_string_from_xml`]: struct.JsonBuilder.html#method.build_string_from_xml
//! [`build_from_xml`]: struct.JsonBuilder.html#method.build_from_xml
//! [`build_from_json_string`]: struct.XmlBuilder.html#method.build_from_json_string
//! [`build_from_json`]: struct.XmlBuilder.html#method.build_from_json
//! [`JsonConfig`]: struct.JsonConfig.html
//! [`JsonBuilder`]: struct.JsonBuilder.html
//! [Builder]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html

#![deny(missing_docs)]

extern crate quick_xml;
extern crate regex;

#[macro_use]
extern crate lazy_static;

mod json;
mod xml;

pub use json::{JsonBuilder, JsonConfig};

pub use xml::{Declaration, Encoding, Indentation, Version, XmlBuilder, XmlConfig};

pub use error::Error as X2JError;

mod error;
mod utils;
