use crate::{
    elements::{LockScope, LockType},
    Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap, DAV_NAMESPACE,
    DAV_PREFIX,
};

/// The `lockentry` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_lockentry).
#[derive(Clone, Debug, PartialEq)]
pub struct LockEntry {
    pub lock_scope: LockScope,
    pub lock_type: LockType,
}

impl Element for LockEntry {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "lockentry";
}

impl TryFrom<&Value> for LockEntry {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match (map.get::<LockScope>(), map.get::<LockType>()) {
            (Some(Ok(lock_scope)), Some(Ok(lock_type))) => Ok(LockEntry {
                lock_scope,
                lock_type,
            }),
            (Some(Err(e)), _) => Err(e),
            (_, Some(Err(e))) => Err(e),
            (None, _) => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("lockscope"),
            )),
            (_, None) => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("locktype"),
            )),
        }
    }
}

impl From<LockEntry> for Value {
    fn from(lock_entry: LockEntry) -> Self {
        let mut map = ValueMap::new();
        map.insert::<LockScope>(lock_entry.lock_scope.into());
        map.insert::<LockType>(lock_entry.lock_type.into());

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elements::{LockEntry, LockScope, LockType},
        FromXml, IntoXml,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<d:lockentry xmlns:d="DAV:">
  <d:lockscope>
    <d:exclusive/>
  </d:lockscope>
  <d:locktype>
    <d:write/>
  </d:locktype>
</d:lockentry>
"#;

        let lock_entry = LockEntry::from_xml(xml).expect("Failed to deserialize LockEntry");

        assert_eq!(lock_entry.lock_scope, LockScope::Exclusive);
        assert_eq!(lock_entry.lock_type, LockType::Write(Default::default()));
    }

    #[test]
    fn test_serialize() {
        let lock_entry = LockEntry {
            lock_scope: LockScope::Shared,
            lock_type: LockType::Write(Default::default()),
        };

        let bytes = lock_entry
            .into_xml()
            .expect("Failed to serialize LockEntry");

        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockentry xmlns:d="DAV:">
  <d:lockscope>
    <d:shared/>
  </d:lockscope>
  <d:locktype>
    <d:write/>
  </d:locktype>
</d:lockentry>
        "#
        .trim();

        assert_eq!(xml, expected_xml);
    }
}
