// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{utils::HeaderIteratorExt, DESTINATION};

/// The `Destination` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_Destination).
#[derive(Clone, Debug, PartialEq)]
pub struct Destination(pub http::Uri);

impl headers::Header for Destination {
    fn name() -> &'static http::HeaderName {
        &DESTINATION
    }
    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        Ok(Self(
            values
                .extract_str()?
                .parse()
                .map_err(|_| headers::Error::invalid())?,
        ))
    }
    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.0.to_string().parse().unwrap()))
    }
}

#[cfg(test)]
#[test]
fn test() {
    use crate::test::test_all;

    test_all([
        (
            "https://example.com/foo",
            Destination("https://example.com/foo".parse().unwrap()),
        ),
        ("/foo/bar/123", Destination("/foo/bar/123".parse().unwrap())),
    ])
}
