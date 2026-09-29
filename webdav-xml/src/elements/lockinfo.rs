use crate::{
    elements::{LockScope, LockType, Owner},
    Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap, DAV_NAMESPACE,
    DAV_PREFIX,
};

/// The `lockinfo` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_lockinfo).
#[derive(Clone, Debug, PartialEq)]
pub struct LockInfo {
    pub lock_scope: LockScope,
    pub lock_type: LockType,
    pub owner: Option<Owner>,
}

impl Element for LockInfo {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "lockinfo";
}

impl TryFrom<&Value> for LockInfo {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match (
            map.get::<LockScope>(),
            map.get::<LockType>(),
            map.get::<Owner>(),
        ) {
            (Some(Ok(lock_scope)), Some(Ok(lock_type)), owner_opt) => {
                let owner = match owner_opt {
                    Some(Ok(owner)) => Some(owner),
                    Some(Err(e)) => return Err(e),
                    None => None,
                };

                Ok(LockInfo {
                    lock_scope,
                    lock_type,
                    owner,
                })
            }
            (Some(Err(e)), _, _) => Err(e),
            (_, Some(Err(e)), _) => Err(e),
            (None, _, _) => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("lockscope"),
            )),
            (_, None, _) => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("locktype"),
            )),
        }
    }
}

impl From<LockInfo> for Value {
    fn from(lock_info: LockInfo) -> Self {
        let mut map = ValueMap::new();
        map.insert::<LockScope>(lock_info.lock_scope.into());
        map.insert::<LockType>(lock_info.lock_type.into());

        if let Some(owner) = lock_info.owner {
            map.insert::<Owner>(owner.into());
        }

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elements::{Href, LockInfo, LockScope, LockType, Owner},
        FromXml, IntoXml, ValueMap,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockinfo xmlns:d="DAV:">
  <d:lockscope>
    <d:exclusive/>
  </d:lockscope>
  <d:locktype>
    <d:write/>
  </d:locktype>
  <d:owner>
    <d:href>http://example.com/user</d:href>
  </d:owner>
</d:lockinfo>
"#;

        let lock_info = LockInfo::from_xml(xml).expect("Failed to deserialize LockInfo");

        let mut owner_values = ValueMap::new();
        owner_values
            .insert::<Href>(Href("http://example.com/user".parse().expect("Invalid URI")).into());

        assert_eq!(
            lock_info,
            super::LockInfo {
                lock_scope: LockScope::Exclusive,
                lock_type: LockType::Write(Default::default()),
                owner: Some(Owner(owner_values)),
            }
        );
    }

    #[test]
    fn test_serialize() {
        let mut owner_values = ValueMap::new();
        owner_values
            .insert::<Href>(Href("http://example.com/user".parse().expect("Invalid URI")).into());

        let lock_info = LockInfo {
            lock_scope: LockScope::Shared,
            lock_type: LockType::Write(Default::default()),
            owner: Some(Owner(owner_values)),
        };

        let bytes = lock_info.into_xml().expect("Failed to serialize LockInfo");
        let xml = String::from_utf8(bytes.to_vec()).unwrap();

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockinfo xmlns:d="DAV:">
  <d:lockscope>
    <d:shared/>
  </d:lockscope>
  <d:locktype>
    <d:write/>
  </d:locktype>
  <d:owner>
    <d:href>http://example.com/user</d:href>
  </d:owner>
</d:lockinfo>
        "#
        .trim();

        assert_eq!(xml, expected_xml);
    }
}
