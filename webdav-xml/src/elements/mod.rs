// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! XML element definitions based on
//! [RFC 4918](http://webdav.org/specs/rfc4918.html#xml.element.definitions).

mod href;
mod multistatus;
mod prop;
mod propfind;
mod propstat;
mod response;
mod responsedescription;
mod status;

pub use self::{
    href::Href,
    multistatus::Multistatus,
    prop::Properties,
    propfind::{Include, Propfind},
    propstat::Propstat,
    response::Response,
    responsedescription::ResponseDescription,
    status::Status,
};
