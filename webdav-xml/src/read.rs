// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::borrow::Cow;

use bytestring::ByteString;

use crate::{element::ElementName, utils::BytesExt, value::ValueMap, Error, Value};

pub(crate) fn read_xml(xml: impl Into<bytes::Bytes>) -> crate::Result<Value> {
    let xml = xml.into();
    let mut reader = XmlReader::new(&xml);
    reader.read_into_value(&xml)
}
struct XmlReader<'x> {
    reader: quick_xml::NsReader<&'x [u8]>,
    last: Option<quick_xml::events::Event<'x>>,
}

impl<'x> XmlReader<'x> {
    fn new(xml: &'x [u8]) -> Self {
        Self {
            reader: quick_xml::NsReader::from_reader(xml),
            last: None,
        }
    }
    fn last(&self) -> Option<&quick_xml::events::Event<'x>> {
        self.last.as_ref()
    }
    fn read_resolved_event(
        &mut self,
    ) -> quick_xml::Result<(
        quick_xml::name::ResolveResult<'_>,
        quick_xml::events::Event<'_>,
    )> {
        let (resolve_result, event) = self.reader.read_resolved_event()?;
        self.last = Some(event.clone());
        Ok((resolve_result, event))
    }

    fn read_into_value(&mut self, xml: &bytes::Bytes) -> crate::Result<Value> {
        use quick_xml::{
            events::{BytesStart, Event},
            name::ResolveResult,
        };

        fn key(
            xml: &bytes::Bytes,
            resolve_result: &ResolveResult,
            tag: &BytesStart<'_>,
        ) -> crate::Result<ElementName<ByteString>> {
            match resolve_result {
                ResolveResult::Bound(ns) => {
                    if ns.as_ref().is_empty() {
                        return Err(Error::InvalidNamespace(xml.maybe_slice_ref(ns.as_ref())));
                    }

                    Ok(ElementName {
                        namespace: Some(xml.maybe_slice_ref(ns.as_ref()).try_into()?),
                        prefix: None,
                        local_name: xml.maybe_slice_ref(tag.local_name().as_ref()).try_into()?,
                    })
                }
                ResolveResult::Unbound | ResolveResult::Unknown(_) => Ok(ElementName {
                    namespace: None,
                    prefix: None,
                    local_name: xml.maybe_slice_ref(tag.name().as_ref()).try_into()?,
                }),
            }
        }

        let mut map = ValueMap::new();

        loop {
            let (resolve_result, event) = self.read_resolved_event()?;
            // dbg!(&event);
            match event {
                Event::Text(text)
                    if std::str::from_utf8(&text)
                        .is_ok_and(|s| s.chars().all(char::is_whitespace)) =>
                {
                    continue
                }
                Event::Text(text) => {
                    // TODO: use ByteString and only reallocate when something was escaped
                    let value = Value::Text(match text.unescape()? {
                        Cow::Borrowed(s) => xml
                            .maybe_slice_ref(s.as_bytes())
                            .try_into()
                            .expect("string is checked by text.unescape() to be valid"),
                        Cow::Owned(s) => s.into(),
                    });
                    let _ = self.read_resolved_event()?;
                    return Ok(value);
                }
                Event::Start(start) => {
                    let key = key(xml, &resolve_result, &start)?;
                    let start_name = xml.maybe_slice_ref(start.name().as_ref());
                    drop(resolve_result);
                    drop(start);

                    map.insert_raw(key, self.read_into_value(xml)?);

                    if !matches!(self.last(), Some(Event::End(end)) if end.name().as_ref() == start_name)
                    {
                        return Err(Error::UnexpectedTag);
                    }
                }
                Event::Empty(tag) => {
                    map.insert_raw(key(xml, &resolve_result, &tag)?, Value::Empty);
                }
                Event::End(_) | Event::Eof => break,
                Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_) => continue,
                Event::CData(_) => todo!(),
            }
        }

        Ok(Value::Map(map))
    }
}

#[cfg(test)]
mod tests {
    use nonempty::nonempty;
    use pretty_assertions::assert_eq;

    use super::*;

    macro_rules! value_map {
        { $($key:expr => $value:expr),* $(,)? } => {
            ValueMap(crate::value::InnerValueMap::from([
                $(
                    (
                        crate::element::ElementName { local_name: ByteString::from_static($key), namespace: None, prefix: None },
                        $value
                    ),
                )*
            ]))
        };
    }

    #[test]
    fn empty() -> eyre::Result<()> {
        let xml = r#"<foo/>"#;
        let value = Value::Map(value_map! {
            "foo" => Value::Empty
        });
        assert_eq!(value, read_xml(xml)?);
        Ok(())
    }

    #[test]
    fn text() -> eyre::Result<()> {
        let xml = r#"<foo>bar</foo>"#;
        let value = Value::Map(value_map! {
            "foo" => Value::Text("bar".into()),
        });
        assert_eq!(value, read_xml(xml)?);
        Ok(())
    }

    #[test]
    fn map() -> eyre::Result<()> {
        let xml = r#"<foo><bar /></foo>"#;
        let value = Value::Map(value_map! {
            "foo" => Value::Map(value_map! {
                "bar" => Value::Empty,
            }),
        });
        assert_eq!(value, read_xml(xml)?);
        Ok(())
    }

    #[test]
    fn list() -> eyre::Result<()> {
        let xml = r#"<foo /><foo />"#;
        let value = Value::Map(value_map! {
            "foo" => Value::List(Box::new(nonempty![Value::Empty, Value::Empty])),
        });
        assert_eq!(value, read_xml(xml)?);
        Ok(())
    }

    #[test]
    // https://github.com/d-k-bo/webdav-rs/issues/2
    fn list_long() -> eyre::Result<()> {
        let xml = r#"<foo /><foo /><foo /><foo /><foo />"#;
        let value = Value::Map(value_map! {
            "foo" => Value::List(Box::new(nonempty![Value::Empty, Value::Empty, Value::Empty, Value::Empty, Value::Empty])),
        });
        assert_eq!(value, read_xml(xml)?);
        Ok(())
    }
}
