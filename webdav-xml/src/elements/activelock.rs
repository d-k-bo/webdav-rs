use crate::{
    elements::{Depth, LockRoot, LockScope, LockToken, LockType, Owner},
    Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap, DAV_NAMESPACE,
    DAV_PREFIX,
};

/// The `activelock` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_activelock).
#[derive(Clone, Debug, PartialEq)]
pub struct ActiveLock {
    pub lock_scope: LockScope,
    pub lock_type: LockType,
    pub depth: Depth,
    pub owner: Option<Owner>,
    pub lock_token: Option<LockToken>,
    pub lock_root: LockRoot,
}

impl Element for ActiveLock {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "activelock";
}

impl TryFrom<&Value> for ActiveLock {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        let lock_scope = match map.get::<LockScope>() {
            Some(Ok(lock_scope)) => lock_scope,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(ExtractElementError::new(
                    ExtractElementErrorKind::MissingElement("lockscope"),
                ))
            }
        };

        let lock_type = match map.get::<LockType>() {
            Some(Ok(lock_type)) => lock_type,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(ExtractElementError::new(
                    ExtractElementErrorKind::MissingElement("locktype"),
                ))
            }
        };

        let depth = match map.get::<Depth>() {
            Some(Ok(depth)) => depth,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(ExtractElementError::new(
                    ExtractElementErrorKind::MissingElement("depth"),
                ))
            }
        };

        let owner = match map.get::<Owner>() {
            Some(Ok(owner)) => Some(owner),
            Some(Err(e)) => return Err(e),
            None => None,
        };

        let lock_token = match map.get::<LockToken>() {
            Some(Ok(lock_token)) => Some(lock_token),
            Some(Err(e)) => return Err(e),
            None => None,
        };

        let lock_root = match map.get::<LockRoot>() {
            Some(Ok(lock_root)) => lock_root,
            Some(Err(e)) => return Err(e),
            None => {
                return Err(ExtractElementError::new(
                    ExtractElementErrorKind::MissingElement("lockroot"),
                ))
            }
        };

        Ok(ActiveLock {
            lock_scope,
            lock_type,
            depth,
            owner,
            lock_token,
            lock_root,
        })
    }
}

impl From<ActiveLock> for Value {
    fn from(active_lock: ActiveLock) -> Self {
        let mut map = ValueMap::new();
        map.insert::<LockScope>(active_lock.lock_scope.into());
        map.insert::<LockType>(active_lock.lock_type.into());
        map.insert::<Depth>(active_lock.depth.into());

        if let Some(owner) = active_lock.owner {
            map.insert::<Owner>(owner.into());
        }

        if let Some(lock_token) = active_lock.lock_token {
            map.insert::<LockToken>(lock_token.into());
        }

        map.insert::<LockRoot>(active_lock.lock_root.into());

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elements::{ActiveLock, Depth, Href, LockRoot, LockScope, LockType},
        FromXml, IntoXml,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:activelock xmlns:d="DAV:">
  <d:lockscope>
    <d:exclusive/>
  </d:lockscope>
  <d:locktype>
    <d:write/>
  </d:locktype>
  <d:depth>infinity</d:depth>
  <d:owner>
    <d:href>http://example.com/user</d:href>
  </d:owner>
  <d:locktoken>
    <d:href>opaquelocktoken:e8d3f4c2-1f4b-4c3a-9f4e-2d3f4c2b1a2b</d:href>
  </d:locktoken>
  <d:lockroot>
    <d:href>http://example.com/resource</d:href>
  </d:lockroot>
</d:activelock>
        "#;

        let active_lock = ActiveLock::from_xml(xml).unwrap();

        assert_eq!(active_lock.lock_scope, LockScope::Exclusive);
        assert_eq!(active_lock.lock_type, LockType::Write(Default::default()));
        assert_eq!(active_lock.depth, Depth::Infinity);

        assert!(active_lock.owner.is_some_and(|owner| {
            let href = owner.get::<Href>().flatten().and_then(|h| h.ok()).unwrap();

            href.0 == "http://example.com/user"
        }));

        assert!(active_lock.lock_token.is_some_and(|lock_token| {
            lock_token.href.0 == "opaquelocktoken:e8d3f4c2-1f4b-4c3a-9f4e-2d3f4c2b1a2b"
        }));

        assert!(active_lock.lock_root.href.0 == "http://example.com/resource");
    }

    #[test]
    fn test_serialize() {
        let active_lock = ActiveLock {
            lock_scope: LockScope::Shared,
            lock_type: LockType::Write(Default::default()),
            depth: Depth::One,
            owner: None,
            lock_token: None,
            lock_root: LockRoot {
                href: Href("http://example.com/resource".parse().expect("Invalid URL")),
            },
        };

        let bytes = active_lock
            .into_xml()
            .expect("Failed to serialize ActiveLock");

        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:activelock xmlns:d="DAV:">
  <d:lockscope>
    <d:shared/>
  </d:lockscope>
  <d:locktype>
    <d:write/>
  </d:locktype>
  <d:depth>1</d:depth>
  <d:lockroot>
    <d:href>http://example.com/resource</d:href>
  </d:lockroot>
</d:activelock>
        "#
        .trim();

        assert_eq!(xml, expected_xml);
    }
}
