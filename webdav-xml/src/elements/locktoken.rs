use crate::{
    elements::Href, Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap,
    DAV_NAMESPACE, DAV_PREFIX,
};

/// The `locktoken` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_locktoken).
#[derive(Clone, Debug, PartialEq)]
pub struct LockToken {
    pub href: Href,
}

impl Element for LockToken {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "locktoken";
}

impl TryFrom<&Value> for LockToken {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match map.get::<Href>() {
            Some(Ok(href)) => Ok(LockToken { href }),
            Some(Err(e)) => Err(e),
            None => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("href"),
            )),
        }
    }
}

impl From<LockToken> for Value {
    fn from(lock_token: LockToken) -> Self {
        let mut map = ValueMap::new();
        map.insert::<Href>(lock_token.href.into());

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elements::{Href, LockToken},
        FromXml, IntoXml,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:locktoken xmlns:d="DAV:">
  <d:href>/path/to/locktoken</d:href>
</d:locktoken>
        "#;

        let lock_token = LockToken::from_xml(xml).expect("Failed to deserialize LockToken");

        assert_eq!(lock_token.href.0, "/path/to/locktoken");
    }

    #[test]
    fn test_serialize() {
        let lock_token = LockToken {
            href: Href("/path/to/locktoken".parse().expect("Failed to parse Href")),
        };

        let bytes = lock_token
            .into_xml()
            .expect("Failed to serialize LockToken");

        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:locktoken xmlns:d="DAV:">
  <d:href>/path/to/locktoken</d:href>
</d:locktoken>
        "#
        .trim();

        assert_eq!(xml, expected_xml);
    }
}
