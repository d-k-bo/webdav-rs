// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fmt::Display, str::FromStr};

use nonempty::NonEmpty;

use crate::{
    utils::{HeaderIteratorExt, NonEmptyExt, ParseString, StrExt},
    CodedUrl, IF,
};

pub use self::error::InvalidIf;

/// The `If` header as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#HEADER_If).
#[derive(Clone, Debug, PartialEq)]
pub enum If {
    NoTagList(Box<NonEmpty<NonEmpty<Condition>>>),
    TaggedList(Box<NonEmpty<(ResourceTag, NonEmpty<NonEmpty<Condition>>)>>),
}

impl headers::Header for If {
    fn name() -> &'static http::HeaderName {
        &IF
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        Ok(values.extract_str()?.parse()?)
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        values.extend(std::iter::once(self.to_string().parse().unwrap()))
    }
}

impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_condition_lists(
            f: &mut std::fmt::Formatter<'_>,
            lists: &NonEmpty<NonEmpty<Condition>>,
        ) -> std::fmt::Result {
            for (i, conditions) in lists.iter().enumerate() {
                if i > 0 {
                    f.write_str(" ")?;
                }

                f.write_str("(")?;
                for (j, condition) in conditions.iter().enumerate() {
                    if j > 0 {
                        f.write_str(" ")?;
                    }
                    condition.fmt(f)?;
                }
                f.write_str(")")?;
            }
            Ok(())
        }

        match self {
            If::NoTagList(lists) => fmt_condition_lists(f, lists)?,
            If::TaggedList(resources) => {
                for (resource_tag, lists) in &**resources {
                    resource_tag.fmt(f)?;
                    f.write_str(" ")?;
                    fmt_condition_lists(f, lists)?
                }
            }
        }
        Ok(())
    }
}

impl FromStr for If {
    type Err = InvalidIf;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        Self::parse(&mut s)
    }
}

impl ParseString for If {
    type Err = InvalidIf;

    fn peek(mut s: &str) -> Result<(Self, &str), Self::Err> {
        s = s.trim();

        match ResourceTag::peek(s) {
            Ok(_) => {
                let resources = NonEmpty::try_collect(std::iter::from_fn(|| {
                    (!s.is_empty()).then(|| {
                        let resource_tag = ResourceTag::parse(&mut s)?;
                        s = s.trim_start();

                        let condition_lists = NonEmpty::try_collect(std::iter::from_fn(|| {
                            (!s.is_empty() && !s.starts_with('<')).then(|| {
                                let conditions = <NonEmpty<Condition>>::parse(&mut s)?;
                                s = s.trim_start();
                                Ok::<_, InvalidIf>(conditions)
                            })
                        }))?
                        .ok_or(InvalidIf::EmptyConditionList)?;

                        Ok::<_, InvalidIf>((resource_tag, condition_lists))
                    })
                }))?
                .ok_or(InvalidIf::EmptyResourceList)?;

                Ok((If::TaggedList(Box::new(resources)), s))
            }
            Err(_) => {
                let condition_lists = NonEmpty::try_collect(std::iter::from_fn(|| {
                    (!s.is_empty()).then(|| {
                        let conditions = <NonEmpty<Condition>>::parse(&mut s)?;
                        s = s.trim_start();
                        Ok::<_, InvalidIf>(conditions)
                    })
                }))?
                .ok_or(InvalidIf::EmptyConditionList)?;

                Ok((If::NoTagList(Box::new(condition_lists)), s))
            }
        }
    }
}

/// Condition used in the `If` header.
#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    StateToken { not: bool, coded_url: CodedUrl },
    ETag { not: bool, etag: String },
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::StateToken { not, coded_url } => {
                if *not {
                    f.write_str("Not ")?;
                }
                coded_url.fmt(f)?;
            }
            Condition::ETag { not, etag } => {
                if *not {
                    f.write_str("Not ")?;
                }
                f.write_fmt(format_args!("[{etag}]"))?;
            }
        }
        Ok(())
    }
}

impl ParseString for Condition {
    type Err = InvalidIf;

    fn peek(mut s: &str) -> Result<(Self, &str), Self::Err> {
        let not = s.starts_with_ignore_ascii_case("NOT");
        if not {
            s = s[3..].trim_start();
        }

        if s.starts_with('[') {
            s = &s[1..];
            let Some(end) = s.find(']') else {
                return Err(InvalidIf::ExpectedChar(']'));
            };

            Ok((
                Condition::ETag {
                    not,
                    etag: s[..end].to_owned(),
                },
                &s[end + 1..],
            ))
        } else {
            Ok((
                Condition::StateToken {
                    not,
                    coded_url: CodedUrl::parse(&mut s).map_err(InvalidIf::CodedUrl)?,
                },
                s,
            ))
        }
    }
}

impl ParseString for NonEmpty<Condition> {
    type Err = InvalidIf;

