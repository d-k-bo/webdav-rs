// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytestring::ByteString;

use crate::{
    elements::Properties, Element, ExtractElementError, ExtractElementErrorKind, Value, ValueMap,
    DAV_NAMESPACE, DAV_PREFIX,
};

/// The `propfind` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_propfind).
#[derive(Clone, Debug, PartialEq)]
pub enum Propfind {
    Propname,
    Allprop { include: Option<Include> },
    Prop(Properties),
}

impl Element for Propfind {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "propfind";
}

impl TryFrom<&Value> for Propfind {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        match (
            map.get::<Propname>(),
            map.get::<Allprop>(),
            map.get::<Properties>(),
        ) {
            (Some(_), None, None) => Ok(Propfind::Propname),
            (None, Some(_), None) => Ok(Propfind::Allprop {
                include: map.get().transpose()?,
            }),
            (None, None, Some(prop)) => Ok(Propfind::Prop(prop?)),
            _ => Err(ExtractElementError::new(
                ExtractElementErrorKind::ConflictingElements(&["propname", "allprop", "include"]),
            )),
        }
    }
}

impl From<Propfind> for Value {
    fn from(propfind: Propfind) -> Self {
        let mut map = ValueMap::new();

        match propfind {
            Propfind::Propname => map.insert::<Propname>(Propname.into()),
            Propfind::Allprop { include } => {
                map.insert::<Allprop>(Allprop.into());

                if let Some(include) = include {
                    map.insert::<Include>(include.into());
                }
            }
            Propfind::Prop(props) => {
                map.insert::<Properties>(props.into());
            }
        };

        Value::Map(map)
    }
}

/// The `propname` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_propname).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Propname;

impl Element for Propname {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "propname";
}

impl TryFrom<&Value> for Propname {
    type Error = ExtractElementError;

    fn try_from(_: &Value) -> Result<Self, Self::Error> {
        Ok(Propname)
    }
}

impl From<Propname> for Value {
    fn from(_: Propname) -> Self {
        Value::Empty
    }
}

/// The `allprop` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_allprop).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Allprop;

impl Element for Allprop {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "allprop";
}

impl TryFrom<&Value> for Allprop {
    type Error = ExtractElementError;

    fn try_from(_: &Value) -> Result<Self, Self::Error> {
        Ok(Allprop)
    }
}

impl From<Allprop> for Value {
    fn from(_: Allprop) -> Self {
        Value::Empty
    }
}

/// The `include` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_include).
#[derive(Clone, Debug, PartialEq)]
pub struct Include(Vec<ByteString>);

impl Element for Include {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "include";
}

impl TryFrom<&Value> for Include {
    type Error = ExtractElementError;

    fn try_from(_: &Value) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<Include> for Value {
    fn from(_: Include) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use bytestring::ByteString;

    use crate::{
        elements::{Properties, Propfind},
        properties::{CreationDate, ETag, LastModified},
        FromXml, IntoXml,
    };

    #[test]
    fn test_deserialize_propfind_properties() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<d:propfind xmlns:d="DAV:">
  <d:prop>
    <d:creationdate/>
    <d:getlastmodified/>
    <d:getetag>W/"123456789"</d:getetag>
  </d:prop>
</d:propfind>
"#;

        let propfind = Propfind::from_xml(xml).expect("Failed to deserialize propfind");

        match propfind {
            Propfind::Prop(props) => {
                assert!(props.get::<CreationDate>().is_some_and(|v| v.is_none()));
                assert!(props.get::<LastModified>().is_some_and(|v| v.is_none()));

                assert!(props
                    .get::<ETag>()
                    .flatten()
                    .and_then(|etag| etag.ok())
                    .is_some_and(|etag| etag.0 == r#"W/"123456789""#));
            }
            _ => panic!("Expected Propfind::Prop variant"),
        }
    }

    #[test]
    fn test_serialize_propfind_properties() {
        let propfind = Propfind::Prop(
            Properties::new()
                .with_name::<CreationDate>()
                .with_name::<LastModified>()
                .with(ETag(ByteString::from(r#"W/"123456789""#)))
                .with_name::<ETag>(),
        );

        let bytes = propfind.into_xml().expect("Failed to serialize propfind");
        let xml = String::from_utf8(bytes.to_vec()).expect("Invalid UTF-8 in serialized XML");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:propfind xmlns:d="DAV:">
  <d:prop>
    <d:creationdate/>
    <d:getlastmodified/>
    <d:getetag>W/"123456789"</d:getetag>
    <d:getetag/>
  </d:prop>
</d:propfind>
"#;

        assert_eq!(xml.trim(), expected_xml.trim());
    }
}
