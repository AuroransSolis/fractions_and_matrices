use std::ops::Range;

use matrices::matrix_base::{AugmentedMatrix, Matrix, MatrixError};

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
                self.matrix.remove(r * self.columns + r)
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
                self.matrix.remove(r * self.columns + r)
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

pub trait AddElements {
    type ElementContainer;

    fn push_row(&mut self, row: Self::ElementContainer);
    fn push_column(&mut self, column: Self::ElementContainer);
    fn try_push_row(&mut self, row: Self::ElementContainer) -> Result<(), MatrixError>;
    fn try_push_column(&mut self, column: Self::ElementContainer) -> Result<(), MatrixError>;
    fn add_row(&mut self, location: usize, row: Self::ElementContainer);
    fn add_column(&mut self, location: usize, column: Self::ElementContainer);
    fn try_add_row(&mut self, location: usize, row: Self::ElementContainer) -> Result<(), MatrixError>;
    fn try_add_column(&mut self, location: usize, column: Self::ElementContainer)
        -> Result<(), MatrixError>;
    fn push_rows(&mut self, rows: Self::ElementContainer);
    fn push_columns(&mut self, columns: Self::ElementContainer);
    fn try_push_rows(&mut self, rows: Self::ElementContainer) -> Result<(), MatrixError>;
    fn try_push_columns(&mut self, columns: Self::ElementContainer) -> Result<(), MatrixError>;
    fn add_rows(&mut self, location: usize, rows: Self::ElementContainer);
    fn add_columns(&mut self, location: usize, columns: Self::ElementContainer);
    fn try_add_rows(&mut self, location: usize, rows: Self::ElementContainer) -> Result<(), MatrixError>;
    fn try_add_columns(&mut self, location: usize, columns: Self::ElementContainer)
        -> Result<(), MatrixError>;
}

