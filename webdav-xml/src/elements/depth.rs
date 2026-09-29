use crate::{
    Element, ExtractElementError, ExtractElementErrorKind, Value, DAV_NAMESPACE, DAV_PREFIX,
};
use std::ops::Deref;

/// The `depth` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_depth).
#[derive(Clone, Debug, PartialEq)]
pub enum Depth {
    Zero,
    One,
    Infinity,
}

impl Element for Depth {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "depth";
}

impl TryFrom<&Value> for Depth {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Text(text) => match text.deref() {
                "0" => Ok(Depth::Zero),
                "1" => Ok(Depth::One),
                "infinity" => Ok(Depth::Infinity),
                _ => Err(ExtractElementError::new(ExtractElementErrorKind::Other(
                    "depth element must have value of 0, 1, or infinity".into(),
                ))),
            },
            _ => Err(ExtractElementError::new(
                ExtractElementErrorKind::InvalidValueType {
                    expected: "text",
                    got: match value {
                        Value::Empty => "empty",
                        Value::Text(_) => "text",
                        Value::Map(_) => "map",
                        Value::List(_) => "list",
                    },
                },
            )),
        }
    }
}

impl From<Depth> for Value {
    fn from(depth: Depth) -> Self {
        let text = match depth {
            Depth::Zero => "0",
            Depth::One => "1",
            Depth::Infinity => "infinity",
        };

        Value::Text(text.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{elements::Depth, FromXml, IntoXml};

    #[test]
    fn test_deserialize() {
        let xml = r#"<D:depth xmlns:D="DAV:">1</D:depth>"#;

        let depth = Depth::from_xml(xml).unwrap();
        assert_eq!(depth, super::Depth::One);

        let xml = r#"<D:depth xmlns:D="DAV:">infinity</D:depth>"#;

        let depth = Depth::from_xml(xml).unwrap();
        assert_eq!(depth, super::Depth::Infinity);

        let xml = r#"<D:depth xmlns:D="DAV:">0</D:depth>"#;

        let depth = Depth::from_xml(xml).unwrap();
        assert_eq!(depth, super::Depth::Zero);
    }

    #[test]
    fn test_serialize() {
        let depth = Depth::Infinity;
        let bytes = depth.into_xml().unwrap();
        let xml = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(
            xml,
            r#"
<?xml version="1.0" encoding="utf-8"?>
<d:depth xmlns:d="DAV:">infinity</d:depth>
            "#
            .trim()
        );

        let depth = Depth::Zero;
        let bytes = depth.into_xml().unwrap();
        let xml = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(
            xml,
            r#"
<?xml version="1.0" encoding="utf-8"?>
<d:depth xmlns:d="DAV:">0</d:depth>
            "#
            .trim()
        );

        let depth = Depth::One;
        let bytes = depth.into_xml().unwrap();
        let xml = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(
            xml,
            r#"
<?xml version="1.0" encoding="utf-8"?>
<d:depth xmlns:d="DAV:">1</d:depth>
            "#
            .trim()
        );
    }
}
