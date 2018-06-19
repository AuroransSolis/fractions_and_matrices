use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use matrices::matrix_base::{Matrix, MatrixError, ROW_ALIGNED, COLUMN_ALIGNED};
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
    ($matrix_imp:ident, $req_imp:ident, $method:ident, $t:ty) => {
        impl <'a, T, U> $matrix_imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: &'a Matrix<U>) -> $t {
                $imp::$method(self, other.clone())
            }
        }

        impl <'a, T, U> $matrix_imp<Matrix<U>> for &'a Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: Matrix<U>) -> $t {
                $imp::method(self.clone(), other)
            }
        }

        impl <'a, 'b, T, U> $matrix_imp<&'b Matrix<U>> for &'a Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: &'b Matrix<U>) -> $t {
                $imp::method(self.clone(), other.clone())
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
        T: AddAssign<U> + Clone,
        U: Into<T> + Clone, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_add(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_add_sub_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        if self.alignment == other.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] += other[i][j].clone().into();
                }
            }
            Ok(self)
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] += other[(i, j)].clone().into();
                }
            }
            Ok(self)
        }
    }
}

matrix_forward_ref_try_binop!{TryAddMatrices, AddAssign, try_add, Matrix<T>}

impl<T, U> TrySubMatrices<Matrix<U>> for Matrix<T>
    where
        T: SubAssign<U> + Clone,
        U: Into<T> + Clone, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_sub(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_add_sub_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        if self.alignment == other.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] -= other[i][j].clone().into();
                }
            }
            Ok(self)
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] -= other[(i, j)].clone().into();
                }
            }
            Ok(self)
        }
    }
}

matrix_forward_ref_try_binop!{TrySubMatrices, SubAssign, try_sub, Matrix<T>}

fn try_mul_div_valid_operation_check(d1: (usize, usize), d2: (usize, usize))
    -> Result<(), MatrixError> {
    if d1.1 != d2.0 {
        return Err(MatrixError::FunctionError("The matrix on the left of the operand does not have the \
            same number of columns as the number of rows in the matrix on the right of the operand."
                .to_string()
        ));
    }
    valid_try_operation_check(d1, d2)
}

impl<T, U> TryMulMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + From<i32>,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        if self.alignment != other.alignment {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, other.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..other.rows {
                    matr[a][b] += (self[a][b].clone() * other[b][a].clone().into()).into();
                }
            }
            matr
        } else {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, other.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..other.rows {
                    matr[a][b] += (self[(a, b)].clone() * other[(b, a)].clone().into()).into();
                }
            }
            matr
        }
    }
}

impl<'a, T, U> TryMulMatrices for Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + From<i32>,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryMulMatrices::try_mul(self, other.clone())
    }
}

impl<'a, T, U> TryMulMatrices<Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + From<i32>,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryMulMatrices::try_mul(self.clone(), other)
    }
}

impl<'a, 'b, T, U> TryMulMatrices<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + From<i32>,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryMulMatrices::try_mul(self.clone(), other.clone())
    }
}

impl<T, U> TryDivMatrices<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        Matrix<U>: Inverse,
        <T as Mul>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        if let Ok(inv) = other.try_inverse() {
            if self.alignment != inv.alignment {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, other.rows), false,
                                             ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..other.rows {
                        matr[a][b] += (self[a][b].clone() * inv[b][a].clone().into()).into();
                    }
                }
                Ok(matr)
            } else {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, other.rows), false,
                                             ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..other.rows {
                        matr[a][b] += (self[(a, b)].clone() * inv[(b, a)].clone().into()).into();
                    }
                }
                Ok(matr)
            }
        } else {
            return MatrixError::FunctionError("Unable to make inverse of divisor matrix!"
                .to_string());
        }
    }
}



impl<'a, T, U> TryDivMatrices for Matrix<T>
    where
        T: Add + AddAssign + Mul + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        Matrix<U>: Inverse,
        <T as Mul>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryDivMatrices::try_div(self, other.clone())
    }
}

impl<'a, T, U> TryDivMatrices<Matrix<U>> for &'a Matrix<T>
    where
        T: Add + AddAssign + Mul + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        Matrix<U>: Inverse,
        <T as Mul>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryDivMatrices::try_div(self.clone(), other)
    }
}

impl<'a, 'b, T, U> TryDivMatrices<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: Add + AddAssign + Mul + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        Matrix<U>: Inverse,
        <T as Mul>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        TryDivMatrices::try_div(self.clone(), other.clone())
    }
}

macro_rules! matrix_forward_ref_try_op_assign {
    ($matrix_imp:ident, $req_imp:ident, $method:ident, $t:ty)  => {
        impl<'a, T, U> $matrix_imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $req_imp + Clone,
                U: Into<T> + Clone, {
            fn $method(&mut self, rhs: &'a Matrix<U>) {
                $matrix_imp::method(self, rhs.clone());
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
        try_add_sub_valid_operation_check(self.get_dimension(), other.get_dimension())?;
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

matrix_forward_ref_try_op_assign!{TryAddAssignMatrices, AddAssign, add_assign, Matrix<T>}

impl<T, U> TrySubAssignMatrices<Matrix<U>> for Matrix<T>
    where
        T: SubAssign + From<U>,
        U: SubAssign<T> + Clone + SubAssign<U>, {
    fn try_sub_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_add_sub_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        if self.alignemnt == other.alignment {
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

matrix_forward_ref_try_op_assign!{TrySubAssignMatrices, SubAssign, sub_assign, Matrix<T>}

impl<T, U> TryMulAssignMatrices<Matrix<U>> for Matrix<T>
    where Matrix<T>: TryMulMatrices<Matrix<U>> {
    fn try_mul_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        match self.try_mul(other) {
            Ok(res) => {
                *self = res;
                Ok(())
            },
            Err(AAA) => Err(AAA)
        }
    }
}

impl<'a, T, U> TryMulAssignMatrices<&'a Matrix<U>> for Matrix<T>
    where Matrix<T>: TryMulMatrices<Matrix<U>> {
    fn try_mul_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension())?;
        match self.try_mul(other) {
            Ok(res) => {
                *self = res;
                Ok(())
            },
            Err(AAA) => Err(AAA)
        }
    }
}

impl<T, U> TryDivAssignMatrices<Matrix<U>> for Matrix<T>
    where Matrix<T>: TryDivMatrices<Matrix<U>> {
    fn try_div_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        match try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension()) {
            Ok(aaa) => return Ok(aaa),
            Err(AAA) => return Err(AAA)
        }
        match self.try_div(other) {
            Ok(res) => {
                *self = res;
                Ok(())
            },
            Err(AAA) => Err(AAA)
        }
    }
}

impl<'a, T, U> TryDivAssignMatrices<&'a Matrix<U>> for Matrix<T>
    where Matrix<T>: TryDivMatrices<Matrix<U>> {
    fn try_div_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        match try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension()) {
            Ok(aaa) => return Ok(aaa),
            Err(AAA) => return Err(AAA)
        }
        match self.try_div(other) {
            Ok(res) => {
                *self = res;
                Ok(())
            },
            Err(AAA) => Err(AAA)
        }
    }
}