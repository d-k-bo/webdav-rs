use crate::{
    elements::Href, Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap,
    DAV_NAMESPACE, DAV_PREFIX,
};

/// The `lockroot` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_lockroot).
#[derive(Clone, Debug, PartialEq)]
pub struct LockRoot {
    pub href: Href,
}

impl Element for LockRoot {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "lockroot";
}

impl TryFrom<&Value> for LockRoot {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match map.get::<Href>() {
            Some(Ok(href)) => Ok(LockRoot { href }),
            Some(Err(e)) => Err(e),
            None => Err(ExtractElementError::new(
                ExtractElementErrorKind::MissingElement("href"),
            )),
        }
    }
}

impl From<LockRoot> for Value {
    fn from(lock_root: LockRoot) -> Self {
        let mut map = ValueMap::new();
        map.insert::<Href>(lock_root.href.into());

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elements::{Href, LockRoot},
        FromXml,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:lockroot xmlns:d="DAV:">
  <d:href>/path/to/resource</d:href>
</d:lockroot>
        "#;

        let lock_root = LockRoot::from_xml(xml).expect("Failed to deserialize LockRoot");

        assert_eq!(
            lock_root.href,
            Href("/path/to/resource".parse().expect("Failed to parse Href"))
        );
    }
}
