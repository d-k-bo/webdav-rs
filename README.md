<!--
SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>

SPDX-License-Identifier: CC0-1.0
-->

# webdav-rs

> [!WARNING]
> ## This project is no longer maintained
>
> ### Why?
>
> As a hobbyist programmer, I have invested a significant amount of my personal time and energy into many projects. Due to increasing personal and professional commitments, I have significant less time to dedicate to software development. I am therefore focusing on the projects that I actively use myself and enjoy working on the most. Recent developments in "AI" have also contributed to a general decline in my interest in software development.
>
> ### What does this mean?
>
> - This project will not be actively maintained for the foreseeable future. Accordingly, it should be considered as deprecated.
> - The project remains available under the terms of the corresponding free software license.
> - You are welcome to redistribute, fork and continue its development under the terms of the said ‍‍‍license.


[![Build Status][ci-badge]][ci-link]
[![REUSE Compliance Check][reuse-badge]][reuse-link]
[![crates.io][crates-badge]][crates-link]
[![Documentation][docs-badge]][docs-link]
![License: MIT OR Apache-2.0][license-badge]

[ci-badge]: https://github.com/d-k-bo/webdav-rs/actions/workflows/ci.yaml/badge.svg?branch=main
[ci-link]: https://github.com/d-k-bo/webdav-rs/actions?query=workflow%3ACI
[reuse-badge]: https://github.com/d-k-bo/webdav-rs/actions/workflows/reuse.yaml/badge.svg?branch=main
[reuse-link]: https://reuse.software/
[crates-badge]: https://img.shields.io/crates/v/webdav-meta
[crates-link]: https://lib.rs/crates/webdav-meta
[docs-badge]: https://img.shields.io/docsrs/webdav-meta
[docs-link]: https://docs.rs/webdav-meta
[license-badge]: https://img.shields.io/crates/l/webdav-meta

<!-- cargo-rdme start -->

Reusable types for implementing WebDAV clients and servers based on
[RFC 4918][rfc].

This crate is intended to be used together with libraries that build on the
general-purpose [`http`][http] crate.

[rfc]: http://webdav.org/specs/rfc4918.html
[http]: https://docs.rs/http/latest/http/

## Usage

```sh
cargo add webdav-meta --rename webdav
```

## Implemented features

<details>
    <summary>HTTP Methods</summary>

HTTP methods are currently defined as static variables, but should be moved
to constants in the future.

- [X] `PROPFIND`
- [X] `PROPPATCH`
- [X] `MKCOL`
- [X] `COPY`
- [X] `MOVE`
- [X] `LOCK`
- [X] `UNLOCK`

</details>
<details>
    <summary>HTTP Headers</summary>

- [X] `DAV`
- [X] `Depth`
- [X] `Destination`
- [X] `If`
- [X] `Lock-Token`
- [X] `Overwrite`
- [X] `Timeout`

</details>
<details>
    <summary>XML Elements</summary>

- [X] `activelock`
- [X] `allprop`: internally implemented for
  `Propfind`
- [X] `collection`: internally implemented for
  `ResourceType`
- [X] `depth`
- [ ] `error`: currently just a string
- [X] `exclusive`: internally implemented for
  `LockScope`
- [X] `href`
- [X] `include`
- [ ] `location`
- [X] `lockentry`
- [X] `lockinfo`
- [X] `lockroot`
- [X] `lockscope`
- [X] `locktoken`
- [X] `locktype`
- [X] `multistatus`
- [X] `owner`
- [X] `prop`
- [X] `propertyupdate`
- [X] `propfind`
- [X] `propname`: internally implemented for
  `Propfind`
- [X] `propstat`
- [X] `remove`: internally implemented for
  `PropertyUpdate`
- [X] `response`
- [X] `responsedescription`
- [X] `set`: internally implemented for
  `PropertyUpdate`
- [X] `shared`: internally implemented for
  `LockScope`
- [ ] `status`
- [X] `timeout`
- [X] `write`: internally implemented for
  `LockType`

</details>
<details>
    <summary>DAV properties</summary>

- [X] `creationdate`
- [X] `displayname`
- [X] `getcontentlanguage`
- [X] `getcontentlength`
- [X] `getcontenttype`
- [X] `getetag`
- [X] `getlastmodified`
- [X] `lockdiscovery`
- [X] `resourcetype`
- [X] `supportedlock`

</details>

<!-- cargo-rdme end -->

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
