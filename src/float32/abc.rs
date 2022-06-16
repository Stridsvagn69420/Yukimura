pub fn abc(a: f32, b: f32, c: f32) -> Result<Solution, &'static str> {
    let tobesquared = b * b - 4.0 * a * c;
    if tobesquared < 0.0 {
        return Err(NO_SOLUTION);
    }
    let x1 = (-b + tobesquared.sqrt()) / (2.0 * a);
    let x2 = (-b - tobesquared.sqrt()) / (2.0 * a);
    return Ok(Solution { x1, x2 });
}

pub fn pq(p: f32, q: f32) -> Result<Solution, &'static str> {
    return abc(1.0, p, q)
}

// Errors
pub const NO_SOLUTION: &str = "4*a*c is negative and thus can't be squarerooted";

// Solution
#[derive(Debug)]
pub struct Solution {
    pub x1: f32,
    pub x2: f32
}