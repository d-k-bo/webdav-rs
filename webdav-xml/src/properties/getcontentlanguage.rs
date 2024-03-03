// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytestring::ByteString;

use crate::{Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX};

/// The `getcontentlanguage` property as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#PROPERTY_getcontentlanguage).
#[derive(Clone, Debug, PartialEq)]
pub struct ContentLanguage(pub ByteString);

impl Element for ContentLanguage {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "getcontentlanguage";
}

impl TryFrom<&Value> for ContentLanguage {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Ok(Self(value.to_text()?.clone()))
    }
}

impl From<ContentLanguage> for Value {
    fn from(ContentLanguage(s): ContentLanguage) -> Value {
        Value::Text(s)
    }
}

#[cfg(test)]
#[test]
fn test() -> eyre::Result<()> {
    use crate::utils::{test_deserialize, test_serialize};

    let xml = r#"<d:getcontentlanguage xmlns:d="DAV:">de-DE, en-CA</d:getcontentlanguage>"#;
    let content_language = ContentLanguage("de-DE, en-CA".into());

    test_deserialize(&content_language, xml)?;
    test_serialize(xml, content_language)?;

    Ok(())
}
