// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Reusable types for implementing WebDAV clients and servers based on
//! [RFC 4918][rfc].
//!
//! This crate is intended to be used together with libraries that build on the
//! general-purpose [`http`][http] crate.
//!
//! [rfc]: http://webdav.org/specs/rfc4918.html
//! [http]: https://docs.rs/http/latest/http/
//!
//! # Usage
//!
//! ```sh
//! cargo add webdav-meta --rename webdav
//! ```
//!
//! # Implemented features
//!
//! <details>
//!     <summary>HTTP Methods</summary>
//!
//! HTTP methods are currently defined as static variables, but should be moved
//! to constants in the future.
//!
//! - [X] [`PROPFIND`](crate::methods::PROPFIND)
//! - [X] [`PROPPATCH`](crate::methods::PROPPATCH)
//! - [X] [`MKCOL`](crate::methods::MKCOL)
//! - [X] [`COPY`](crate::methods::COPY)
//! - [X] [`MOVE`](crate::methods::MOVE)
//! - [X] [`LOCK`](crate::methods::LOCK)
//! - [X] [`UNLOCK`](crate::methods::UNLOCK)
//!
//! </details>
//! <details>
//!     <summary>HTTP Headers</summary>
//!
//! - [X] [`DAV`](crate::headers::Dav)
//! - [X] [`Depth`](crate::headers::Depth)
//! - [X] [`Destination`](crate::headers::Destination)
//! - [X] [`If`](crate::headers::If)
//! - [X] [`Lock-Token`](crate::headers::LockToken)
//! - [X] [`Overwrite`](crate::headers::Overwrite)
//! - [X] [`Timeout`](crate::headers::Timeout)
//!
//! </details>
//! <details>
//!     <summary>XML Elements</summary>
//!
//! - [ ] `activelock`
//! - [X] `allprop`: internally implemented for
//!   [`Propfind`](crate::xml::elements::Propfind)
//! - [X] `collection`: internally implemented for
//!   [`ResourceType`](crate::xml::properties::ResourceType)
//! - [ ] `depth`
//! - [ ] `error`: currently just a string
//! - [ ] `exclusive`
//! - [X] [`href`](crate::xml::elements::Href)
//! - [X] [`include`](crate::xml::elements::Include)
//! - [ ] `location`
//! - [ ] `lockentry`
//! - [ ] `lockinfo`
//! - [ ] `lockroot`
//! - [ ] `lockscope`
//! - [ ] `locktoken`
//! - [ ] `locktype`
//! - [X] [`multistatus`](crate::xml::elements::Multistatus)
//! - [ ] `owner`
//! - [X] [`prop`](crate::xml::elements::Properties)
//! - [ ] `propertyupdate`
//! - [X] [`propfind`](crate::xml::elements::Propfind)
//! - [X] `propname`: internally implemented for
//!   [`Propfind`](crate::xml::elements::Propfind)
//! - [X] [`propstat`](crate::xml::elements::Propstat)
//! - [ ] `remove`
//! - [X] [`response`](crate::xml::elements::Response)
//! - [X] [`responsedescription`](crate::xml::elements::ResponseDescription)
//! - [ ] `set`
//! - [ ] `shared`
//! - [ ] `status`
//! - [ ] `timeout`
//! - [ ] `write`
//!
//! </details>
//! <details>
//!     <summary>DAV properties</summary>
//!
//! - [X] [`creationdate`](crate::xml::properties::CreationDate)
//! - [X] [`displayname`](crate::xml::properties::DisplayName)
//! - [X] [`getcontentlanguage`](crate::xml::properties::ContentLanguage)
//! - [X] [`getcontentlength`](crate::xml::properties::ContentLength)
//! - [X] [`getcontenttype`](crate::xml::properties::ContentType)
//! - [X] [`getetag`](crate::xml::properties::ETag)
//! - [X] [`getlastmodified`](crate::xml::properties::LastModified)
//! - [X] [`lockdiscovery`](crate::xml::properties::LockDiscovery)
//! - [X] [`resourcetype`](crate::xml::properties::ResourceType)
//! - [X] [`supportedlock`](crate::xml::properties::SupportedLock)
//!
//! </details>

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "headers")]
#[cfg_attr(docsrs, doc(cfg(feature = "headers")))]
#[doc(inline)]
pub use webdav_headers as headers;

#[cfg(feature = "methods")]
#[cfg_attr(docsrs, doc(cfg(feature = "methods")))]
#[doc(inline)]
pub use webdav_methods as methods;

#[cfg(feature = "xml")]
#[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
#[doc(inline)]
pub use webdav_xml as xml;
