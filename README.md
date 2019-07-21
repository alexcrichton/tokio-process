## The tokio-process crate has been moved into the [tokio](https://github.com/tokio-rs/tokio) Git repository.

Please do not use this repo anymore.

# tokio-process

An implementation of process management for Tokio

[![Build Status](https://travis-ci.com/alexcrichton/tokio-process.svg?branch=master)](https://travis-ci.com/alexcrichton/tokio-process)
[![Build status](https://ci.appveyor.com/api/projects/status/43c8g7fy801e5902?svg=true)](https://ci.appveyor.com/project/alexcrichton/tokio-process)
[![Crates.io](https://img.shields.io/crates/v/tokio-process.svg?maxAge=2592000)](https://crates.io/crates/tokio-process)
[![Coverage](https://img.shields.io/codecov/c/github/alexcrichton/tokio-process/master.svg)](https://codecov.io/gh/alexcrichton/tokio-process)

[Documentation](https://docs.rs/tokio-process)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-process = "0.2"
```

Next, add this to your crate:

```rust
extern crate tokio_process;
```


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in tokio-process by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
