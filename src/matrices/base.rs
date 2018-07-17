#![allow(dead_code)]

use num::{Zero, One};

use std::ops::{Index, IndexMut, Range};
use std::fmt;
use std::mem::swap;

use fractions::base::Fraction;

/// Return value of [`get_alignment()`].
///
/// This describes whether a `Matrix<T>` or `AugmentedMatrix<T>` is row-aligned (where rows are
/// contiguous in memory) or column-aligned (where columns are contiguous in memory).
///
/// [`get_alignment()`]: ../base/struct.Matrix.html#method.get_alignment
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Alignment {
    RowAligned,
    ColumnAligned
}

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
    ($($target_type:ty) *) => ($(
        impl<T> Index<(usize, usize)> for $target_type {
            type Output = T;

            fn index<'a>(&'a self, index: (usize, usize)) -> &'a T {
                match self.alignment {
                    Alignment::RowAligned => &self[index.0][index.1],
                    Alignment::ColumnAligned => &self[index.1][index.0]
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
    ($($target_type:ty, $name:ident, {
        $splat_doc_expr:expr,
        $new_doc_expr:expr,
        $new_from_vec_doc_expr:expr,
        $set_matrix_doc_expr:expr,
        $in_place_transpose_doc_expr:expr,
        $row_align_doc_expr:expr,
        $column_align_doc_expr:expr,
        $get_alignment_doc_expr:expr,
        $num_rows_doc_expr:expr,
        $is_row_aligned_doc_expr:expr,
        $is_column_aligned_doc_expr:expr,
        $exactly_equal_doc_expr:expr
    });* ) => ($(
        impl<T: Clone> $target_type {
            #[doc = $splat_doc_expr]
            pub fn splat(value: &T, dimension: (usize, usize), alignment: Alignment) -> Self {
                let matr = vec![value.clone(); dimension.0 * dimension.1];
                $name {
                    rows: dimension.0,
                    columns: dimension.1,
                    matrix: matr,
                    alignment: alignment
                }
            }

            #[doc = $new_doc_expr]
            pub fn new(dimension: (usize, usize), alignment: Alignment) -> Self {
                let matr: Vec<T> = Vec::with_capacity(dimension.0 * dimension.1);
                if alignment == Alignment::RowAligned {
                    $name {
                        rows: dimension.0,
                        columns: dimension.1,
                        matrix: matr,
                        alignment: alignment
                    }
                } else {
                    $name {
                        rows: dimension.1,
                        columns: dimension.0,
                        matrix: matr,
                        alignment: alignment
                    }
                }
            }

            #[doc = $new_from_vec_doc_expr]
            pub fn new_from_vec(dimension: (usize, usize), vec: Vec<T>, alignment: Alignment)
                -> Result<$target_type, MatrixError> {
                if vec.len() != dimension.0 * dimension.1 {
                    return Err(MatrixError::InitError("The supplied vec does not have the same \
                    number of elements as the dimension specifies.".to_string()));
                }
                if alignment == Alignment::RowAligned {
                    Ok($name {
                        rows: dimension.0,
                        columns: dimension.1,
                        matrix: vec,
                        alignment: alignment
                    })
                } else {
                    Ok($name {
                        rows: dimension.1,
                        columns: dimension.0,
                        matrix: vec,
                        alignment: alignment
                    })
                }
            }

            #[doc = $set_matrix_doc_expr]
            pub fn set_matrix(&mut self, vec: Vec<T>) {
                assert!(vec.len() % self.rows == 0 && vec.len() % self.columns == 0);
                self.matrix = vec;
            }

            #[doc = $in_place_transpose_doc_expr]
            pub fn in_place_transpose(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => self.column_align(),
                    Alignment::ColumnAligned => self.row_align()
                }
            }

            #[doc = $row_align_doc_expr]
            pub fn row_align(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => return,
                    Alignment::ColumnAligned => {
                        let mut tmp = self.matrix.clone();
                        let mut cur_pos = 0;
                        for r in 0..self.num_rows() {
                            for c in 0..self.num_columns() {
                                swap(&mut self[(r, c)], &mut tmp[cur_pos]);
                                cur_pos += 1;
                            }
                        }
                        swap(&mut self.matrix, &mut tmp);
                        swap(&mut self.rows, &mut self.columns);
                        self.alignment = Alignment::RowAligned;
                    }
                }
            }

            #[doc = $column_align_doc_expr]
            pub fn column_align(&mut self) {
                match self.alignment {
                    Alignment::RowAligned => {
                        let mut tmp = self.matrix.clone();
                        let mut cur_pos = 0;
                        for c in 0..self.num_columns() {
                            for r in 0..self.num_rows() {
                                swap(&mut self[(r, c)], &mut tmp[cur_pos]);
                                cur_pos += 1;
                            }
                        }
                        swap(&mut self.matrix, &mut tmp);
                        swap(&mut self.rows, &mut self.columns);
                        self.alignment = Alignment::ColumnAligned;
                    },
                    Alignment::ColumnAligned => return
                }
            }
        }

        impl<T> $target_type {
            #[doc = $get_alignment_doc_expr]
            pub fn get_alignment(&self) -> Alignment {
                match self.alignment {
                    Alignment::RowAligned => Alignment::RowAligned,
                    Alignment::ColumnAligned => Alignment::ColumnAligned
                }
            }

            #[doc = $num_rows_doc_expr]
            pub fn num_rows(&self) -> usize {
                match self.alignment {
                    Alignment::RowAligned => self.rows,
                    Alignment::ColumnAligned => self.columns
                }
            }

            #[doc = $is_row_aligned_doc_expr]
            pub fn is_row_aligned(&self) -> bool {
                match self.alignment {
                    Alignment::RowAligned => true,
                    Alignment::ColumnAligned => false,
                }
            }

            #[doc = $is_column_aligned_doc_expr]
            pub fn is_column_aligned(&self) -> bool {
                match self.alignment {
                    Alignment::RowAligned => false,
                    Alignment::ColumnAligned => true
                }
            }
        }

        impl<T: PartialEq> $target_type {
            #[doc = $exactly_equal_doc_expr]
            pub fn exactly_equal_to(&self, other: &Matrix<T>) -> bool {
                if self.alignment != other.alignment {
                    return false;
                }
                if self.dimension() != other.dimension() {
                    return false;
                }
                for a in 0..self.rows {
                    for b in 0..self.columns {
                        if self[a][b] != other[a][b] {
                            return false;
                        }
                    }
                }
                true
            }
        }
    )*)
}

