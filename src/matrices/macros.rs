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