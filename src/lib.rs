
#![doc(html_root_url = "https://docs.rs/not_again/0.2.3")]
#![doc = include_str!("../README.md")]

#[macro_export]
macro_rules! dbg_once {
    ($($arg:tt)*) => {{
        {
            use ::core::sync::atomic::{AtomicBool, Ordering};
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
macro_rules! dbg_if_hash_ne {
    ($val:expr $(,)?) => {
        match $val {
            tmp => {

                use ::core::{hash::{Hash, Hasher}, sync::atomic::{AtomicU64, Ordering}};
                static HASH: AtomicU64 = AtomicU64::new(0);
                let mut s = ::std::hash::DefaultHasher::new();
                tmp.hash(&mut s);
                let current_hash = s.finish();
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

#[macro_export]
macro_rules! dbg_if_ne {
    ($val:expr, $type:tt, $ne:expr) => {
        {
            use ::core::{sync::atomic::{AtomicBool, Ordering}};
            static FIRST: AtomicBool = AtomicBool::new(true);
            let first = FIRST.swap(false, Ordering::Relaxed);
            match $val {
                tmp => {
                    static_atomic!(VALUE: $type);
                    if VALUE.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| (first || $ne(v, tmp)).then_some(tmp)).is_ok() {
                        ::std::eprintln!("[{}:{}:{}] {} = {:#?}",
                                         ::std::file!(), ::std::line!(), ::std::column!(), ::std::stringify!($val), &tmp);
                    }
                    tmp
                }
            }
        }
    };

    ($val:expr, $type:tt) => {
        dbg_if_ne!($val, $type, |last_value, new_value| last_value != new_value)
    };
}

#[macro_export]
macro_rules! dbg_if_relative_ne {
    ($val:expr, $type:tt, $($arg:tt)*) => {
        dbg_if_ne!($val, $type, |a, b| ::approx::relative_ne!(a, b, $($arg)*))
    };

    ($val:expr, $type:tt) => {
        dbg_if_relative_ne!($val, $type,)
    };
}

#[cfg(feature = "float")]
#[macro_export]
macro_rules! dbg_if_abs_diff_ne {
    ($val:expr, $type:tt, $($arg:tt)*) => {
        dbg_if_ne!($val, $type, |a, b| ::approx::abs_diff_ne!(a, b, $($arg)*))
    };

    ($val:expr, $type:tt) => {
        dbg_if_abs_diff_ne!($val, $type,)
    };
}

#[cfg(feature = "float")]
#[macro_export]
macro_rules! dbg_if_ulps_ne {
    ($val:expr, $type:tt, $($arg:tt)*) => {
        dbg_if_ne!($val, $type, |a, b| ::approx::ulps_ne!(a, b, $($arg)*))
    };

    ($val:expr, $type:tt) => {
        dbg_if_ulps_ne!($val, $type,)
    };
}

#[macro_export]
macro_rules! static_atomic {
    ($name:ident: u8) => {
        static $name: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(0);
    };
    ($name:ident: u16) => {
        static $name: ::core::sync::atomic::AtomicU16 = ::core::sync::atomic::AtomicU16::new(0);
    };
    ($name:ident: u32) => {
        static $name: ::core::sync::atomic::AtomicU32 = ::core::sync::atomic::AtomicU32::new(0);
    };
    ($name:ident: u64) => {
        static $name: ::core::sync::atomic::AtomicU64 = ::core::sync::atomic::AtomicU64::new(0);
    };
    ($name:ident: usize) => {
        static $name: ::core::sync::atomic::AtomicUsize = ::core::sync::atomic::AtomicUsize::new(0);
    };
    ($name:ident: i8) => {
        static $name: ::core::sync::atomic::AtomicI8 = ::core::sync::atomic::AtomicI8::new(0);
    };
    ($name:ident: i16) => {
        static $name: ::core::sync::atomic::AtomicI16 = ::core::sync::atomic::AtomicI16::new(0);
    };
    ($name:ident: i32) => {
        static $name: ::core::sync::atomic::AtomicI32 = ::core::sync::atomic::AtomicI32::new(0);
    };
    ($name:ident: i64) => {
        static $name: ::core::sync::atomic::AtomicI64 = ::core::sync::atomic::AtomicI64::new(0);
    };
    ($name:ident: isize) => {
        static $name: ::core::sync::atomic::AtomicIsize = ::core::sync::atomic::AtomicIsize::new(0);
    };
    ($name:ident: f32) => {
        $crate::static_atomic_float!($name: f32);
    };
    ($name:ident: f64) => {
        $crate::static_atomic_float!($name: f64);
    };
}

#[cfg(feature = "float")]
#[macro_export]
macro_rules! static_atomic_float {
    ($name:ident: f32) => {
        static $name: ::atomic_float::AtomicF32 = ::atomic_float::AtomicF32::new(0.0);
    };
    ($name:ident: f64) => {
        static $name: ::atomic_float::AtomicF64 = ::atomic_float::AtomicF64::new(0.0);
    };
}

#[cfg(not(feature = "float"))]
#[macro_export]
macro_rules! static_atomic_float {
    ($name:ident: $type:tt) => {
        compile_error!("Feature \"float\" must be enabled on \"not_again\" crate.");
    };
}

#[cfg(test)]
mod test {

    fn capture_stderr<F: FnOnce()>(f: F) -> String {
        use std::io::Read;
        use gag::BufferRedirect;
        let mut buf = BufferRedirect::stderr().unwrap();
        f();
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        output
    }

    fn strip_dbg(input: String) -> String {
        use regex::Regex;
        let r = Regex::new("\\[.*\\] ").unwrap();
        // r.replace_all(&input.trim(), "[...]").to_string()
        r.replace_all(&input.trim(), "").to_string()
    }

    mod test_dbg {
        use super::*;

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
        fn test_dbg_if_hash_ne_multiple() {
            fn f(x: usize, y: u64) {
                dbg_if_hash_ne!(x, y);
            }

            let output = strip_dbg(capture_stderr(|| {
                f(1, 3);
                f(2, 3);
            }));
            assert_eq!(&output[..], "x = 1\ny = 3\nx = 2");
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
                dbg_if_ne!(x, isize);
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

    #[cfg(feature = "float")]
    mod float_tests {
        use approx::relative_eq;
        use super::*;

        #[test]
        fn test_approx() {
            assert!(relative_eq!(1.0, 1.1, epsilon = 0.2));
            // assert!(relative_eq!(1, 1)); Not implemented
        }

        #[test]
        fn test_dbg_if_ne_f32() {
            fn f(x: f32) {
                dbg_if_ne!(x, f32, |a, b| ::approx::relative_ne!(a, b, epsilon = 0.1));
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
        fn test_dbg_if_ne_f64() {
            fn f(x: f64) {
                dbg_if_ne!(x, f64, |a, b| ::approx::relative_ne!(a, b, epsilon = 0.1));
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
    }
}
