// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(rustdoc::redundant_explicit_links)]

//! Definitions and (de)serialization for WebDAV XML elements as defined
//! in [RFC 4918](http://webdav.org/specs/rfc4918.html#xml.element.definitions).
//!
//! Since WebDAV uses XML namespaces and supports custom elements in the
//! `<DAV:prop />` element, we can't rely on e. g. `serde` to (de)serialize
//! XML elements.
//!
//! Instead, this crate uses the [`Element`](crate::Element) trait to define an
//! element and [`FromXml`](crate::FromXml)/[`IntoXml`](crate::IntoXml) for
//! (de)serialization.

mod element;
pub mod elements;
mod error;
pub mod properties;
mod read;
mod utils;
mod value;
mod write;

use bytes::{BufMut, Bytes};

#[doc(no_inline)]
pub use nonempty;

pub use self::{
    element::Element,
    error::{Error, ExtractElementError, ExtractElementErrorKind, Result, XmlError},
    value::{Value, ValueMap},
};

/// The default WebDAV namespace
pub const DAV_NAMESPACE: &str = "DAV:";
/// The default WebDAV namespace prefix
pub const DAV_PREFIX: &str = "d";

/// Performs deserialization from XML.
///
/// This trait is automatically implemented for any type which implements the
/// `Element + for<'v> TryFrom<&'v Value, Error = ExtractElementError>` traits.
/// As such, `FromXml` shouldn't be implemented directly: [`Element`] and
/// [`TryFrom<&Value>`] should be implemented instead, and you get the `FromXml`
/// implementation for free.
pub trait FromXml: Sized {
    fn from_xml(xml: impl Into<bytes::Bytes>) -> crate::Result<Self>;
}

impl FromXml for Value {
    fn from_xml(xml: impl Into<bytes::Bytes>) -> crate::Result<Self> {
        Ok(crate::read::read_xml(xml)?)
    }
}

impl<E> FromXml for E
where
    E: Element + for<'v> TryFrom<&'v Value, Error = ExtractElementError>,
{
    fn from_xml(xml: impl Into<bytes::Bytes>) -> crate::Result<Self> {
        Ok(Value::from_xml(xml)?
            .to_map()?
            .get::<E>()
            .required::<E>()??)
    }
}

/// Performs serialization to XML.
///
/// This trait is automatically implemented for any type which implements the
/// `Element + Into<Value>` traits.
/// As such, `IntoXml` shouldn't be implemented directly: [`Element`] and
/// [`Into<Value>`] should be implemented instead, and you get the `IntoXml`
/// implementation for free.
pub trait IntoXml: Sized {
    fn write_xml(self, writer: impl std::io::Write) -> crate::Result<()>;
    fn into_xml(self) -> crate::Result<Bytes> {
        let mut xml = bytes::BytesMut::new().writer();
        self.write_xml(&mut xml)?;
        Ok(xml.into_inner().freeze())
    }
}

impl<T> IntoXml for T
where
    T: Element + Into<Value>,
{
    fn write_xml(self, writer: impl std::io::Write) -> crate::Result<()> {
        Ok(crate::write::write_xml::<T>(writer, self.into())?)
    }
}

/// A type that can't be instantiated.
///
/// Prevents not yet implemented types from being instantiated.
#[derive(Clone, Debug, PartialEq)]
enum Todo {}

pub(crate) trait OptionExt<T> {
    fn required<E: Element>(self) -> std::result::Result<T, ExtractElementError>;
}
impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn required<E: Element>(self) -> std::result::Result<T, ExtractElementError> {
        match self {
            Some(v) => Ok(v),
            None => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement(E::LOCAL_NAME),
            )),
        }
    }
}
