# not_again

![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/shanecelis/not_again/actions/workflows/rust.yml/badge.svg)](https://github.com/shanecelis/not_again/actions)
  [![crates-io](https://img.shields.io/crates/v/not_again.svg)](https://crates.io/crates/not_again)
  [![api-docs](https://docs.rs/not_again/badge.svg)](https://docs.rs/not_again)

This crate provides `dbg!` variants suitable for use in loops.

> ♫ Here I go again on my own,  
> using print statements is all I've ever known,  
> like a drifter I was born to debug alone,  
> but I've made up my mind.   
> I ain't wasting no more time.  
> Here I go `not_again` on my code. ♫

## Summary

The macro `dbg_once!`only prints its value the first time.

The macro `dbg_if_ne!` only prints changed values.

The macro `dbg_if_hash_ne!` only prints on changed hash values.

## Example


## 


## Motivation

```rust
fn f(x: u8) -> u8 {
  dbg!(x) + 1
}
assert_eq!(f(1), 2);
```

The `dbg!` macro is great. It's like being able to add a probe right into your
code without disturbing everything since it works on expressions. For straight
shot code, it is perfect. For code in tight loops, however, it does leave
something to be desired. Your terminal will scream, "x = 1" at you again and
again until you say, "No, not again."

### 

```rust
fn f(x: u8) -> u8 {
    let mut accum = 0;
    for i in 0..10 {
        accum += dbg!(x) + 1;
    }
    accum
}
assert_eq!(f(1), 20);
```

## License

This crate is licensed, at your option, under either the

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or
- [MIT license](http://opensource.org/licenses/MIT).

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
