# debugless-unwrap

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/debugless-unwrap)
[![Crates.io](https://img.shields.io/crates/v/debugless-unwrap)](https://crates.io/crates/debugless-unwrap)
[![Docs.rs](https://docs.rs/debugless-unwrap/badge.svg)](https://docs.rs/crates/debugless-unwrap)

![Rust 1.46.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.46.0&color=grey)
[![Build Status](https://travis-ci.com/Tamschi/debugless-unwrap.svg?branch=unstable)](https://travis-ci.com/Tamschi/debugless-unwrap/branches)
![Crates.io - License](https://img.shields.io/crates/l/debugless-unwrap/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/debugless-unwrap)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/debugless-unwrap)](https://github.com/Tamschi/debugless-unwrap/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/debugless-unwrap)](https://github.com/Tamschi/debugless-unwrap/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/debugless-unwrap.svg)](https://web.crev.dev/rust-reviews/crate/debugless-unwrap/)

This library provides alternatives to the standard `.unwrap`* methods on `Result` and `Option` that don't require `Debug` to be implemented on the unexpected variant.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add debugless-unwrap
```

## Example

```rust
use assert_panic::assert_panic;
use debugless_unwrap::*;

#[derive(Copy, Clone)]
struct T;

let some = Some(T);
let none = Option::<T>::None;
let ok = Result::<T, T>::Ok(T);
let err = Result::<T, T>::Err(T);

none.debugless_unwrap_none();
ok.debugless_unwrap();
err.debugless_unwrap_err();

assert_panic!(some.debugless_unwrap_none());
assert_panic!({ err.debugless_unwrap(); });
assert_panic!({ ok.debugless_unwrap_err(); });
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`debugless-unwrap` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
