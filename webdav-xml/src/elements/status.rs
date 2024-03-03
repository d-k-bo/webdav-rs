// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fmt::Display, str::FromStr};

use crate::{Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX};

/// The `status` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_status).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Status(pub http::StatusCode);

impl Element for Status {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "status";
}

impl From<http::StatusCode> for Status {
    fn from(code: http::StatusCode) -> Self {
        Self(code)
    }
}

impl FromStr for Status {
    type Err = InvalidStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        parts.next().ok_or_else(|| InvalidStatus(s.into()))?;
        let status_code = parts
            .next()
            .and_then(|code| code.parse().ok())
            .ok_or_else(|| InvalidStatus(s.into()))?;
        Ok(Self(status_code))
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("HTTP/1.1 {}", self.0))
    }
}
impl TryFrom<&Value> for Status {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.to_text()?.parse() {
            Ok(status) => Ok(status),
            Err(e) => Err(ExtractElementError::other(e)),
        }
    }
}
impl From<Status> for Value {
    fn from(status: Status) -> Value {
        status.to_string().into()
    }
}

#[derive(Debug)]
pub struct InvalidStatus(String);

impl std::fmt::Display for InvalidStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid status: {}", self.0)
    }
}

impl std::error::Error for InvalidStatus {}

#[cfg(test)]
#[test]
fn test() -> eyre::Result<()> {
    assert_eq!("HTTP/1.1 200 OK", Status(http::StatusCode::OK).to_string());
    assert_eq!(
        Status::from_str("HTTP/1.1 200 OK")?,
        Status(http::StatusCode::OK)
    );

    Ok(())
}
