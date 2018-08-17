//! Provides methods for popping/removing/pushing/inserting row/rows/column/columns to matrices
//! and augmented matrices.

use std::ops::Range;

use matrices::base::{AugmentedMatrix, Matrix, MatrixError};

impl<T> Matrix<T> {
    /// Remove the last column from a matrix, like `pop()` for vectors.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// let mut foo = matrix![
    ///     0 1 2;
    ///     3 4 5
    /// ];
    /// foo.pop_column();
    /// let bar = matrix![
    ///     0 1;
    ///     3 4
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    pub fn pop_column(&mut self) {
        if self.is_column_aligned() {
            for _ in 0..self.rows {
                drop(self.matrix.pop());
            }
            self.rows -= 1;
        } else {
            for c in (1..self.num_rows() + 1).rev() {
                self.matrix.remove(self.columns * c - 1);
            }
            self.columns -= 1;
        }
    }

    /// Removes a column from a matrix. Panics on out of bounds.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// let mut foo = matrix![
    ///      0  1  2  3  4;
    ///      5  6  7  8  9;
    ///     10 11 12 13 14;
    ///     15 16 17 18 19;
    ///     20 21 22 23 24
    /// ];
    /// foo.remove_column(2);
    /// let bar = matrix![
    ///      0  1  3  4;
    ///      5  6  8  9;
    ///     10 11 13 14;
    ///     15 16 18 19;
    ///     20 21 23 24
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    pub fn remove_column(&mut self, column: usize) {
        assert!(column <= self.num_columns());
        if column == self.num_columns() {
            self.pop_column();
            return;
        }
        if self.is_column_aligned() {
            self.matrix.drain(column * self.rows..(column + 1) * self.rows);
            self.rows -= 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                self.matrix.remove(r * self.columns + column);
            }
            self.columns -= 1;
        }
    }
}

impl<T> AugmentedMatrix<T> {
    /// Removes the last column from an augmented matrix, similarly to `pop()` for vectors.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    /// let mut foo = augmented_matrix![
    ///      0  1  2  3 => 0;
    ///      4  5  6  7 => 1;
    ///      8  9 10 11 => 2;
    ///     12 13 14 15 => 3
    /// ];
    /// foo.pop_column();
    /// let bar = augmented_matrix![
    ///      0  1  2 => 0;
    ///      4  5  6 => 1;
    ///      8  9 10 => 2;
    ///     12 13 14 => 3
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    pub fn pop_column(&mut self) {
        if self.is_column_aligned() {
            for _ in 0..self.rows {
                drop(self.matrix.pop());
            }
            self.rows -= 1;
        } else {
            for c in (1..self.num_rows() + 1).rev() {
                self.matrix.remove(self.columns * c - 1);
            }
            self.columns -= 1;
        }
    }

    /// Removes a specified column from an augmented matrix, similarly to `remove(n)` for vectors.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    /// let mut foo = augmented_matrix![
    ///      0  1  2  3 => 0;
    ///      4  5  6  7 => 1;
    ///      8  9 10 11 => 2;
    ///     12 13 14 15 => 3
    /// ];
    /// foo.remove_column(1);
    /// let bar = augmented_matrix![
    ///      0  2  3 => 0;
    ///      4  6  7 => 1;
    ///      8 10 11 => 2;
    ///     12 14 15 => 3
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    pub fn remove_column(&mut self, column: usize) {
        assert!(column <= self.num_columns());
        if column == self.num_columns() {
            self.pop_column();
            return;
        }
        if self.is_column_aligned() {
            self.matrix.drain(column * self.rows..(column + 1) * self.rows);
            self.rows -= 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                self.matrix.remove(r * self.columns + column);
            }
            self.columns -= 1;
        }
    }
}

macro_rules! pop_remove_rows_columns {
    ($($target_type:ty {
        $pop_row_expr:expr,
        $remove_row_expr:expr,
        $remove_rows_expr:expr,
        $remove_columns_expr:expr
    }),*) => ($(
        impl<T> $target_type {
            #[doc = $pop_row_expr]
            pub fn pop_row(&mut self) {
                if self.is_row_aligned() {
                    for _ in 0..self.columns {
                        drop(self.matrix.pop());
                    }
                    self.rows -= 1;
                } else {
                    let r_max = if self.is_row_aligned() {
                        self.columns
                    } else {
                        self.rows
                    };
                    for r in (1..r_max).rev() {
                        self.matrix.remove(r_max * r - 1);
                    }
                    self.columns -= 1;
                }
            }

            #[doc = $remove_row_expr]
            pub fn remove_row(&mut self, row: usize) {
                assert!(row <= self.num_rows());
                if row == self.num_rows() {
                    self.pop_row();
                    return;
                }
                if self.is_row_aligned() {
                    self.matrix.drain(row * self.columns..(row + 1) * self.columns);
                    self.rows -= 1;
                } else {
                    for c in (0..self.rows).rev() {
                        self.matrix.remove(c * self.rows + row);
                    }
                    self.columns -= 1;
                }
            }

            #[doc = $remove_rows_expr]
            pub fn remove_rows(&mut self, rows: Range<usize>) {
                assert!(rows.start <= self.num_rows());
                assert!(rows.end < self.num_rows() + 1);
                for r in rows.rev() {
                    self.remove_row(r);
                }
            }

            #[doc = $remove_columns_expr]
            pub fn remove_columns(&mut self, columns: Range<usize>) {
                assert!(columns.start <= self.num_columns());
                assert!(columns.end < self.num_columns() + 1);
                for c in columns.rev() {
                    self.remove_column(c);
                }
            }
        }
    )*)
}

