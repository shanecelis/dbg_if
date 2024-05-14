# not_again

![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/not_again/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/not_again/actions)
[![crates-io](https://img.shields.io/crates/v/not_again.svg)](https://crates.io/crates/not_again)
[![api-docs](https://docs.rs/not_again/badge.svg)](https://docs.rs/not_again)

`dbg!` in the loop without terminal woes.

## Summary

The macro [`dbg_once!`] only prints its value the first time.

```rust
use not_again::dbg_once;
for i in 0..10 {
    dbg_once!(i); // Outputs: [src/lib.rs:9:9] x = 0
}
```

The macro [`dbg_if_ne!`] only prints changed values.

```rust
use not_again::dbg_if_ne;
fn f(x: u8) -> u8 {
    dbg_if_ne!(x, u8)
}
f(1); // Outputs: [src/lib.rs:58:9] x = 1
f(1); // No output.
f(2); // Outputs: [src/lib.rs:58:9] x = 2
```

The macro [`dbg_if_hash_ne!`] only prints on changed hash values.

```rust
use not_again::dbg_if_hash_ne;
let mut s: String = "hello".into();
fn f(x: &str) -> &str {
    dbg_if_hash_ne!(x)
}
f(&s); // Outputs: [src/lib.rs:37:9] x = "hello"
f(&s); // No output.
s.push('!');
f(&s); // Outputs: [src/lib.rs:37:9] x = "hello!"
```

The sister macros [`once!`], [`was_ne!`], and [`was_hash_ne!`] return true instead
of printing.

### Feature "float"

If the feature "float" is enabled, these macros are available:

- [`abs_diff_ne_args!`] accepts `epsilon` argument,
- [`relative_ne_args!`] accepts `epsilon` and `relative_max` arguments,
- and [`ulps_ne_args!`] accepts `epsilon` and `ulps_max` arguments.

These can be given as the third argument to [`was_ne!`] or [`dbg_if_ne!`]. See
the [`approx`] crate for more details.

```rust
#[cfg(feature = "float")]
{
use not_again::{dbg_if_ne, abs_diff_ne_args};
fn f(x: f32) -> f32 {
    dbg_if_ne!(x, f32, abs_diff_ne_args!(epsilon = 1.0))
}
f(1.0); // Outputs: [src/lib.rs:42:9] x = 1.0
f(1.5); // No output.
f(2.0); // No output.
f(2.1); // Outputs: [src/lib.rs:42:9] x = 2.1
}
```

## Goals

- Ease debugging inspection without resorting to a debugger.

## Motivation

```rust
fn f(x: u8) -> u8 {
  dbg!(x) + 1
}
assert_eq!(f(1), 2);
```

The `dbg!` macro is great. It's like being able to add a probe right into your
code without disturbing everything since it works on expressions and lets them
"pass thru." For straight shot code, it is perfect. 

### But Not In Loops

```rust
fn f(x: u8) -> u8 {
    let mut accum = 0;
    for i in 0..100 {
        accum += dbg!(x);
    }
    accum
}
```

```text
[src/main.rs:59:18] x = 1 
[src/main.rs:59:18] x = 1 
[src/main.rs:59:18] x = 1 
...^C
```

For code in tight loops, however, `dbg!` leaves something to be desired. The
terminal screams, "x = 1" again and again. There has got to be a better way.

### Can We Do Better?

Yes! Let's take note of the value at the call site&mdash;with a static atomic
variable&mdash;and instead of spamming the terminal with the same information, let's
only emit information when it has changed with [`dbg_if_ne!`].

```rust
use not_again::dbg_if_ne;
fn f(x: u8) -> u8 {
    let mut accum = 0;
    for i in 0..5 {
        accum += dbg_if_ne!(x, u8);
    }
    accum
}
f(1); // Outputs: [src/main.rs:59:18] x = 1 
```

### But I Have Non-Atomic Values?

That's fine. Can they be hashed? Because hashes can be stored in an `AtomicU64`
at the call site. Just use [`dbg_if_hash_ne!`].

## Tests

Some tests require a particular setup in order to run successfully. A couple of
aliases have been placed in `.cargo/config.toml` to run these tests.

- `cargo test` runs the `was*` tests.
- `cargo test-output` runs above and the `dbg*` tests which tests its output, requires `--nocapture` and single threaded execution.
- `cargo test-all` runs above and the float features.

## License

This crate is licensed, at your option, under either 

- the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or
- the [MIT license](http://opensource.org/licenses/MIT).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Thank you to [Philipp Oppermann](https://github.com/phil-opp) for his crate
[`once`](https://github.com/phil-opp/rust-once). I initially thought I'd only
write `dbg_once!` and submit a PR. But once I got going I realized `dbg_if_ne!`
would be useful too and these are all require `std`; `once` is a `no_std` crate.
So `not_again` is inspired and informed by `once` but it actually doesn't share
any code with `once`.