macro_rules! add_elements_impl {
    ($target_type:ty {
        $element_container:ty,
        rows {
            push_row: $pr_expr:expr,
            add_row: $ar_expr:expr,
            push_rows: $prs_expr:expr,
            add_rows: $ars_expr:expr
        },
        columns {
            push_column: $pc_expr:block,
            add_column: $ac_expr:block,
            push_columns: $pcs_expr:block,
            add_columns: $acs_expr:block
        }
    }) => {
        impl<'a, T> AddElements for $target_type {
            type ElementContainer = $element_container;

            fn push_row(&mut self, row: Self::ElementContainer) {
                assert!(row.len() == self.num_columns());
                if self.is_row_aligned() {
                    $pr_expr;
                    self.rows += 1;
                } else {
                    for c in (0..self.num_columns()).rev() {
                        self.matrix.insert(self.num_rows() * c + self.num_rows(), row[c]);
                    }
                    self.columns += 1;
                }
            }

            fn push_column(&mut self, column: Self::ElementContainer) {
                assert!(column.len() == self.num_rows());
                if self.is_column_aligned() {
                    $pc_expr;
                    self.rows += 1;
                } else {
                    for r in (0..self.num_rows()).rev() {
                        self.matrix.insert(self.num_columns() * r + self.num_columns(), column[r]);
                    }
                    self.columns += 1;
                }
            }

            fn try_push_row(&mut self, row: Self::ElementContainer) -> Result<(), MatrixError> {
                if row.len() != self.num_columns() {
                    return Err(MatrixError::FunctionError("Unable to push row to matrix - the row \
                    doesn't have the same number of elements as the matrix rows do.".to_string()));
                }
                if self.is_row_aligned() {
                    $pr_expr;
                    self.rows += 1;
                } else {
                    for c in (0..self.num_columns()).rev() {
                        self.matrix.insert(self.num_rows() * c + self.num_rows(), row[c]);
                    }
                    self.columns += 1;
                }
                Ok(())
            }

            fn try_push_column(&mut self, column: Self::ElementContainer) -> Result<(), MatrixError> {
                if column.len() != self.num_rows() {
                    return Err(MatrixError::FunctionError("Unable to push column to matrix - the \
                    column doesn't have the same number of elements as the matrix columns do."
                        .to_string()));
                }
                if self.is_column_aligned() {
                    $pc_expr;
                    self.rows += 1;
                } else {
                    for r in (0..self.num_rows()).rev() {
                        self.matrix.insert(self.num_columns() * r + self.num_columns(), column[r]);
                    }
                    self.columns += 1;
                }
                Ok(())
            }

            fn add_row(&mut self, location: usize, row: Self::ElementContainer) {
                assert!(row.len() == self.num_columns());
                assert!(location <= self.num_rows());
                if self.is_row_aligned() {
                    let (left, right) = self.matrix.split_at(location * self.num_columns());
                    let mut left = left.to_vec();
                    $ar_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.rows += 1;
                } else {
                    for c in (0..self.num_columns()).rev() {
                        self.matrix.insert(self.num_rows() * c + location, row[c]);
                    }
                    self.columns += 1;
                }
            }

            fn add_column(&mut self, location: usize, column: Self::ElementContainer) {
                assert!(column.len() == self.num_rows());
                assert!(location <= self.num_column());
                if self.is_column_aligned() {
                    let(left, right) = self.matrix.split_at(location * self.num_rows());
                    let mut left = left.to_vec();
                    $ac_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.rows += 1;
                } else {
                    for r in (0..self.num_rows()).rev() {
                        self.matrix.insert(self.num_columns() * r + location, column[r]);
                    }
                    self.columns += 1;
                }
            }

            fn try_add_row(&mut self, location: usize, row: Self::ElementContainer)
                -> Result<(), MatrixError> {
                if row.len() != self.num_columns() {
                    return Err(MatrixError::FunctionError("Attempted to add a row with an \
                    incorrect number of elements.".to_string()));
                }
                if !(location <= self.num_rows()) {
                    return Err(MatrixError::FunctionError("Attempted to add a row at an invalid \
                    index.".to_string()));
                }
                if self.is_row_aligned() {
                    let (left, right) = self.matrix.split_at(location * self.num_columns());
                    let mut left = left.to_vec();
                    $ar_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.rows += 1;
                } else {
                    for c in (0..self.num_columns()).rev() {
                        self.matrix.insert(self.num_rows() * c + location, row[c]);
                    }
                    self.columns += 1;
                }
                Ok(())
            }

            fn try_add_column(&mut self, location: usize, column: Self::ElementContainer)
                -> Result<(), MatrixError> {
                if column.len() != self.num_rows() {
                    return Err(MatrixError::FunctionError("Attempted to add a column with an \
                    incorrect number of elements.".to_string()));
                }
                if !(location <= self.num_columns()) {
                    return Err(MatrixError::FunctionError("Attemped to add a column at an invalid \
                    index.".to_string()));
                }
                if self.is_column_aligned() {
                    let (left, right) = self.matrix.split_at(location * self.num_rows());
                    let mut left = left.to_vec();
                    $ac_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.rows += 1;
                } else {
                    for r in (0..self.num_rows()).rev() {
                        self.matrix.insert(self.num_columns() * r + location, column[r]);
                    }
                    self.columns += 1;
                }
                Ok(())
            }

            fn push_rows(&mut self, rows: Self::ElementContainer) {
                assert!(rows.len() % self.num_columns() == 0);
                if self.is_row_aligned() {
                    $prs_expr;
                    self.rows += rows.len() / self.num_columns();
                } else {
                    for r in 0..rows.len() / self.num_columns() {
                        self.push_row(rows[r * self.num_columns()..(r + 1) * self.num_columns()]);
                    }
                    self.columns += rows.len() / self.num_columns();
                }
            }

            fn push_columns(&mut self, columns: Self::ElementContainer) {
                assert!(columns.len() % self.num_rows() == 0);
                if self.is_column_aligned() {
                    $ars_expr;
                    self.rows += rows.len() / self.num_columns();
                } else {
                    for c in 0..columns.len() / self.num_rows() {
                        self.push_column(columns[c * self.num_rows()..(c + 1) * self.num_rows()]);
                    }
                    self.columns += rows.len() / self.num_columns();
                }
            }

            fn try_push_rows(&mut self, rows: Self::ElementContainer) -> Result<(), MatrixError> {
                if rows.len() % self.num_columns() != 0 {
                    return Err(MatrixError::FunctionError("Attempted to push rows where the total \
                    number of elements is not divisible by the number of elements per row."
                        .to_string()));
                }
                if self.is_row_aligned() {
                    $prs_expr;
                    self.rows += rows.len() / self.num_columns();
                } else {
                    for r in 0..rows.len() / self.num_columns() {
                        self.push_row(rows[r * self.columns()..(r + 1) * self.num_columns()]);
                    }
                    self.columns += rows.len() / self.num_columns();
                }
                Ok(())
            }

            fn try_push_columns(&mut self, columns: Self::ElementContainer) -> Result<(), MatrixError> {
                if columns.len() % self.num_rows() != 0 {
                    return Err(MatrixError::FunctionError("Attempted to push columns where the \
                    total number of elements is not divisible by the number of columns per row."
                        .to_string()));
                }
                if self.is_column_aligned() {
                    $pcs_expr;
                    self.rows += rows.len() / self.num_columns();
                } else {
                    for c in 0..columns.len() / self.num_rows() {
                        self.push_column(columns[c * self.num_rows()..(c + 1) * self.num_rows()]);
                    }
                    self.columns += rows.len() / self.num_columns();
                }
                Ok(())
            }

            fn add_rows(&mut self, location: usize, rows: Self::ElementContainer) {
                assert!(rows.len() % self.num_columns() == 0);
                assert!(location <= self.num_rows());
                if self.is_row_aligned() {
                    let (left, right) = self.matrix.split_at(location * self.num_columns());
                    let mut left = left.to_vec();
                    $ars_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.rows += rows.len() / self.num_columns();
                } else {
                    for r in (0..rows.len() / self.num_columns()).rev() {
                        for c in (0..self.num_columns()).rev() {
                            self.matrix.insert(self.num_rows() * c + location,
                                rows[r * self.num_columns() + c]);
                        }
                    }
                    self.columns += rows.len() / self.num_columns();
                }
            }

            fn add_columns(&mut self, location: usize, columns: Self::ElementContainer) {
                assert!(columns.len() % self.num_rows() == 0);
                assert!(location <= self.num_columns());
                if self.is_column_aligned() {
                    let (left, right) = self.matrix.split_at(location * self.num_rows());
                    let mut left = left.to_vec();
                    $acs_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.columns += columns.len() / self.num_columns();
                } else {
                    for c in (0..columns.len() / self.num_rows()).rev() {
                        for r in (0..self.num_rows()).rev() {
                            self.matrix.insert(self.num_columns() * c + location,
                                columns[r * self.num_rows() + c]);
                        }
                    }
                    self.columns += columns.len() / self.num_rows();
                }
            }

            fn try_add_rows(&mut self, location: usize, rows: Self::ElementContainer)
                -> Result<(), MatrixError> {
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
                    let (left, right) = self.matrix.split_at(location * self.num_columns());
                    let mut left = left.to_vec();
                    $ars_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.rows += rows.len() / self.num_columns();
                } else {
                    for r in (0..rows.len() / self.num_columns()).rev() {
                        for c in (0..self.num_columns()).rev() {
                            self.matrix.insert(self.num_rows() * c + location,
                                rows[r * self.num_columns() + c]);
                        }
                    }
                    self.columns += rows.len() / self.num_columns();
                }
                Ok(())
            }

            fn try_add_columns(&mut self, location: usize, columns: Self::ElementContainer)
                -> Result<(), MatrixError> {
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
                    let (left, right) = self.matrix.split_at(location * self.num_rows());
                    let mut left = left.to_vec();
                    $acs_expr;
                    left.extend_from_slice(right);
                    self.matrix = left;
                    self.columns += columns.len() / self.num_columns();
                } else {
                    for c in (0..columns.len() / self.num_rows()).rev() {
                        for r in (0..self.num_rows()).rev() {
                            self.matrix.insert(self.num_columns() * c + location,
                                columns[r * self.num_rows() + c]);
                        }
                    }
                    self.columns += columns.len() / self.num_rows();
                }
                Ok(())
            }
        }
    }
}