pop_remove_rows_columns!{Matrix<T> {
    "Removes the last row from a matrix, similarly to `pop()` for vectors.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = matrix![
         0  1  2  3  4  5;
         6  7  8  9 10 11;
        12 13 14 15 16 17
    ];
    foo.pop_row();
    let bar = matrix![
        0  1  2  3  4  5;
        6  7  8  9 10 11
    ];
    assert_eq!(foo, bar);
    ```",
    "Removes a given row from a matrix, similarly to `remove()` for vectors. Panics if the specified
    row is outside of the bounds of the matrix.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = matrix![
         0  1  2  3  4  5;
         6  7  8  9 10 11;
        12 13 14 15 16 17
    ];
    foo.remove_row(0);
    let bar = matrix![
         6  7  8  9 10 11;
        12 13 14 15 16 17
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    # use fractions_and_matrices::matrices::extras::AddElements;
    let mut foo = matrix![
         0  1  2  3  4  5;
         6  7  8  9 10 11;
        12 13 14 15 16 17
    ];
    foo.remove_row(4);
    ```",
    "Removes a `Range<usize>` of rows from a `Matrix<T>`. Panics if the range goes outside of the
    bounds of the matrix.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = matrix![
        0  1  2;
        3  4  5;
        6  7  8;
        9 10 11
    ];
    foo.remove_rows(0..2);
    let bar = matrix![
        6  7  8;
        9 10 11
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = matrix![
        0  1  2;
        3  4  5;
        6  7  8;
        9 10 11
    ];
    foo.remove_rows(2..6);
    ```",
    "Removes a `Range<usize>` of columns from a `Matrix<T>`. Panics if the specified range goes
    outside of the bounds of the matrix.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = matrix![
         0  1  2  3  4  5;
         6  7  8  9 10 11;
        12 13 14 15 16 17
    ];
    foo.remove_columns(1..4);
    let bar = matrix![
         0  4  5;
         6 10 11;
        12 16 17
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = matrix![
         0  1  2  3  4  5;
         6  7  8  9 10 11;
        12 13 14 15 16 17
    ];
    foo.remove_columns(4..7);
    ```"
}, AugmentedMatrix<T> {
    "Removes the last row from an augmented matrix, similarly to `pop()` for vectors.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
         0  1  2  3  4  5 => 0;
         6  7  8  9 10 11 => 1;
        12 13 14 15 16 17 => 2
    ];
    foo.pop_row();
    let bar = augmented_matrix![
        0  1  2  3  4  5 => 0;
        6  7  8  9 10 11 => 1
    ];
    assert_eq!(foo, bar);
    ```",
    "Removes a given row from an augmented matrix, similarly to `remove()` for vectors. Panics if
    the specified row is outside of the bounds of the augmented matrix.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
         0  1  2  3  4  5 => 0;
         6  7  8  9 10 11 => 1;
        12 13 14 15 16 17 => 2
    ];
    foo.remove_row(0);
    let bar = augmented_matrix![
         6  7  8  9 10 11 => 1;
        12 13 14 15 16 17 => 2
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
         0  1  2  3  4  5 => 0;
         6  7  8  9 10 11 => 1;
        12 13 14 15 16 17 => 2
    ];
    foo.remove_row(4);
    ```",
    "Removes a `Range<usize>` of rows from an `AugmentedMatrix<T>`. Panics if the range goes outside
    of the bounds of the augmented matrix.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
        0  1  2 => 0;
        3  4  5 => 1;
        6  7  8 => 2;
        9 10 11 => 3
    ];
    foo.remove_rows(0..2);
    let bar = augmented_matrix![
        6  7  8 => 2;
        9 10 11 => 3
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
        0  1  2 => 0;
        3  4  5 => 1;
        6  7  8 => 2;
        9 10 11 => 3
    ];
    foo.remove_rows(2..5);
    ```",
    "Removes a `Range<usize>` of columns from an `AugmentedMatrix<T>`. Panics if the specified range
    goes outside of the bounds of the augmented matrix.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
         0  1  2  3  4  5 => 0;
         6  7  8  9 10 11 => 1;
        12 13 14 15 16 17 => 2
    ];
    foo.remove_columns(1..4);
    let bar = augmented_matrix![
         0  4  5 => 0;
         6 10 11 => 1;
        12 16 17 => 2
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = augmented_matrix![
         0  1  2  3  4  5 => 0;
         6  7  8  9 10 11 => 1;
        12 13 14 15 16 17 => 2
    ];
    foo.remove_columns(4..8);
    ```"
}}

