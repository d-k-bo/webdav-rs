// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{borrow::Cow, collections::HashMap};

use bytestring::ByteString;

use crate::{
    element::{Element, ElementExt, ElementName},
    Value, XmlError,
};

pub(crate) fn write_xml<E: Element>(
    writer: impl std::io::Write,
    value: Value,
) -> Result<(), XmlError> {
    let mut writer = XmlWriter {
        inner: quick_xml::Writer::new_with_indent(writer, b' ', 2),
        namespaces: HashMap::new(),
    };
    writer.inner.write_event(quick_xml::events::Event::Decl(
        quick_xml::events::BytesDecl::new("1.0", Some("utf-8"), None),
    ))?;

    let name = E::element_name();
    writer.resolve_namespaces(&name, &value);
    writer.write_toplevel(&name, value)
}

struct XmlWriter<W>
where
    W: std::io::Write,
{
    inner: quick_xml::Writer<W>,
    namespaces: HashMap<ByteString, ByteString>,
}

impl<W> XmlWriter<W>
where
    W: std::io::Write,
{
    fn add_namespace(&mut self, name: &ElementName<ByteString>) {
        if let Some(namespace) = &name.namespace {
            if !self.namespaces.contains_key(namespace) {
                self.namespaces.insert(
                    namespace.clone(),
                    name.prefix.as_ref().cloned().unwrap_or_else(|| "NS".into()),
                );
                // TODO: handle collisions
            }
        }
    }
    fn resolve_namespaces(&mut self, name: &ElementName<ByteString>, value: &Value) {
        match value {
            Value::Text(_) | Value::Empty => self.add_namespace(name),
            Value::List(list) => {
                for value in list.as_ref() {
                    self.resolve_namespaces(name, value);
                }
            }
            Value::Map(map) => {
                self.add_namespace(name);
                for (name, value) in &map.0 {
                    self.resolve_namespaces(name, value);
                }
            }
        }
    }
}

impl<W> XmlWriter<W>
where
    W: std::io::Write,
{
    fn name<'n>(&self, name: &'n ElementName<ByteString>) -> Cow<'n, str> {
        match &name.namespace {
            Some(namespace) => Cow::Owned(format!(
                "{prefix}:{local_name}",
                prefix = self
                    .namespaces
                    .get(namespace)
                    .expect("all namespaces should be resolved in the first pass"),
                local_name = name.local_name
            )),
            None => Cow::Borrowed(&name.local_name),
        }
    }
    fn write_toplevel(
        &mut self,
        name: &ElementName<ByteString>,
        value: Value,
    ) -> Result<(), XmlError> {
        use quick_xml::{
            escape::partial_escape,
            events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
        };

        let raw_name = self.name(name);

        match value {
            Value::Map(map) => {
                let mut start = BytesStart::new(&*raw_name);
                for (namespace, prefix) in &self.namespaces {
                    start.push_attribute(Attribute::from((
                        &*format!("xmlns:{prefix}"),
                        &**namespace,
                    )));
                }

                self.inner.write_event(Event::Start(start))?;
                for (tag, value) in map.0 {
                    self.write_value(&tag, value)?;
                }
                self.inner
                    .write_event(Event::End(BytesEnd::new(raw_name)))?;

                Ok(())
            }
            Value::Text(text) => {
                let mut start = BytesStart::new(&*raw_name);
                for (namespace, prefix) in &self.namespaces {
                    start.push_attribute(Attribute::from((
                        &*format!("xmlns:{prefix}"),
                        &**namespace,
                    )));
                }

                self.inner.write_event(Event::Start(start))?;
                self.inner
                    .write_event(Event::Text(BytesText::from_escaped(partial_escape(&text))))?;
                self.inner
                    .write_event(Event::End(BytesEnd::new(raw_name)))?;

                Ok(())
            }
            _ => unimplemented!(),
        }
    }
    fn write_value(
        &mut self,
        name: &ElementName<ByteString>,
        value: Value,
    ) -> Result<(), XmlError> {
        use quick_xml::{
            escape::partial_escape,
            events::{BytesEnd, BytesStart, BytesText, Event},
        };

        let raw_name = self.name(name);

        match value {
            Value::Empty => {
                self.inner
                    .write_event(Event::Empty(BytesStart::new(raw_name)))?;
            }
            Value::Text(text) => {
                self.inner
                    .write_event(Event::Start(BytesStart::new(&*raw_name)))?;
                self.inner
                    .write_event(Event::Text(BytesText::from_escaped(partial_escape(&text))))?;
                self.inner
                    .write_event(Event::End(BytesEnd::new(raw_name)))?;
            }
            Value::List(list) => {
                for value in *list {
                    self.write_value(name, value)?;
                }
            }
            Value::Map(map) => {
                self.inner
                    .write_event(Event::Start(BytesStart::new(&*raw_name)))?;
                for (tag, value) in map.0 {
                    self.write_value(&tag, value)?;
                }
                self.inner
                    .write_event(Event::End(BytesEnd::new(raw_name)))?;
            }
        }

        Ok(())
    }
}
