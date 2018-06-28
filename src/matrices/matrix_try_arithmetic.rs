use num::{Zero, One};

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::mem::swap;

use matrices::matrix_base::{Matrix, MatrixError};
use matrices::matrix_transforms::Inverse;

trait TryAddMatrices<Other = Self> {
    type Output;

    fn try_add(self, other: Other) -> Self::Output;
}

trait TrySubMatrices<Other = Self> {
    type Output;

    fn try_sub(self, other: Other) -> Self::Output;
}

trait TryMulMatrices<Other = Self> {
    type Output;

    fn try_mul(self, other: Other) -> Self::Output;
}

trait TryDivMatrices<Other = Self> {
    type Output;

    fn try_div(self, other: Other) -> Self::Output;
}

macro_rules! matrix_forward_ref_try_binop {
    ($matrix_imp:ident -> $output:ty, $req_imp:ident, $method:ident, $op:tt) => {
        impl <'a, T, U> $matrix_imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            type Output = $output;

            fn $method(mut self, other: &'a Matrix<U>) -> Self::Output {
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        self[(i, j)] $op other[(i, j)].clone().into();
                    }
                }
                Ok(self)
            }
        }

        impl <'a, T, U> $matrix_imp<Matrix<U>> for &'a Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            type Output = $output;

            fn $method(self, other: Matrix<U>) -> Self::Output {
                let mut s = self.clone();
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        s[(i, j)] $op other[(i, j)].clone().into();
                    }
                }
                Ok(s)
            }
        }

        impl <'a, 'b, T, U> $matrix_imp<&'b Matrix<U>> for &'a Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            type Output = $output;

            fn $method(self, other: &'b Matrix<U>) -> Self::Output {
                let mut s = self.clone();
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        s[(i, j)] $op other[(i, j)].clone().into();
                    }
                }
                Ok(s)
            }
        }
    }
}

fn valid_try_operation_check(d1: (usize, usize), d2: (usize, usize)) -> Result<(), MatrixError> {
    if d1.0 == 0 {
        return Err(MatrixError::FunctionError(
            "Matrix on the left of the operand has 0 rows.".to_string()
        ));
    }
    if d1.1 == 0 {
        return Err(MatrixError::FunctionError(
            "Matrix on the left of the operand has 0 columns.".to_string()
        ));
    }
    if d2.0 == 0 {
        return Err(MatrixError::FunctionError(
            "Matrix on the right of the operand has 0 rows.".to_string()
        ));
    }
    if d2.1 == 0 {
        return Err(MatrixError::FunctionError(
            "Matrix on the right of the operand has 0 columns.".to_string()
        ));
    }
    Ok(())
}

fn try_add_sub_valid_operation_check(d1: (usize, usize), d2: (usize, usize))
    -> Result<(), MatrixError> {
    if d1.0 != d2.0 && d1.1 != d2.1 {
        return Err(MatrixError::FunctionError(
            "The matrices do not have an equal number of rows or columns.".to_string()
        ));
    }
    if d1.0 != d2.0 {
        return Err(MatrixError::FunctionError(
            "The matrices do not have an equal number of rows.".to_string()
        ));
    }
    if d1.1 != d2.1 {
        return Err(MatrixError::FunctionError(
            "The matrices do not have an equal number of columns.".to_string()
        ));
    }
    valid_try_operation_check(d1, d2)
}

impl<T, U> TryAddMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign<T> + Clone,
        U: Into<T> + Clone, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_add(mut self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_add_sub_valid_operation_check(self.dimension(), other.dimension())?;
        for i in 0..self.rows {
            for j in 0..self.columns {
                self[(i, j)] += other[(i, j)].clone().into();
            }
        }
        Ok(self)
    }
}

matrix_forward_ref_try_binop!{
    TryAddMatrices -> Result<Matrix<T>,
    MatrixError>,
    AddAssign,
    try_add,
    +=
}

impl<T, U> TrySubMatrices<Matrix<U>> for Matrix<T>
    where
        T: SubAssign<T> + Clone,
        U: Into<T> + Clone, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_sub(mut self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_add_sub_valid_operation_check(self.dimension(), other.dimension())?;
        for i in 0..self.rows {
            for j in 0..self.columns {
                self[(i, j)] -= other[(i, j)].clone().into();
            }
        }
        Ok(self)
    }
}

