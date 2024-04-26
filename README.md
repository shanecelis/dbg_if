# not_again

![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/not_again/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/not_again/actions)
  [![crates-io](https://img.shields.io/crates/v/not_again.svg)](https://crates.io/crates/not_again)
  [![api-docs](https://docs.rs/not_again/badge.svg)](https://docs.rs/not_again)

This crate provides `dbg_if_ne!` variants that only print changed values.

## Example

```rust
fn f(x: u8) -> u8 {
  dbg!(x) + 1
}
assert_eq!(f(1), 2);
```

## Motivation

The `dbg!` macro is great. It's like being able to add a probe right into your
code without disturbing everything since it works on expressions. For straight
shot code, it is perfect. For code in tight loops, however, it does leave
something to be desired. Your terminal will scream, "x = 1" at you again and
again until you say, "No, not again."

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
