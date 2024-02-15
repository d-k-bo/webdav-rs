<!--
SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>

SPDX-License-Identifier: CC0-1.0
-->

# webdav-xml

<!-- cargo-rdme start -->

Definitions and (de)serialization for WebDAV XML elements as defined
in [RFC 4918](http://webdav.org/specs/rfc4918.html#xml.element.definitions).

Since WebDAV uses XML namespaces and supports custom elements in the
`<DAV:prop />` element, we can't rely on e. g. `serde` to (de)serialize
XML elements.

Instead, this crate uses the `Element` trait to define an
element and [`FromXml`](https://docs.rs/webdav-xml/latest/webdav_xml/trait.FromXml.html)/[`IntoXml`](https://docs.rs/webdav-xml/latest/webdav_xml/trait.IntoXml.html) for
(de)serialization.

<!-- cargo-rdme end -->

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
