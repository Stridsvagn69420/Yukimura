//! # Yukimura
//! A program and library written in Rust for mathematical things I learned in Maths and Physics
//! 
//! It's currently still WIP.

/// Bases
/// 
/// Bases that either belong to both math and physics or none of them.
/// Basically the `other` of this library.
pub mod base;

#[cfg(feature = "math")]
/// Math
/// 
/// Collection of math-related functions.
pub mod math;

#[cfg(feature = "physics")]
/// Physics
/// 
/// Collection of physics-related functions.
pub mod physics;

#[cfg(feature = "printer")]
/// Stateless Printer
/// 
/// Lightweight Printer. Wrapper for various printing methods and macros.
pub mod printer;

#[cfg(feature = "fracs")]
/// Attemped Fractions
/// 
/// **THIS IS ABORTED RELIC CODE!!!**
/// **DO NOT USE THIS FOR PRODUCTION**
pub mod fracs;