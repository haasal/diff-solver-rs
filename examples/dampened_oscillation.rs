use diff_solve::{save_to_file, wrappers::solve_linear};

fn main() {
    let solution = solve_linear(
        vec![1., 0.1, 1.],
        vec![1., 1.],
        |x| 0.5 * (x * 5.).sin(),
        0.,
        200.,
        1000000,
    );
    save_to_file(solution, "data.dat");
}