matrix_base_impls!{AugmentedMatrix<T>, AugmentedMatrix,
{
    "Makes a new matrix where all values are the supplied value.
    # Example
    ```rust
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix,
    #    Alignment::{RowAligned, ColumnAligned}};
    let foo = AugmentedMatrix::splat(&2, (2, 2), RowAligned);
    let mut bar = AugmentedMatrix::new((2, 2), RowAligned);
    bar.set_matrix(vec![2, 2, 2, 2]);
    assert_eq!(foo, bar);
    ```",
    "Creates a new empty matrix. Its contents can be initialized with `.set_matrix()`.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::{RowAligned,
    #    ColumnAligned}};
    let mut foo = AugmentedMatrix::new((3, 3), ColumnAligned);
    foo.set_matrix(vec![0, 3, 6, 1, 4, 7, 2, 5, 8]);
    let bar = augmented_matrix![
        0 1 => 2;
        3 4 => 5;
        6 7 => 8
    ];
    assert_eq!(foo, bar);
    ```",
    "Makes a new `AugmentedMatrix<T>` from a supplied `Vec<T>` and `(usize, usize)` designating the
    dimension. The product of the two tuple elements and the length of the `Vec<T>` must be
    equal to get an `Ok(AugmentedMatrix<T>)`.
    # Examples
    ```rust
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let aug_matr_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let aug_matr_res = AugmentedMatrix::new_from_vec((4, 4), aug_matr_vec, RowAligned);
    assert!(aug_matr_res.is_ok());
    ```
    ```rust
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let aug_matr_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let aug_matr_res = AugmentedMatrix::new_from_vec((3, 4), aug_matr_vec, RowAligned);
    assert!(aug_matr_res.is_err());
    ```",
    "Sets the contents of a currently existing `AugmentedMatrix<T>` to a provided `Vec<T>`. The
    provided vector must have the same number of elements as are in the matrix before modification.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = AugmentedMatrix::new((3, 3), RowAligned);
    foo.set_matrix(vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let bar = augmented_matrix![
        8 7 => 6;
        5 4 => 3;
        2 1 => 0
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned};
    let mut foo = AugmentedMatrix::new((3, 2), RowAligned);
    foo.set_matrix(vec![0, 1, 2, 3]);
    ```",
    "Swaps the alignment of an augmented matrix (row-aligned => column-aligned and vice versa).
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{AugmentedMatrix,
    #    Alignment::{RowAligned, ColumnAligned}};
    let mut foo = augmented_matrix![
        0 1 => 2;
        3 4 => 5;
        6 7 => 8
    ];
    foo.in_place_transpose();
    let bar = AugmentedMatrix::new_from_vec((3, 3), vec![0, 3, 6, 1, 4, 7, 2, 5, 8], ColumnAligned)
        .unwrap();
    assert_eq!(foo, bar);
    ```",
    "",
    "",
    "",
    "",
    "",
    "",
    ""
};
Matrix<T>, Matrix,
{
    "Makes a new matrix where all values are the supplied value.
    # Example
    ```rust
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix,
    #    Alignment::{RowAligned, ColumnAligned}};
    let foo = Matrix::splat(&2, (2, 2), RowAligned);
    let mut bar = Matrix::new((2, 2), RowAligned);
    bar.set_matrix(vec![2, 2, 2, 2]);
    assert_eq!(foo, bar);
    ```",
    "Creates a new empty matrix. Its contents can be initialized with [`set_matrix()`].
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::{RowAligned, ColumnAligned}};
    let mut foo = Matrix::new((3, 2), ColumnAligned);
    foo.set_matrix(vec![0, 2, 4, 1, 3, 5]);
    let bar = matrix![
        0 1;
        2 3;
        4 5
    ];
    assert_eq!(foo, bar);
    ```
    [`set_matrix()`]: ../base/struct.Matrix.html#method.set_matrix",
    "Makes a new `Matrix<T>` from a supplied `Vec<T>` and `(usize, usize)` designating the
    dimension. The product of the two tuple elements and the length of the `Vec<T>` must be
    equal to get an `Ok(Matrix<T>)`.
    # Examples
    ```rust
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let matr_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let matr_res = Matrix::new_from_vec((4, 4), matr_vec, RowAligned);
    assert!(matr_res.is_ok());
    ```
    ```rust
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let matr_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let matr_res = Matrix::new_from_vec((3, 4), matr_vec, RowAligned);
    assert!(matr_res.is_err());
    ```",
    "Sets the contents of a currently existing `Matrix<T>` to a provided `Vec<T>`. The provided
    vector must have the same number of elements as are in the matrix before modification.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = Matrix::new((3, 3), RowAligned);
    foo.set_matrix(vec![8, 7, 6, 5, 4, 3, 2, 1, 0]);
    let bar = matrix![
        8 7 6;
        5 4 3;
        2 1 0
    ];
    assert_eq!(foo, bar);
    ```
    # Panics
    ```should_panic
    # extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    let mut foo = Matrix::new((3, 2), RowAligned);
    foo.set_matrix(vec![0, 1, 2, 3]);
    ```",
    "Swaps the alignment of a matrix (row-aligned => column-aligned and vice versa).
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::{RowAligned, ColumnAligned}};
    let mut foo = matrix![
        0 1 2;
        3 4 5;
        6 7 8
    ];
    foo.in_place_transpose();
    let bar = Matrix::new_from_vec((3, 3), vec![0, 3, 6, 1, 4, 7, 2, 5, 8], ColumnAligned).unwrap();
    assert_eq!(foo, bar);
    ```",
    "Row-aligns a matrix. If a matrix is already row-aligned, then nothing happens.
    # Example
    ```rust
    # #[macro_use] extern crate fractions_and_matrices;
    # use fractions_and_matrices::matrices::base::{Matrix, Alignment::{RowAligned, ColumnAligned}};
    let foo = Matrix::new_from_vec((5, 5), vec![0, 5, 10, 15, 20, 1, 6, 11, 16, 21, 2, 7, 12, 17,
        22, 3, 8, 13, 18, 23, 4, 9, 14, 19, 24], ColumnAligned).unwrap();
    let mut bar = foo.clone();
    bar.row_align();
    assert_eq!(foo[0][0], bar[0][0]);
    assert_eq!(foo[0][4], bar[4][0]);
    assert_eq!(foo[4][4], bar[4][4]);
    assert_eq!(foo[4][0], bar[0][4]);
    assert_eq!(foo, bar);
    ```",
    "",
    "",
    "",
    "",
    "",
    ""
}}

impl<T> Matrix<T> {
    /// Gets the dimension of a given matrix as a `(usize, usize)` tuple.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// let foo = matrix![
    ///     0 1 2;
    ///     3 4 5;
    ///     6 7 8
    /// ];
    /// assert_eq!(foo.dimension(), (3, 3));
    /// ```
    pub fn dimension(&self) -> (usize, usize) {
        match self.alignment {
            Alignment::RowAligned => (self.rows, self.columns),
            Alignment::ColumnAligned => (self.columns, self.rows)
        }
    }

    /// Gets the number of columns of a given matrix.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned};
    /// let foo = matrix![
    ///     0 1;
    ///     2 3
    /// ];
    /// assert_eq!(foo.num_columns(), 2);
    /// ```
    pub fn num_columns(&self) -> usize {
        match self.alignment {
            Alignment::RowAligned => self.columns,
            Alignment::ColumnAligned => self.rows
        }
    }
}

impl<T> AugmentedMatrix<T> {
    /// Gets the dimension of a given `AugmentedMatrix<T>` as a `(usize, usize)` tuple. NB: the
    /// returned dimension does not include the solution column.
    pub fn dimension(&self) -> (usize, usize) {
        match self.alignment {
            Alignment::RowAligned => (self.rows, self.columns - 1),
            Alignment::ColumnAligned => (self.columns, self.rows - 1)
        }
    }

    /// Gets the number of columns in a given augmented matrix. NB: the returned value does not
    /// include the solution column.
    pub fn num_columns(&self) -> usize {
        match self.alignment {
            Alignment::RowAligned => self.columns - 1,
            Alignment::ColumnAligned => self.rows - 1
        }
    }
}

/// Used to specify general types of errors in matrices.
pub enum MatrixError {
    /// Returned when an attempt to create a `Matrix<T>` or `AugmentedMatrix<T>` fails.
    InitError(String),
    /// Returned in case of failure by methods with a `try_` prefix in any "transforms" files.
    TransformError(String),
    /// Returned in case of failure by methods or functions with a `try_` prefix in any "arithmetic"
    /// or "functions" files.
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

/// Used for conveniently testing whether a matrix/augmented matrix is a unit or creating a unit
/// `Matrix<T>`/`AugmentedMatrix<T>`.
pub trait Unit {
    /// Constructs a unit `Matrix<T>`/`AugmentedMatrix<T>`.
    fn unit(dimension: usize) -> Self;
    /// Tests whether a `Matrix<T>`/`AugmentedMatrix<T>` is of a unit dimension.
    fn is_unit_dimension(&self) -> bool;
    /// Tests whether a `Matrix<T>`/`AugmentedMatrix<T>` is a unit matrix.
    fn is_unit(&self) -> bool;
}

impl<T: PartialEq + Clone + Zero + One> Unit for Matrix<T> {
    /// Creates a unit `Matrix<T>`.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned, Unit};
    /// let foo: Matrix<i32> = Matrix::unit(3);
    /// let bar = matrix![
    ///     1 0 0;
    ///     0 1 0;
    ///     0 0 1
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    fn unit(dimension: usize) -> Matrix<T> {
        let mut res = Matrix::splat(&T::zero(), (dimension, dimension), Alignment::RowAligned);
        for a in 0..res.rows {
            res[(a, a)] = T::one();
        }
        res
    }

    /// Basically just a convenience check to see whether the number of rows and columns are equal.
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned, Unit};
    /// let foo = matrix![
    ///     1 0 1;
    ///     0 1 0;
    ///     1 0 1
    /// ];
    /// assert!(foo.is_unit_dimension());
    /// ```
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned, Unit};
    /// let foo = matrix![
    ///     0 1 0;
    ///     1 0 1
    /// ];
    /// assert!(!foo.is_unit_dimension());
    /// ```
    fn is_unit_dimension(&self) -> bool {
        self.rows == self.columns
    }

    /// Checks to see whether a given matrix is a unit matrix.
    /// # Examples
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned, Unit};
    /// let foo = matrix![
    ///     1 0 0;
    ///     0 1 0;
    ///     0 0 1
    /// ];
    /// assert!(foo.is_unit());
    /// ```
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{Matrix, Alignment::RowAligned, Unit};
    /// let foo = matrix![
    ///     1 0 1;
    ///     0 1 0
    /// ];
    /// assert!(!foo.is_unit());
    /// ```
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
    /// Creates a unit `AugmentedMatrix<T>`.
    /// # Example
    /// ```rust
    /// # #[macro_use] extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::matrices::base::{AugmentedMatrix, Alignment::RowAligned,
    /// #    Unit};
    /// let foo: AugmentedMatrix<i32> = AugmentedMatrix::unit(3);
    /// let bar = augmented_matrix![
    ///     1 0 0 => 0;
    ///     0 1 0 => 0;
    ///     0 0 1 => 0
    /// ];
    /// assert_eq!(foo, bar);
    /// ```
    fn unit(dimension: usize) -> AugmentedMatrix<T> {
        let mut res = AugmentedMatrix::splat(&T::zero(), (dimension, dimension + 1),
                                             Alignment::RowAligned);
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