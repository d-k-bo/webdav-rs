use crate::{
    Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap, DAV_NAMESPACE,
    DAV_PREFIX,
};

/// The `lockscope` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_lockscope).
#[derive(Clone, Debug, PartialEq)]
pub enum LockScope {
    Exclusive,
    Shared,
}

impl Element for LockScope {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "lockscope";
}

impl TryFrom<&Value> for LockScope {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match (map.get::<Exclusive>(), map.get::<Shared>()) {
            (Some(_), None) => Ok(LockScope::Exclusive),
            (None, Some(_)) => Ok(LockScope::Shared),
            _ => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("exclusive or shared"),
            )),
        }
    }
}

impl From<LockScope> for Value {
    fn from(lock_scope: LockScope) -> Self {
        let mut map = ValueMap::new();

        match lock_scope {
            LockScope::Exclusive => map.insert::<Exclusive>(Exclusive.into()),
            LockScope::Shared => map.insert::<Shared>(Shared.into()),
        };

        Value::Map(map)
    }
}

/// The `exclusive` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_exclusive).
#[derive(Clone, Debug, PartialEq)]
pub struct Exclusive;

impl Element for Exclusive {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "exclusive";
}

impl TryFrom<&Value> for Exclusive {
    type Error = ExtractElementError;

    fn try_from(_: &Value) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl From<Exclusive> for Value {
    fn from(_: Exclusive) -> Self {
        Value::Empty
    }
}

/// The `shared` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_shared).
#[derive(Clone, Debug, PartialEq)]
pub struct Shared;

impl Element for Shared {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "shared";
}

impl TryFrom<&Value> for Shared {
    type Error = ExtractElementError;

    fn try_from(_: &Value) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl From<Shared> for Value {
    fn from(_: Shared) -> Self {
        Value::Empty
    }
}

#[cfg(test)]
mod tests {
    use crate::{elements::LockScope, FromXml, IntoXml};

    #[test]
    fn test_deserialize_exclusive() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockscope xmlns:d="DAV:">
  <d:exclusive/>
</d:lockscope>
        "#;

        let lock_scope = LockScope::from_xml(xml).expect("Failed to deserialize LockScope");

        assert_eq!(lock_scope, LockScope::Exclusive);
    }

    #[test]
    fn test_deserialize_shared() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockscope xmlns:d="DAV:">
  <d:shared/>
</d:lockscope>
        "#;

        let lock_scope = LockScope::from_xml(xml).expect("Failed to deserialize LockScope");

        assert_eq!(lock_scope, LockScope::Shared);
    }

    #[test]
    fn test_serialize_exclusive() {
        let lock_scope = LockScope::Exclusive;

        let bytes = lock_scope
            .into_xml()
            .expect("Failed to serialize LockScope");

        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockscope xmlns:d="DAV:">
  <d:exclusive/>
</d:lockscope>
        "#
        .trim();

        assert_eq!(xml, expected_xml);
    }

    #[test]
    fn test_serialize_shared() {
        let lock_scope = LockScope::Shared;

        let bytes = lock_scope
            .into_xml()
            .expect("Failed to serialize LockScope");

        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockscope xmlns:d="DAV:">
  <d:shared/>
</d:lockscope>
        "#
        .trim();

        assert_eq!(xml, expected_xml);
    }
}
