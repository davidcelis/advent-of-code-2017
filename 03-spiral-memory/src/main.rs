extern crate nalgebra as na;

use std::cmp;
use na::DMatrix;

fn main() {
    let input = 325489;

    println!("Part 1: {}", solve(input));
}

fn solve(input: usize) -> usize {
    let size = (input as f64).sqrt().ceil() as usize;
    let size = if size % 2 == 0 { size + 1 } else { size };
    let matrix = generate_matrix(size);

    calculate_steps_from(input, matrix)
}

fn calculate_steps_from(address: usize, matrix: DMatrix<usize>) -> usize {
    let vec_position = matrix.iter().position(|&n| n == address).unwrap();
    let (row, column) = matrix.vector_to_matrix_index(vec_position);
    let goal = (matrix.nrows() / 2) as usize;

    let rows = cmp::max(row, goal) - cmp::min(row, goal);
    let cols = cmp::max(column, goal) - cmp::min(column, goal);

    rows + cols
}

fn generate_matrix(size: usize) -> DMatrix<usize> {
    DMatrix::from_fn(size, size, |i,j| {
        let x = cmp::min(cmp::min(i, j), cmp::min(size - 1 - i, size - 1 - j));

        if i <= j {
            return (size - 2 * x) * (size - 2 * x) - (i - x) - (j - x);
        } else {
            return (size - 2 * x - 2) * (size - 2 * x - 2) + (i - x) + (j - x);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part_1() {
        assert_eq!(0,  solve(1));
        assert_eq!(3,  solve(12));
        assert_eq!(2,  solve(23));
        assert_eq!(31, solve(1024));
    }
}
