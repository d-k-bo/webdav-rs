// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{utils::HeaderIteratorExt, CodedUrl, LOCK_TOKEN};

/// The `Lock-Token` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_Lock-Token).
#[derive(Clone, Debug, PartialEq)]
pub struct LockToken(pub CodedUrl);

impl headers::Header for LockToken {
    fn name() -> &'static http::HeaderName {
        &LOCK_TOKEN
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        values
            .extract_str()?
            .parse()
            .map(Self)
            .map_err(|_| headers::Error::invalid())
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.0.to_string().parse().unwrap()))
    }
}