pub trait AddElements<T> {
    fn push_row<R: AsRef<[T]>>(&mut self, row: R);
    fn push_column<R: AsRef<[T]>>(&mut self, column: R);
    fn try_push_row<R: AsRef<[T]>>(&mut self, row: R) -> Result<(), MatrixError>;
    fn try_push_column<R: AsRef<[T]>>(&mut self, column: R) -> Result<(), MatrixError>;
    fn insert_row<R: AsRef<[T]>>(&mut self, location: usize, row: R);
    fn insert_column<R: AsRef<[T]>>(&mut self, location: usize, column: R);
    fn try_insert_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) -> Result<(), MatrixError>;
    fn try_insert_column<R: AsRef<[T]>>(&mut self, location: usize, column: R)
        -> Result<(), MatrixError>;
    fn push_rows<R: AsRef<[T]>>(&mut self, rows: R);
    fn push_columns<R: AsRef<[T]>>(&mut self, columns: R);
    fn try_push_rows<R: AsRef<[T]>>(&mut self, rows: R) -> Result<(), MatrixError>;
    fn try_push_columns<R: AsRef<[T]>>(&mut self, columns: R) -> Result<(), MatrixError>;
    fn insert_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R);
    fn insert_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R);
    fn try_insert_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) -> Result<(), MatrixError>;
    fn try_insert_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R)
        -> Result<(), MatrixError>;
}

// Macro removed for now until I better understand why it wasn't working. Once I do, I'll swap it
// back in to reduce this section back to its original ~600 LoC.

use std::fmt::Display;

