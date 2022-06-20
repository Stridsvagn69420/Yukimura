//! # Math
//! Collection of math-related functions.

#[cfg(feature = "quadratic")]
/// Quadratic equations
/// 
/// This module contains the quadratic formula for solving quadratic equations.
/// 
/// # Examples
/// 
/// ```
/// use yukimura::math::quadratic;
/// 
/// let solution = quadratic::abc(1.0, -6.0, 9.0).unwrap();
/// println!("{}", solution);
/// ```
pub mod quadratic;