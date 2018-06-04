use std::ops::{Add, AddAssign, SubAssign, Mul};

use matrices::matrix_base::{Matrix, MatrixError};

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
    ($imp:ident, $method:ident, $t:ty) => {
        impl <'a, T, U> $imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone,
                <T as $imp>::Output: Into<T>, {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: &'a Matrix<U>) -> $t {
                $imp::$method(self, other.clone())
            }
        }

        impl <'a, T, U> $imp<Matrix<U>> for &'a Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone,
                <T as $imp>::Output: Into<T>, {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: Matrix<U>) -> $t {
                $imp::method(self.clone(), other)
            }
        }

        impl <'a, 'b, T, U> $imp<&'b Matrix<U>> for &'a Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone,
                <T as $imp>::Output: Into<T>, {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: &'b Matrix<U>) -> $t {
                $imp::method(self.clone(), other.clone())
            }
        }
    }
}

macro_rules! try_add_sub_check {
    () => {
        match try_add_sub_valid_operation_check(self.get_dimension(), other.get_dimension()) {
            Ok(aaa) => return Ok(aaa),
            Err(AAA) => return Err(AAA)
        }
    }
}

fn valid_try_operation_check(d1: (usize, usize), d2: (usize, usize)) -> Result<(), MatrixError> {
    if d1.0 == 0 {
        return MatrixError::FunctionError(
            "Matrix on the left of the operand has 0 rows.".to_string()
        );
    }
    if d1.1 == 0 {
        return MatrixError::FunctionError(
            "Matrix on the left of the operand has 0 columns.".to_string()
        );
    }
    if d2.0 == 0 {
        return MatrixError::FunctionError(
            "Matrix on the right of the operand has 0 rows.".to_string()
        );
    }
    if d2.1 == 0 {
        return MatrixError::FunctionError(
            "Matrix on the right of the operand has 0 columns.".to_string()
        );
    }
}

fn try_add_sub_valid_operation_check(d1: (usize, usize), d2: (usize, usize))
    -> Result<(), MatrixError> {
    if d1.0 != d2.0 && d1.1 != d2.1 {
        return MatrixError::FunctionError(
            "The matrices do not have an equal number of rows or columns.".to_string()
        );
    }
    if d1.0 != d2.0 {
        return MatrixError::FunctionError(
            "The matrices do not have an equal number of rows.".to_string()
        );
    }
    if d1.1 != d2.1 {
        return MatrixError::FunctionError(
            "The matrices do not have an equal number of columns.".to_string()
        );
    }
    valid_try_operation_check(d1, d2)
}

