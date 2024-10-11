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

    let mut col_info = vec![vec![]; width];
    let mut row_info = vec![vec![]; height];

    for col in 0..width {
        let mut num = 0;
        for row in 0..height {
            if grid[row][col] {
                num += 1;
            } else if num > 0 {
                col_info[col].push(num);
                num = 0;
            }
        }
        if num > 0 {
            col_info[col].push(num);
        }
    }
    for row in 0..height {
        let mut num = 0;
        for col in 0..width {
            if grid[row][col] {
                num += 1;
            } else if num > 0 {
                row_info[row].push(num);
                num = 0;
            }
        }
        if num > 0 {
            row_info[row].push(num);
        }
    }

    Problem { col_info, row_info }
}
