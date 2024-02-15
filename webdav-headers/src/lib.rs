// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! HTTP headers for WebDAV as defined in
//! [RFC 4918](http://webdav.org/specs/rfc4918.html#http.headers.for.distributed.authoring)
//! implementing the [`headers::Header`] trait.

mod dav;
mod depth;
mod destination;
mod if_;
mod lock_token;
mod overwrite;
mod timeout;
mod utils;

use self::utils::ParseString;

pub use self::{
    coded_url::{CodedUrl, InvalidCodedUrl},
    dav::{ComplianceClass, Dav, InvalidComplianceClass, Tokens},
    depth::Depth,
    destination::Destination,
    if_::{Condition, If, InvalidIf, ResourceTag},
    lock_token::LockToken,
    names::*,
    overwrite::Overwrite,
    timeout::Timeout,
};

mod names {
    /// Header name of the [`DAV`](super::Dav) header.
    pub static DAV: headers::HeaderName = headers::HeaderName::from_static("dav");
    /// Header name of the [`Depth`](super::Depth) header.
    pub static DEPTH: headers::HeaderName = headers::HeaderName::from_static("depth");
    /// Header name of the [`Destination`](super::Destination) header.
    pub static DESTINATION: headers::HeaderName = headers::HeaderName::from_static("destination");
    /// Header name of the [`If`](super::If) header.
    pub static IF: headers::HeaderName = headers::HeaderName::from_static("if");
    /// Header name of the [`LockToken`](super::LockToken) header.
    pub static LOCK_TOKEN: headers::HeaderName = headers::HeaderName::from_static("lock-token");
    /// Header name of the [`Overwrite`](super::Overwrite) header.
    pub static OVERWRITE: headers::HeaderName = headers::HeaderName::from_static("overwrite");
    /// Header name of the [`Timeout`](super::Timeout) header.
    pub static TIMEOUT: headers::HeaderName = headers::HeaderName::from_static("timeout");
}

mod coded_url {
    use crate::utils::ParseString;

    pub use self::error::InvalidCodedUrl;

    /// Coded-URL used in the `DAV` and `If` headers
    #[derive(Clone, Debug, PartialEq)]
    pub struct CodedUrl(pub uniresid::AbsoluteUri);

    impl std::fmt::Display for CodedUrl {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "<{}>", self.0)
        }
    }

    impl std::str::FromStr for CodedUrl {
        type Err = InvalidCodedUrl;

        fn from_str(mut s: &str) -> Result<Self, Self::Err> {
            Self::parse(&mut s)
        }
    }

    impl ParseString for CodedUrl {
        type Err = InvalidCodedUrl;

        fn peek(mut s: &str) -> Result<(Self, &str), Self::Err> {
            if s.starts_with('<') {
                s = &s[1..];
            } else {
                return Err(InvalidCodedUrl::ExpectedChar('<'));
            }
            let Some(end) = s.find('>') else {
                return Err(InvalidCodedUrl::ExpectedChar('>'));
            };

            let uri = uniresid::AbsoluteUri::parse(&s[..end]).map_err(InvalidCodedUrl::Uri)?;

            Ok((CodedUrl(uri), &s[end + 1..]))
        }
    }

    mod error {
        /// Error returned when parsing [`CodedUrl`](super::CodedUrl) from a
        /// string fails.
        #[derive(Debug)]
        pub enum InvalidCodedUrl {
            ExpectedChar(char),
            Uri(uniresid::Error),
        }

        impl std::fmt::Display for InvalidCodedUrl {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::ExpectedChar(c) => write!(f, "expected '{c}'"),
                    Self::Uri(..) => write!(f, "invalid Absolute-URI"),
                }
            }
        }

        impl std::error::Error for InvalidCodedUrl {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    Self::Uri(e) => Some(e),
                    _ => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    // based on https://github.com/hyperium/headers/blob/2b9fc5be92f0346482aa6d09917a434a56ade3f3/src/common/mod.rs#L72-L88
    #[track_caller]
    pub(crate) fn test_decode<T: headers::Header>(values: &[&str]) -> Option<T> {
        use headers::HeaderMapExt;
        let mut map = http::HeaderMap::new();
        for val in values {
            map.append(T::name(), val.parse().unwrap());
        }
        map.typed_get()
    }

    pub(crate) fn test_encode<T: headers::Header>(header: T) -> http::HeaderMap {
        use headers::HeaderMapExt;
        let mut map = http::HeaderMap::new();
        map.typed_insert(header);
        map
    }

    #[track_caller]
    pub(crate) fn test<T>(s: &'static str, header: T)
    where
        T: headers::Header + std::fmt::Debug + PartialEq,
    {
        use pretty_assertions::assert_eq;

        assert_eq!(
            header,
            match test_decode::<T>(&[s]) {
                Some(header) => header,
                None => panic!("failed to decode \"{s}\""),
            }
        );
        assert_eq!(s, test_encode(header)[T::name()]);
    }

    #[track_caller]
    pub(crate) fn test_all<T>(testcases: impl IntoIterator<Item = (&'static str, T)>)
    where
        T: headers::Header + std::fmt::Debug + PartialEq,
    {
        for (s, header) in testcases {
            test(s, header)
        }
    }
}
