pub fn abc(a: f64, b: f64, c: f64) -> Result<Solution, &'static str> {
    let squarerooted = b * b - 4.0 * a * c;
    if squarerooted < 0.0 {
        return Err(NO_SOLUTION);
    }
    if squarerooted == 0.0 {
        return Ok(Solution {
            x1: -b / (2.0 * a),
            x2: f64::NAN
        });
    }
    let x1 = (-b + squarerooted.sqrt()) / (2.0 * a);
    let x2 = (-b - squarerooted.sqrt()) / (2.0 * a);
    Ok(Solution { x1, x2 })
}

pub fn pq(p: f64, q: f64) -> Result<Solution, &'static str> {
    abc(1.0, p, q)
}

// Errors
pub const NO_SOLUTION: &str = "b² - 4ac is negative and thus can't be squarerooted";

// Solution
#[derive(Debug)]
pub struct Solution {
    pub x1: f64,
    pub x2: f64
}