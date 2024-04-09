use diff_solve::{solver::solve, Params, F};

fn main() {
    // f'(x) = x -> f(x) = 2 * sin(x)/x (should be pi)
    let phi = |x: F, dx: F, _df: &[F]| 2. * x.sin() / x;
    let params = Params::new(0., 1000., vec![0.], Box::new(phi), 10000000);
    let solution = solve(&params);
    println!("Pi = {:?}", solution.last().unwrap().1);
    // save_to_file(solution, "data.dat");
}
