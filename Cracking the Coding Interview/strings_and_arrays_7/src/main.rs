/// The object of this exercise is to rotate an NxN-matrix by 90°.
/// The second question is if this can be done in place.
use ndarray::prelude::*;
use ndarray::{Array, Ix2};

/// Rotating a matrix 90° just means transposing it and then inverting the order of its rows.
/// This is of course an operation in place, so we are done.
fn main() {
    // define an example matrix
    let mut example: Array<i32, Ix2> = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    // rotate it by 90°
    example = example.t().to_owned();
    example.invert_axis(Axis(1));
    println!("{}", example);
}
