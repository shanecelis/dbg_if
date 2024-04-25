#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(test, not(feature = "std")))]
extern crate std;

// Re-export libcore using an alias so that the macros can work in no_std
// crates while remaining compatible with normal crates.
#[doc(hidden)]
pub extern crate core as __core;

/** This macro can be used to ensure that a function is called only once. It panics if the function
is called a second time.

# Example

Using the macro:

```rust
#[macro_use]
extern crate once;

fn init() {
assert_has_not_been_called!();

// code that should only run once
}

fn main() {
    init();
    // init(); // "assertion failed: called == false"
}
```

Custom error message:

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
**/
#[macro_export]
macro_rules! assert_has_not_been_called {
    () => {
        assert_has_not_been_called!("assertion failed: has_run == false");
    };
    ($($arg:tt)+) => {{
        fn assert_has_not_been_called() {
            use $crate::__core::sync::atomic::{AtomicBool, Ordering};
            static CALLED: AtomicBool = AtomicBool::new(false);
            let called = CALLED.swap(true, Ordering::Relaxed);
            assert!(called == false, $($arg)+);
        }
        assert_has_not_been_called();
    }};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! dbg_once {
    ($($arg:tt)*) => {{
        {
            use $crate::__core::sync::atomic::{AtomicBool, Ordering};
            static CALLED: AtomicBool = AtomicBool::new(false);
            let called = CALLED.swap(true, Ordering::Relaxed);
            if !called {
                std::dbg!($($arg)+)
            } else {
                $($arg)+
            }
        }
    }};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! dbg_if_hash_ne {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {

            use $crate::__core::{hash::{Hash, Hasher}, sync::atomic::{AtomicUsize, Ordering}};
            static HASH: AtomicUsize = AtomicUsize::new(0);
            let mut s = ::std::hash::DefaultHasher::new();
            tmp.hash(&mut s);
            // let current_hash = std::dbg!(s.finish() as usize);
            let current_hash = s.finish() as usize;
            if HASH.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |h| (h != current_hash).then_some(current_hash)).is_ok() {
               ::std::eprintln!("[{}:{}:{}] {} = {:#?}",
                    ::std::file!(), ::std::line!(), ::std::column!(), ::std::stringify!($val), &tmp);
            }
            tmp
        }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_if_hash_ne!($val)),+,)
    };
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! dbg_if_ne {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
            use $crate::__core::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
            static FIRST: AtomicBool = AtomicBool::new(true);
            let first = FIRST.swap(false, Ordering::Relaxed);
            // static VALUE: AtomicIsize = AtomicIsize::new(0);
            static VALUE: atomic::Atomic<isize> = atomic::Atomic::new(0);
            // let new_value: isize = (tmp / $tol).try_into().expect("Can't make isize");
            if VALUE.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| (first || v != tmp).then_some(tmp)).is_ok() {
               ::std::eprintln!("[{}:{}:{}] {} = {:#?}",
                    ::std::file!(), ::std::line!(), ::std::column!(), ::std::stringify!($val), &tmp);
            }
            tmp
        }
        }
    };

    // ($val:expr $(,)?) => {
    //      $crate::dbg_if_ne!($val, 1)
    // };
    // ($($val:expr),+ $(,)?) => {
    //     ($($crate::dbg_if_hash_ne!($val)),+,)
    // };
}
use atomic_float::{AtomicF64, AtomicF32};

// pub struct Atomic<T> {
//     value: T,
// }


// impl Atomic<f64> {
//     fn atomic(&self) -> AtomicF64 {
//         AtomicF64::new(0.0)
//     }
// }

// impl Atomic<f32> {
//     fn atomic(&self) -> AtomicF32 {
//         AtomicF32::new(0.0)
//     }
// }

macro_rules! static_atomic {
    ($name:ident, u8) => {
        static $name: AtomicU8 = AtomicU8::new(0.0);
    };
    ($name:ident, u16) => {
        static $name: AtomicU16 = AtomicU16::new(0.0);
    };
    ($name:ident, u32) => {
        static $name: AtomicU32 = AtomicU32::new(0.0);
    };
    ($name:ident, u64) => {
        static $name: AtomicU64 = AtomicU64::new(0.0);
    };
    ($name:ident, usize) => {
        static $name: AtomicUsize = AtomicUsize::new(0.0);
    };
    ($name:ident, i8) => {
        static $name: AtomicI8 = AtomicI8::new(0.0);
    };
    ($name:ident, i16) => {
        static $name: AtomicI16 = AtomicI16::new(0.0);
    };
    ($name:ident, i32) => {
        static $name: AtomicI32 = AtomicI32::new(0.0);
    };
    ($name:ident, i64) => {
        static $name: AtomicI64 = AtomicI64::new(0.0);
    };
    ($name:ident, isize) => {
        static $name: AtomicIsize = AtomicIsize::new(0.0);
    };
    ($name:ident, f32) => {
        static $name: AtomicF32 = AtomicF32::new(0.0);
    };
    ($name:ident, f64) => {
        static $name: AtomicF64 = AtomicF64::new(0.0);
    };
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! dbg_if_relative_ne {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    ($val:expr, $type:tt, $($arg:tt)*) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        {
            use $crate::__core::{any::Any, sync::atomic::{AtomicBool, AtomicIsize, Ordering}};
            use atomic_float::{AtomicF64, AtomicF32};
            static FIRST: AtomicBool = AtomicBool::new(true);
            let first = FIRST.swap(false, Ordering::Relaxed);
            match $val {
                tmp => {
                    static_atomic!(VALUE, $type);
                    if VALUE.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| (first || ::approx::relative_ne!(v, tmp, $($arg)+)).then_some(tmp)).is_ok() {
                        ::std::eprintln!("[{}:{}:{}] {} = {:#?}",
                                         ::std::file!(), ::std::line!(), ::std::column!(), ::std::stringify!($val), &tmp);
                    }
                    tmp
                }
            }
        }
    };

    ($val:expr, $type:tt) => {
        dbg_if_relative_ne!($val, $type,)
    }
}


