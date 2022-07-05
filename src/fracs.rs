//! # Warning: ARBORTED
//!
//! I'm only keeping this in here so that I don't lose my progress of it.
//! This was supposed to integrate fractions, because you can solve literally anything with them.
//! However, it turned out to be way too complicated and I decided to just abort it, at least for now.

use std::fmt::{Display, Debug, Formatter, Result};
use crate::base;

#[derive(Clone)]
pub struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    pub fn new(num: i32, denom: i32) -> Fraction {
        Fraction {
            numerator: num,
            denominator: denom,
        }
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let g = self.simplify();
        write!(f, "{}/{}", g.numerator, g.denominator)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let g = self.simplify();
        write!(f, "{}/{}", g.numerator, g.denominator)
    }
}

impl Frac for Fraction {
    /// New Fraction
    /// 
    /// Creates a new Fraction with the given numerator and denominator
    fn new(num: i32, denom: i32) -> Self {
        Fraction {
            numerator: num,
            denominator: denom,
        }
    }

    /// Numerator
    /// 
    /// Returns the numerator of the Fraction.
    fn numerator(&self) -> i32 {
        self.numerator
    }

    /// Denominator
    /// 
    /// Returns the denominator of the Fraction.
    fn denominator(&self) -> i32 {
        self.denominator
    }
    /// Return simplified Fraction
    /// 
    /// Returns the current [Fraction] in simplest form.
    fn simplify(&self) -> Fraction {
        let gcd = base::gcd(self.numerator, self.denominator);
        Fraction {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd
        }
    }
    /// Simplify Fraction
    /// 
    /// Simplifies the Fraction by dividing the numerator and denominator by the GCD.
    /// No new Fraction is created, but the numerator and denominator of the current item are changed.
    fn simplify_self(&mut self) {
        let gcd = base::gcd(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
    }

    /// Add Fraction
    /// 
    /// Adds the given Fraction with the current Fraction and returns the result.
    fn add(&self, other: &Self) -> Self {
        Fraction {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * other.denominator
        }
    }

    /// Add Fraction to current Fraction
    ///
    /// Adds the given Fraction with the current Fraction.
    /// No new Fraction is created, but the numerator and denominator of the current item are changed.
    fn add_self(&mut self, other: &Self) {
        self.numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        self.denominator = self.denominator * other.denominator;
    }

    /// Subtract Fraction
    /// 
    /// Subtracts the given Fraction with the current Fraction and returns the result.
    fn sub(&self, other: &Self) -> Self {
        Fraction {
            numerator: self.numerator * other.denominator - other.numerator * self.denominator,
            denominator: self.denominator * other.denominator
        }
    }

    /// Subtract Fraction from current Fraction
    /// 
    /// Subtracts the given Fraction with the current Fraction.
    /// No new Fraction is created, but the numerator and denominator of the current item are changed.
    fn sub_self(&mut self, other: &Self) {
        self.numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        self.denominator = self.denominator * other.denominator;
    }

    /// Multiply Fraction
    /// 
    /// Multiplies the given Fraction with the current Fraction and returns the result.
    fn mul(&self, other: &Self) -> Self {
        Fraction {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator
        }
    }

    /// Multiply Fraction with current Fraction
    /// 
    /// Multiplies the given Fraction with the current Fraction.
    /// No new Fraction is created, but the numerator and denominator of the current item are changed.
    fn mul_self(&mut self, other: &Self) {
        self.numerator = self.numerator * other.numerator;
        self.denominator = self.denominator * other.denominator;
    }

    /// Divide Fraction
    /// 
    /// Divides the given Fraction with the current Fraction and returns the result.
    fn div(&self, other: &Self) -> Self {
        Fraction {
            numerator: self.numerator * other.denominator,
            denominator: self.denominator * other.numerator
        }
    }

    /// Divide Fraction with current Fraction
    /// 
    /// Divides the given Fraction with the current Fraction.
    /// No new Fraction is created, but the numerator and denominator of the current item are changed.
    fn div_self(&mut self, other: &Self) {
        self.numerator = self.numerator * other.denominator;
        self.denominator = self.denominator * other.numerator;
    }
}

/// Trait for fractions
/// 
/// This trait allows you to write a custom data type that still supports the basic operations of a fraction.
/// Every function in this crate allows every datatype that implements this trait, although I don't expect any other datatypes to be used.
pub trait Frac {
    /// Creates new instance of Frac
    fn new(num: i32, denom: i32) -> Self;

    /// Get the numerator of the fraction
    fn numerator(&self) -> i32;

    /// Get the denominator of the fraction
    fn denominator(&self) -> i32;

    /// Simplify the fraction (returning a new one)
    fn simplify(&self) -> Self;

    /// Simplify the fraction (changing the current one)
    fn simplify_self(&mut self);
    
    /// Adds two fractions and returns a new one
    fn add(&self, other: &Self) -> Self;

    /// Adds two fractions and changes the current one
    fn add_self(&mut self, other: &Self);

    /// Subtracts two fractions and returns a new one
    fn sub(&self, other: &Self) -> Self;

    /// Subtracts two fractions and changes the current one
    fn sub_self(&mut self, other: &Self);

    /// Multiplies two fractions and returns a new one
    fn mul(&self, other: &Self) -> Self;

    /// Multiplies two fractions and changes the current one
    fn mul_self(&mut self, other: &Self);

    /// Divides two fractions and returns a new one
    fn div(&self, other: &Self) -> Self;

    /// Divides two fractions and changes the current one
    fn div_self(&mut self, other: &Self);
}
