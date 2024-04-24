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

#[macro_export]
macro_rules! dbg_if_changed {
    ($($arg:tt)*) => {{
        {
            use $crate::__core::{hash::{Hash, Hasher}, sync::atomic::{AtomicUsize, Ordering}};
            static HASH: AtomicUsize = AtomicUsize::new(0);
            let mut s = std::hash::DefaultHasher::new();
            let value = $($arg)+;
            value.hash(&mut s);
            // let current_hash = std::dbg!(s.finish() as usize);
            let current_hash = s.finish() as usize;
            match HASH.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |h| (h != current_hash).then_some(current_hash)) {
                Ok(_) => {
                    // It has changed.
                    // eprintln!("changed");
                    std::dbg!($($arg)+)
                }
                Err(_) => {
                    // It has not changed.
                    // eprintln!("not changed");
                    $($arg)+
                }
            }
        }
    }};
}

#[cfg(test)]
mod test {

    mod test_dbg {
    use std::borrow::Cow;
    use std::io::Read;
    use gag::BufferRedirect;
    use regex::Regex;
#[test]
fn test_run_once() {
    fn init() {
        dbg_once!("hi");
    }
    init();
}

    fn capture_stderr<F: FnOnce()>(f: F) -> String {
        let mut buf = BufferRedirect::stderr().unwrap();
        f();
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        output
    }

    fn strip_dbg(input: &str) -> Cow<'_, str> {
        let r = Regex::new("\\[.*\\]").unwrap();
        r.replace_all(&input.trim(), "[...]")
    }


#[test]
fn test_dbg_once() {
    fn f() {
        dbg_once!("hi");
    }
    let output = capture_stderr(|| {
                   f();
                   f();
    });

    let output = strip_dbg(&output);
    assert_eq!(&output[..], "[...] \"hi\" = \"hi\"");
}

#[test]
fn test_dbg_if_changed() {
    fn f(x: usize) {
        dbg_if_changed!(x);
    }
    f(1);
    f(1);
    f(2);
}

#[test]
fn test_pass_thru() {
    fn a() {
        let _x: usize = dbg_once!(1);
    }
    a();
    a();
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
