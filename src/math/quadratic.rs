/// ABC formula
/// 
/// This function returns the solution of a quadratic equation as a tuple.
/// Note that the values can be `f32::NaN`
/// 
/// # Examples
/// ```
/// use yukimura::math::quadratic;
/// 
/// let solution = quadratic::abc(1.0, -6.0, 9.0);
/// println!("x1: {}", solution.0);
/// println!("x2: {}", solution.1);
/// ```
pub fn abc(a: f32, b: f32, c: f32) -> (f32, f32) {
    let squarerooted = b * b - 4.0 * a * c;
    if squarerooted < 0.0 {
        return (f32::NAN, f32::NAN);
    }
    if squarerooted == 0.0 {
        return (-b / (2.0 * a),f32::NAN);
    }
    let x1 = (-b + squarerooted.sqrt()) / (2.0 * a);
    let x2 = (-b - squarerooted.sqrt()) / (2.0 * a);
    (x1, x2)
}


/// PQ formula
/// 
/// This essentially is just the [ABC](abc)-formula but `a` is always 1.
/// 
/// # Examples
/// ```
/// use yukimura::math::quadratic;
/// 
/// let solutionABC = quadratic::abc(1.0, -6.0, 9.0);
/// let solutionPQ = quadratic::pq(-6.0, 9.0);
/// assert_eq!(solutionABC.0, solutionPQ.0);
/// ```
pub fn pq(p: f32, q: f32) -> (f32, f32) {
    abc(1.0, p, q)
}