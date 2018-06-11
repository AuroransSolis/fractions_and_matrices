#![allow(dead_code)]

use std::fmt;

use fractions::fractions::Fraction;

#[derive(Clone)]
pub struct FracMatrix {
    pub dimension: (usize, usize),
    pub matrix: Vec<Vec<Fraction>>,
    pub augmented: bool
}

#[derive(Eq)]
pub enum Alignment {
    RowAligned,
    ColumnAligned
}

pub const ROW_ALIGNED: Alignment = Alignment::RowAligned;

pub const COLUMN_ALIGNED: Alignment = Alignment::ColumnAligned;

#[derive(Clone)]
pub struct Matrix<T> {
    pub(crate) rows: usize,
    pub(crate) columns: usize,
    pub(crate) matrix: Vec<T>,
    pub(crate) alignment: Alignment
}

#[derive(Clone)]
pub struct AugmentedMatrix<T> {
    pub(crate) rows: usize,
    pub(crate) columns: usize,
    pub(crate) matrix: Vec<T>,
    pub(crate) alignment: Alignment
}

macro_rules! matrix_base_impls {
    ($($target_type:ty, $name:ident);* ) => ($(
        impl<T: Clone> $target_type {
            pub fn splat(value: &T, dimension: (usize, usize), alignment: Alignment) -> Self {
                let matr = vec![*value; dimension.0 * dimension.1];
                $name {
                    rows: dimension.0,
                    columns: dimension.1,
                    matrix: matr,
                    alignment: alignment
                }
            }

            pub fn new(dimension: (usize, usize), alignment: Alignment) -> Self {
                let matr: Vec<T> = Vec::with_capacity(dimension.0 * dimension.1);
                $name {
                    rows: dimension.0,
                    columns: dimension.1,
                    matrix: matr,
                    alignment: alignment
                }
            }

            fn in_place_transpose(&mut self) {
                let mut tmp = vec![*self[(0, 0)]; self.dimension.0 * self.dimension.1];
                std::mem::swap(&mut self.matrix, &mut tmp);
                let mut cur_pos = 0;
                for a in 0..self.dimension.1 {
                    for b in 0..self.dimension.0 {
                        std::mem::swap(&mut self.matrix[cur_pos], &mut tmp[a + b * self.dimension.1]);
                        cur_pos += 1;
                    }
                }
            }

            pub fn row_align(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => return,
                    Alignment::ColumnAligned => {
                        self.in_place_transpose();
                        std::mem::swap(&mut self.dimension.0, &mut self.dimension.1);
                        self.alignment = Alignment::ColumnAligned;
                    }
                }
            }

            pub fn column_align(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => {
                        self.in_place_transpose();
                        std::mem::swap(&mut self.dimension.0, &mut self.dimension.1);
                        self.alignment = Alignment::RowAligned;
                    },
                    Alignment::ColumnAligned => return
                }
            }
        }

        impl<T> $target_type {
            pub fn dimension(&self) -> (usize, usize) {
                match self.alignment {
                    &Alignment::RowAligned => (self.rows, self.columns),
                    &Alignment::ColumnAligned => (self.columns, self.rows)
                }
            }

            pub fn num_rows(&self) -> usize {
                match self.alignment {
                    &Alignment::RowAligned => self.rows,
                    &Alignment::ColumnAligned => self.columns
                }
            }

            pub fn num_columns(&self) -> usize {
                match self.alignment {
                    &Alignment::RowAligned => self.columns,
                    &Alignment::ColumnAligned => self.rows
                }
            }

            pub fn get_alignemt(&self) -> Alignment {
                match self.alignment {
                    &Alignment::RowAligned => Alignment::RowAligned,
                    &Alignment::ColumnAligned => Alignment::ColumnAligned
                }
            }

            pub fn is_row_aligned(&self) -> bool {
                match self.alignment {
                    &Alignment::RowAligned => true,
                    &Alignment::ColumnAligned => false,
                }
            }

            pub fn is_column_aligned(&self) -> bool {
                match self.alignment {
                    &Alignment::RowAligned => false,
                    &Alignment::ColumnAligned => true
                }
            }
        }
    )*)
}

matrix_base_impls!{AugmentedMatrix<T>, AugmentedMatrix; Matrix<T>, Matrix}

pub enum MatrixError {
    InitError(String),
    TransformError(String),
    FunctionError(String)
}

impl fmt::Debug for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MatrixError::InitError(ref e) => write!(f, "Initialization error: {}", e),
            &MatrixError::TransformError(ref e) => write!(f, "Row/Matrix operation error: {}", e),
            &MatrixError::FunctionError(ref e) => write!(f, "Function error: {}", e)
        }
    }
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MatrixError::InitError(ref e) => write!(f, "Initialization error: {}", e),
            &MatrixError::TransformError(ref e) => write!(f, "Row/Matrix operation error: {}", e),
            &MatrixError::FunctionError(ref e) => write!(f, "Function error: {}", e)
        }
    }
}