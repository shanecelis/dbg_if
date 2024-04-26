#![doc(html_root_url = "https://docs.rs/not_again/0.2.3")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

/// Calls [std::dbg] exactly once per callsite.
///
/// ```rust
/// use not_again::dbg_once;
/// for i in 0..10 {
///     dbg_once!(i); // Outputs: [src/lib.rs:9:9] x = 0
/// }
/// ```
#[macro_export]
macro_rules! dbg_once {
    ($($arg:tt)*) => {{
        {
            use ::core::sync::atomic::{AtomicBool, Ordering};
            static FIRST: AtomicBool = AtomicBool::new(true);
            let called = FIRST.swap(false, Ordering::Relaxed);
            if called {
                std::dbg!($($arg)+)
            } else {
                $($arg)+
            }
        }
    }};
}

/// Calls [std::dbg] if the argument is not equal to its prior value.
///
/// ```rust
/// use not_again::dbg_if_ne;
/// fn f(x: u8) -> u8 {
///     dbg_if_ne!(x, u8) + 1
/// }
/// f(1); // Outputs: [src/lib.rs:58:9] x = 1
/// f(1); // No output.
/// f(2); // Outputs: [src/lib.rs:58:9] x = 2
/// ```
///
/// # Use a closure as third argument
///
/// This macro accepts a third argument for a function or closure "not equal" or
/// `ne` with this signature: `FnMut<T>(T, T) -> bool`.
///
/// ```rust
/// use not_again::dbg_if_ne;
/// for i in 0..=20 {
///     dbg_if_ne!(i, i8,
///         |last_value: i8, new_value: i8|
///             (new_value - last_value).abs() >= 10);
/// }
/// // Outputs: [src/lib.rs:58:9] i = 0
/// // Outputs: [src/lib.rs:58:9] i = 10
/// // Outputs: [src/lib.rs:58:9] i = 20
/// ```
#[macro_export]
macro_rules! dbg_if_ne {
    ($val:expr, $type:tt, $ne:expr) => {
        {
            use ::core::{sync::atomic::{AtomicBool, Ordering}};
            static FIRST: AtomicBool = AtomicBool::new(true);
            let first = FIRST.swap(false, Ordering::Relaxed);
            match $val {
                tmp => {
                    $crate::static_atomic!(VALUE: $type);
                    let ne_fn = $ne;
                    if VALUE.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| (first || ne_fn(v, tmp)).then_some(tmp)).is_ok() {
                        ::std::eprintln!("[{}:{}:{}] {} = {:#?}",
                                         ::std::file!(), ::std::line!(), ::std::column!(), ::std::stringify!($val), &tmp);
                    }
                    tmp
                }
            }
        }
    };

    ($val:expr, $type:tt $(,)?) => {
        $crate::dbg_if_ne!($val, $type, |last_value, new_value| last_value != new_value)
    };
}

/// Calls [std::dbg] if the argument's hash is not equal to its prior value.
///
/// ```rust
/// use not_again::dbg_if_hash_ne;
/// let mut s: String = "hello".into();
/// fn f(x: &mut String) {
///     dbg_if_hash_ne!(x);
/// }
/// f(&mut s); // Outputs: [src/lib.rs:37:9] x = "hello"
/// f(&mut s); // No output.
/// s.push('!');
/// f(&mut s); // Outputs: [src/lib.rs:37:9] x = "hello!"
/// ```
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

/// Calls [std::dbg] if the float argument is not equal to its prior value.
///
/// ```rust
/// use not_again::dbg_if_relative_ne;
/// fn f(x: f32) -> f32 {
///     dbg_if_relative_ne!(x, f32) + 0.1
/// }
/// f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
/// f(1.0); // No output.
/// f(1.1); // Outputs: [src/lib.rs:58:9] x = 1.1
/// ```
///
/// # Arguments
///
/// Accepts arguments that [approx::relative_ne] accept.
/// ```rust
/// use not_again::dbg_if_relative_ne;
/// fn f(x: f32) -> f32 {
///     dbg_if_relative_ne!(x, f32, epsilon = 1.0)
/// }
/// f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
/// f(1.5); // No output.
/// f(2.0); // No output.
/// f(2.1); // Outputs: [src/lib.rs:58:9] x = 2.1
/// ```
#[cfg(feature = "float")]
#[macro_export]
macro_rules! dbg_if_relative_ne {
    ($val:expr, $type:tt, $($arg:tt)*) => {
        $crate::dbg_if_ne!($val, $type, |a, b| ::approx::relative_ne!(a, b, $($arg)*))
    };

    ($val:expr, $type:tt) => {
        dbg_if_relative_ne!($val, $type,)
    };
}

/// Calls [std::dbg] if the float argument is not equal to its prior value.
///
/// ```rust
/// use not_again::dbg_if_abs_diff_ne;
/// fn f(x: f32) -> f32 {
///     dbg_if_abs_diff_ne!(x, f32) + 0.1
/// }
/// f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
/// f(1.0); // No output.
/// f(1.1); // Outputs: [src/lib.rs:58:9] x = 1.1
/// ```
///
/// # Arguments
///
/// Accepts arguments that [approx::abs_diff_ne] accept.
/// ```rust
/// use not_again::dbg_if_abs_diff_ne;
/// fn f(x: f32) -> f32 {
///     dbg_if_abs_diff_ne!(x, f32, epsilon = 1.0)
/// }
/// f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
/// f(1.5); // No output.
/// f(2.0); // No output.
/// f(2.1); // Outputs: [src/lib.rs:58:9] x = 2.1
/// ```
#[cfg(feature = "float")]
#[macro_export]
macro_rules! dbg_if_abs_diff_ne {
    ($val:expr, $type:tt, $($arg:tt)*) => {
        $crate::dbg_if_ne!($val, $type, |a, b| ::approx::abs_diff_ne!(a, b, $($arg)*))
    };

    ($val:expr, $type:tt) => {
        dbg_if_abs_diff_ne!($val, $type,)
    };
}

/// Calls [std::dbg] if the float argument is not equal to its prior value.
///
/// ```rust
/// use not_again::dbg_if_ulps_ne;
/// fn f(x: f32) -> f32 {
///     dbg_if_ulps_ne!(x, f32) + 0.1
/// }
/// f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
/// f(1.0); // No output.
/// f(1.1); // Outputs: [src/lib.rs:58:9] x = 1.1
/// ```
///
/// # Arguments
///
/// Accepts arguments that [approx::ulps_ne] accept.
/// ```rust
/// use not_again::dbg_if_ulps_ne;
/// fn f(x: f32) -> f32 {
///     dbg_if_ulps_ne!(x, f32, epsilon = 1.0)
/// }
/// f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
/// f(1.5); // No output.
/// f(2.0); // No output.
/// f(2.1); // Outputs: [src/lib.rs:58:9] x = 2.1
/// ```
#[cfg(feature = "float")]
#[macro_export]
macro_rules! dbg_if_ulps_ne {
    ($val:expr, $type:tt, $($arg:tt)*) => {
        $crate::dbg_if_ne!($val, $type, |a, b| ::approx::ulps_ne!(a, b, $($arg)*))
    };

    ($val:expr, $type:tt) => {
        dbg_if_ulps_ne!($val, $type,)
    };
}

#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
#[cfg(not(feature = "float"))]
#[macro_export]
macro_rules! static_atomic_float {
    ($name:ident: $type:tt) => {
        compile_error!(
            "Feature \"float\" must be enabled on \"not_again\" crate to use atomic floats."
        );
    };
}
