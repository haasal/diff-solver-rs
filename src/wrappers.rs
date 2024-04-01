use crate::{F, Params};
use crate::solver::solve;

/// Solve linear DE. Params are given as prefactors to derivatives.
/// At index 0 is the 0th order derivative parameter.
/// The equation has the form (in order 2):
/// `a_2 * f''(x) + a_1 * f'(x) + a_0 f(x) + g(x) = 0`
///
/// # Arguments
///
/// * `params`: linear parameters
/// * `df0`: initial conditions
/// * `g`: inhomogeneity
///
/// returns: Solution
pub fn solve_linear(params: Vec<F>, df0: Vec<F>, g: fn(x:F) -> F, x0: F, x1: F, iterations: usize) -> Vec<(F, F)> {
    if df0.len() != params.len()-1 {
        panic!("params must have matching length with df0")
    }

    let phi = move |x: F, dx: F, df: &[F]| -> F {
        if df.len() != params.len() {
            panic!("df must have same length as params")
        }

        let mut sum = 0.;
        for i in 0..params.len()-1 {
            sum += params[i] * df[i] / dx.powi(i as i32);
        };
        sum /= params.last().unwrap();

        -sum - g(x)
    };

    let params = Params::new(x0, x1, df0, Box::new(phi), iterations);
    solve(&params)
}