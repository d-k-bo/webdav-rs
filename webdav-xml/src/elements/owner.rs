use crate::{
    element::ElementName, Element, ExtractElementError, Value, ValueMap, DAV_NAMESPACE, DAV_PREFIX,
};
use bytestring::ByteString;

/// The `owner` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_owner).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Owner(pub ValueMap);

impl Owner {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with<E>(mut self, e: E) -> Self
    where
        E: Element + Into<Value>,
    {
        self.0.insert::<E>(e.into());
        self
    }
    pub fn with_name<E>(mut self) -> Self
    where
        E: Element,
    {
        self.0.insert::<E>(Value::Empty);
        self
    }
    /// List the names of the properties in this `owner` element.
    pub fn names(&self) -> impl Iterator<Item = &ElementName<ByteString>> {
        self.0 .0.keys()
    }
}

impl Owner {
    /// Read a specific element from this `owner` element.
    ///
    /// Returns
    /// - `None` if the element doesn't exist
    /// - `Some(None)` if the element exists and is empty
    /// - `Some(Some(Ok(_)))` if the element exists and was successfully
    ///   extracted
    /// - `Some(Some(Err(_)))` if the element exists and extraction failed
    pub fn get<'v, P>(&'v self) -> Option<Option<Result<P, ExtractElementError>>>
    where
        P: Element + TryFrom<&'v Value, Error = ExtractElementError>,
    {
        self.0.get_optional()
    }
}

impl Element for Owner {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "owner";
}

impl TryFrom<&Value> for Owner {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.to_map().cloned().map(Self)
    }
}

impl From<Owner> for Value {
    fn from(Owner(map): Owner) -> Self {
        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        elements::{Href, Owner},
        FromXml, IntoXml, ValueMap,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:owner xmlns:d="DAV:">
  <d:href>http://example.com/user1</d:href>
</d:owner>
        "#;

        let owner = Owner::from_xml(xml).expect("Failed to deserialize Owner");

        let owner_href = owner.get::<Href>().flatten();

        assert!(owner_href
            .and_then(|h| h.ok())
            .is_some_and(|h| { h.0 == "http://example.com/user1" }));
    }

    #[test]
    fn test_serialize() {
        let mut owner_values = ValueMap::new();
        owner_values
            .insert::<Href>(Href("http://example.com/user1".parse().expect("Invalid URI")).into());

        let owner = Owner(owner_values);

        let bytes = owner.into_xml().expect("Failed to serialize Owner");
        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:owner xmlns:d="DAV:">
  <d:href>http://example.com/user1</d:href>
</d:owner>
"#;

        assert_eq!(xml.trim(), expected_xml.trim());
    }
}
