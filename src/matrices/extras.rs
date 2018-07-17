use std::ops::Range;

use matrices::base::{AugmentedMatrix, Matrix, MatrixError};

impl<T> Matrix<T> {
    pub fn pop_column(&mut self) {
        if self.is_column_aligned() {
            for _ in 0..self.rows {
                drop(self.matrix.pop());
            }
            self.rows -= 1;
        } else {
            for c in 0..self.num_rows() {
                self.matrix.remove((self.columns - 1) * self.rows + c);
            }
            self.columns -= 1;
        }
    }

    pub fn remove_column(&mut self, column: usize) {
        if column == self.rows {
            self.pop_column();
            return;
        }
        if self.is_column_aligned() {
            self.matrix.drain(column * self.rows..(column + 1) * self.rows);
            self.rows -= 1;
        } else {
            for r in 0..self.num_rows() {
                self.matrix.remove(r * self.columns + r);
            }
            self.columns -= 1;
        }
    }
}

impl<T> AugmentedMatrix<T> {
    pub fn remove_column(&mut self, column: usize) {
        if column == self.rows {
            return;
        }
        if self.is_column_aligned() {
            self.matrix.drain(column * self.rows..(column + 1) * self.rows);
            self.rows -= 1;
        } else {
            for r in 0..self.num_rows() {
                self.matrix.remove(r * self.columns + r);
            }
            self.columns -= 1;
        }
    }
}

macro_rules! pop_remove_rows_columns {
    ($($target_type:ty),*) => ($(
        impl<T> $target_type {
            pub fn pop_row(&mut self) {
                if self.is_row_aligned() {
                    for _ in 0..self.columns {
                        drop(self.matrix.pop());
                    }
                    self.rows -= 1;
                } else {
                    for c in 0..self.num_columns() {
                        self.matrix.remove((self.columns - 1) * self.rows + c);
                    }
                    self.columns -= 1;
                }
            }

            pub fn remove_row(&mut self, row: usize) {
                if row == self.rows {
                    self.pop_row();
                    return;
                }
                if self.is_row_aligned() {
                    self.matrix.drain(row * self.columns..(row + 1) * self.columns);
                } else {
                    for c in 0..self.num_columns() {
                        self.matrix.remove(row * self.columns + c);
                    }
                }
            }

            pub fn remove_rows(&mut self, rows: Range<usize>) {
                for r in rows {
                    self.remove_row(r);
                }
            }

            pub fn remove_columns(&mut self, columns:  Range<usize>) {
                for c in columns {
                    self.remove_column(c);
                }
            }
        }
    )*)
}

pop_remove_rows_columns!{Matrix<T>, AugmentedMatrix<T>}

pub trait AddElements<T> {
    fn push_row<R: AsRef<[T]>>(&mut self, row: R);
    fn push_column<R: AsRef<[T]>>(&mut self, column: R);
    fn try_push_row<R: AsRef<[T]>>(&mut self, row: R) -> Result<(), MatrixError>;
    fn try_push_column<R: AsRef<[T]>>(&mut self, column: R) -> Result<(), MatrixError>;
    fn add_row<R: AsRef<[T]>>(&mut self, location: usize, row: R);
    fn add_column<R: AsRef<[T]>>(&mut self, location: usize, column: R);
    fn try_add_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) -> Result<(), MatrixError>;
    fn try_add_column<R: AsRef<[T]>>(&mut self, location: usize, column: R)
        -> Result<(), MatrixError>;
    fn push_rows<R: AsRef<[T]>>(&mut self, rows: R);
    fn push_columns<R: AsRef<[T]>>(&mut self, columns: R);
    fn try_push_rows<R: AsRef<[T]>>(&mut self, rows: R) -> Result<(), MatrixError>;
    fn try_push_columns<R: AsRef<[T]>>(&mut self, columns: R) -> Result<(), MatrixError>;
    fn add_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R);
    fn add_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R);
    fn try_add_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) -> Result<(), MatrixError>;
    fn try_add_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R)
        -> Result<(), MatrixError>;
}

// Macro removed for now until I better understand why it wasn't working. Once I do, I'll swap it
// back in to reduce this section back to its original ~600 LoC.

impl<T: Clone> AddElements<T> for Matrix<T> {
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

    fn add_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) {
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

    fn add_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) {
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

    fn try_add_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) -> Result<(), MatrixError> {
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

    fn try_add_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) -> Result<(), MatrixError> {
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

    fn add_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) {
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
            for r in (0..rows.len() / self.num_columns()).rev() {
                for c in (0..self.num_columns()).rev() {
                    let insert_loc = self.num_rows() * c + location;
                    let rows_loc = r * self.num_columns() + c;
                    self.matrix.insert(insert_loc, rows[rows_loc].clone());
                }
            }
        }
    }

    fn add_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R) {
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

    fn try_add_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) -> Result<(), MatrixError> {
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

    fn try_add_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R)
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

impl<T: Clone> AddElements<T> for AugmentedMatrix<T> {
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
                let insert_loc = self.num_columns() * r + self.num_columns();
                self.matrix.insert(insert_loc, column[r].clone());
            }
            self.columns += 1;
        }
    }

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

    fn add_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) {
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

    fn add_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) {
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

    fn try_add_row<R: AsRef<[T]>>(&mut self, location: usize, row: R) -> Result<(), MatrixError> {
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

    fn try_add_column<R: AsRef<[T]>>(&mut self, location: usize, column: R) -> Result<(), MatrixError> {
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

    fn add_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) {
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

    fn add_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R) {
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

    fn try_add_rows<R: AsRef<[T]>>(&mut self, location: usize, rows: R) -> Result<(), MatrixError> {
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

    fn try_add_columns<R: AsRef<[T]>>(&mut self, location: usize, columns: R) -> Result<(), MatrixError> {
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