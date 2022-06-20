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