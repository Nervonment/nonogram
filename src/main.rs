use nonogram::{problem::Problem, solver::Solver, solver_1010::Solver1010};

fn main() {
    // 10 * 10 nonogram
    let problem = Problem {
        col_info: vec![
            vec![1, 1, 4],
            vec![4],
            vec![6, 2],
            vec![1, 1],
            vec![5],
            vec![1, 3],
            vec![1, 1, 6],
            vec![4],
            vec![5],
            vec![4],
        ],
        row_info: vec![
            vec![1, 3, 2],
            vec![3, 2],
            vec![1, 1, 1, 1, 2],
            vec![3, 3],
            vec![1, 1, 3],
            vec![1, 2],
            vec![3, 3],
            vec![2, 2],
            vec![3, 2],
            vec![3, 1],
        ],
    };

    // 20 * 20 nonograms are too complex for this algorithm now.
    //
    // let problem = Problem {
    //     col_info: vec![
    //         vec![3, 7],
    //         vec![2, 6],
    //         vec![1, 4, 7],
    //         vec![1, 1, 5],
    //         vec![2, 6, 5],
    //         vec![3, 2, 1],
    //         vec![2, 1, 3, 3],
    //         vec![4, 1, 3],
    //         vec![4, 2, 4],
    //         vec![6, 3, 4],
    //         vec![1, 3, 6],
    //         vec![3, 2, 4, 3],
    //         vec![1, 1, 1, 1, 2],
    //         vec![2, 6, 2],
    //         vec![6, 1, 1],
    //         vec![1, 3, 4, 1, 5],
    //         vec![1, 3, 11],
    //         vec![1, 1, 9],
    //         vec![2, 2, 7],
    //         vec![2, 1, 4],
    //     ],
    //     row_info: vec![
    //         vec![3, 3, 5],
    //         vec![6, 1, 2],
    //         vec![4, 3, 1, 1],
    //         vec![4, 1, 1],
    //         vec![6, 5],
    //         vec![3, 1, 1],
    //         vec![1, 1, 2, 2, 1],
    //         vec![1, 1, 1, 2],
    //         vec![5, 1, 2, 3],
    //         vec![3, 6, 6],
    //         vec![1, 2, 1, 1, 4],
    //         vec![1, 7],
    //         vec![2, 3],
    //         vec![1, 1, 2, 6],
    //         vec![3, 5, 4],
    //         vec![5, 5, 5],
    //         vec![12, 5],
    //         vec![5, 1, 2, 4],
    //         vec![5, 8],
    //         vec![5, 2, 3],
    //     ],
    // };

    println!("columns: {:?}", problem.col_info);
    println!("rows: {:?}", problem.row_info);

    let mut solver = Solver1010::new(&problem);
    let solution = solver.solve();

    if solution.is_some() {
        println!("{}", solution.unwrap());
    }
}
