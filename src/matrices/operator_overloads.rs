use num::Zero;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Range};
use std::cmp::PartialEq;
use std::fmt::Display;

use matrices::base::{AugmentedMatrix, Matrix, Alignment};
use matrices::transforms::Inverse;

macro_rules! partial_eq_impl {
    ($($target_type:ty | $ref_target_type:ty),*) => ($(
        impl<T: PartialEq> PartialEq for $target_type {
            fn eq(&self, other: $ref_target_type) -> bool {
                if self.num_columns() != other.num_columns() {
                    return false;
                }
                if self.num_rows() != other.num_rows() {
                    return false;
                }
                for i in 0..self.num_rows() {
                    for j in 0..self.num_columns() {
                        if self[(i, j)] != other[(i, j)] {
                            return false;
                        }
                    }
                }
                true
            }
        }
    )*)
}

partial_eq_impl!{Matrix<T> | &Matrix<T>, AugmentedMatrix<T> | &AugmentedMatrix<T>}

fn valid_operation_check(d1: (usize, usize), d2: (usize, usize), ) {
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
}

fn add_sub_valid_operation_check(d1: (usize, usize), d2: (usize, usize)) {
    if d1.0 != d2.0 && d1.1 != d2.1 {
        panic!("The matrices do not have an equal number of rows or columns.");
    }
    if d1.0 != d2.0 {
        panic!("The matrices do not have an equal number of rows.");
    }
    if d1.1 != d2.1 {
        panic!("The matrices do not have an equal number of columns.");
    }
    valid_operation_check(d1, d2);
}

impl<T, U> Add<Matrix<U>> for Matrix<T>
    where
        T: AddAssign<T> + Clone,
        U: Into<T> + Clone, {
    type Output = Matrix<T>;

    fn add(mut self, rhs: Matrix<U>) -> Self {
        add_sub_valid_operation_check(self.dimension(), rhs.dimension());
        if self.alignment == rhs.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] += rhs[i][j].clone().into();
                }
            }
            self
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] += rhs[(i, j)].clone().into();
                }
            }
            self
        }
    }
}

impl<'a, T, U> Add<&'a Matrix<U>> for Matrix<T>
    where
        T: AddAssign<T> + Clone,
        U: Clone,
        Matrix<T>: Add<Matrix<U>>,
        <Matrix<T> as Add<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn add(self, rhs: &'a Matrix<U>) -> Self {
        (self + rhs.clone()).into()
    }
}

impl<'a, T, U> Add<Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign<T> + Clone,
        U: Clone,
        Matrix<T>: Add<Matrix<U>>,
        <Matrix<T> as Add<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<U>) -> Self::Output {
        (self.clone() + rhs).into()
    }
}

impl<'a, 'b, T, U> Add<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign<T> + Clone,
        U: Clone,
        Matrix<T>: Add<Matrix<U>>,
        <Matrix<T> as Add<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn add(self, rhs: &'b Matrix<U>) -> Self::Output {
        (self.clone() + rhs.clone()).into()
    }
}

impl<T, U> Sub<Matrix<U>> for Matrix<T>
    where
        T: SubAssign<T> + Clone,
        U: Into<T> + Clone, {
    type Output = Matrix<T>;

    fn sub(mut self, rhs: Matrix<U>) -> Self {
        add_sub_valid_operation_check(self.dimension(), rhs.dimension());
        if self.alignment == rhs.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] -= rhs[i][j].clone().into();
                }
            }
            self
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] -= rhs[(i, j)].clone().into();
                }
            }
            self
        }
    }
}

impl<'a, T, U> Sub<&'a Matrix<U>> for Matrix<T>
    where
        T: SubAssign<T> + Clone,
        U: Clone,
        Matrix<T>: Sub<Matrix<U>>,
        <Matrix<T> as Sub<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn sub(self, rhs: &'a Matrix<U>) -> Self {
        (self - rhs.clone()).into()
    }
}

impl<'a, T, U> Sub<Matrix<U>> for &'a Matrix<T>
    where
        T: SubAssign<T> + Clone,
        U: Clone,
        Matrix<T>: Sub<Matrix<U>>,
        <Matrix<T> as Sub<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Matrix<U>) -> Self::Output {
        (self.clone() - rhs).into()
    }
}

impl<'a, 'b, T, U> Sub<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: SubAssign<T> + Clone,
        U: Clone,
        Matrix<T>: Sub<Matrix<U>>,
        <Matrix<T> as Sub<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn sub(self, rhs: &'b Matrix<U>) -> Self::Output {
        (self.clone() - rhs.clone()).into()
    }
}

fn mul_div_valid_operation_check(d1: (usize, usize), d2: (usize, usize)) {
    if d1.1 != d2.0 {
        panic!("The matrix on the left of the operand does not have the same number of columns as \
        the number of rows in the matrix on the right of the operand.");
    }
    valid_operation_check(d1, d2);
}

impl<T, U> Mul<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + Zero,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<U>) -> Self {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        if self.alignment != rhs.alignment {
            let mut matr = Matrix::splat(&T::zero(), (self.rows, rhs.rows), self.alignment.clone());
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[(a,b)] += (self[a][b].clone() * rhs[b][a].clone().into()).into();
                }
            }
            matr
        } else {
            let mut matr = Matrix::splat(&T::zero(), (self.rows, rhs.rows), self.alignment.clone());
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[(a,b)] += (self[(a, b)].clone() * rhs[(b, a)].clone().into()).into();
                }
            }
            matr
        }
    }
}

