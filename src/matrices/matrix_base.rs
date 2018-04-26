#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use fractions::fractions::Fraction;

#[derive(Clone)]
pub struct FracMatrix {
    pub dimension: (usize, usize),
    pub matrix: Vec<Vec<Fraction>>,
    pub augmented: bool
}

#[derive(Clone)]
pub struct Matrix<T> {
    pub dimension: (usize, usize),
    pub matrix: Vec<Vec<T>>,
    pub augmented: bool
}

impl<T: Clone> Matrix<T> {
    pub fn splat(value: &T, dimension: (usize, usize), augmented: bool) -> Self {
        let mut matr: Vec<Vec<T>> = Vec::new();
        for _i in 0..dimension.0 {
            let mut row: Vec<T> = Vec::new();
            for _j in 0..dimension.1 {
                row.push(value.clone());
            }
            matr.push(row);
        }
        Matrix {
            dimension: dimension,
            matrix: matr,
            augmented: augmented
        }
    }

    pub fn new(dimension: (usize, usize), augmented: bool) -> Self {
        let mut matr: Vec<Vec<T>> = Vec::with_capacity(dimension.0);
        for _ in 0..dimension.0 {
            matr.push({
                let tmp: Vec<T> = Vec::with_capacity(dimension.1);
                tmp
            });
        }
        Matrix {
            dimension: dimension,
            matrix: matr,
            augmented: augmented
        }
    }
}

#[macro_export]
macro_rules! matrix {
    ($(($b:expr), ($($v:expr),*)||*)*) => {{
        let mut matr = Vec::new();
        $(
            let mut row = Vec::new();
            $(
                row.push($v);
            )*
            matr.push(row);
        )*
        for i in 0..matr.len() - 1 {
            for j in i + 1..matr.len() {
                if matr[i].len() != matr[j].len() {
                    panic!("Invalid matrix - rows do not all have the same length.");
                }
            }
        }
        Matrix {
            dimension: (matr.len(), matr[0].len()),
            matrix: matr,
            augmented: $b
        }
    }};
}

fn valid_operation_check(d1: &(usize, usize), d2: &(usize, usize), augmented_bools: (bool, bool)) {
    if d1.0 == 0 {
        panic!("Matrix on the left of the operand has 0 rows.");
    }
    if d1.1 == 0 {
        panic!("Matrix on the left of the operand has 0 columns.");
    }
    if d2.0 == 0 {
        panic!("Matrix on the right of the operand has 0 rows.");
    }
    if d2.1 == 0 {
        panic!("Matrix on the right of the operand has 0 columns.");
    }
    if augmented_bools.0 {
        panic!("Attempted to do matrix arithmetic with an augmented matrix (left).");
    }
    if augmented_bools.1 {
        panic!("Attempted to do matrix arithmetic with an augmented matrix (right).");
    }
}

fn add_sub_valid_operation_check(d1: &(usize, usize), d2: &(usize, usize),
                            augmented_bools: (bool, bool)) {
    if d1.0 != d2.0 && d1.1 != d2.1 {
        panic!("The matrices do not have an equal number of rows or columns.");
    }
    if d1.0 != d2.0 {
        panic!("The matrices do not have an equal number of rows.");
    }
    if d1.1 != d2.1 {
        panic!("The matrices do not have an equal number of columns.");
    }
    valid_operation_check(d1, d2, augmented_bools);
}

impl<T, U> Add<Matrix<U>> for Matrix<T>
    where
        T: Add + Clone + From<U> + From<<T as Add<T>>::Output>,
        U: Add<T> + Clone + Add<U>,
        <T as Add>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn add(mut self, rhs: Matrix<U>) -> Self {
        add_sub_valid_operation_check(&self.dimension, &rhs.dimension,
                                 (self.augmented, rhs.augmented));
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                self.matrix[i][j] = (self.matrix[i][j].clone() + rhs.matrix[i][j].clone().into())
                    .into();
            }
        }
        self
    }
}

impl<T, U> AddAssign<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + From<U>,
        U: AddAssign<T> + Clone + AddAssign<U>, {
    fn add_assign(&mut self, rhs: Matrix<U>) {
        add_sub_valid_operation_check(&self.dimension, &rhs.dimension,
                                 (self.augmented, rhs.augmented));
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                self.matrix[i][j] += rhs.matrix[i][j].clone().into();
            }
        }
    }
}

