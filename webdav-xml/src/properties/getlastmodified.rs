// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use httpdate::HttpDate;

use crate::{Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX};

/// The `getlastmodified` property as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#PROPERTY_getlastmodified).
#[derive(Clone, Debug, PartialEq)]
pub struct LastModified(pub HttpDate);

impl Element for LastModified {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "getlastmodified";
}

impl TryFrom<&Value> for LastModified {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.to_text()?.parse() {
            Ok(date) => Ok(Self(date)),
            Err(e) => Err(ExtractElementError::other(e)),
        }
    }
}

impl From<LastModified> for Value {
    fn from(LastModified(date): LastModified) -> Value {
        date.to_string().into()
    }
}

#[cfg(test)]
#[test]
fn test() -> eyre::Result<()> {
    use std::time::SystemTime;

    use time::OffsetDateTime;

    use crate::utils::{test_deserialize, test_serialize};

    let xml =
        r#"<d:getlastmodified xmlns:d="DAV:">Mon, 12 Jan 1998 09:25:56 GMT</d:getlastmodified>"#;
    let last_modified = LastModified(HttpDate::from(SystemTime::from(OffsetDateTime::new_utc(
        time::Date::from_calendar_date(1998, time::Month::January, 12)?,
        time::Time::from_hms(9, 25, 56)?,
    ))));

    test_deserialize(&last_modified, xml)?;
    test_serialize(xml, last_modified)?;

    Ok(())
}
