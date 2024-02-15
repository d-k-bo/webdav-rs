// SPDX-FileCopyrightText: d-k-bo <d-k-bo@mailbox.org>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! HTTP methods for WebDAV as defined in
//! [RFC 4918][rfc].
//!
//! Unfortunately, the [`http`][http] crate
//! [doesn't support creating custom `Method`s constants yet][http-pr],
//! so they are currently defined as static variables using
//! [`once_cell::sync::Lazy`][lazy].
//!
//! [rfc]: http://webdav.org/specs/rfc4918.html#http.methods.for.distributed.authoring
//! [http]: https://docs.rs/http/latest/http/
//! [http-pr]: https://github.com/hyperium/http/pull/595
//! [lazy]: https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html

use http::Method;
use once_cell::sync::Lazy;

macro_rules! method {
    ($name:ident ) => {
        #[doc = concat!(
            "The `",
            stringify!($name),
            "` method as defined in [RFC 4918](http://webdav.org/specs/rfc4918.html#METHOD_",
            stringify!($name),
            ")."
        )]
        pub static $name: once_cell::sync::Lazy<Method> =
            Lazy::new(|| Method::from_bytes(stringify!($name).as_bytes()).unwrap());
    };
}

method!(PROPFIND);
method!(PROPPATCH);
method!(MKCOL);
method!(COPY);
method!(MOVE);
method!(LOCK);
method!(UNLOCK);
