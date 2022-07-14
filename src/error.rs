use std::{error::Error as StdError, fmt};

use quick_xml::Error as XmlError;
use serde_json::error::Error as JsonError;
use std::{str::Utf8Error, string::FromUtf8Error};

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorKind {
  Syntax,
  Encoding,
  Unknown
}

impl ErrorKind {
  pub fn to_str(&self) -> &'static str {
    match *self {
      ErrorKind::Syntax => "parse",
      ErrorKind::Encoding => "encoding",
      ErrorKind::Unknown => "unknown"
    }
  }
}

/// This type represents all possible errors that can occur when converting to or from XML and JSON
pub struct Error {
  details: String,
  desc:    &'static str,
  kind:    ErrorKind
}

impl Error {
  /// Initialize a new Error with `ErrorKind`
  pub fn new<T: Into<String>>(kind: ErrorKind, detail: T) -> Error {
    let desc = kind.to_str();
    Error {
      kind,
      desc,
      details: detail.into()
    }
  }

  /// Error kind
  pub fn kind(&self) -> ErrorKind {
    self.kind.clone()
  }

  /// Error details
  pub fn details(&self) -> String {
    self.details.clone()
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.kind.to_str(), self.details)
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    self.desc
  }
}

impl From<JsonError> for Error {
  fn from(e: JsonError) -> Self {
    Error::new(ErrorKind::Syntax, format!("{}", e))
  }
}

impl From<XmlError> for Error {
  fn from(e: XmlError) -> Self {
    Error::new(ErrorKind::Unknown, format!("{}", e))
  }
}

impl From<FromUtf8Error> for Error {
  fn from(e: FromUtf8Error) -> Self {
    Error::new(ErrorKind::Encoding, format!("{}", e))
  }
}

impl From<Utf8Error> for Error {
  fn from(e: Utf8Error) -> Self {
    Error::new(ErrorKind::Encoding, format!("{}", e))
  }
}
