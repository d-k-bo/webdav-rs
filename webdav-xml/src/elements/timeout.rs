use crate::{
    Element, ExtractElementError, ExtractElementErrorKind, Value, DAV_NAMESPACE, DAV_PREFIX,
};

/// The `timeout` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_timeout).
#[derive(Clone, Debug, PartialEq)]
pub enum Timeout {
    Seconds(u32),
    Infinite,
}

impl Element for Timeout {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "timeout";
}

impl TryFrom<&Value> for Timeout {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value.to_text() {
            Ok(text) => {
                if text == "Infinite" {
                    Ok(Timeout::Infinite)
                } else if let Some(stripped) = text.strip_prefix("Second-") {
                    match stripped.parse::<u32>() {
                        Ok(seconds) => Ok(Timeout::Seconds(seconds)),
                        Err(_) => Err(ExtractElementError::new(ExtractElementErrorKind::Other(
                            "Timeout element has invalid Second value".into(),
                        ))),
                    }
                } else {
                    Err(ExtractElementError::new(ExtractElementErrorKind::Other(
                        "Timeout element must be 'Infinite' or 'Second-<number>'".into(),
                    )))
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl From<Timeout> for Value {
    fn from(timeout: Timeout) -> Self {
        let text = match timeout {
            Timeout::Infinite => "Infinite".into(),
            Timeout::Seconds(seconds) => format!("Second-{}", seconds).into(),
        };

        Value::Text(text)
    }
}

#[cfg(test)]
mod tests {
    use crate::{elements::Timeout, FromXml, IntoXml};

    #[test]
    fn test_deserialize_seconds() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<d:timeout xmlns:d="DAV:">Second-600</d:timeout>
        "#;

        let timeout = Timeout::from_xml(xml).expect("Failed to deserialize Timeout");

        assert_eq!(timeout, Timeout::Seconds(600));
    }

    #[test]
    fn test_deserialize_infinite() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<d:timeout xmlns:d="DAV:">Infinite</d:timeout>
        "#;

        let timeout = Timeout::from_xml(xml).expect("Failed to deserialize Timeout");

        assert_eq!(timeout, Timeout::Infinite);
    }

    #[test]
    fn test_serialize_seconds() {
        let timeout = super::Timeout::Seconds(1200);

        let bytes = timeout.into_xml().expect("Failed to serialize Timeout");
        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:timeout xmlns:d="DAV:">Second-1200</d:timeout>
        "#;

        assert_eq!(xml.trim(), expected_xml.trim());
    }

    #[test]
    fn test_serialize_infinite() {
        let timeout = super::Timeout::Infinite;

        let bytes = timeout.into_xml().expect("Failed to serialize Timeout");
        let xml = String::from_utf8(bytes.to_vec()).expect("Failed to convert bytes to string");

        let expected_xml = r#"
<?xml version="1.0" encoding="utf-8"?>
<d:timeout xmlns:d="DAV:">Infinite</d:timeout>
        "#;

        assert_eq!(xml.trim(), expected_xml.trim());
    }
}
