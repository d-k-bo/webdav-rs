// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

use crate::{utils::HeaderIteratorExt, CodedUrl, ParseString, DAV};

pub use self::error::InvalidComplianceClass;

/// The `DAV` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_DAV).
#[derive(Clone, Debug, PartialEq)]
pub struct Dav(pub Vec<ComplianceClass>);

impl headers::Header for Dav {
    fn name() -> &'static http::HeaderName {
        &DAV
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        Ok(Self(
            values
                .extract_str()?
                .split(',')
                .map(str::trim)
                .map(ComplianceClass::from_str)
                .collect::<Result<_, _>>()?,
        ))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.0.iter().join(",").try_into().unwrap()))
    }
}

/// Compliance class identifiers used in the `DAV` header.
#[derive(Clone, Debug, PartialEq)]
pub enum ComplianceClass {
    One,
    Two,
    Three,
    CodedUrl(Box<CodedUrl>),
    Tokens(Tokens),
}
impl Display for ComplianceClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => f.write_str("1"),
            Self::Two => f.write_str("2"),
            Self::Three => f.write_str("3"),
            Self::CodedUrl(uri) => uri.fmt(f),
            Self::Tokens(s) => s.fmt(f),
        }
    }
}
impl FromStr for ComplianceClass {
    type Err = InvalidComplianceClass;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(InvalidComplianceClass::Empty),
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            _ => match CodedUrl::peek(s) {
                Ok((coded_url, _)) => Ok(Self::CodedUrl(Box::new(coded_url))),
                Err(_) => Ok(Self::Tokens(s.parse()?)),
            },
        }
    }
}

/// A freeform compliance class identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tokens(String);

impl Tokens {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Tokens {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Tokens> for String {
    fn from(Tokens(s): Tokens) -> Self {
        s
    }
}

impl std::fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Tokens {
    type Err = InvalidComplianceClass;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().find(|c| {
            !c.is_ascii()
                || c.is_ascii_control()
                || c.is_ascii_punctuation()
                || c.is_ascii_whitespace()
        }) {
            Some(c) => Err(InvalidComplianceClass::InvalidChar(c)),
            None => Ok(Self(s.to_owned())),
        }
    }
}

mod error {
    /// Error returned when parsing [`ComplianceClass`](super::ComplianceClass)
    /// from a string fails.
    #[derive(Debug)]
    pub enum InvalidComplianceClass {
        Empty,
        InvalidChar(char),
    }

    impl std::fmt::Display for InvalidComplianceClass {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => f.write_str("empty compliance class"),
                Self::InvalidChar(c) => write!(f, "invalid character in compliance class '{c:?}'"),
            }
        }
    }

    impl std::error::Error for InvalidComplianceClass {}

    impl From<InvalidComplianceClass> for headers::Error {
        fn from(_: InvalidComplianceClass) -> Self {
            headers::Error::invalid()
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    use crate::test::test;

    test(
        "1,2,3,<https://example.com/foo>,foobar",
        Dav(vec![
            ComplianceClass::One,
            ComplianceClass::Two,
            ComplianceClass::Three,
            ComplianceClass::CodedUrl(Box::new(CodedUrl(
                uniresid::AbsoluteUri::parse("https://example.com/foo").unwrap(),
            ))),
            ComplianceClass::Tokens("foobar".parse().unwrap()),
        ]),
    )
}
