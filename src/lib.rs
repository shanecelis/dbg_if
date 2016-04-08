#![no_std]

#[cfg(test)]
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

#[test]
fn test_run_once() {
    fn init() {
        assert_has_not_been_called!();
    }
    init();
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