matrix_forward_ref_try_binop!{
    TrySubMatrices -> Result<Matrix<T>,
    MatrixError>,
    SubAssign,
    try_sub,
    -=
}

fn try_mul_div_valid_operation_check(d1: (usize, usize), d2: (usize, usize))
    -> Result<(), MatrixError> {
    if d1.1 != d2.0 {
        return Err(MatrixError::FunctionError("The matrix on the left of the operand does not have \
        the same number of columns as the number of rows in the matrix on the right of the operand."
                .to_string()
        ));
    }
    valid_try_operation_check(d1, d2)
}

impl<T, U> TryMulMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + Zero,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_mul_div_valid_operation_check(self.dimension(), other.dimension())?;
        let mut matr = Matrix::splat(&T::zero(), (self.rows, other.rows), self.alignment.clone());
        for a in 0..self.rows {
            for b in 0..other.rows {
                matr[(a, b)] += (self[(a, b)].clone() * other[(b, a)].clone().into()).into();
            }
        }
        Ok(matr)
    }
}

impl<'a, T, U> TryMulMatrices<&'a Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + Zero,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: &'a Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        let mut matr = Matrix::splat(&T::zero(), (self.rows, other.rows), self.alignment.clone());
        for a in 0..self.rows {
            for b in 0..other.rows {
                matr[(a, b)] += (self[(a, b)].clone() * other[(b, a)].clone().into()).into();
            }
        }
        Ok(matr)
    }
}

impl<'a, T, U> TryMulMatrices<Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + Zero,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        let mut matr = Matrix::splat(&T::zero(), (self.rows, other.rows), self.alignment.clone());
        for a in 0..self.rows {
            for b in 0..other.rows {
                matr[(a, b)] += (self[(a, b)].clone() * other[(b, a)].clone().into()).into();
            }
        }
        Ok(matr)
    }
}

impl<'a, 'b, T, U> TryMulMatrices<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + Zero,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: &'b Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        let mut matr = Matrix::splat(&T::zero(), (self.rows, other.rows), self.alignment.clone());
        for a in 0..self.rows {
            for b in 0..other.rows {
                matr[(a, b)] += (self[(a, b)].clone() * other[(b, a)].clone().into()).into();
            }
        }
        Ok(matr)
    }
}

impl<T, U> TryDivMatrices<Matrix<U>> for Matrix<T>
    where
        Matrix<T>: TryMulMatrices<Matrix<U>>,
        Matrix<U>: Inverse,
        <Matrix<T> as TryMulMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_mul_div_valid_operation_check(self.dimension(), other.dimension())?;
        let inv = other.try_inverse()?;
        (self.try_mul(inv)).into()
    }
}

impl<'a, T, U> TryDivMatrices<&'a Matrix<U>> for Matrix<T>
    where
        U: Clone,
        Matrix<T>: TryMulMatrices<Matrix<U>>,
        Matrix<U>: Inverse,
        <Matrix<T> as TryMulMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: &'a Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryDivMatrices::try_div(self, other.clone())
    }
}

impl<'a, T, U> TryDivMatrices<Matrix<U>> for &'a Matrix<T>
    where
        T: Clone,
        Matrix<T>: TryMulMatrices<Matrix<U>>,
        Matrix<U>: Inverse,
        <Matrix<T> as TryMulMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryDivMatrices::try_div(self.clone(), other)
    }
}

impl<'a, 'b, T, U> TryDivMatrices<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: Clone,
        U: Clone,
        Matrix<T>: TryMulMatrices<Matrix<U>>,
        Matrix<U>: Inverse,
        <Matrix<T> as TryMulMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: &'b Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryDivMatrices::try_div(self.clone(), other.clone())
    }
}

macro_rules! matrix_forward_ref_try_op_assign {
    ($matrix_imp:ident, $req_imp:ident, $method:ident, $t:ty)  => {
        impl<'a, T, U> $matrix_imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $req_imp<U> + Clone,
                U: Into<T> + Clone,
                Matrix<T>: $matrix_imp<Matrix<U>> {
            fn $method(&mut self, rhs: &'a Matrix<U>) -> Result<(), MatrixError> {
                $matrix_imp::$method(self, rhs.clone())
            }
        }
    }
}

