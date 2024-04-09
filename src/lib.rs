pub mod solver;
pub mod wrappers;

use std::fs::File;
use std::io::Write;
use std::path::Path;

pub type F = f64;

pub struct Params {
    x0: F,
    dx: F,
    df0: Vec<F>,
    iterations: usize,
    phi: Box<dyn Fn(F, F, &[F]) -> F>,
}

impl Params {
    /// Closure g(x, dx, df)
    pub fn new(
        x0: F,
        x1: F,
        mut df0: Vec<F>,
        phi: Box<dyn Fn(F, F, &[F]) -> F>,
        iterations: usize,
    ) -> Self {
        let dx = (x0 - x1).abs() / iterations as F;
        df0.iter_mut()
            .enumerate()
            .for_each(|(n, df)| *df *= dx.powi(n as i32));
        Self {
            x0,
            df0,
            phi,
            iterations,
            dx,
        }
    }

    pub fn dx(&self) -> F {
        self.dx
    }

    pub fn df0(&self) -> &[F] {
        &self.df0
    }
}

pub fn save_to_file<P: AsRef<Path>>(solution: Vec<(F, F)>, path: P) {
    let mut f = File::create(path).unwrap();
    for (x, fx) in solution.iter() {
        writeln!(f, "{} {}", x, fx).unwrap();
    }
}
