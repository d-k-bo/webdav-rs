// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytestring::ByteString;

use crate::{Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX};

/// The `displayname` property as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#PROPERTY_displayname).
#[derive(Clone, Debug, PartialEq)]
pub struct DisplayName(pub ByteString);

impl Element for DisplayName {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "displayname";
}

impl TryFrom<&Value> for DisplayName {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.to_text()?.clone()))
    }
}

impl From<DisplayName> for Value {
    fn from(DisplayName(s): DisplayName) -> Value {
        Value::Text(s)
    }
}

#[cfg(test)]
#[test]
fn test() -> eyre::Result<()> {
    use crate::utils::{test_deserialize, test_serialize};

    let xml = r#"<d:displayname xmlns:d="DAV:">Example HTML resource</d:displayname>"#;
    let display_name = DisplayName("Example HTML resource".into());

    test_deserialize(&display_name, xml)?;
    test_serialize(xml, display_name)?;

    Ok(())
}
