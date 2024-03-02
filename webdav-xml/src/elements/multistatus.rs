// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use nonempty::NonEmpty;

use crate::{
    elements::{response::Response, ResponseDescription},
    value::ValueMap,
    Element, Error, Value, DAV_NAMESPACE, DAV_PREFIX,
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
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let map = value.to_map()?;

        fn iter_response_items(
            mut acc: Vec<Response>,
            value: &Value,
        ) -> Result<Vec<Response>, Error> {
            if value.is_list() {
                for item in value.to_list()? {
                    if item.is_list() {
                        acc = iter_response_items(acc, item)?;
                    } else {
                        acc.push(Response::try_from(item)?);
                    }
                }
            } else if value.is_map() {
                acc.push(Response::try_from(value)?);
            }

            Ok(acc)
        }

        let mut response = Vec::new();
        for (_, value) in &map.0 {
            response.extend(iter_response_items(vec![], value)?);
        }

        Ok(Multistatus {
            response,
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
mod test {
    use crate::FromXml as _;

    use super::*;

    #[test]
    fn test_should_parse_single_prop() -> eyre::Result<()> {
        use http::StatusCode;

        use crate::{
            elements::{Href, Status},
            FromXml,
        };

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
    fn test_should_parse_multi_files() {
        let response = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <D:multistatus xmlns:D="DAV:"
          xmlns:ns0="DAV:">
          <D:response xmlns:lp2="http://apache.org/dav/props/"
            xmlns:lp1="DAV:">
            <D:href>/ciao/</D:href>
            <D:propstat>
              <D:prop>
                <lp1:resourcetype>
                  <D:collection/>
                </lp1:resourcetype>
                <lp1:creationdate>2024-03-02T15:44:46Z</lp1:creationdate>
                <lp1:getlastmodified>Sat, 02 Mar 2024 15:44:46 GMT</lp1:getlastmodified>
                <lp1:getetag>"1a-612af5f3d72b2"</lp1:getetag>
                <D:supportedlock>
                  <D:lockentry>
                    <D:lockscope>
                      <D:exclusive/>
                    </D:lockscope>
                    <D:locktype>
                      <D:write/>
                    </D:locktype>
                  </D:lockentry>
                  <D:lockentry>
                    <D:lockscope>
                      <D:shared/>
                    </D:lockscope>
                    <D:locktype>
                      <D:write/>
                    </D:locktype>
                  </D:lockentry>
                </D:supportedlock>
                <D:lockdiscovery/>
                <D:getcontenttype>httpd/unix-directory</D:getcontenttype>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
          </D:response>
          <D:response xmlns:lp2="http://apache.org/dav/props/"
            xmlns:lp1="DAV:">
            <D:href>/ciao/pippo/</D:href>
            <D:propstat>
              <D:prop>
                <lp1:resourcetype>
                  <D:collection/>
                </lp1:resourcetype>
                <lp1:creationdate>2024-03-02T15:40:53Z</lp1:creationdate>
                <lp1:getlastmodified>Sat, 02 Mar 2024 15:40:53 GMT</lp1:getlastmodified>
                <lp1:getetag>"0-612af5150498f"</lp1:getetag>
                <D:supportedlock>
                  <D:lockentry>
                    <D:lockscope>
                      <D:exclusive/>
                    </D:lockscope>
                    <D:locktype>
                      <D:write/>
                    </D:locktype>
                  </D:lockentry>
                  <D:lockentry>
                    <D:lockscope>
                      <D:shared/>
                    </D:lockscope>
                    <D:locktype>
                      <D:write/>
                    </D:locktype>
                  </D:lockentry>
                </D:supportedlock>
                <D:lockdiscovery/>
                <D:getcontenttype>httpd/unix-directory</D:getcontenttype>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
          </D:response>
          <D:response xmlns:lp2="http://apache.org/dav/props/"
            xmlns:lp1="DAV:">
            <D:href>/ciao/build.rs</D:href>
            <D:propstat>
              <D:prop>
                <lp1:resourcetype/>
                <lp1:creationdate>2024-03-02T15:44:46Z</lp1:creationdate>
                <lp1:getcontentlength>486</lp1:getcontentlength>
                <lp1:getlastmodified>Sat, 02 Mar 2024 15:44:46 GMT</lp1:getlastmodified>
                <lp1:getetag>"1e6-612af5f3d72b2"</lp1:getetag>
                <lp2:executable>F</lp2:executable>
                <D:supportedlock>
                  <D:lockentry>
                    <D:lockscope>
                      <D:exclusive/>
                    </D:lockscope>
                    <D:locktype>
                      <D:write/>
                    </D:locktype>
                  </D:lockentry>
                  <D:lockentry>
                    <D:lockscope>
                      <D:shared/>
                    </D:lockscope>
                    <D:locktype>
                      <D:write/>
                    </D:locktype>
                  </D:lockentry>
                </D:supportedlock>
                <D:lockdiscovery/>
                <D:getcontenttype>application/rls-services+xml</D:getcontenttype>
              </D:prop>
              <D:status>HTTP/1.1 200 OK</D:status>
            </D:propstat>
          </D:response>
        </D:multistatus>
                
"#;

        let multistatus = Multistatus::from_xml(response).unwrap();
        assert_eq!(multistatus.response.len(), 3);
    }
}