impl<T, U> Sub<Matrix<U>> for Matrix<T>
    where
        T: Sub + Clone + From<U> + From<<T as Sub<T>>::Output>,
        U: Sub<T> + Clone + Sub<U>,
        <T as Sub>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn sub(mut self, rhs: Matrix<U>) -> Self {
        add_sub_valid_operation_check(&self.dimension, &rhs.dimension,
                                 (self.augmented, rhs.augmented));
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                self.matrix[i][j] = (self.matrix[i][j].clone() - rhs.matrix[i][j].clone().into())
                    .into();
            }
        }
        self
    }
}

impl<T, U> SubAssign<Matrix<U>> for Matrix<T>
    where
        T: SubAssign + From<U>,
        U: SubAssign<T> + Clone + SubAssign<U>, {
    fn sub_assign(&mut self, rhs: Matrix<U>) {
        add_sub_valid_operation_check(&self.dimension, &rhs.dimension,
                                 (self.augmented, rhs.augmented));
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.1 {
                self.matrix[i][j] -= rhs.matrix[i][j].clone().into();
            }
        }
    }
}

fn mul_div_valid_operation_check(d1: &(usize, usize), d2: &(usize, usize),
                                  augmented_bools: (bool, bool)) {
    if d1.1 != d2.0 {
        panic!("The matrix on the left of the operand does not have the same number of columns as \
        the number of rows in the matrix on the right of the operand.");
    }
    valid_operation_check(d1, d2, augmented_bools);
}

impl<T, U> Mul<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + From<i32>
            + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        <T as Mul>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<U>) -> Self {
        mul_div_valid_operation_check(&self.dimension, &rhs.dimension,
                                      (self.augmented, rhs.augmented));
        let mut ret = Matrix::new((self.dimension.0, rhs.dimension.1), false);
        for a in 0..self.dimension.0 {
            for o in 0..rhs.dimension.1 {
                let mut total = T::from(0);
                let other_column = (0..rhs.dimension.0)
                    .map(|i| rhs.matrix[i][o].clone().into()).collect::<Vec<T>>();
                for b in 0..self.dimension.1 {
                    let new = self.matrix[a][b].clone() * other_column[b].clone().into();
                    total += new.into();
                }
                ret.matrix[a][o] = total;
            }
        }
        ret
    }
}

impl<T, U> MulAssign<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + MulAssign + Clone + From<i32>
            + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>, {
    fn mul_assign(&mut self, rhs: Matrix<U>) {
        mul_div_valid_operation_check(&self.dimension, &rhs.dimension,
                                      (self.augmented, rhs.augmented));
        let mut ret = Matrix::new((self.dimension.0, rhs.dimension.1), false);
        for a in 0..self.dimension.0 {
            for o in 0..rhs.dimension.1 {
                let mut total = T::from(0i32);
                let other_column = (0..rhs.dimension.0)
                    .map(|i| rhs.matrix[i][o].clone().into()).collect::<Vec<T>>();
                for b in 0..self.dimension.1 {
                    let new = self.matrix[a][b].clone() * other_column[b].clone().into();
                    total += new.into();
                }
                ret.matrix[a][o] = total;
            }
        }
        *self = ret;
    }
}

impl<T, U> Div<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + MulAssign + Div + DivAssign + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        <T as Mul>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn div(self, rhs: Matrix<U>) -> Self {
        mul_div_valid_operation_check(&self.dimension, &rhs.dimension,
                                      (self.augmented, rhs.augmented));
        let mut ret = Matrix::new((self.dimension.0, rhs.dimension.1), false);
        for a in 0..self.dimension.0 {
            for o in 0..rhs.dimension.1 {
                let mut total = T::from(0);
                let other_column = (0..rhs.dimension.0)
                    .map(|i| rhs.matrix[i][o].clone().into()).collect::<Vec<T>>();
                for b in 0..self.dimension.1 {
                    let new = self.matrix[a][b].clone() * other_column[b].clone().into();
                    total += new.into();
                }
                ret.matrix[a][o] = total;
            }
        }
        ret
    }
}

