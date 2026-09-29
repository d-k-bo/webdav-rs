// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytestring::ByteString;

use crate::{
    element::{Element, ElementName},
    properties::{
        ContentLanguage, ContentLength, ContentType, CreationDate, DisplayName, ETag, LastModified,
        LockDiscovery, ResourceType, SupportedLock,
    },
    value::{Value, ValueMap},
    ExtractElementError, DAV_NAMESPACE, DAV_PREFIX,
};

/// The `prop` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_prop).
///
/// This element can contain arbitrary child elements and supports extracting
/// them using [`Properties::get()`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Properties(ValueMap);

impl Properties {
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
}

impl Properties {
    /// Read a specific property from this `prop` element.
    ///
    /// Returns
    /// - `None` if the property doesn't exist
    /// - `Some(None)` if the property exists and is empty
    /// - `Some(Some(Ok(_)))` if the property exists and was successfully
    ///   extracted
    /// - `Some(Some(Err(_)))` if the property exists and extraction failed
    pub fn get<'v, P>(&'v self) -> Option<Option<Result<P, ExtractElementError>>>
    where
        P: Element + TryFrom<&'v Value, Error = ExtractElementError>,
    {
        self.0.get_optional()
    }
    /// List the names of the properties in this `prop` element.
    pub fn names(&self) -> impl Iterator<Item = &ElementName<ByteString>> {
        self.0 .0.keys()
    }
}

impl Properties {
    /// Read the `creationdate` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn creationdate(&self) -> Option<Option<Result<CreationDate, ExtractElementError>>> {
        self.get()
    }
    /// Read the `displayname` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn displayname(&self) -> Option<Option<Result<DisplayName, ExtractElementError>>> {
        self.get()
    }
    /// Read the `getcontentlanguage` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn getcontentlanguage(
        &self,
    ) -> Option<Option<Result<ContentLanguage, ExtractElementError>>> {
        self.get()
    }
    /// Read the `getcontentlength` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn getcontentlength(&self) -> Option<Option<Result<ContentLength, ExtractElementError>>> {
        self.get()
    }
    /// Read the `getcontenttype` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn getcontenttype(&self) -> Option<Option<Result<ContentType, ExtractElementError>>> {
        self.get()
    }
    /// Read the `getetag` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn getetag(&self) -> Option<Option<Result<ETag, ExtractElementError>>> {
        self.get()
    }
    /// Read the `getlastmodified` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn getlastmodified(&self) -> Option<Option<Result<LastModified, ExtractElementError>>> {
        self.get()
    }
    /// Read the `lockdiscovery` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn lockdiscovery(&self) -> Option<Option<Result<LockDiscovery, ExtractElementError>>> {
        self.get()
    }
    /// Read the `resourcetype` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn resourcetype(&self) -> Option<Option<Result<ResourceType, ExtractElementError>>> {
        self.get()
    }
    /// Read the `supportedlock` property.
    ///
    /// See [`Properties::get()`] for an overview of the possible return values.
    pub fn supportedlock(&self) -> Option<Option<Result<SupportedLock, ExtractElementError>>> {
        self.get()
    }
}

impl Element for Properties {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "prop";
}

impl TryFrom<&Value> for Properties {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.to_map().cloned().map(Self)
    }
}

impl From<Properties> for Value {
    fn from(Properties(map): Properties) -> Value {
        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::Properties;
    use crate::properties::*;
    use crate::FromXml;
    use crate::IntoXml;

    #[test]
    fn test_deserialize() -> eyre::Result<()> {
        use pretty_assertions::assert_eq;

        let xml = r#"
    <d:prop xmlns:d="DAV:">
        <d:getlastmodified>Mon, 30 Sep 2019 12:13:02 GMT</d:getlastmodified>
        <d:getcontentlength>0</d:getcontentlength>
        <d:resourcetype/>
        <d:getetag>&quot;a28785e285ce0de0738676814705c4e1&quot;</d:getetag>
        <d:getcontenttype>text/plain</d:getcontenttype>
    </d:prop>
    "#;

        let prop = Properties::from_xml(xml)?;

        assert_eq!(
            prop.getlastmodified().unwrap().unwrap()?,
            LastModified("Mon, 30 Sep 2019 12:13:02 GMT".parse()?)
        );
        assert_eq!(prop.getcontentlength().unwrap().unwrap()?, ContentLength(0));
        assert!(prop.resourcetype().unwrap().is_none());
        assert_eq!(
            prop.getetag().unwrap().unwrap()?,
            ETag(r#""a28785e285ce0de0738676814705c4e1""#.into())
        );
        assert_eq!(
            prop.getcontenttype().unwrap().unwrap()?,
            ContentType(mime::TEXT_PLAIN)
        );

        Ok(())
    }

    #[test]
    fn test_serialize() {
        let prop = Properties::new()
            .with(LastModified(
                "Mon, 30 Sep 2019 12:13:02 GMT".parse().unwrap(),
            ))
            .with(ContentLength(0))
            .with_name::<ResourceType>()
            .with(ETag(r#""a28785e285ce0de0738676814705c4e1""#.into()))
            .with_name::<ETag>()
            .with(ContentType(mime::TEXT_PLAIN));

        let bytes = prop.into_xml().expect("serialization failed");

        let xml = String::from_utf8(bytes.to_vec()).expect("invalid UTF-8");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:prop xmlns:d="DAV:">
  <d:getlastmodified>Mon, 30 Sep 2019 12:13:02 GMT</d:getlastmodified>
  <d:getcontentlength>0</d:getcontentlength>
  <d:resourcetype/>
  <d:getetag>"a28785e285ce0de0738676814705c4e1"</d:getetag>
  <d:getetag/>
  <d:getcontenttype>text/plain</d:getcontenttype>
</d:prop>
"#;

        assert_eq!(xml.trim(), expected_xml.trim());
    }
}