impl<T, U> TryAddMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign<T> + Clone,
        U: Into<T> + Clone, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_add(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_add_sub_check!{}
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

matrix_forward_ref_try_binop!{TryAddMatrices, try_add, Matrix<T>}

impl<T, U> TrySubMatrices<Matrix<U>> for Matrix<T>
    where
        T: SubAssign<T> + Clone,
        U: Into<T> + Clone, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_sub(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_add_sub_check!{}
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

fn try_mul_div_valid_operation_check(d1: (usize, usize), d2: (usize, usize)) {
    if d1.1 != d2.0 {
        return MatrixError::FunctionError("The matrix on the left of the operand does not have the \
            same number of columns as the number of rows in the matrix on the right of the operand."
                .to_string()
        );
    }
    valid_try_operation_check(d1, d2);
}

macro_rules! try_mul_div_check {
    () => {
        match try_mul_div_valid_operation_check(self.get_dimension(), other.get_dimension()) {
            Ok(aaa) => return Ok(aaa),
            Err(AAA) => return Err(AAA)
        }
    }
}

macro_rules! matrix_forward_ref_try_md {
    (mt) => {
        T: AddAssign + Mul<T> + Clone + From<i32>, U: Into<T> + Clone, <T as Mul<T>>::Output: Into<T>
    };
    (dt) => {
        matrix_forward_ref_try_md!(mt), Matrix<U>: Inverse
    };
    ($imp:ident, $method:ident, $t:ty, $mtdt:expr) => {
        impl<'a, matrix_forward_ref_try_md!($mtdt)> $imp<&'a Matrix<U>> for Matrix<T> {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $method(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
                $imp::$method(self, other.clone())
            }
        }

        impl<'a, matrix_forward_ref_try_md!($mtdt)> $imp<Matrix<U>> for &'a Matrix<T> {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $method(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
                $imp::$method(self.clone(), other)
            }
        }

        impl<'a, 'b, matrix_forward_ref_try_md!($mtdt)> $imp<&'b Matrix<U>> for &'a Matrix<T> {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $method(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
                $imp::$method(self.clone(), other.clone())
            }
        }
    }
}

impl<T, U> TryMulMatrices<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul<T> + Clone + From<i32>,
        U: Into<T> + Clone,
        <T as Mul<T>>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_mul(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_mul_div_check!{}
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

matrix_forward_ref_try_md!{TryMulMatrices, try_mul, Matrix<T>, mt}

impl<T, U> TryDivMatrices<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        Matrix<U>: Inverse,
        <T as Mul>::Output: Into<T>, {
    type Output = Result<Matrix<T>, MatrixError>;

    fn try_div(self, other: Matrix<U>) -> Result<Matrix<T>, MatrixError> {
        try_mul_div_check!{}
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

macro_rules! matrix_forward_ref_try_op_assign {
    ($imp:ident, $method:ident, $t:ty)  => {
        impl<'a, T, U> $imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone, {
            fn $method(&mut self, rhs: &'a Matrix<U>) {
                $imp::method(self, rhs.clone());
            }
        }
    }
}

trait TryAddAssign<Other = Self> {
    fn try_add_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

trait TrySubAssign<Other = Self> {
    fn try_sub_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

trait TryMulAssign<Other = Self> {
    fn try_mul_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

trait TryDivAssign<Other = Self> {
    fn try_div_assign(&mut self, other: Other) -> Result<(), MatrixError>;
}

impl<T, U> TryAddAssign<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Clone,
        U: Into<T> + Clone, {
    fn try_add_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_add_sub_check!{}
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

matrix_forward_ref_try_op_assign!{AddAssign, add_assign, Matrix<T>}

impl<T, U> SubAssign<Matrix<U>> for Matrix<T>
    where
        T: SubAssign + From<U>,
        U: SubAssign<T> + Clone + SubAssign<U>, {
    fn sub_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_add_sub_check!{}
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

matrix_forward_ref_try_op_assign!{SubAssign, sub_assign, Matrix<T>}

impl<T, U> TryMulAssignMatrices <Matrix<U>> for Matrix<T>
    where Matrix<T>: TryMulMatrices<Matrix<U>> {
    fn mul_assign(&mut self, other: Matrix<U>) -> Result<(), MatrixError> {
        try_mul_div_check!{}
        match self.try_mul(other) {
            Ok(res) => *self = res,
            Err(AAA) => Err(AAA)
        }
    }
}

impl<T, U> DivAssign<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>, {
    fn div_assign(&mut self, other: Matrix<U>) {
        mul_div_valid_operation_check(self.get_dimension(), other.get_dimension());
        if let Ok(inv) = other.inverse() {
            if self.alignment != inv.alignment {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, other.rows), false, ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..other.rows {
                        matr[a][b] += (self[a][b].clone() + inv[b][a].clone().into()).into();
                    }
                }
                *self = matr;
                Ok(())
            } else {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, other.rows), false, ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..other.rows {
                        matr[a][b] += (self[(a, b)].clone() + inv[(b, a)].clone().into()).into();
                    }
                }
                *self = matr;
                Ok(())
            }
        } else {
            Err(MatrixError::FunctionError("Unable to make inverse of divisor matrix!"
                .to_string()));
        }
    }
}

matrix_forward_ref_try_op_assign!{DivAssign, div_assign, Matrix<T>}