impl<T, U> DivAssign<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>, {
    fn div_assign(&mut self, rhs: Matrix<U>) {
        mul_div_valid_operation_check(&self.dimension, &rhs.dimension,
                                      (self.augmented, rhs.augmented));
        let mut ret = Matrix::new((self.dimension.0, rhs.dimension.1), false);
        for a in 0..self.dimension.0 {
            for o in 0..rhs.dimension.1 {
                let mut total = T::from(0i32);
                let other_column = (0..rhs.dimension.0)
                    .map(|i| rhs.matrix[i][o].clone().into()).collect::<Vec<T>>();
                for b in 0..self.dimension.1 {
                    let new = self.matrix[a][b].clone() * other_column[b].clone().into();
                    total += new.into();
                }
                ret.matrix[a][o] = total;
            }
        }
        *self = ret;
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.dimension.1);
        for _ in 0..self.dimension.1 {
            longest_in_column.push(0);
        }
        for a in 0..self.dimension.0 {
            for b in 0..self.dimension.1 {
                if format!("{:?}", self.matrix[a][b]).len() > longest_in_column[b] {
                    longest_in_column[b] = format!("{:?}", self.matrix[a][b]).len();
                }
            }
        }
        for a in 0..self.dimension.0 {
            let mut line = format!("Row {}: ", a); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.dimension.1 {
                let mut spacer_left = String::from("");
                let elem_string = format!("{:?}", self.matrix[a][b]);
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if self.augmented {
                    if b == self.dimension.1 - 1 && self.augmented {
                        line = format!("{}| {}{}", line, spacer_left, elem_string);
                    } else if b == self.dimension.1 - 2 {
                        line = format!("{}{}{} ", line, spacer_left, elem_string);
                    } else {
                        line = format!("{}{}{}  ", line, spacer_left, elem_string);
                    }
                } else {
                    if b == self.dimension.1 - 1 {
                        line = format!("{}{}{}", line, spacer_left, elem_string);
                    } else {
                        line = format!("{}{}{}, ", line, spacer_left, elem_string);
                    }
                }
            }
            // Add line to matrix string, add newline if it's not the last line
            if a < self.dimension.0 {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", format!("Dimension: ({}, {}) | Augmented: {}\n{}", self.dimension.0,
                                self.dimension.1, self.augmented, matr))
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.dimension.1);
        for _ in 0..self.dimension.1 {
            longest_in_column.push(0);
        }
        for a in 0..self.dimension.0 {
            for b in 0..self.dimension.1 {
                if self.matrix[a][b].to_string().len() > longest_in_column[b] {
                    longest_in_column[b] = self.matrix[a][b].to_string().len();
                }
            }
        }
        for a in 0..self.dimension.0 {
            let mut line = String::from(""); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            if a == 0 {
                line = format!("⎡ {}", line);
            } else if a == self.dimension.0 - 1 {
                line = format!("⎣ {}", line);
            } else {
                line = format!("⎢ {}", line);
            }
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.dimension.1 {
                let mut spacer_left = String::from("");
                let elem_string = self.matrix[a][b].to_string();
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if self.augmented {
                    if b == self.dimension.1 - 1 && self.augmented {
                        line = format!("{}| {}{}", line, spacer_left, elem_string);
                    } else if b == self.dimension.1 - 2 {
                        line = format!("{}{}{} ", line, spacer_left, elem_string);
                    } else {
                        line = format!("{}{}{}  ", line, spacer_left, elem_string);
                    }
                } else {
                    if b == self.dimension.1 - 1 {
                        line = format!("{}{}{}", line, spacer_left, elem_string);
                    } else {
                        line = format!("{}{}{}, ", line, spacer_left, elem_string);
                    }
                }
            }
            // Append appropriate end symbol for bracket section at the end of each line
            if a == 0 {
                line = format!("{} ⎤", line);
            } else if a == self.dimension.0 - 1 {
                line = format!("{} ⎦", line);
            } else {
                line = format!("{} ⎥", line);
            }
            // Add line to matrix string, add newline if it's not the last line
            if a == self.dimension.0 - 1 {
                matr = format!("{}{}", matr, line);
            } else {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", matr)
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

impl fmt::Debug for FracMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matr = self.to_string();
        let mut new_matr = "".to_string();
        let no_lines = matr.lines().count();
        for (i, line) in matr.lines().enumerate().take_while(|&(i, _)| i < no_lines - 1) {
            new_matr = format!("{}Row {}: {}\n", new_matr, i, line);
        }
        new_matr = format!("{}Row {}: {}", new_matr, no_lines - 1,
                           new_matr.lines().rev().next().unwrap());
        write!(f, "{}", new_matr)
    }
}

impl fmt::Display for FracMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matr = self.to_string();
        write!(f, "{}", matr)
    }
}