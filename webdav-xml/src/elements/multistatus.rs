// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use nonempty::NonEmpty;

use crate::{
    elements::{response::Response, ResponseDescription},
    value::ValueMap,
    Element, ExtractElementError, Value, DAV_NAMESPACE, DAV_PREFIX,
};

/// The `multistatus` XML element as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_multistatus).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Multistatus {
    pub response: Vec<Response>,
    pub responsedescription: Option<ResponseDescription>,
}

impl Element for Multistatus {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "multistatus";
}

impl TryFrom<&Value> for Multistatus {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        Ok(Multistatus {
            response: map.iter_all().collect::<Result<_, _>>()?,
            responsedescription: map.get().transpose()?,
        })
    }
}

impl From<Multistatus> for Value {
    fn from(
        Multistatus {
            response,
            responsedescription,
        }: Multistatus,
    ) -> Value {
        let mut map = ValueMap::new();

        map.insert::<Response>(
            match NonEmpty::collect(response.into_iter().map(Value::from)) {
                Some(responses) => Value::List(Box::new(responses)),
                None => Value::Empty,
            },
        );
        if let Some(responsedescription) = responsedescription {
            map.insert::<ResponseDescription>(responsedescription.into())
        }

        Value::Map(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::FromXml as _;

    use super::*;

    #[test]
    fn parse_propfind_example() -> eyre::Result<()> {
        use http::StatusCode;

        use crate::elements::{Href, Status};

        // http://webdav.org/specs/rfc4918.html#n-example---retrieving-named-properties
        let xml = r#"
        <?xml version="1.0" encoding="utf-8" ?>
        <D:multistatus xmlns:D="DAV:">
          <D:response xmlns:R="http://ns.example.com/boxschema/">
            <D:href>http://www.example.com/file</D:href>
            <D:propstat>
              <D:prop>
                <R:bigbox>
                  <R:BoxType>Box type A</R:BoxType>
                </R:bigbox>
                <R:author>
                  <R:Name>J.J. Johnson</R:Name>
                </R:author>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
            <D:propstat>
              <D:prop><R:DingALing/><R:Random/></D:prop>
              <D:status>HTTP/1.1 403 Forbidden</D:status>
              <D:responsedescription> The user does not have access to the
         DingALing property.
              </D:responsedescription>
            </D:propstat>
          </D:response>
          <D:responsedescription> There has been an access violation error.
          </D:responsedescription>
        </D:multistatus>
        "#;
        let multistatus = Multistatus::from_xml(xml)?;

        assert!(matches!(
          &multistatus.response[0],
            Response::Propstat {
                href, propstat,
                responsedescription: None,
                ..
            } if href == &Href(http::Uri::from_static("http://www.example.com/file"))
              && propstat[0].status == Status(StatusCode::OK)
              && propstat[1].status == Status(StatusCode::FORBIDDEN)
              && propstat[1].responsedescription.is_some()
        ));

        Ok(())
    }

    #[test]
    fn parse_multiple_responses() -> eyre::Result<()> {
        let response = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <D:multistatus xmlns:D="DAV:">
          <D:response>
            <D:href>/foo/</D:href>
            <D:propstat>
              <D:prop>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
          </D:response>
          <D:response>
            <D:href>/bar/</D:href>
            <D:propstat>
              <D:prop>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
          </D:response>
          <D:response>
            <D:href>/baz/</D:href>
            <D:propstat>
              <D:prop>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
          </D:response>
        </D:multistatus>  
        "#;

        let multistatus = Multistatus::from_xml(response)?;
        assert_eq!(multistatus.response.len(), 3);

        Ok(())
    }
}