trait TryAddAssignMatrices<Other = Self> {
    fn try_add_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

trait TrySubAssignMatrices<Other = Self> {
    fn try_sub_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

trait TryMulAssignMatrices<Other = Self> {
    fn try_mul_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

trait TryDivAssignMatrices<Other = Self> {
    fn try_div_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

impl<T, U> TryAddAssignMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Clone,
        U: Into<T> + Clone, {
    fn try_add_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_add_sub_valid_operation_check(self.dimension(), other.dimension())?;
        if self.alignment == other.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] += other[i][j].clone().into();
                }
            }
            Ok(())
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] += other[(i, j)].clone().into();
                }
            }
            Ok(())
        }
    }
}

matrix_forward_ref_try_op_assign!{TryAddAssignMatrices, AddAssign, try_add_assign, Matrix<T>}

impl<T, U> TrySubAssignMatrices<Matrix<U>> for Matrix<T>
    where
        T: SubAssign + From<U>,
        U: SubAssign<T> + Clone + SubAssign<U>, {
    fn try_sub_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_add_sub_valid_operation_check(self.dimension(), other.dimension())?;
        if self.alignment == other.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] -= other[i][j].clone().into();
                }
            }
            Ok(())
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] -= other[(i, j)].clone().into();
                }
            }
            Ok(())
        }
    }
}

matrix_forward_ref_try_op_assign!{TrySubAssignMatrices, SubAssign, try_sub_assign, Matrix<T>}

impl<T, U> TryMulAssignMatrices<Matrix<U>> for Matrix<T>
    where
        T: Clone + Mul + AddAssign + Zero,
        <T as Mul>::Output: Into<T>,
        U: Into<T> + Clone,
        Matrix<T>: TryMulMatrices<Matrix<U>>,
        <Matrix<T> as TryMulMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    fn try_mul_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_valid_operation_check(self.dimension(), other.dimension())?;
        let mut matr = Matrix::splat(&T::zero(), (self.rows, other.rows), self.alignment.clone());
        swap(self, &mut matr);
        for a in 0..self.rows {
            for b in 0..other.rows {
                self[(a, b)] += (matr[(a, b)].clone() * other[(b, a)].clone().into()).into();
            }
        }
        Ok(())
    }
}

impl<'a, T, U> TryMulAssignMatrices<&'a Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul + Clone + Zero,
        U: Into<T> + Clone,
        <T as Mul>::Output: Into<T>,
        Matrix<T>: TryMulMatrices<Matrix<U>>,
        <Matrix<T> as TryMulMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    fn try_mul_assign(&mut self, other: &'a Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_valid_operation_check(self.dimension(), other.dimension())?;
        let mut matr = Matrix::splat(&T::zero(), (self.rows, other.rows), self.alignment.clone());
        swap(self, &mut matr);
        for a in 0..self.rows {
            for b in 0..other.rows {
                self[(a, b)] += (matr[(a, b)].clone() * other[(b, a)].clone().into()).into();
            }
        }
        Ok(())
    }
}

impl<T, U> TryDivAssignMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul + Clone + Zero,
        U: Into<T> + AddAssign + SubAssign + MulAssign + DivAssign + Div + PartialOrd + PartialEq
            + Zero + One + Clone,
        <T as Mul>::Output: Into<T>,
        <U as Div>::Output: Into<U>,
        Matrix<T>: TryDivMatrices<Matrix<U>>,
        <Matrix<T> as TryDivMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    fn try_div_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_valid_operation_check(self.dimension(), other.dimension())?;
        let inv = other.clone().try_inverse()?;
        self.try_mul_assign(inv)
    }
}

impl<'a, T, U> TryDivAssignMatrices<&'a Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul + Clone + Zero,
        U: Into<T> + AddAssign + SubAssign + MulAssign + DivAssign + Div + PartialOrd + PartialEq
        + Zero + One + Clone,
        <T as Mul>::Output: Into<T>,
        <U as Div>::Output: Into<U>,
        Matrix<T>: TryDivMatrices<Matrix<U>>,
        <Matrix<T> as TryDivMatrices<Matrix<U>>>::Output: Into<Result<Matrix<T>, MatrixError>> {
    fn try_div_assign(&mut self, other: &'a Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_valid_operation_check(self.dimension(), other.dimension())?;
        let inv = other.clone().try_inverse()?;
        self.try_mul_assign(inv)
    }
}