    fn peek(mut s: &str) -> Result<(Self, &str), Self::Err> {
        if s.starts_with('(') {
            s = s[1..].trim_start();
        } else {
            return Err(InvalidIf::ExpectedChar('('));
        }
        let conditions = NonEmpty::try_collect(std::iter::from_fn(|| {
            (!s.starts_with(')')).then(|| {
                let condition = Condition::parse(&mut s)?;
                s = s.trim_start();
                Ok::<_, InvalidIf>(condition)
            })
        }))?
        .ok_or(InvalidIf::EmptyConditionList)?;

        Ok((conditions, s[1..].trim_start()))
    }
}

/// Resource tag used in the `If` header.
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceTag(pub http::Uri);

impl Display for ResourceTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

impl FromStr for ResourceTag {
    type Err = InvalidIf;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        Self::parse(&mut s)
    }
}

impl ParseString for ResourceTag {
    type Err = InvalidIf;

    fn peek(mut s: &str) -> Result<(Self, &str), Self::Err> {
        if s.starts_with('<') {
            s = &s[1..];
        } else {
            return Err(InvalidIf::ExpectedChar('<'));
        }
        let Some(end) = s.find('>') else {
            return Err(InvalidIf::ExpectedChar('>'));
        };

        let uri = http::Uri::from_str(&s[..end]).map_err(InvalidIf::Uri)?;

        Ok((ResourceTag(uri), &s[end + 1..]))
    }
}

mod error {
    use crate::InvalidCodedUrl;

    /// Error returned when parsing [`If`](super::If) from a string fails.
    #[derive(Debug)]
    pub enum InvalidIf {
        ExpectedChar(char),
        EmptyConditionList,
        EmptyResourceList,
        CodedUrl(InvalidCodedUrl),
        Uri(http::uri::InvalidUri),
    }

    impl std::fmt::Display for InvalidIf {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ExpectedChar(c) => write!(f, "expected '{c}'"),
                Self::EmptyConditionList => f.write_str("empty condition list"),
                Self::EmptyResourceList => f.write_str("empty resource list"),
                Self::CodedUrl(..) => f.write_str("invalid Coded-URL"),
                Self::Uri(..) => f.write_str("invalid URI"),
            }
        }
    }

    impl std::error::Error for InvalidIf {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::Uri(e) => Some(e),
                _ => None,
            }
        }
    }

    impl From<InvalidIf> for headers::Error {
        fn from(_: InvalidIf) -> Self {
            headers::Error::invalid()
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    use nonempty::nonempty;

    use crate::test::test_all;

    test_all([
        (
            // http://webdav.org/specs/rfc4918.html#if.header.evaluation.example.no-tag
            r#"(<urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2> ["I am an ETag"]) (["I am another ETag"])"#,
            If::NoTagList(Box::new(nonempty![
                nonempty![
                    Condition::StateToken {
                        not: false,
                        coded_url: CodedUrl(
                            uniresid::AbsoluteUri::parse(
                                "urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2"
                            )
                            .unwrap()
                        )
                    },
                    Condition::ETag {
                        not: false,
                        etag: r#""I am an ETag""#.into()
                    },
                ],
                nonempty![
                    Condition::ETag {
                        not: false,
                        etag: r#""I am another ETag""#.into()
                    },
                ],
            ])),
        ),
        (
            // http://webdav.org/specs/rfc4918.html#rfc.section.10.4.7
            "(Not <urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2> <urn:uuid:58f202ac-22cf-11d1-b12d-002035b29092>)",
            If::NoTagList(Box::new(nonempty![
                nonempty![
                    Condition::StateToken {
                        not: true,
                        coded_url: CodedUrl(
                            uniresid::AbsoluteUri::parse("urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2")
                                .unwrap()
                        )
                    },
                    Condition::StateToken {
                        not: false,
                        coded_url: CodedUrl(
                            uniresid::AbsoluteUri::parse("urn:uuid:58f202ac-22cf-11d1-b12d-002035b29092")
                                .unwrap()
                        )
                    },
                ],
            ]))
        ),
        (
            // http://webdav.org/specs/rfc4918.html#rfc.section.10.4.9
            r#"</resource1> (<urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2> [W/"A weak ETag"]) (["strong ETag"])"#,
            If::TaggedList(Box::new(nonempty![
                (
                    ResourceTag("/resource1".parse().unwrap()) ,
                    nonempty![
                        nonempty![
                            Condition::StateToken {
                                not: false,
                                coded_url: CodedUrl(
                                    uniresid::AbsoluteUri::parse(
                                        "urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2"
                                    )
                                    .unwrap()
                                )
                            },
                            Condition::ETag {
                                not: false,
                                etag: r#"W/"A weak ETag""#.into()
                            }
                        ],
                        nonempty![Condition::ETag {
                            not: false,
                            etag: r#""strong ETag""#.into()
                        }],
                    ]
                ),
            ]))
        ),
    ]);
}
