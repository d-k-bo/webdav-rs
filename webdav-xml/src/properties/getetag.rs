// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytestring::ByteString;

use crate::{Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX};

/// The `getetag` property as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#PROPERTY_getetag).
#[derive(Clone, Debug, PartialEq)]
pub struct ETag(pub ByteString);

impl Element for ETag {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "getetag";
}

impl TryFrom<&Value> for ETag {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.to_text()?.clone()))
    }
}

impl From<ETag> for Value {
    fn from(ETag(s): ETag) -> Value {
        Value::Text(s)
    }
}

#[cfg(test)]
#[test]
fn test() -> eyre::Result<()> {
    use crate::utils::{test_deserialize, test_serialize};

    let xml = r#"<d:getetag xmlns:d="DAV:">"10c24bc-4ab-457e1c1f"</d:getetag>"#;
    let etag = ETag(r#""10c24bc-4ab-457e1c1f""#.into());

    test_deserialize(&etag, xml)?;
    test_serialize(xml, etag)?;

    Ok(())
}
