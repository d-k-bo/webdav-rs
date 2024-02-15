// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{utils::HeaderIteratorExt, DEPTH};

/// The `Depth` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_Depth).
#[derive(Clone, Debug, PartialEq)]
pub enum Depth {
    Zero,
    One,
    Infinity,
}
impl headers::Header for Depth {
    fn name() -> &'static http::HeaderName {
        &DEPTH
    }
    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        match values.take_one().map(|v| v.as_bytes())? {
            b"0" => Ok(Depth::Zero),
            b"1" => Ok(Depth::One),
            b"infinity" => Ok(Depth::Infinity),
            _ => Err(headers::Error::invalid()),
        }
    }
    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(match self {
            Depth::Zero => headers::HeaderValue::from_static("0"),
            Depth::One => headers::HeaderValue::from_static("1"),
            Depth::Infinity => headers::HeaderValue::from_static("infinity"),
        }))
    }
}

#[cfg(test)]
#[test]
fn test() {
    use crate::test::test_all;

    test_all([
        ("0", Depth::Zero),
        ("1", Depth::One),
        ("infinity", Depth::Infinity),
    ]);
}
