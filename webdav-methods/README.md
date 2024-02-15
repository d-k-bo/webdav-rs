<!--
SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>

SPDX-License-Identifier: CC0-1.0
-->

# webdav-methods

<!-- cargo-rdme start -->

HTTP methods for WebDAV as defined in
[RFC 4918][rfc].

Unfortunately, the [`http`][http] crate
[doesn't support creating custom `Method`s constants yet][http-pr],
so they are currently defined as static variables using
[`once_cell::sync::Lazy`][lazy].

[rfc]: http://webdav.org/specs/rfc4918.html#http.methods.for.distributed.authoring
[http]: https://docs.rs/http/latest/http/
[http-pr]: https://github.com/hyperium/http/pull/595
[lazy]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html

<!-- cargo-rdme end -->

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
