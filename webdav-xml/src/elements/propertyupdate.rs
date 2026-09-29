use crate::{
    elements::Properties, Element, ExtractElementError, Value, ValueMap, DAV_NAMESPACE, DAV_PREFIX,
};

/// The `propertyupdate` XML element as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_propertyupdate).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyUpdate {
    pub set: Option<Set>,
    pub remove: Option<Remove>,
}

impl Element for PropertyUpdate {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "propertyupdate";
}

impl TryFrom<&Value> for PropertyUpdate {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        let set = match map.get::<Set>() {
            Some(set_value) => Some(set_value?),
            None => None,
        };

        let remove = match map.get::<Remove>() {
            Some(remove_value) => Some(remove_value?),
            None => None,
        };

        Ok(PropertyUpdate { set, remove })
    }
}

impl From<PropertyUpdate> for Value {
    fn from(property_update: PropertyUpdate) -> Value {
        let mut map = ValueMap::new();

        if let Some(set) = property_update.set {
            map.insert::<Set>(set.into());
        }

        if let Some(remove) = property_update.remove {
            map.insert::<Remove>(remove.into());
        }

        Value::Map(map)
    }
}

/// The `remove` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_remove).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Remove {
    pub properties: Properties,
}

impl Element for Remove {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "remove";
}

impl TryFrom<&Value> for Remove {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        let properties = match map.get::<Properties>() {
            Some(prop_value) => prop_value?,
            None => {
                return Err(ExtractElementError::new(
                    crate::ExtractElementErrorKind::MissingElement("prop"),
                ))
            }
        };

        Ok(Remove { properties })
    }
}

impl From<Remove> for Value {
    fn from(remove: Remove) -> Value {
        let mut map = ValueMap::new();
        map.insert::<Properties>(remove.properties.into());

        Value::Map(map)
    }
}

/// The `set` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_set).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Set {
    pub properties: Properties,
}

impl Element for Set {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "set";
}

impl TryFrom<&Value> for Set {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        let properties = match map.get::<Properties>() {
            Some(prop_value) => prop_value?,
            None => {
                return Err(ExtractElementError::new(
                    crate::ExtractElementErrorKind::MissingElement("prop"),
                ))
            }
        };

        Ok(Set { properties })
    }
}

impl From<Set> for Value {
    fn from(set: Set) -> Value {
        let mut map = ValueMap::new();
        map.insert::<Properties>(set.properties.into());

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use time::OffsetDateTime;

    use crate::{
        elements::{Properties, PropertyUpdate, Remove, Set},
        properties::{ContentLanguage, DisplayName, LastModified},
        FromXml, IntoXml,
    };

    #[test]
    fn test_deserialize() {
        let xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:propertyupdate xmlns:d="DAV:">
  <d:set>
    <d:prop>
      <d:displayname>New Name</d:displayname>
    </d:prop>
  </d:set>
  <d:remove>
    <d:prop>
      <d:getcontentlanguage/>
    </d:prop>
  </d:remove>
</d:propertyupdate>
        "#;

        let property_update =
            PropertyUpdate::from_xml(xml).expect("Failed to deserialize PropertyUpdate");

        let set_props = Properties::new().with::<DisplayName>(DisplayName("New Name".into()));

        let remove_props = Properties::new().with_name::<ContentLanguage>();

        let expected = PropertyUpdate {
            set: Some(Set {
                properties: set_props,
            }),
            remove: Some(Remove {
                properties: remove_props,
            }),
        };

        assert_eq!(property_update, expected);
    }

    #[test]
    fn test_serialize() {
        let set_props = Properties::new().with::<LastModified>(LastModified(
            SystemTime::from(OffsetDateTime::new_utc(
                time::Date::from_calendar_date(1998, time::Month::January, 12)
                    .expect("invalid date"),
                time::Time::from_hms(9, 25, 56).expect("invalid time"),
            ))
            .into(),
        ));

        let remove_props = Properties::new().with_name::<ContentLanguage>();

        let property_update = PropertyUpdate {
            set: Some(Set {
                properties: set_props,
            }),
            remove: Some(Remove {
                properties: remove_props,
            }),
        };

        let bytes = property_update
            .into_xml()
            .expect("Failed to serialize PropertyUpdate");

        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:propertyupdate xmlns:d="DAV:">
  <d:set>
    <d:prop>
      <d:getlastmodified>Mon, 12 Jan 1998 09:25:56 GMT</d:getlastmodified>
    </d:prop>
  </d:set>
  <d:remove>
    <d:prop>
      <d:getcontentlanguage/>
    </d:prop>
  </d:remove>
</d:propertyupdate>
        "#;

        assert_eq!(xml.trim(), expected_xml.trim());
    }
}
