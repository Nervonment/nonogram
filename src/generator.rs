use rand::random;

use crate::problem::Problem;

pub fn random_nonogram(width: usize, height: usize, density: f64) -> Problem {
    let mut grid = vec![vec![false; width]; height];

    for row in &mut grid {
        for cell in row {
            if random::<f64>() < density {
                *cell = true;
            }
        }
    }

    Problem::from(grid)
}
