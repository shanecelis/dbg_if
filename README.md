# Once

[![Build Status](https://travis-ci.org/phil-opp/rust-once.svg?branch=master)](https://travis-ci.org/phil-opp/rust-once)

This `#![no_std]` crate provides an `assert_has_not_been_called!` macro that panics if the surrounding function is called twice. Useful for initialization functions.

[Documentation](https://crates.fyi/crates/once/)

## Example

```rust
#[macro_use]
extern crate once;

fn init() {
    assert_has_not_been_called!("the init function must only be called {}", "once");
}

fn main() {
    init();
    // init(); // "the init function must only be called once"
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
