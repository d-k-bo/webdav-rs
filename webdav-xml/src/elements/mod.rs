// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! XML element definitions based on
//! [RFC 4918](http://webdav.org/specs/rfc4918.html#xml.element.definitions).

mod activelock;
mod depth;
mod href;
mod lockentry;
mod lockinfo;
mod lockroot;
mod lockscope;
mod locktoken;
mod locktype;
mod multistatus;
mod owner;
mod prop;
mod propertyupdate;
mod propfind;
mod propstat;
mod response;
mod responsedescription;
mod status;
mod timeout;

pub use self::{
    activelock::ActiveLock,
    depth::Depth,
    href::Href,
    lockentry::LockEntry,
    lockinfo::LockInfo,
    lockroot::LockRoot,
    lockscope::{Exclusive, LockScope, Shared},
    locktoken::LockToken,
    locktype::{LockType, Write},
    multistatus::Multistatus,
    owner::Owner,
    prop::Properties,
    propertyupdate::{PropertyUpdate, Remove, Set},
    propfind::{Include, Propfind},
    propstat::Propstat,
    response::Response,
    responsedescription::ResponseDescription,
    status::Status,
    timeout::Timeout,
};
