// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use bytes::Bytes;
use nonempty::{nonempty, NonEmpty};

pub(crate) trait NonEmptyExt<T>: Sized {
    fn try_collect<E>(iter: impl IntoIterator<Item = Result<T, E>>) -> Result<Option<Self>, E>;
}
impl<T> NonEmptyExt<T> for NonEmpty<T> {
    /// Since [`NonEmpty`] can't implement [`FromIterator`], this provides
    /// something equivalent to [`Iterator::collect::<Result<NonEmpty<T,
    /// E>>>()`].
    fn try_collect<E>(iter: impl IntoIterator<Item = Result<T, E>>) -> Result<Option<Self>, E> {
        let mut list: Option<NonEmpty<T>> = None;
        for item in iter {
            match &mut list {
                Some(list) => list.push(item?),
                None => list = Some(nonempty![item?]),
            }
        }
        Ok(list)
    }
}

pub(crate) trait BytesExt {
    fn maybe_slice_ref(&self, subset: &[u8]) -> Bytes;
}
impl BytesExt for Bytes {
    /// Similar to [`Bytes::slice_ref`], but creates a new allocation if
    /// `subset` is out of bounds.
    fn maybe_slice_ref(&self, subset: &[u8]) -> Bytes {
        if subset.is_empty() {
            return Bytes::new();
        }

        let bytes_p = self.as_ptr() as usize;
        let bytes_len = self.len();

        let sub_p = subset.as_ptr() as usize;
        let sub_len = subset.len();

        if sub_p >= bytes_p && sub_p + sub_len <= bytes_p + bytes_len {
            let sub_offset = sub_p - bytes_p;

            self.slice(sub_offset..(sub_offset + sub_len))
        } else {
            Bytes::copy_from_slice(subset)
        }
    }
}

#[cfg(test)]
#[track_caller]
pub(crate) fn test_serialize<E: crate::IntoXml>(
    expected_xml: &'static str,
    element: E,
) -> crate::Result<()> {
    use bytestring::ByteString;
    use pretty_assertions::assert_str_eq;

    fn trim_xml(xml: &str) -> &str {
        xml.trim()
            .trim_start_matches(r#"<?xml version="1.0" encoding="utf-8"?>"#)
            .trim()
    }

    let xml = ByteString::try_from(element.into_xml()?).unwrap();
    assert_str_eq!(trim_xml(expected_xml), trim_xml(&xml));

    Ok(())
}

#[cfg(test)]
#[track_caller]
pub(crate) fn test_deserialize<E: crate::FromXml + PartialEq + std::fmt::Debug>(
    expected_element: &E,
    xml: &'static str,
) -> crate::Result<()> {
    use pretty_assertions::assert_eq;

    assert_eq!(expected_element, &E::from_xml(xml)?);

    Ok(())
}