add_elements_impl!{
    Matrix<T> {
        &'a [T],
        rows {
            push_row: self.matrix.extend_from_slice(row),
            add_row: left.extend_from_slice(row),
            push_rows: self.matrix.extend_from_slice(rows),
            add_rows: left.extend_from_slice(rows)
        },
        columns {
            push_column: {self.matrix.extend_from_slice(column)},
            add_column: {left.extend_from_slice(column)},
            push_columns: {self.matrix.extend_from_slice(columns)},
            add_columns: {left.extend_from_slice(columns)}
        }
    }
}

add_elements_impl!{
    AugmentedMatrix<T> {
        &'a [T],
        rows {
            push_row: self.matrix.extend_from_slice(row),
            add_row: left.extend_from_slice(row),
            push_rows: self.matrix.extend_from_slice(rows),
            add_rows: left.extend_from_slice(rows)
        },
        columns {
            push_column: {
                let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(column);
                left.extend_from_slice(right);
            },
            add_column: {left.extend_from_slice(column)},
            push_columns: {
                let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
                let mut left = left.to_vec();
                left.extend_from_slice(columns);
                left.extend_from_slice(right);
            },
            add_columns: {left.extend_from_slice(columns)}
        }
    }
}

add_elements_impl!{
    Matrix<T> {
        &'a mut Vec<T>,
        rows {
            push_row: self.matrix.append(row),
            add_row: left.append(row),
            push_rows: self.matrix.append(rows),
            add_rows: left.append(rows)
        },
        columns {
            push_column: {self.matrix.append(column)},
            add_column: {left.append(column)},
            push_columns: {self.matrix.append(columns)},
            add_columns: {left.append(columns)}
        }
    }
}

add_elements_impl!{
    AugmentedMatrix<T> {
        &'a mut Vec<T>,
        rows {
            push_row: self.matrix.append(row),
            add_row: left.append(row),
            push_rows: self.matrix.append(rows),
            add_rows: left.append(rows)
        },
        columns {
            push_column: {
                let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
                let mut left = left.to_vec();
                left.append(column);
                left.extend_from_slice(right);
            },
            add_column: {left.append(column)},
            push_columns: {
                let (left, right) = self.matrix.split_at(self.num_columns() * self.num_rows());
                let mut left = left.to_vec();
                left.append(columns);
                left.extend_from_slice(right);
            },
            add_columns: {left.append(columns)}
        }
    }
}