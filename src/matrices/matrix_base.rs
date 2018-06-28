#![allow(dead_code)]

use num::{Zero, One};

use std::ops::{Index, IndexMut, Range};
use std::fmt;
use std::mem::swap;

use fractions::fractions::Fraction;

#[derive(PartialEq, Clone)]
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

macro_rules! matrix_index_methods {
    ($($target_type:ty)* ) => ($(
        impl<T> Index<(usize, usize)> for $target_type {
            type Output = T;

            fn index<'a>(&'a self, index: (usize, usize)) -> &'a T {
                match self.alignment {
                    Alignment::RowAligned => &self.matrix[index.0 * self.num_columns() + index.1],
                    Alignment::ColumnAligned => &self.matrix[index.0 * self.num_rows() + index.1]
                }
            }
        }

        impl<T> Index<usize> for $target_type {
            type Output = [T];

            fn index<'a>(&'a self, index: usize) -> &'a [T] {
                &self.matrix[(index * self.columns)..((index + 1) * self.columns)]
            }
        }

        impl<T> Index<Range<usize>> for $target_type {
            type Output = [T];

            fn index<'a>(&'a self, index: Range<usize>) -> &'a [T] {
                &self.matrix.as_slice()[(index.start * self.columns)..(index.end * self.columns)]
            }
        }

        impl<T> IndexMut<(usize, usize)> for $target_type {
            fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut T {
                match self.alignment {
                    Alignment::RowAligned => &mut self[index.0][index.1],
                    Alignment::ColumnAligned => &mut self[index.1][index.0]
                }
            }
        }

        impl<T> IndexMut<usize> for $target_type {
            fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [T] {
                &mut self.matrix.as_mut_slice()[(index * self.columns)..((index + 1) * self.columns)]
            }
        }

        impl<T> IndexMut<Range<usize>> for $target_type {
            fn index_mut<'a>(&'a mut self, index: Range<usize>) -> &'a mut [T] {
                &mut self.matrix.as_mut_slice()[(index.start * self.columns)..(index.end * self.columns)]
            }
        }
    )*)
}

matrix_index_methods!{AugmentedMatrix<T> Matrix<T>}

macro_rules! matrix_base_impls {
    ($($target_type:ty, $name:ident);* ) => ($(
        impl<T: Clone> $target_type {
            pub fn splat(value: &T, dimension: (usize, usize), alignment: Alignment) -> Self {
                let matr = vec![value.clone(); dimension.0 * dimension.1];
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
                let mut tmp = vec![self[(0, 0)].clone(); self.rows * self.columns];
                swap(&mut self.matrix, &mut tmp);
                let mut cur_pos = 0;
                for a in 0..self.dimension().1 {
                    for b in 0..self.dimension().0 {
                        swap(&mut self.matrix[cur_pos], &mut tmp[a + b * self.columns]);
                        cur_pos += 1;
                    }
                }
            }

            pub fn row_align(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => return,
                    Alignment::ColumnAligned => {
                        self.in_place_transpose();
                        swap(&mut self.rows, &mut self.columns);
                        self.alignment = Alignment::ColumnAligned;
                    }
                }
            }

            pub fn column_align(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => {
                        self.in_place_transpose();
                        swap(&mut self.rows, &mut self.columns);
                        self.alignment = Alignment::RowAligned;
                    },
                    Alignment::ColumnAligned => return
                }
            }
        }

        impl<T> $target_type {
            pub fn get_alignemt(&self) -> Alignment {
                match self.alignment {
                    Alignment::RowAligned => Alignment::RowAligned,
                    Alignment::ColumnAligned => Alignment::ColumnAligned
                }
            }

            pub fn num_rows(&self) -> usize {
                match self.alignment {
                    Alignment::RowAligned => self.rows,
                    Alignment::ColumnAligned => self.columns
                }
            }

            pub fn is_row_aligned(&self) -> bool {
                match self.alignment {
                    Alignment::RowAligned => true,
                    Alignment::ColumnAligned => false,
                }
            }

            pub fn is_column_aligned(&self) -> bool {
                match self.alignment {
                    Alignment::RowAligned => false,
                    Alignment::ColumnAligned => true
                }
            }
        }
    )*)
}

matrix_base_impls!{AugmentedMatrix<T>, AugmentedMatrix; Matrix<T>, Matrix}

impl<T> Matrix<T> {
    pub fn dimension(&self) -> (usize, usize) {
        match self.alignment {
            Alignment::RowAligned => (self.rows, self.columns),
            Alignment::ColumnAligned => (self.columns, self.rows)
        }
    }

    pub fn num_columns(&self) -> usize {
        match self.alignment {
            Alignment::RowAligned => self.columns,
            Alignment::ColumnAligned => self.rows
        }
    }
}

impl<T> AugmentedMatrix<T> {
    pub fn dimension(&self) -> (usize, usize) {
        match self.alignment {
            Alignment::RowAligned => (self.rows, self.columns - 1),
            Alignment::ColumnAligned => (self.columns, self.rows - 1)
        }
    }

    pub fn num_columns(&self) -> usize {
        match self.alignment {
            Alignment::RowAligned => self.columns - 1,
            Alignment::ColumnAligned => self.rows - 1
        }
    }
}

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

pub trait Unit {
    fn unit(dimension: usize) -> Self;
    fn is_unit_dimension(&self) -> bool;
    fn is_unit(&self) -> bool;
}

impl<T: PartialEq + Clone + Zero + One> Unit for Matrix<T> {
    fn unit(dimension: usize) -> Matrix<T> {
        let mut res = Matrix::splat(&T::zero(), (dimension, dimension), ROW_ALIGNED);
        for a in 0..res.rows {
            res[(a, a)] = T::one();
        }
        res
    }

    fn is_unit_dimension(&self) -> bool {
        self.rows == self.columns
    }

    fn is_unit(&self) -> bool {
        if !self.is_unit_dimension() {
            return false;
        }
        for r in 0..self.rows {
            for c in 0..self.columns {
                if r != c {
                    if !self[(r, c)].is_zero() {
                        return false;
                    }
                } else {
                    if !self[(r, c)].is_one() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl<T: PartialEq + Clone + Zero + One> Unit for AugmentedMatrix<T> {
    fn unit(dimension: usize) -> AugmentedMatrix<T> {
        let mut res = AugmentedMatrix::splat(&T::zero(), (dimension, dimension + 1), ROW_ALIGNED);
        for a in 0..res.rows {
            res[(a, a)] = T::one();
        }
        res
    }

    fn is_unit_dimension(&self) -> bool {
        self.rows + 1 == self.columns
    }

    fn is_unit(&self) -> bool {
        if !self.is_unit_dimension() {
            return false;
        }
        for r in 0..self.rows {
            for c in 0..self.columns - 1 {
                if r != c {
                    if !self[(r, c)].is_zero() {
                        return false;
                    }
                } else {
                    if !self[(r, c)].is_one() {
                        return false;
                    }
                }
            }
        }
        true
    }
}