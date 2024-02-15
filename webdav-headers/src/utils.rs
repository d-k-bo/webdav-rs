// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use nonempty::NonEmpty;

/// Helper trait as an alternative to [`FromStr`](std::str::FromStr) that
/// doesn't consume the remaining string.
pub(crate) trait ParseString: Sized {
    type Err;
    fn peek(s: &str) -> Result<(Self, &str), Self::Err>;
    fn parse(s: &mut &str) -> Result<Self, Self::Err> {
        let (slf, remaining) = Self::peek(s)?;
        *s = remaining;
        Ok(slf)
    }
}

pub(crate) trait HeaderIteratorExt<'i> {
    fn take_one(self) -> Result<&'i http::HeaderValue, headers::Error>;
    fn extract_str(self) -> Result<&'i str, headers::Error>;
}

impl<'i, I: Iterator<Item = &'i http::HeaderValue>> HeaderIteratorExt<'i> for I {
    fn take_one(mut self) -> Result<&'i http::HeaderValue, headers::Error> {
        let item = self.next().ok_or_else(headers::Error::invalid)?;
        match self.next() {
            Some(_) => Err(headers::Error::invalid()),
            None => Ok(item),
        }
    }
    fn extract_str(self) -> Result<&'i str, headers::Error> {
        self.take_one()?
            .to_str()
            .map_err(|_| headers::Error::invalid())
    }
}

pub(crate) trait NonEmptyExt<T>: Sized {
    fn try_collect<I, E>(iter: I) -> Result<Option<Self>, E>
    where
        I: IntoIterator<Item = Result<T, E>>;
}
impl<T> NonEmptyExt<T> for NonEmpty<T> {
    fn try_collect<I, E>(iter: I) -> Result<Option<Self>, E>
    where
        I: IntoIterator<Item = Result<T, E>>,
    {
        let mut iter = iter.into_iter();
        let Some(head) = iter.next().transpose()? else {
            return Ok(None);
        };
        let tail = iter.collect::<Result<_, E>>()?;
        Ok(Some(NonEmpty { head, tail }))
    }
}

pub(crate) trait StrExt {
    fn starts_with_ignore_ascii_case(&self, s: &str) -> bool;
    fn strip_prefix_ignore_ascii_case(&self, s: &str) -> Option<&Self>;
}

impl StrExt for str {
    fn starts_with_ignore_ascii_case(&self, s: &str) -> bool {
        self.len() >= s.len() && self[..s.len()].eq_ignore_ascii_case(s)
    }
    fn strip_prefix_ignore_ascii_case(&self, s: &str) -> Option<&Self> {
        self.starts_with_ignore_ascii_case(s)
            .then_some(&self[s.len()..])
    }
}