impl<T: Clone + Display> AddElements<T> for Matrix<T> {
    /// Pushes a row to a `Matrix<T>`, similarly to `push()` for vectors. Panics if the length of
    /// the supplied row is not equal to the number of columns in the matrix and .
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 2;
    ///     3 4 5
    /// ];
    /// foo.push_row([6, 7, 8]);
    /// let bar = matrix![
    ///     0 1 2;
    ///     3 4 5;
    ///     6 7 8
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 2;
    ///     3 4 5
    /// ];
    /// foo.push_row([6, 7]);
    /// ```
    fn push_row<R: AsRef<[T]>>(&mut self, row: R) {
        let row = row.as_ref();
        assert_eq!(row.len(), self.num_columns());
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(row);
            self.rows += 1;
        } else {
            for c in (0..self.num_columns()).rev() {
                let insert_loc = self.num_rows() * c + self.num_rows();
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
    }

    /// Push a column to a matrix, similarly to `push()` for vectors. Panics if the length of the
    /// supplied column is not equal to the number of rows in the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1;
    ///     3 4;
    ///     6 7
    /// ];
    /// foo.push_column([2, 5, 8]);
    /// let bar = matrix![
    ///     0 1 2;
    ///     3 4 5;
    ///     6 7 8
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1;
    ///     3 4;
    ///     6 7
    /// ];
    /// foo.push_column([2, 5, 8, 11]);
    /// ```
    fn push_column<R: AsRef<[T]>>(&mut self, column: R) {
        let column = column.as_ref();
        assert_eq!(column.len(), self.num_rows());
        if self.is_column_aligned() {
            self.matrix.extend_from_slice(column);
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = self.num_columns() * r + self.num_columns();
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
    }

    /// Attempts to push a row to a matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0  1  2  3;
    ///     4  5  6  7;
    ///     8  9 10 11
    /// ];
    /// assert!(foo.try_push_row([12, 13, 14, 15]).is_ok());
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_push_row([0, 1, 2]).is_err());
    /// assert_eq!(foo, bar);
    /// ```
    fn try_push_row<R: AsRef<[T]>>(&mut self, row: R) -> Result<(), MatrixError> {
        let row = row.as_ref();
        if row.len() != self.num_columns() {
            return Err(MatrixError::FunctionError("Unable to push row to matrix - the row \
                    doesn't have the same number of elements as the matrix rows do.".to_string()));
        }
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(row);
            self.rows += 1;
        } else {
            for c in (0..self.num_columns()).rev() {
                let insert_loc = self.num_rows() * c + self.num_rows();
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    /// Attempts to push a column to a given matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2;
    ///      4  5  6;
    ///      8  9 10;
    ///     12 13 14
    /// ];
    /// assert!(foo.try_push_column([3, 7, 11, 15]).is_ok());
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_push_column([0, 1, 2]).is_err());
    /// assert_eq!(foo, bar);
    /// ```
    fn try_push_column<R: AsRef<[T]>>(&mut self, column: R) -> Result<(), MatrixError> {
        let column = column.as_ref();
        if column.len() != self.num_rows() {
            return Err(MatrixError::FunctionError("Unable to push column to matrix - the \
                    column doesn't have the same number of elements as the matrix columns do."
                .to_string()));
        }
        if self.is_column_aligned() {
            self.matrix.extend_from_slice(column);
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc  = self.num_columns() * r + self.num_columns();
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    /// Inserts a row into a `Matrix<T>`. Panics if the length of the supplied row is not equal to
    /// the number of columns in the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///     12 13 14 15
    /// ];
    /// foo.insert_row(2, [8, 9, 10, 11]);
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///     12 13 14 15
    /// ];
    /// foo.insert_row(2, [8, 9]);
    /// ```
    fn insert_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) {
        let row = row.as_ref();
        assert_eq!(row.len(), self.num_columns());
        assert!(location <= self.num_rows());
        if self.is_row_aligned() {
            let new  = {
                let (left, right) = self.matrix.split_at(location * self.num_columns());
                let mut left = left.to_vec();
                left.extend_from_slice(row);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for c in (0..self.num_columns()).rev() {
                let insert_loc = self.num_rows() * c + location;
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
    }

    /// Inserts a column into a matrix. Panics if the length of the supplied column is not equal to
    /// the number of rows in the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 2;
    ///     3 5;
    ///     6 8
    /// ];
    /// foo.insert_column(1, [1, 4, 7]);
    /// let bar = matrix![
    ///     0 1 2;
    ///     3 4 5;
    ///     6 7 8
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 2;
    ///     3 5;
    ///     6 8
    /// ];
    /// foo.insert_column(1, [1, 4, 7, 10]);
    /// ```
    fn insert_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) {
        let column = column.as_ref();
        assert_eq!(column.len(), self.num_rows());
        assert!(location <= self.num_columns());
        if self.is_column_aligned() {
            let new = {
                let(left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(column);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = self.num_columns() * r + location;
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
    }

    /// Attempts to insert a row into a given matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///     12 13 14 15
    /// ];
    /// assert!(foo.try_insert_row(2, [8, 9, 10, 11]).is_ok());
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_insert_row(6, [16, 17, 18, 19]).is_err());
    /// assert!(foo.try_insert_row(1, [0, 1]).is_err());
    /// ```
    fn try_insert_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) -> Result<(), MatrixError> {
        let row = row.as_ref();
        if row.len() != self.num_columns() {
            return Err(MatrixError::FunctionError("Attempted to add a row with an \
                    incorrect number of elements.".to_string()));
        }
        if !(location <= self.num_rows()) {
            return Err(MatrixError::FunctionError("Attempted to add a row at an invalid \
                    index.".to_string()));
        }
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_columns());
                let mut left = left.to_vec();
                left.extend_from_slice(row);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for c in (0..self.num_columns()).rev() {
                let insert_loc = self.num_rows() * c + location;
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    /// Attempts to insert a column into a matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 2;
    ///     3 5;
    ///     6 8
    /// ];
    /// assert!(foo.try_insert_column(1, [1, 4, 7]).is_ok());
    /// let bar = matrix![
    ///     0 1 2;
    ///     3 4 5;
    ///     6 7 8
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_insert_column(4, [0, 1, 2]).is_err());
    /// assert!(foo.try_insert_column(0, [0, 1, 2, 3]).is_err());
    /// ```
    fn try_insert_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) -> Result<(), MatrixError> {
        let column = column.as_ref();
        if column.len() != self.num_rows() {
            return Err(MatrixError::FunctionError("Attempted to add a column with an \
                    incorrect number of elements.".to_string()));
        }
        if !(location <= self.num_columns()) {
            return Err(MatrixError::FunctionError("Attemped to add a column at an invalid \
                    index.".to_string()));
        }
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(column);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = self.num_columns() * r + location;
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    /// Push rows to the end of a matrix. Panics if the total length of the supplied rows is not
    /// divisible by the number of columns in the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 2
    /// ];
    /// foo.push_rows([3, 4, 5, 6, 7, 8]);
    /// let bar = matrix![
    ///     0 1 2;
    ///     3 4 5;
    ///     6 7 8
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 2
    /// ];
    /// foo.push_rows([3, 4, 5, 6, 7, 8, 9]);
    /// ```
    fn push_rows<R: AsRef<[T]>>(&mut self, rows: R) {
        let rows = rows.as_ref();
        assert_eq!(rows.len() % self.num_columns(), 0);
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(rows);
            self.rows += rows.len() / self.num_columns();
        } else {
            for r in 0..rows.len() / self.num_columns() {
                let rows_range = r * self.num_columns()..(r + 1) * self.num_columns();
                self.push_row(&rows[rows_range]);
            }
        }
    }

    /// Pushes columns to a matrix. Panics if the total length of the supplied columns is not
    /// divisible by the number of rows in the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0;
    ///      5;
    ///     10;
    ///     15;
    ///     20
    /// ];
    /// foo.push_columns([1, 6, 11, 16, 21, 2, 7, 12, 17, 22, 3, 8, 13, 18, 23, 4, 9, 14, 19, 24]);
    /// let bar = matrix![
    ///      0  1  2  3  4;
    ///      5  6  7  8  9;
    ///     10 11 12 13 14;
    ///     15 16 17 18 19;
    ///     20 21 22 23 24
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0;
    ///     5;
    ///     10;
    ///     15;
    ///     20
    /// ];
    /// foo.push_columns([1, 6, 11, 16, 21, 2, 7, 12]);
    /// ```
    fn push_columns<R: AsRef<[T]>>(&mut self, columns: R) {
        let columns = columns.as_ref();
        assert_eq!(columns.len() % self.num_rows(), 0);
        if self.is_column_aligned() {
            self.matrix.extend_from_slice(columns);
            self.rows += columns.len() / self.num_columns();
        } else {
            for c in 0..columns.len() / self.num_rows() {
                let columns_range = c * self.num_rows()..(c + 1) * self.num_rows();
                self.push_column(&columns[columns_range]);
            }
        }
    }

    /// Attempts to insert multiple rows into a matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0  1  2  3  4  5  6;
    ///     7  8  9 10 11 12 13
    /// ];
    /// assert!(foo.try_push_rows([14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27])
    ///     .is_ok());
    /// let bar = matrix![
    ///      0  1  2  3  4  5  6;
    ///      7  8  9 10 11 12 13;
    ///     14 15 16 17 18 19 20;
    ///     21 22 23 24 25 26 27
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_push_rows([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).is_err());
    /// ```
    fn try_push_rows<R: AsRef<[T]>>(&mut self, rows: R) -> Result<(), MatrixError> {
        let rows = rows.as_ref();
        if rows.len() % self.num_columns() != 0 {
            return Err(MatrixError::FunctionError("Attempted to push rows where the total \
                    number of elements is not divisible by the number of elements per row."
                .to_string()));
        }
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(rows);
            self.rows += rows.len() / self.num_columns();
        } else {
            for r in 0..rows.len() / self.num_columns() {
                let rows_range = r * self.num_columns()..(r + 1) * self.num_columns();
                self.push_row(&rows[rows_range]);
            }
        }
        Ok(())
    }

    /// Attempts to push columns to a matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1;
    ///      4  5;
    ///      8  9;
    ///     12 13
    /// ];
    /// assert!(foo.try_push_columns([2, 6, 10, 14, 3, 7, 11, 15]).is_ok());
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_push_columns([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).is_err());
    /// ```
    fn try_push_columns<R: AsRef<[T]>>(&mut self, columns: R) -> Result<(), MatrixError> {
        let columns = columns.as_ref();
        if columns.len() % self.num_rows() != 0 {
            return Err(MatrixError::FunctionError("Attempted to push columns where the \
                    total number of elements is not divisible by the number of columns per row."
                .to_string()));
        }
        if self.is_column_aligned() {
            self.matrix.extend_from_slice(columns);
            self.rows += columns.len() / self.num_columns();
        } else {
            for c in 0..columns.len() / self.num_rows() {
                let columns_range = c * self.num_rows()..(c + 1) * self.num_rows();
                self.push_column(&columns[columns_range]);
            }
        }
        Ok(())
    }

    /// Inserts rows at a given location into a matrix. Panics if the total length of the supplied
    /// rows is not divisible by the number of columns in the matrix, or if the insert location is
    /// outside of the bounds of the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2  3  4;
    ///     15 16 17 18 19;
    ///     20 21 22 23 24
    /// ];
    /// foo.column_align();
    /// foo.insert_rows(1, [5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    /// let bar = matrix![
    ///      0  1  2  3  4;
    ///      5  6  7  8  9;
    ///     10 11 12 13 14;
    ///     15 16 17 18 19;
    ///     20 21 22 23 24
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 2 3
    /// ];
    /// foo.insert_rows(3, [3, 4, 5, 6, 7, 8]);
    /// ```
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2  3;
    ///     12 13 14 15
    /// ];
    /// foo.insert_rows(1, [4, 5, 6, 7, 8, 9, 10]);
    /// ```
    fn insert_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) {
        let rows = rows.as_ref();
        assert_eq!(rows.len() % self.num_columns(), 0);
        assert!(location <= self.num_rows());
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_columns());
                let mut left = left.to_vec();
                left.extend_from_slice(rows);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += rows.len() / self.num_columns();
        } else {
            for l in (0..rows.len() / self.num_columns()).rev() {
                let rs_range: Range<usize> = l * self.num_columns()..(l + 1) * self.num_columns();
                for (i, e) in rows[rs_range].iter().enumerate().rev() {
                    let insert_loc = i % self.num_columns() * self.num_rows() + location;
                    self.matrix.insert(insert_loc, e.clone());
                }
                self.columns += 1;
            }
        }
    }

    /// Inserts columns at a given location into a matrix. Panics if the total length of the
    /// supplied columns is not divisible by the number of rows in the matrix, or if the insert
    /// location is outside of the bounds of the matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 4;
    ///     5 6 9
    /// ];
    /// foo.insert_columns(2, [2, 7, 3, 8]);
    /// let bar = matrix![
    ///     0 1 2 3 4;
    ///     5 6 7 8 9
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 4;
    ///     5 6 9
    /// ];
    /// foo.insert_columns(2, [0, 1, 2]);
    /// let bar = matrix![
    ///     0 1 2 3 4;
    ///     5 6 7 8 9
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///     0 1 4;
    ///     5 6 9
    /// ];
    /// foo.insert_columns(4, [0, 1, 2]);
    /// ```
    fn insert_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R) {
        let columns = columns.as_ref();
        assert_eq!(columns.len() % self.num_rows(), 0);
        assert!(location <= self.num_columns());
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(columns);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.columns += columns.len() / self.num_columns();
        } else {
            for l in (0..columns.len() / self.num_rows()).rev() {
                let cs_range: Range<usize> = l * self.num_rows()..(l + 1) * self.num_rows();
                for (i, e) in columns[cs_range].iter().enumerate().rev() {
                    let insert_loc = i % self.num_rows() * self.num_columns() + location;
                    self.matrix.insert(insert_loc, e.clone());
                }
                self.columns += 1;
            }
        }
    }

    /// Attempts to insert a row into a given matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  1  2  3;
    ///     12 13 14 15
    /// ];
    /// assert!(foo.try_insert_rows(1, [4, 5, 6, 7, 8, 9, 10, 11]).is_ok());
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.num_rows() == 4);
    /// assert!(foo.try_insert_rows(5, [0, 1, 2, 3]).is_err());
    /// assert!(foo.try_insert_rows(1, [0, 1, 2, 3, 4, 5]).is_err());
    /// ```
    fn try_insert_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) -> Result<(), MatrixError> {
        let rows = rows.as_ref();
        if rows.len() % self.num_columns() != 0 {
            return Err(MatrixError::FunctionError("Attempted to push rows where the total \
                    number of elements is not divisible by the number of elements per row."
                .to_string()));
        }
        if !(location <= self.num_rows()) {
            return Err(MatrixError::FunctionError("Attempted to add rows at an invalid \
                    index.".to_string()));
        }
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_columns());
                let mut left = left.to_vec();
                left.extend_from_slice(rows);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += rows.len() / self.num_columns();
        } else {
            for r in (0..rows.len() / self.num_columns()).rev() {
                for c in (0..self.num_columns()).rev() {
                    let insert_loc = self.num_rows() * c + location;
                    let rows_loc = r * self.num_columns() + c;
                    self.matrix.insert(insert_loc, rows[rows_loc].clone());
                }
            }
        }
        Ok(())
    }

    /// Attempts to insert columns into a given matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::Matrix;
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = matrix![
    ///      0  3;
    ///      4  7;
    ///      8 11;
    ///     12 15
    /// ];
    /// assert!(foo.try_insert_columns(1, [1, 5, 9, 13, 2, 6, 10, 14]).is_ok());
    /// let bar = matrix![
    ///      0  1  2  3;
    ///      4  5  6  7;
    ///      8  9 10 11;
    ///     12 13 14 15
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_insert_columns(10, [0, 1, 2, 3]).is_err());
    /// assert!(foo.try_insert_columns(4, [0, 1]).is_err());
    /// ```
    fn try_insert_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R)
                       -> Result<(), MatrixError> {
        let columns = columns.as_ref();
        if columns.len() % self.num_rows() != 0 {
            return Err(MatrixError::FunctionError("Attempted to push columns where the \
                    total number of elements is not divisible by the number of columns per row."
                .to_string()));
        }
        if !(location <= self.num_columns()) {
            return Err(MatrixError::FunctionError("Attemped to add columns at an invalid \
                    index.".to_string()));
        }
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(columns);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.columns += columns.len() / self.num_columns();
        } else {
            for l in (0..columns.len() / self.num_rows()).rev() {
                let cs_range: Range<usize> = l * self.num_rows()..(l + 1) * self.num_rows();
                for (i, e) in columns[cs_range].iter().enumerate().rev() {
                    let insert_loc = i % self.num_rows() * self.num_columns() + location;
                    self.matrix.insert(insert_loc, e.clone());
                }
                self.columns += 1;
            }
        }
        Ok(())
    }
}

