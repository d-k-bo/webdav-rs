// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use indexmap::IndexMap;

use crate::{
    element::ElementExt, value::ValueMap, Element, ExtractElementError, Value, DAV_NAMESPACE,
    DAV_PREFIX,
};

/// The `resourcetype` property as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#PROPERTY_resourcetype).
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceType(ValueMap);

impl Element for ResourceType {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "resourcetype";
}

impl ResourceType {
    pub fn empty() -> Self {
        Self(Default::default())
    }
    pub fn collection() -> Self {
        Self({
            let mut map = IndexMap::with_capacity(1);
            map.insert(Collection::element_name(), Value::Empty);
            ValueMap::from(map)
        })
    }
    pub fn is_collection(&self) -> bool {
        self.0
            .as_ref()
            .contains_key(&Collection::element_name::<&str>())
    }
    pub fn get<'v, T>(&'v self) -> Option<Result<T, ExtractElementError>>
    where
        T: Element + TryFrom<&'v Value, Error = ExtractElementError>,
    {
        self.0.get::<T>()
    }
}

impl TryFrom<&Value> for ResourceType {
    type Error = ExtractElementError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.to_map().cloned().map(Self)
    }
}

impl From<ResourceType> for Value {
    fn from(ResourceType(map): ResourceType) -> Value {
        Value::Map(map)
    }
}

/// The `collection` XML element as defined in
/// [RFC 4918](http://webdav.org/specs/rfc4918.html#ELEMENT_collection).
pub struct Collection;

impl Element for Collection {
    const NAMESPACE: &'static str = DAV_NAMESPACE;
    const PREFIX: &'static str = DAV_PREFIX;
    const LOCAL_NAME: &'static str = "collection";
}
