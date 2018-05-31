use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use matrices::matrix_base::{AugmentedMatrix, Matrix, MatrixError};
use matrices::matrix_format::Separator;

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

macro_rules! make_try_fn_contents {
    {1 $other_ident:ident, ($self_loc_1:ident, $self_loc_2:ident),
        ($other_loc_1:ident, $other_loc_2:ident), $new_matr_val:tt, $op:tt} => {
        if self.dimension() != $other_ident.dimension() {
            return Err(MatrixError::OpError(
                "Matrices are not of the same dimension - unable to perform addition."
                    .to_string()
            ));
        }
    };
    {2 $other_ident:ident, ($self_loc_1:ident, $self_loc_2:ident),
        ($other_loc_1:ident, $other_loc_2:ident), $new_matr_val:tt, $op:tt} => {
        if self.alignment == $other_ident.alignment {
            let mut res = self.clone();
            for i in 0..self.rows {
                for j in 0..self.columns {
                    res[i][j] $new_matr_val (self[$self_loc_1][$self_loc_2].clone()
                        $op rhs[$other_loc_1][$other_loc_2].clone().into()).into();
                }
            }
            Ok(res)
        } else {
            let mut res = self.clone();
            for i in 0..self.rows {
                for j in 0..self.columns {
                    res[(i, j)] = (self[(i, j)].clone() $op rhs[(i, j)].clone().into()).into();
                }
            }
            Ok(res)
        }
    };
}

macro_rules! forward_matrix_op {
    {
        impl<T: $trait_:ident, U> $actual_trait:ident<Matrix<U>> for Matrix<T> {
            fn $fn_name:ident(self, other: Matrix<T>) -> Self::Output {
                $fn_contents1:expr,
                $fn_contents2:expr
            }
        }
    } => {
        impl<T, U> $actual_trait<Matrix<U>> for Matrix<T>
            where
                T: $trait_ + Clone,
                U: Into<T> + Clone,
                <T as $trait_>::Output: Into<T>, {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $fn_name(self, other: Matrix<T>) -> Self::Output {
                $fn_contents1
                $fn_contents2
            }
        }

        impl<'a, T, U> $trait_<&'a Matrix<U>> for Matrix<T>
            where
                T: $trait_ + Clone,
                U: Into<T> + Clone,
                <T as $trait_>::Output: Into<T>, {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $fn_name(self, other: &'a Matrix<T>) -> Self::Output {
                $fn_contents1
                $fn_contents2
            }
        }

        impl<'a, T, U> $trait_<Matrix<U>> for &'a Matrix<T>
            where
                T: $trait_ + Clone,
                U: Into<T> + Clone,
                <T as $trait_>::Output: Into<T>, {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $fn_name(&'a self, other: Matrix<T>) -> Self::Output {
                $fn_contents1
                $fn_contents2
            }
        }

        impl<'a, 'b, T> $trait_<&'b Matrix<U>> for &'a Matrix<T>
            where
                T: $trait_ + Clone,
                U: Into<T> + Clone,
                <T as $trait_>::Output: Into<T>, {
            type Output = Result<Matrix<T>, MatrixError>;

            fn $fn_name(&'a self, other: &'b Matrix<T>) -> Self::Output {
                $fn_contents1
                $fn_contents2
            }
        }
    }
}

forward_matrix_op!{
    impl<T: Add, U> TryAddMatrices<Matrix<U>> for Matrix<T> {
        fn try_add(self, other: Matrix<T>) -> Self::Output {
            make_try_as_fn_contents!{1 +},
            make_try_as_fn_contents!{2 +}
        }
    }
}

forward_matrix_op!{
    impl<T: Sub, U> TrySubMatrices<Matrix<U>> for Matrix<T> {
        fn try_sub(self, Other: Matrix<T>) -> Self::Output {
            make_try_as_fn_contents!{1 +},
            make_try_as_fn_contents!{2 +}
        }
    }
}

macro_rules! gen_matrix_mul_fn_body {
    {1 $other_ident:ident} => {
        if self.dimension().1 != $other_ident.dimension().0 {
            return Err(MatrixError::OpError("Column count of first matrix is not equal to \
                row count of second matrix.".to_string()));
        }
    };
    {2 $other_ident:ident} => {
        if self.alignment != $other_ident.alignment {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, $other_ident.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..$other_ident.rows {
                    matr[a][b] += self[a][b].clone() * $other_ident[b][a].clone();
                }
            }
            matr
        } else {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[a][b] += (self[(a, b)].clone() + rhs[(b, a)].clone().into()).into();
                }
            }
            matr
        }
    };
}

forward_matrix_op!{
    impl<T: Mul, U> TryMulMatrices<Matrix<U>> for Matrix<T> {
        fn try_mul(self, other: Matrix<T>) -> Self::Output {
            gen_matrix_mul_fn_body!{1 other},
            gen_matrix_mul_fn_body!{2 other}
        }
    }
}

macro_rules! gen_matrix_div_fn_body {
    {} => {
        let mut inv = other.inverse();
        gen_matrix_mul_fn_body!{1 inv},
        gen_matrix_mul_fn_body!{2 inv}
    }
}

impl<T, U> TryDivMatrices<Matrix<U>> for Matrix<T>
    where
        T: Div + Clone,
        U: Into<T> + Clone,
        Matrix<U>: TryMatrixInverse,
        <T as Div>::Output: Into<T>, {
    type Output = Matrix<T>;

    fn try_div(self, other: Matrix<U>) -> Self::Output {
        gen_matrix_div_fn_body!{}
    }
}

impl<'a, T, U> TryDivMatrices<&'a Matrix<U>> for Matrix<T>
    where
        T: Div + Clone,
        U: Into<T> + Clone,
        Matrix<U>: TryMatrixInverse,
        <T as Div>::Output: Into<T>, {
    type Output = Matrix<T>;

    fn try_div(self, other: &'a Matrix<U>) -> Self::Output {
        gen_matrix_div_fn_body!{}
    }
}

impl<'a, T, U> TryDivMatrices<Matrix<U>> for &'a Matrix<T>
    where
        T: Div + Clone,
        U: Into<T> + Clone,
        Matrix<U>: TryMatrixInverse,
        <T as Div>::Output: Into<T>, {
    type Output = Matrix<T>;

    fn try_div(&'a self, other: Matrix<U>) -> Self::Output {
        gen_matrix_div_fn_body!{}
    }
}

impl<'a, 'b, T, U> TryDivMatrices<&'b Matrix<U>> for &'a Matrix<T>
    where
        T: Div + Clone,
        U: Into<T> + Clone,
        Matrix<U>: TryMatrixInverse,
        <T as Div>::Output: Into<T>, {
    type Output = Matrix<T>;

    fn try_div(&'a self, other: &'b Matrix<U>) -> Self::Output {
        gen_matrix_div_fn_body!{}
    }
}

/*trait TryAddAssignMatrices {
    type Output;

    fn try_addassign(&mut self, other: &Self) -> Self::Output;
}*/