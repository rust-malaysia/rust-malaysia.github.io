#![feature(test)]

extern crate test;

#[test]
fn test_add() {
    assert_eq!(add(1, 1), 2);
    assert_eq!(add(1, 1), 3);
}

#[test]
#[ignore]
fn test_huh() {
    assert_eq!(add(1, 1), 3);
}

#[test]
#[ignore]
fn test_rust_my() {
    assert!("rust malaysia".contains("rust"));
}

/// Divide x by y.
///
/// # Examples
///
/// ```
/// # let x = -1;
/// # let y = -1;
/// assert_eq!(tests::div(x, y), 1);
/// ```
pub fn div(x: i32, y: i32) -> i32 {
    x / y
}

#[test]
#[ignore]
fn test_div() {
    assert_eq!(div(2, 1), 2);
    assert_eq!(div(1, 1), 1);
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_sad() {
    assert_eq!(div(1, 0), 0);
}

/// Multiply x with y.
///
/// # Examples
///
/// ```
/// assert_eq!(tests::mul(2, 3), 6);
/// ```
pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

use test::Bencher;

#[bench]
fn bench_add_fn(b: &mut Bencher) {
    b.iter(|| add(1, 2));
}

#[bench]
fn bench_add_native(b: &mut Bencher) {
    let x = test::black_box(1);
    let y = test::black_box(2);
    b.iter(|| x + y);
}