impl<T: Clone> AddElements<T> for AugmentedMatrix<T> {
    /// Appends a row to the end of an augmented matrix. Panics of the supplied row does not have
    /// a length equal to the number of columns in the augmented matrix (including the solution
    /// column).
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::AugmentedMatrix;
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = augmented_matrix![
    ///     1 0 0 0 => 0;
    ///     0 1 0 0 => 1;
    ///     0 0 1 0 => 2
    /// ];
    /// foo.push_row([0, 0, 0, 1, 3]);
    /// let bar = augmented_matrix![
    ///     1 0 0 0 => 0;
    ///     0 1 0 0 => 1;
    ///     0 0 1 0 => 2;
    ///     0 0 0 1 => 3
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::AugmentedMatrix;
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = augmented_matrix![
    ///     1 0 0 0 => 0;
    ///     0 1 0 0 => 1;
    ///     0 0 1 0 => 2
    /// ];
    /// foo.push_row([0, 0, 0, 1]);
    /// ```
    fn push_row<R: AsRef<[T]>>(&mut self, row: R) {
        let row = row.as_ref();
        assert_eq!(row.len(), self.num_columns() + 1);
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(row);
            self.rows += 1;
        } else {
            for c in (0..self.num_columns() + 1).rev() {
                let insert_loc = self.num_rows() * c + self.num_rows();
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
    }

    /// Pushes a column to an augmented matrix. Panics if the length of the column is not equal to
    /// the number of rows in the augmented matrix. **NOTE:** the pushed column will be to the
    /// **left** of the solution column.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::AugmentedMatrix;
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = augmented_matrix![
    ///     1  2 =>  4;
    ///     5  6 =>  8;
    ///     9 10 => 12
    /// ];
    /// foo.push_column([3, 7, 11]);
    /// let bar = augmented_matrix![
    ///     1  2  3 =>  4;
    ///     5  6  7 =>  8;
    ///     9 10 11 => 12
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::AugmentedMatrix;
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = augmented_matrix![
    ///     1  2 =>  4;
    ///     5  6 =>  8;
    ///     9 10 => 12
    /// ];
    /// foo.push_column([0, 1]);
    /// ```
    fn push_column<R: AsRef<[T]>>(&mut self, column: R) {
        let column = column.as_ref();
        assert_eq!(column.len(), self.num_rows());
        if self.is_column_aligned() {
            let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
            let mut left = left.to_vec();
            left.extend_from_slice(column);
            left.extend_from_slice(right);
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = (self.num_columns() + 1) * r + self.num_columns();
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
    }

    /// Attempts to push a row to an augmented matrix. Returns an error if the length of the
    /// provided row is not equal to the number of columns in the augmented matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::AugmentedMatrix;
    /// # use fractions_and_matrices::matrices::extras::AddElements;
    /// let mut foo = augmented_matrix![
    ///     1 0 0 => 0;
    ///     0 1 0 => 1
    /// ];
    /// assert!(foo.try_push_row([0, 0, 1, 2]).is_ok());
    /// let bar = augmented_matrix![
    ///     1 0 0 => 0;
    ///     0 1 0 => 1;
    ///     0 0 1 => 2
    /// ];
    /// assert_eq!(foo, bar);
    /// assert!(foo.try_push_row([0]).is_err());
    /// ```
    fn try_push_row<R: AsRef<[T]>>(&mut self, row: R) -> Result<(), MatrixError> {
        let row = row.as_ref();
        if row.len() != self.num_columns() + 1 {
            return Err(MatrixError::FunctionError("Unable to push row to matrix - the row \
                    doesn't have the same number of elements as the matrix rows do.".to_string()));
        }
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(row);
            self.rows += 1;
        } else {
            for c in (0..self.num_columns() + 1).rev() {
                let insert_loc = self.num_rows() * c + self.num_rows();
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    /// Attempts to push a column to an augmented matrix. Returns an error if the length of the
    fn try_push_column<R: AsRef<[T]>>(&mut self, column: R) -> Result<(), MatrixError> {
        let column = column.as_ref();
        if column.len() != self.num_rows() {
            return Err(MatrixError::FunctionError("Unable to push column to matrix - the \
                    column doesn't have the same number of elements as the matrix columns do."
                .to_string()));
        }
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(column);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = self.num_columns() * r + self.num_columns();
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    fn insert_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) {
        let row = row.as_ref();
        assert_eq!(row.len(), self.num_columns() + 1);
        assert!(location <= self.num_rows());
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * (self.num_columns() + 1));
                let mut left = left.to_vec();
                left.extend_from_slice(row);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for c in (0..self.num_columns() + 1).rev() {
                let insert_loc = self.num_rows() * c + location;
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
    }

    fn insert_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) {
        let column = column.as_ref();
        assert_eq!(column.len(), self.num_rows());
        assert!(location <= self.num_columns());
        if self.is_column_aligned() {
            let new = {
                let(left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(column);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = self.num_columns() * r + location;
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
    }

    fn try_insert_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) -> Result<(), MatrixError> {
        let row = row.as_ref();
        if row.len() != self.num_columns() + 1 {
            return Err(MatrixError::FunctionError("Attempted to add a row with an \
                    incorrect number of elements.".to_string()));
        }
        if !(location <= self.num_rows()) {
            return Err(MatrixError::FunctionError("Attempted to add a row at an invalid \
                    index.".to_string()));
        }
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * (self.num_columns() + 1));
                let mut left = left.to_vec();
                left.extend_from_slice(row);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for c in (0..self.num_columns() + 1).rev() {
                let insert_loc = self.num_rows() * c + location;
                self.matrix.insert(insert_loc, row[c].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    fn try_insert_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) -> Result<(), MatrixError> {
        let column = column.as_ref();
        if column.len() != self.num_rows() {
            return Err(MatrixError::FunctionError("Attempted to add a column with an \
                    incorrect number of elements.".to_string()));
        }
        if !(location <= self.num_columns()) {
            return Err(MatrixError::FunctionError("Attemped to add a column at an invalid \
                    index.".to_string()));
        }
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(column);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += 1;
        } else {
            for r in (0..self.num_rows()).rev() {
                let insert_loc = self.num_columns() * r + location;
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
        Ok(())
    }

    fn push_rows<R: AsRef<[T]>>(&mut self, rows: R) {
        let rows = rows.as_ref();
        assert_eq!(rows.len() % (self.num_columns() + 1), 0);
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(rows);
            self.rows += rows.len() / (self.num_columns() + 1);
        } else {
            for r in 0..rows.len() / (self.num_columns() + 1) {
                let rows_range = r * (self.num_columns() + 1)..(r + 1) * (self.num_columns() + 1);
                self.push_row(&rows[rows_range]);
            }
        }
    }

    fn push_columns<R: AsRef<[T]>>(&mut self, columns: R) {
        let columns = columns.as_ref();
        assert_eq!(columns.len() % self.num_rows(), 0);
        if self.is_column_aligned() {
            let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
            let mut left = left.to_vec();
            left.extend_from_slice(columns);
            left.extend_from_slice(right);
            self.rows += columns.len() / self.num_columns();
        } else {
            for c in 0..columns.len() / self.num_rows() {
                let columns_range = c * self.num_columns()..(c + 1) * self.num_columns();
                self.push_column(&columns[columns_range]);
            }
        }
    }

    fn try_push_rows<R: AsRef<[T]>>(&mut self, rows: R) -> Result<(), MatrixError> {
        let rows = rows.as_ref();
        if rows.len() % (self.num_columns() + 1) != 0 {
            return Err(MatrixError::FunctionError("Attempted to push rows where the total \
                    number of elements is not divisible by the number of elements per row."
                .to_string()));
        }
        if self.is_row_aligned() {
            self.matrix.extend_from_slice(rows);
            self.rows += rows.len() / (self.num_columns() + 1);
        } else {
            for r in 0..rows.len() / (self.num_columns() + 1) {
                let rows_range = r * (self.num_columns() + 1)..(r + 1) * (self.num_columns() + 1);
                self.push_row(&rows[rows_range]);
            }
        }
        Ok(())
    }

    fn try_push_columns<R: AsRef<[T]>>(&mut self, columns: R) -> Result<(), MatrixError> {
        let columns = columns.as_ref();
        if columns.len() % self.num_rows() != 0 {
            return Err(MatrixError::FunctionError("Attempted to push columns where the \
                    total number of elements is not divisible by the number of columns per row."
                .to_string()));
        }
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(columns);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += columns.len() / self.num_columns();
        } else {
            for c in 0..columns.len() / self.num_rows() {
                let columns_range = c * self.num_rows()..(c + 1) * self.num_rows();
                self.push_column(&columns[columns_range]);
            }
        }
        Ok(())
    }

    fn insert_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) {
        let rows = rows.as_ref();
        assert_eq!(rows.len() % (self.num_columns() + 1), 0);
        assert!(location <= self.num_rows());
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * (self.num_columns() + 1));
                let mut left = left.to_vec();
                left.extend_from_slice(rows);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += rows.len() / (self.num_columns() + 1);
        } else {
            for r in (0..rows.len() / (self.num_columns() + 1)).rev() {
                for c in (0..self.num_columns() + 1).rev() {
                    let insert_loc = self.num_rows() * c + location;
                    let rows_loc = r * (self.num_columns() + 1) + c;
                    self.matrix.insert(insert_loc, rows[rows_loc].clone());
                }
            }
            self.columns += rows.len() / (self.num_columns() + 1);
        }
    }

    fn insert_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R) {
        let columns = columns.as_ref();
        assert_eq!(columns.len() % self.num_rows(), 0);
        assert!(location <= self.num_columns());
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(columns);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.columns += columns.len() / self.num_columns();
        } else {
            for c in (0..columns.len() / self.num_rows()).rev() {
                for r in (0..self.num_rows()).rev() {
                    let insert_loc = self.num_columns() * c + location;
                    let columns_loc = r * self.num_rows() + c;
                    self.matrix.insert(insert_loc, columns[columns_loc].clone());
                }
            }
            self.columns += columns.len() / self.num_rows();
        }
    }

    fn try_insert_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) -> Result<(), MatrixError> {
        let rows = rows.as_ref();
        if rows.len() % (self.num_columns() + 1) != 0 {
            return Err(MatrixError::FunctionError("Attempted to push rows where the total \
                    number of elements is not divisible by the number of elements per row."
                .to_string()));
        }
        if !(location <= self.num_rows()) {
            return Err(MatrixError::FunctionError("Attempted to add rows at an invalid \
                    index.".to_string()));
        }
        if self.is_row_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * (self.num_columns() + 1));
                let mut left = left.to_vec();
                left.extend_from_slice(rows);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.rows += rows.len() / (self.num_columns() + 1);
        } else {
            for r in (0..rows.len() / (self.num_columns() + 1)).rev() {
                for c in (0..self.num_columns() + 1).rev() {
                    let insert_loc = self.num_rows() * c + location;
                    let rows_loc = r * (self.num_columns() + 1) + c;
                    self.matrix.insert(insert_loc, rows[rows_loc].clone());
                }
            }
            self.columns += rows.len() / (self.num_columns() + 1);
        }
        Ok(())
    }

    fn try_insert_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R) -> Result<(), MatrixError> {
        let columns = columns.as_ref();
        if columns.len() % self.num_rows() != 0 {
            return Err(MatrixError::FunctionError("Attempted to push columns where the \
                    total number of elements is not divisible by the number of columns per row."
                .to_string()));
        }
        if !(location <= self.num_columns()) {
            return Err(MatrixError::FunctionError("Attemped to add columns at an invalid \
                    index.".to_string()));
        }
        if self.is_column_aligned() {
            let new = {
                let (left, right) = self.matrix.split_at(location * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(columns);
                left.extend_from_slice(right);
                left
            };
            self.matrix = new;
            self.columns += columns.len() / self.num_columns();
        } else {
            for c in (0..columns.len() / self.num_rows()).rev() {
                for r in (0..self.num_rows()).rev() {
                    let insert_loc = self.num_columns() * c + location;
                    let columns_loc = r * self.num_rows() + c;
                    self.matrix.insert(insert_loc, columns[columns_loc].clone());
                }
            }
            self.columns += columns.len() / self.num_rows();
        }
        Ok(())
    }
}