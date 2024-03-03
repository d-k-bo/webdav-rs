// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytes::Bytes;

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type to wrap all errors that might occur when (de)serializing.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Xml(XmlError),
    ExtractElement(ExtractElementError),
}

impl From<XmlError> for Error {
    fn from(e: XmlError) -> Self {
        Self::Xml(e)
    }
}

impl From<ExtractElementError> for Error {
    fn from(e: ExtractElementError) -> Self {
        Self::ExtractElement(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Xml(e) => write!(f, "{e}"),
            Self::ExtractElement(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Xml(e) => Some(e),
            Self::ExtractElement(e) => Some(e),
        }
    }
}

/// Returned when extracting an element from a [`Value`](crate::Value) fails.
#[derive(Debug)]
pub struct ExtractElementError {
    pub kind: ExtractElementErrorKind,
    #[cfg(debug_assertions)]
    location: &'static std::panic::Location<'static>,
}

impl ExtractElementError {
    #[track_caller]
    pub fn new(kind: ExtractElementErrorKind) -> Self {
        Self {
            kind,
            #[cfg(debug_assertions)]
            location: std::panic::Location::caller(),
        }
    }
    #[track_caller]
    pub fn other(e: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Self::new(ExtractElementErrorKind::Other(e.into()))
    }
}

impl std::fmt::Display for ExtractElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;

        #[cfg(debug_assertions)]
        write!(f, " at {}", self.location)?;

        Ok(())
    }
}

impl std::error::Error for ExtractElementError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ExtractElementErrorKind::Other(e) => Some(&**e),
            _ => None,
        }
    }
}

/// The reason why extracting an element failed.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExtractElementErrorKind {
    InvalidValueType {
        expected: &'static str,
        got: &'static str,
    },
    MissingElement(&'static str),
    ConflictingElements(&'static [&'static str]),
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl std::fmt::Display for ExtractElementErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValueType { expected, got } => {
                write!(f, "expected value of type `{expected}`, got `{got}`")
            }
            Self::MissingElement(name) => write!(f, "missing `{name}` element"),
            Self::ConflictingElements(names) => {
                for (i, name) in names.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "`{name}`")?;
                }
                f.write_str(" must not be used at the same time")
            }
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

/// Returned when reading or writing XML failed.
#[derive(Debug)]
pub enum XmlError {
    InvalidNamespace(Bytes),
    UnexpectedTag,
    Utf8(std::str::Utf8Error),
    Xml(quick_xml::Error),
}

impl From<std::str::Utf8Error> for XmlError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<quick_xml::Error> for XmlError {
    fn from(e: quick_xml::Error) -> Self {
        Self::Xml(e)
    }
}

impl std::fmt::Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNamespace(bytes) => write!(f, "invalid namespace declaration: {bytes:?}"),
            Self::Xml(e) => write!(f, "{e}"),
            Self::UnexpectedTag => write!(f, "unexpected tag"),
            Self::Utf8(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for XmlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Xml(e) => Some(e),
            Self::Utf8(e) => Some(e),
            _ => None,
        }
    }
}
