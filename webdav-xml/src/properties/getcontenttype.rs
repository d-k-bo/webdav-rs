// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use mime::Mime;

use crate::{Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX};

/// The `getcontenttype` property as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#PROPERTY_getcontenttype).
#[derive(Clone, Debug, PartialEq)]
pub struct ContentType(pub Mime);

impl Element for ContentType {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "getcontenttype";
}

impl TryFrom<&Value> for ContentType {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.to_text()?.parse() {
            Ok(mime) => Ok(Self(mime)),
            Err(e) => Err(ExtractElementError::other(e)),
        }
    }
}

impl From<ContentType> for Value {
    fn from(ContentType(content_type): ContentType) -> Value {
        content_type.to_string().into()
    }
}

#[cfg(test)]
#[test]
fn test() -> eyre::Result<()> {
    use crate::utils::{test_deserialize, test_serialize};

    let xml = r#"<d:getcontenttype xmlns:d="DAV:">text/html</d:getcontenttype>"#;
    let content_type = ContentType(mime::TEXT_HTML);

    test_deserialize(&content_type, xml)?;
    test_serialize(xml, content_type)?;

    Ok(())
}
