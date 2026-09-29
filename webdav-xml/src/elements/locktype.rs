use crate::{
    Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap, DAV_NAMESPACE,
    DAV_PREFIX,
};

/// The `locktype` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_locktype).
#[derive(Clone, Debug, PartialEq)]
pub enum LockType {
    Write(Write),
}

impl Element for LockType {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "locktype";
}

impl TryFrom<&Value> for LockType {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match map.get::<Write>() {
            Some(Ok(write)) => Ok(LockType::Write(write)),
            Some(Err(e)) => Err(e),
            None => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("write"),
            )),
        }
    }
}

impl From<LockType> for Value {
    fn from(lock_type: LockType) -> Self {
        match lock_type {
            LockType::Write(write) => {
                let mut map = ValueMap::new();
                map.insert::<Write>(write.into());

                Value::Map(map)
            }
        }
    }
}

/// The `write` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_write).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Write;

impl Element for Write {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "write";
}

impl TryFrom<&Value> for Write {
    type Error = ExtractElementError;

    fn try_from(_: &Value) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl From<Write> for Value {
    fn from(_: Write) -> Self {
        Value::Empty
    }
}

#[cfg(test)]
mod tests {
    use crate::{elements::LockType, FromXml, IntoXml};

    #[test]
    fn test_deserialize() {
        let xml = r#"<d:locktype xmlns:d="DAV:"><d:write/></d:locktype>"#;

        let value = LockType::from_xml(xml).unwrap();
        let expected = LockType::Write(Default::default());
        assert_eq!(value, expected);
    }

    #[test]
    fn test_serialize() {
        let lock_type = LockType::Write(Default::default());

        let bytes = lock_type.into_xml().unwrap();
        let xml = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(
            xml,
            r#"
<?xml version="1.0" encoding="utf-8"?>
<d:locktype xmlns:d="DAV:">
  <d:write/>
</d:locktype>
        "#
            .trim()
        );
    }
}
