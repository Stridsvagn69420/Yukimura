//! # Base
//! 
//! Module for basic math functions.

/// Greatest Commom Divisor
///
/// Gets the GCD of two numbers.
/// This is used for Yukimura's Fraction datatype.
pub fn greatest_common_divisor(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}

/// Add numbers in range
/// 
/// Adds numbers together from a specific range,
/// e.g. `from = 1` and `to = 100` would do `1+2+3+...+98+99+100`.
/// 
/// # Examples
/// ```
/// use yukimura::base;
/// let result = base::add_range(1, 100);
/// assert_eq!(result, 5050);
/// ```
pub fn add_range(from: i64, to: i64) -> i64 {
    let mut result = 0;
    for n in from..(to+1) {
        result += n;
    };
    result
}