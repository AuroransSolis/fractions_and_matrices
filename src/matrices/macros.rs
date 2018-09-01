//! Definition for the `matrix!` and `augmented_matrix!` macros.

use matrices::base::Alignment::{ColumnAligned, RowAligned};

/// Allows the user to create a matrix in a manner visually similar to an actual matrix. Panics if
/// any row does not have the same number of elements as the rest.
/// # Example
/// ```rust
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
/// let foo = matrix![
///     0 1 2;
///     3 4 5;
///     6 7 8
/// ];
/// let bar = Matrix::new_from_vec((3, 3), vec![0, 1, 2, 3, 4, 5, 6, 7, 8], RowAligned).unwrap();
/// assert_eq!(foo, bar);
/// ```
/// # Panics
/// ```should_panic
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
/// let foo = matrix![
///     0 1 2;
///     3 4;
///     5 6 7
/// ];
/// ```
#[macro_export]
macro_rules! matrix {
    ($($($val:expr) *);*) => {{
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        $(
            let row = [$($val),*];
            for l in &lens {
                if row.len() != *l {
                    panic!("A row/column had an incorrect number of elements.");
                }
            }
            lens.push(row.len());
            matr.extend_from_slice(&row);
        )*
        use $crate::matrices::base::Alignment::RowAligned;
        let mut res = Matrix::new((matr.len() / lens[0], lens[0]), RowAligned);
        res.set_matrix(matr);
        res
    }};
}

/// Allows the user to create an augmented matrix in a manner visually similar to an actual
/// augmented matrix. Panics if any row does not have the same number of elements as the rest.
/// # Example
/// ```rust
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
/// let foo = augmented_matrix![
///     0 1 => 2;
///     3 4 => 5;
///     6 7 => 8
/// ];
/// let bar = AugmentedMatrix::new_from_vec((3, 3), vec![0, 1, 2, 3, 4, 5, 6, 7, 8], RowAligned)
///     .unwrap();
/// assert_eq!(foo, bar);
/// ```
/// # Panics
/// ```should_panic
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
/// let foo = augmented_matrix![
///     0 1 2 => 3;
///     4 5 => 6;
///     7 8 => 9
/// ];
/// ```
#[macro_export]
macro_rules! augmented_matrix {
    ($($($val:expr) * => $sol_val:expr);*) => {{
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        let mut solution_column = Vec::new();
        $(
            let row = [$($val),*];
            solution_column.push($sol_val);
            for l in &lens {
                if row.len() != *l {
                    panic!("A row/column had an incorrect number of elements.");
                }
            }
            lens.push(row.len());
            matr.extend_from_slice(&row);
        )*
        if solution_column.len() != matr.len() / lens[0] {
            panic!("Solution column had an incorrect number of elements.");
        } else {
            matr.push(solution_column[solution_column.len() - 1]);
            for r in (0..solution_column.len()).rev().skip(1) {
                matr.insert((r + 1) * lens[0], solution_column[r]);
            }
            use $crate::matrices::base::Alignment::RowAligned;
            let mut res = AugmentedMatrix::new((solution_column.len(), lens[0] + 1), RowAligned);
            res.set_matrix(matr);
            res
        }
    }};
}

