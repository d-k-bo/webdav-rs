// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use http::HeaderValue;

use crate::{
    utils::{HeaderIteratorExt, StrExt},
    TIMEOUT,
};

/// The `Timeout` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_Timeout).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Timeout {
    Seconds(u32),
    Infinite,
}

impl headers::Header for Timeout {
    fn name() -> &'static http::HeaderName {
        &TIMEOUT
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let s = values.extract_str()?;

        if s.eq_ignore_ascii_case("Infinite") {
            Ok(Self::Infinite)
        } else if let Some(seconds) = s
            .strip_prefix_ignore_ascii_case("Second-")
            .and_then(|s| s.parse().ok())
        {
            Ok(Self::Seconds(seconds))
        } else {
            Err(headers::Error::invalid())
        }
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(match self {
            Self::Seconds(seconds) => format!("Second-{seconds}").try_into().unwrap(),
            Self::Infinite => HeaderValue::from_static("Infinite"),
        }))
    }
}

#[cfg(test)]
#[test]
fn test() {
    use crate::test::test_all;

    test_all([
        ("Second-123", Timeout::Seconds(123)),
        ("Infinite", Timeout::Infinite),
    ])
}
