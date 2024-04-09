use crate::{Params, F};

pub fn solve(params: &Params) -> Vec<(F, F)> {
    // preparation
    let mut df = params.df0.clone();
    df.push(0.);
    let f0 = if params.df0.is_empty() {
        0.
    } else {
        params.df0[0]
    };
    let mut solution = vec![(params.x0, f0)];

    // iterate to get solution
    for i in 1..=params.iterations {
        solve_step(i, &mut df, params);
        solution.push((params.x0 + params.dx * i as F, df[0]));
    }

    solution
}

fn solve_step(i: usize, df: &mut Vec<F>, Params { phi, x0, dx, .. }: &Params) {
    // solve highest order
    *df.last_mut().unwrap() = phi(x0 + dx * i as F, *dx, &df) * dx.powi(df.len() as i32 - 1);

    // solve rest iteratively
    for n in 0..df.len() - 1 {
        df[n] = df[n + 1] + df[n]
    }
}
