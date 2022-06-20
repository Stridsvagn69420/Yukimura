use std::fmt;

/// ABC formula
/// 
/// This function returns the solution of a quadratic equation.
/// 
/// # Examples
/// ```
/// use yukimura::math::quadratic;
/// 
/// let solution = quadratic::abc(1.0, -6.0, 9.0).unwrap();
/// println!("{}", solution);
/// ```
pub fn abc(a: f32, b: f32, c: f32) -> Result<Solution, &'static str> {
    let squarerooted = b * b - 4.0 * a * c;
    if squarerooted < 0.0 {
        return Err(NO_SOLUTION);
    }
    if squarerooted == 0.0 {
        return Ok(Solution {
            x1: -b / (2.0 * a),
            x2: f32::NAN
        });
    }
    let x1 = (-b + squarerooted.sqrt()) / (2.0 * a);
    let x2 = (-b - squarerooted.sqrt()) / (2.0 * a);
    Ok(Solution { x1, x2 })
}


/// PQ formula
/// 
/// This essentially is just the [ABC](abc)-formula but `a` is always 1.
/// 
/// # Examples
/// ```
/// use yukimura::math::quadratic;
/// 
/// let solutionABC = quadratic::abc(1.0, -6.0, 9.0).unwrap();
/// let solutionPQ = quadratic::pq(-6.0, 9.0).unwrap();
/// assert_eq!(solutionABC.x1, solutionPQ.x1);
/// ```
pub fn pq(p: f32, q: f32) -> Result<Solution, &'static str> {
    abc(1.0, p, q)
}

// Errors
pub const NO_SOLUTION: &str = "b²-4ac is negative and thus can't be squarerooted";
pub const WRONG_INPUT: &str = "Input is not a number";

/// Quadratic Solution
/// 
/// Just a little struct for returning the solutions of a quadratic equation.
/// Note that `x2` can be `NAN` if the equation has only one solution.
pub struct Solution {
    pub x1: f32,
    pub x2: f32
}

impl fmt::Debug for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x2.is_nan() {
            write!(f, "x1: {}", self.x1)
        } else {
            write!(f, "x1: {}, x2: {}", self.x1, self.x2)
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x2.is_nan() {
            write!(f, "x1: {}", self.x1)
        } else {
            write!(f, "x1: {}, x2: {}", self.x1, self.x2)
        }
    }
}