/// Allows the user to get a window into a matrix or augmented matrix. There are four distinct ways
/// of using this macro:
/// - Getting a single row or column (`window!(matrix, r: n)` or `window!(matrix, c: n)`)
/// - Getting part of a single row or column (`window!(matrix, (r, c_start..c_end))`
/// or `window!(matrix, (r_start..r_end, c))`)
/// - Getting multiple rows or columns (`window!(matrix, r: r_start..r_end)` or
/// `window!(matrix, c: c_start..c_end)`)
/// - Getting parts of multiple rows or columns
/// (`window!(matrix, (r_start..r_end, c_start..c_end))`)
///
/// Notes:
/// - This macro always returns a Matrix<T>.
/// - When using augmented matrices, the solution column is not included in the window matrix
/// when the first or third methods are used. It's a bit inconvenient, yes, but (parts of) the
/// solution column can be included if the second or fourth methods are used.
/// # Examples
/// ```rust
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
/// let matrix: Matrix<u32> = matrix![
///      0  1  2  3;
///      4  5  6  7;
///      8  9 10 11;
///     12 13 14 15
/// ];
/// let window_single_row = window!(matrix, r: 1);
/// let wsr: Matrix<u32> = Matrix::new_from_vec((1, 4), vec![4, 5, 6, 7], RowAligned).unwrap();
/// assert_eq!(window_single_row, wsr);
/// let window_single_column = window!(matrix, c: 3);
/// let wsc: Matrix<u32> = Matrix::new_from_vec((1, 4), vec![3, 7, 11, 15], RowAligned).unwrap();
/// assert_eq!(window_single_column, wsc);
/// ```
/// ```rust
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{Matrix, AugmentedMatrix, Alignment::RowAligned};
/// let augmented_matrix: AugmentedMatrix<u32> = augmented_matrix![
///     1 0 0 => 0;
///     0 1 0 => 1;
///     0 0 1 => 2
/// ];
/// let window_partial_row = window!(augmented_matrix, (1, 1..4));
/// let wpr: Matrix<u32> = Matrix::new_from_vec((1, 3), vec![1, 0, 1], RowAligned)
///     .unwrap();
/// assert_eq!(window_partial_row, wpr);
/// let window_partial_column = window!(augmented_matrix, (2, 0..2));
/// let wpc: Matrix<u32> = Matrix::new_from_vec((1, 2), vec![0, 0], RowAligned).unwrap();
/// ```
/// ```rust
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
/// let matrix: Matrix<u32> = matrix![
///     0 0 0 0 1;
///     0 0 0 1 0;
///     0 0 1 0 0;
///     0 1 0 0 0;
///     1 0 0 0 0
/// ];
/// let rows = window!(matrix, r: (1..4));
/// let r: Matrix<u32> = Matrix::new_from_vec((3, 5),
///     vec![0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0], RowAligned).unwrap();
/// assert_eq!(rows, r);
/// let columns = window!(matrix, c: (3..5));
/// let c: Matrix<u32> = Matrix::new_from_vec((5, 2), vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0],
///     RowAligned).unwrap();
/// assert_eq!(columns, c);
/// ```
/// ```rust
/// # #[macro_use] extern crate fractions_and_matrices;
/// # use fractions_and_matrices::matrices::base::{Matrix, AugmentedMatrix, Alignment::RowAligned};
/// let augmented_matrix: AugmentedMatrix<u32> = augmented_matrix![
///     1 0 0 0 => 0;
///     0 1 0 0 => 1;
///     0 0 1 0 => 2;
///     0 0 0 1 => 3
/// ];
/// let window_1 = window!(augmented_matrix, (1..3, 1..6));
/// let w1: Matrix<u32> = Matrix::new_from_vec((2, 4), vec![1, 0, 0, 1, 0, 1, 0, 2],
///     RowAligned).unwrap();
/// assert_eq!(window_1, w1);
/// let window_2 = window!(augmented_matrix, (0..3, 0..3));
/// let w2: Matrix<u32> = Matrix::new_from_vec((3, 3), vec![1, 0, 0, 0, 1, 0, 0, 0, 1],
///     RowAligned).unwrap();
/// assert_eq!(window_2, w2);
/// ```
#[macro_export]
macro_rules! window {
    ($matrix:ident, r: $r:tt) => {
        if matrix.is_row_aligned() {
            use $crate::matrices::base::Alignment::RowAligned;
            Matrix::new_from_vec((1, $matrix.num_columns()), $matrix[$r].clone().into_vec(),
                RowAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::ColumnAligned;
            let mut vec = Vec::with_capacity($matrix.num_columns());
            for c in 0..$matrix.num_columns() {
                vec.push($matrix[c][$r].clone());
            }
            Matrix::new_from_vec((1, $matrix.num_columns()), vec, ColumnAligned).unwrap()
        }
    };
    ($matrix:ident, c: $c:tt) => {
        if matrix.is_column_aligned() {
            use $crate::matrices::base::Alignment::ColumnAligned;
            Matrix::new_from_vec(($matrix.num_rows(), 1), $matrix[$c].clone().into_vec(),
                ColumnAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::RowAligned;
            let mut vec = Vec::with_capacity($matrix.num_columns());
            for r in 0..$matrix.num_rows() {
                vec.push($matrix[r][$c].clone());
            }
            Matrix::new_from_vec(($matrix.num_rows(), 1), vec, RowAligned).unwrap()
        }
    };
    ($matrix:ident, ($r:tt, $c_start:tt..$c_end:tt)) => {
        if matrix.is_row_aligned() {
            use $crate::matrices::base::Alignment::RowAligned;
            Matrix::new_from_vec((1, $matrix.num_columns()),
                $matrix[$r][$c_start..$c_end].clone().into_vec(), RowAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::ColumnAligned;
            let mut vec = Vec::with_capacity($matrix.num_columns());
            for c in $c_start..$c_end {
                vec.push($matrix[c][$r].clone());
            }
            Matrix::new_from_vec((1, $c_end - $c_start), vec, ColumnAligned).unwrap()
        }
    };
    ($matrix:ident, ($r_start:tt..$r_end:tt, $c:tt)) => {
        if matrix.is_column_aligned() {
            use $crate::matrices::base::Alignment::ColumnAligned;
            Matrix::new_from_vec(($matrix.num_rows(), 1),
                $matrix[$c][$r_start..$r_end].clone().into_vec(), ColumnAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::RowAligned;
            let mut vec = Vec::with_capacity($matrix.num_columns());
            for r in $r_start..$r_end {
                vec.push($matrix[r][$c].clone());
            }
            Matrix::new_from_vec(($r_end - $r_start, 1), vec, RowAligned).unwrap()
        }
    };
    ($matrix:ident, r: ($r_start:tt..$r_end:tt)) => {
        if $matrix.is_row_aligned() {
            use $crate::matrices::base::Alignment::RowAligned;
            Matrix::new_from_vec(($r_end - $r_start, $matrix.num_columns()),
                $matrix[$r_start..$r_end].clone().into_vec(), RowAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::ColumnAligned;
            let mut vec = Vec::with_capacity(($r_end - $r_start) as usize * $matrix.num_columns());
            for r in $r_start..$r_end {
                for c in 0..$matrix.num_columns() {
                    vec.push($matrix[c][r].clone());
                }
            }
            Matrix::new_from_vec(($r_end - $r_start, $matrix.num_columns()), vec, ColumnAligned)
                .unwrap()
        }
    };
    ($matrix:ident, c: ($c_start:tt..$c_end:tt)) => {
        if $matrix.is_column_aligned() {
            use $crate::matrices::base::Alignment::ColumnAligned;
            Matrix::new_from_vec(($matrix.num_rows(), $c_end - $c_start),
                $matrix[$c_start..$c_end].clone().into_vec(), ColumnAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::RowAligned;
            let mut vec = Vec::with_capacity(($c_end - $c_start) as usize * $matrix.num_rows());
            for c in $c_start..$c_end {
                for r in 0..$matrix.num_rows() {
                    vec.push($matrix[r][c].clone());
                }
            }
            Matrix::new_from_vec(($matrix.num_rows(), $c_end - $c_start), vec, RowAligned).unwrap()
        }
    };
    ($matrix:ident, ($r_start:tt..$r_end:tt, $c_start:tt..$c_end:tt)) => {
        if $matrix.is_row_aligned() {
            use $crate::matrices::base::Alignment::RowAligned;
            let mut vec = Vec::new();
            for r in $r_start..$r_end {
                vec.extend_from_slice(&$matrix[r][$c_start..$c_end]);
            }
            Matrix::new_from_vec(($r_end - $r_start, $c_end - $c_start), vec, RowAligned).unwrap()
        } else {
            use $crate::matrices::base::Alignment::ColumnAligned;
            let mut vec = Vec::new();
            for c in $c_start..$c_end {
                vec.extend_from_slice(&$matrix[c][$r_start..$r_end]);
            }
            Matrix::new_from_vec(($r_end - $r_start, $c_end - $c_start), vec, ColumnAligned)
                .unwrap()
        }
    };
}