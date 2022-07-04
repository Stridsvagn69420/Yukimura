//! # Base
//! 
//! Module for basic math functions.

/// Greatest Commom Divisor
///
/// Gets the GCD of two numbers.
/// 
/// # Examples
/// ```
/// use yukimura::base;
/// let result = base::gcd(72, 666);
/// assert_eq!(result, 18);
/// ```
pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}


/// Lowest Common Multiple
/// 
/// Gets the LCM of two numbers.
/// It's literally just `a * b / gcd(a, b)`. That's the entire source code.
/// Fucking amazing!
/// 
/// # Examples
/// ```
/// use yukimura::base;
/// let result = base::lcm(20, 35);
/// assert_eq!(result, 140);
/// ```
pub fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
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