impl<'a, T, U> Mul<&'a Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul + MulAssign<T> + Clone + Zero,
        U: Clone,
        <T as Mul>::Output: Into<T>,
        Matrix<T>: Mul<Matrix<U>>,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &'a Matrix<U>) -> Self {
        (self * rhs.clone()).into()
    }
}

impl<'a, T, U> Mul<Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign + Mul + MulAssign<T> + Clone + Zero,
        U: Clone,
        <T as Mul>::Output: Into<T>,
        Matrix<T>: Mul<Matrix<U>>,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<U>) -> Self::Output {
        (self.clone() * rhs).into()
    }
}

impl<'a, 'b, T, U> Mul<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign + Mul + MulAssign<T> + Clone + Zero,
        U: Clone,
        <T as Mul>::Output: Into<T>,
        Matrix<T>: Mul<Matrix<U>>,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<U>) -> Self::Output {
        (self.clone() * rhs.clone()).into()
    }
}

impl<T, U> Div<Matrix<U>> for Matrix<T>
    where
        Matrix<U>: Inverse + Clone,
        Matrix<T>: Mul<Matrix<U>>,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn div(self, rhs: Matrix<U>) -> Self {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        let mut inv = rhs.clone();
        inv.inverse();
        (self * inv).into()
    }
}

impl<'a, T, U> Div<&'a Matrix<U>> for Matrix<T>
    where
        Matrix<U>: Inverse + Clone,
        Matrix<T>: Mul<Matrix<U>>,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn div(self, rhs: &'a Matrix<U>) -> Matrix<T> {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        let mut inv = rhs.clone();
        inv.inverse();
        (self * inv).into()
    }
}

impl<'a, T, U> Div<Matrix<U>> for &'a Matrix<T>
    where
        T: Clone,
        Matrix<U>: Inverse + Clone,
        Matrix<T>: Mul<Matrix<U>>,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn div(self, rhs: Matrix<U>) -> Matrix<T> {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        let mut inv = rhs.clone();
        inv.inverse();
        (self.clone() * inv).into()
    }
}

impl<'a, 'b, T, U> Div<&'b Matrix<U>> for &'a Matrix<T>
    where
        Matrix<U>: Inverse + Clone,
        Matrix<T>: Mul<Matrix<U>> + Clone,
        <Matrix<T> as Mul<Matrix<U>>>::Output: Into<Matrix<T>> {
    type Output = Matrix<T>;

    fn div(self, rhs: &'b Matrix<U>) -> Matrix<T> {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        let mut inv = rhs.clone();
        inv.inverse();
        (self.clone() * inv).into()
    }
}

macro_rules! matrix_operator_overload_assign_impl {
    ($imp:ident, $method:ident, $tokens:tt) => {
        impl<'a, T, U> $imp<&'a Matrix<U>> for Matrix<T> where
            U: Clone,
            Matrix<T>: $imp<Matrix<U>> {
            fn $method(&mut self, rhs: &'a Matrix<U>) {
                *self $tokens rhs.clone()
            }
        }
    }
}

impl<T, U> AddAssign<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Clone,
        U: Into<T> + Clone, {
    fn add_assign(&mut self, rhs: Matrix<U>) {
        add_sub_valid_operation_check(self.dimension(), rhs.dimension());
        if self.alignment == rhs.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] += rhs[i][j].clone().into();
                }
            }
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] += rhs[(i, j)].clone().into();
                }
            }
        }
    }
}

matrix_operator_overload_assign_impl!{AddAssign, add_assign, +=}

impl<T, U> SubAssign<Matrix<U>> for Matrix<T>
    where
        T: SubAssign + From<U>,
        U: SubAssign<T> + Clone + SubAssign<U>, {
    fn sub_assign(&mut self, rhs: Matrix<U>) {
        add_sub_valid_operation_check(self.dimension(), rhs.dimension());
        if self.alignment == rhs.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] -= rhs[i][j].clone().into();
                }
            }
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] -= rhs[(i, j)].clone().into();
                }
            }
        }
    }
}

matrix_operator_overload_assign_impl!{SubAssign, sub_assign, -=}

impl<T, U> MulAssign<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + MulAssign + Clone + Zero
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>, {
    fn mul_assign(&mut self, rhs: Matrix<U>) {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        if self.alignment != rhs.alignment {
            let mut matr = Matrix::splat(&T::zero(), (self.rows, rhs.rows), self.alignment.clone());
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[(a, b)] += (self[a][b].clone() + rhs[b][a].clone().into()).into();
                }
            }
            *self = matr;
        } else {
            let mut matr = Matrix::splat(&T::zero(), (self.rows, rhs.rows), self.alignment.clone());
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[(a, b)] += (self[(a, b)].clone() + rhs[(b, a)].clone().into()).into();
                }
            }
            *self = matr;
        }
    }
}

matrix_operator_overload_assign_impl!{MulAssign, mul_assign, *=}

impl<T, U> DivAssign<Matrix<U>> for Matrix<T>
    where Matrix<U>: Inverse + Clone, Matrix<T>: MulAssign<Matrix<U>>, {
    fn div_assign(&mut self, rhs: Matrix<U>) {
        mul_div_valid_operation_check(self.dimension(), rhs.dimension());
        let mut inv = rhs.clone();
        inv.inverse();
        *self *= inv;
    }
}

matrix_operator_overload_assign_impl!{DivAssign, div_assign, /=}