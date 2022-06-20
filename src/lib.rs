//! # Yukimura
//! A program and library written in Rust for mathematical things I learned in Maths and Physics
//! 
//! It's currently still WIP.


pub mod base;

#[cfg(feature = "fracs")]
pub mod fracs;

#[cfg(feature = "math")]
pub mod math;

#[cfg(feature = "physics")]
pub mod physics;