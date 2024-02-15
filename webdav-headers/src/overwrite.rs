// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::OVERWRITE;

/// The `Overwrite` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_Overwrite-Token).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Overwrite {
    F,
    #[default]
    T,
}

impl headers::Header for Overwrite {
    fn name() -> &'static http::HeaderName {
        &OVERWRITE
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        values
            .next()
            .map(headers::HeaderValue::as_bytes)
            .and_then(|s| match s {
                b"F" | b"f" => Some(Overwrite::F),
                b"T" | b"t" => Some(Overwrite::T),
                _ => None,
            })
            .ok_or_else(headers::Error::invalid)
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(match self {
            Overwrite::F => headers::HeaderValue::from_static("F"),
            Overwrite::T => headers::HeaderValue::from_static("T"),
        }))
    }
}

#[cfg(test)]
#[test]
fn test() {
    use crate::test::test_all;

    test_all([("F", Overwrite::F), ("T", Overwrite::T)])
}
