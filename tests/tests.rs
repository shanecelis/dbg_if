
fn capture_stderr<F: FnOnce()>(f: F) -> String {
    use gag::BufferRedirect;
    use std::io::Read;
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
    r.replace_all(input.trim(), "").to_string()
}

mod test_dbg {
    use not_again::*;
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
            dbg_if_hash_ne!({
                *x += 1;
                *x
            });
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
    use not_again::*;
    use super::*;
    use approx::relative_eq;

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
    fn test_dbg_if_ne_f32_with_function_name() {
        fn my_ne(a: f32, b: f32) -> bool {
            ::approx::relative_ne!(a, b, epsilon = 0.1)
        }
        fn f(x: f32) {
            dbg_if_ne!(x, f32, my_ne);
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

mod nocapture {

    #[test]
    fn test_dbg_if_ne() {
        use not_again::dbg_if_ne;
        fn f(x: u8) -> u8 {
            dbg_if_ne!(x, u8) + 1
        }
        f(1);
        f(1);
    }
    #[test]
    fn test_dbg_if_hash_ne() {
        use not_again::dbg_if_hash_ne;
        let mut s: String = "hello".into();
        fn f(x: &mut String) {
            dbg_if_hash_ne!(x);
        }
        f(&mut s); // Outputs: [src/lib.rs:37:9] x = "hello"
        f(&mut s); // No output.
        s.push('!');
        f(&mut s); // Outputs: [src/lib.rs:37:9] x = "hello!"
    }

    #[test]
    fn test_dbg_if_ne_closure() {
        use not_again::dbg_if_ne;
        for i in 0..=20 {
            dbg_if_ne!(i, i8, |last_value: i8, new_value: i8| (new_value - last_value).abs() >= 10);
        }
        // Outputs: [src/lib.rs:58:9] i = 0
        // Outputs: [src/lib.rs:58:9] i = 10
        // Outputs: [src/lib.rs:58:9] i = 20
    }

    #[cfg(feature = "float")]
    mod float {

        #[test]
        fn test_dbg_if_relative_ne() {
            use not_again::dbg_if_relative_ne;
            fn f(x: f32) -> f32 {
                dbg_if_relative_ne!(x, f32) + 0.1
            }
            f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
            f(1.0); // No output.
            f(1.1); // Outputs: [src/lib.rs:58:9] x = 1.1
        }

        #[test]
        fn test_dbg_if_relative_ne_with_args() {
            use not_again::dbg_if_relative_ne;
            fn f(x: f32) -> f32 {
                dbg_if_relative_ne!(x, f32, epsilon = 1.0)
            }
            f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
            f(1.5); // No output.
            f(2.0); // No output.
            f(2.1); // Outputs: [src/lib.rs:58:9] x = 2.1
        }

        #[test]
        fn test_dbg_if_abs_diff_ne() {
            use not_again::dbg_if_abs_diff_ne;
            fn f(x: f32) -> f32 {
                dbg_if_abs_diff_ne!(x, f32) + 0.1
            }
            f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
            f(1.0); // No output.
            f(1.1); // Outputs: [src/lib.rs:58:9] x = 1.1
        }

        #[test]
        fn test_dbg_if_abs_diff_ne_with_args() {
            use not_again::dbg_if_abs_diff_ne;
            fn f(x: f32) -> f32 {
                dbg_if_abs_diff_ne!(x, f32, epsilon = 1.0)
            }
            f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
            f(1.5); // No output.
            f(2.0); // No output.
            f(2.1); // Outputs: [src/lib.rs:58:9] x = 2.1
        }

        #[test]
        fn test_dbg_if_ulps_ne() {
            use not_again::dbg_if_ulps_ne;
            fn f(x: f32) -> f32 {
                dbg_if_ulps_ne!(x, f32) + 0.1
            }
            f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
            f(1.0); // No output.
            f(1.1); // Outputs: [src/lib.rs:58:9] x = 1.1
        }

        #[test]
        fn test_dbg_if_ulps_ne_with_args() {
            use not_again::dbg_if_ulps_ne;
            fn f(x: f32) -> f32 {
                dbg_if_ulps_ne!(x, f32, epsilon = 1.0)
            }
            f(1.0); // Outputs: [src/lib.rs:58:9] x = 1.0
            f(1.5); // No output.
            f(2.0); // No output.
            f(2.1); // Outputs: [src/lib.rs:58:9] x = 2.1
        }
    }
}
