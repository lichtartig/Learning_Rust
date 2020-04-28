/// The object of this exercise is to write an algorithm that searches for zeros in a MxN matrix and
/// if it finds them replaces the entire row and column by zeros.
use ndarray::prelude::*;
use ndarray::{Array, Ix2};

fn main() {
    // define an example array
    let mut example: Array<i32, Ix2> = array![[1, 0, 2, 3], [4, 5, 0, 6], [7, 8, 9, 10]];
    zero_row_column(&mut example);
    println!("{}", example);
}

/// This is the function that performs the task. Basically we just scan over the array elements and
/// everytime we find a zero entry, we keep it's coord{inates. Once all zeros have been identified,
/// we set the corresponding rows and columns to zero.
fn zero_row_column(matrix: &mut Array<i32, Ix2>) {
    // find all rows and columns that contain a zero element
    let shape = matrix.shape();
    let mut zero_row_indices: Vec<usize> = Vec::new();
    let mut zero_col_indices: Vec<usize> = Vec::new();
    for i in 0..shape[0] {
        for j in 0..shape[1] {
            if matrix[[i, j]] == 0 {
                zero_row_indices.push(i);
                zero_col_indices.push(j);
            }
        }
    }

    // set all rows to zero that contain a zero
    for r in &zero_row_indices {
        let row = matrix.row_mut(*r);
        for element in row {
            *element = 0;
        }
    }

    // set columns to zero that contain a zero
    for c in &zero_col_indices {
        let col = matrix.column_mut(*c);
        for element in col {
            *element = 0;
        }
    }
}