#[cfg(test)]
mod test {

    #[cfg(feature = "std")]
    mod test_dbg {
        // use std::borrow::Cow;
        use std::io::Read;
        use gag::BufferRedirect;
        use regex::Regex;
        use approx::relative_eq;

        #[test]
        fn test_approx() {
            assert!(relative_eq!(1.0, 1.1, epsilon = 0.2));
            // assert!(relative_eq!(1, 1)); Not implemented
        }

        fn capture_stderr<F: FnOnce()>(f: F) -> String {
            let mut buf = BufferRedirect::stderr().unwrap();
            f();
            let mut output = String::new();
            buf.read_to_string(&mut output).unwrap();
            output
        }

        fn strip_dbg(input: String) -> String {
            let r = Regex::new("\\[.*\\] ").unwrap();
            // r.replace_all(&input.trim(), "[...]").to_string()
            r.replace_all(&input.trim(), "").to_string()
        }

        #[test]
        fn test_run_once() {
            fn f() {
                dbg_once!("hi");
            }
            let output = strip_dbg(capture_stderr(|| {
                f();
            }));
            assert_eq!(&output[..], "\"hi\" = \"hi\"");
        }

        #[test]
        fn test_dbg_once() {
            fn f() {
                dbg_once!("hi");
            }
            let output = strip_dbg(capture_stderr(|| {
                f();
                f();
            }));
            assert_eq!(&output[..], "\"hi\" = \"hi\"");
        }

        #[test]
        fn test_dbg_if_hash_ne() {
            fn f(x: usize) {
                dbg_if_hash_ne!(x);
            }

            let output = strip_dbg(capture_stderr(|| {
                f(1);
                f(2);
            }));
            assert_eq!(&output[..], "x = 1\nx = 2");
        }

        #[test]
        fn test_dbg_if_hash_ne_eval_once() {
            fn f(x: &mut usize) {
                dbg_if_hash_ne!({ *x += 1; *x });
            }

            let output = strip_dbg(capture_stderr(|| {
                let mut x: usize = 0;
                f(&mut x);
            }));
            assert_eq!(&output[..], "{ *x += 1; *x } = 1");
        }

        #[test]
        fn test_dbg_if_ne() {
            fn f(x: isize) {
                dbg_if_ne!(x);
            }

            let output = strip_dbg(capture_stderr(|| {
                let mut x: isize = 1;
                f(x);
                f(x);
                x += 1;
                f(x);
            }));
            assert_eq!(&output[..], "x = 1\nx = 2");
        }

        #[test]
        fn test_dbg_if_relative_ne_f32() {
            fn f(x: f32) {
                dbg_if_relative_ne!(x, f32, epsilon = 0.1);
            }

            let output = strip_dbg(capture_stderr(|| {
                let mut x: f32 = 1.1;
                f(x);
                f(x);
                x += 0.1;
                f(x);
            }));
            assert_eq!(&output[..], "x = 1.1\nx = 1.2");
        }

        #[test]
        fn test_dbg_if_relative_ne_f64() {
            fn f(x: f64) {
                dbg_if_relative_ne!(x, f64, epsilon = 0.1);
            }

            let output = strip_dbg(capture_stderr(|| {
                let mut x: f64 = 1.1;
                f(x);
                f(x);
                x += 0.1;
                f(x);
            }));
            assert_eq!(&output[..], "x = 1.1\nx = 1.2000000000000002");
        }

        #[test]
        fn test_pass_thru() {
            fn a() {
                let _x: usize = dbg_once!(1);
            }

            let output = strip_dbg(capture_stderr(|| {
                a();
                a();
            }));
            assert_eq!(&output[..], "1 = 1");
        }

    }

    #[test]
    fn test_run_once_different_fns() {
        fn init1() {
            assert_has_not_been_called!();
        }
        fn init2() {
            assert_has_not_been_called!();
        }
        init1();
        init2();
    }

    #[test]
    #[should_panic]
    fn test_run_twice() {
        fn init() {
            assert_has_not_been_called!();
        }
        init();
        init();
    }

    #[test]
    fn test_hygiene1() {
        fn init() {
            assert_has_not_been_called!();

            #[allow(dead_code)]
            fn assert_has_not_been_called() {}
        }
        init();
    }

    #[test]
    fn test_hygiene2() {
        fn init() {
            assert_has_not_been_called!();

            #[allow(dead_code)]
            static CALLED: i32 = 42;
        }
        init();
    }
}
