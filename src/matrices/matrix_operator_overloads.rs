use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Index, IndexMut};

use matrices::matrix_base::{AugmentedMatrix, Matrix, Alignment};

macro_rules! matrix_index_methods {
    ($($target_type:ty)* ) => ($(
        impl<T> Index<(usize, usize)> for $target_type {
            type Output = T;

            fn index<'a>(&'a self, index: (usize, usize)) -> &'a T {
                match self.alignment {
                    &Alignment::RowAligned => &self[row][column],
                    &Alignment::ColumnAligned => &self[column][row]
                }
            }
        }

        impl<T> Index<usize> for $target_type {
            type Output = [T];

            fn index<'a>(&'a self, index: usize) -> &'a [T] {
                self.matrix.as_slice()[(index * self.columns)..((index + 1) * self.columns)]
            }
        }

        impl<T> IndexMut<(usize, usize)> for $target_type {
            fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut T {
                match self.alignment {
                    &Alignment::RowAligned => &mut self[row][column],
                    &Alignment::ColumnAligned => &mut self[column][row]
                }
            }
        }

        impl<T> IndexMut<usize> for $target_type {
            fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [T] {
                self.matrix.as_mut_slice()[(index * self.columns)..((index + 1) * self.columns)]
            }
        }
    )*)
}

matrix_index_methods!{AugmentedMatrix<T> Matrix<T>}

fn valid_operation_check(d1: &(usize, usize), d2: &(usize, usize), ) {
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

fn add_sub_valid_operation_check(d1: &(usize, usize), d2: &(usize, usize)) {
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

macro_rules! matrix_forward_ref_binop {
    ($imp:ident, $method:ident, $t:ty) => {
        impl <'a, T, U> $imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone,
                <T as $imp>::Output: Into<T>, {
            type Output = <$t as $imp>::Output;

            fn $method(self, rhs: &'a Matrix<U>) -> $t {
                $imp::$method(self, rhs.clone())
            }
        }

        impl <'a, T, U> $imp<Matrix<U>> for &'a Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone,
                <T as $imp>::Output: Into<T>, {
            type Output = <$t as $imp>::Output;

            fn $method(self, rhs: Matrix<U>) -> $t {
                $imp::method(self.clone(), rhs)
            }
        }

        impl <'a, 'b, T, U> $imp<&'b Matrix<U>> for &'a Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone,
                <T as $imp>::Output: Into<T>, {
            type Output = <$t as $imp>::Output;

            fn $method(self, rhs: &'b Matrix<U>) -> $t {
                $imp::method(self.clone(), rhs.clone())
            }
        }
    }
}

impl<T, U> Add<Matrix<U>> for Matrix<T>
    where
        T: Add + Clone,
        U: Into<T> + Clone,
        <T as Add>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn add(mut self, rhs: Matrix<U>) -> Self {
        add_sub_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if self.alignment == rhs.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] = (self[i][j].clone() + rhs[i][j].clone().into()).into();
                }
            }
            self
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] = (self[(i, j)].clone() + rhs[(i, j)].clone().into())
                        .into();
                }
            }
            self
        }
    }
}

matrix_forward_ref_binop!{Add, add, Matrix<T>}

impl<'a, T, U> AddAssign<&'a Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Clone,
        U: Into<T> + Clone, {
    fn add_assign(&mut self, rhs: &'a Matrix<U>) {
        matrix_t_addassign_opt_ref_matrix_u!{}
    }
}

impl<T, U> Sub<Matrix<U>> for Matrix<T>
    where
        T: Sub + Clone,
        U: Into<T> + Clone,
        <T as Sub>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn sub(mut self, rhs: Matrix<U>) -> Self {
        add_sub_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if self.alignment == rhs.alignment {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[i][j] = (self[i][j].clone() + rhs[i][j].clone().into()).into();
                }
            }
            self
        } else {
            for i in 0..self.rows {
                for j in 0..self.columns {
                    self[(i, j)] = (self[(i, j)].clone() + rhs[(i, j)].clone().into())
                        .into();
                }
            }
            self
        }
    }
}

matrix_forward_ref_binop!{Sub, sub, Matrix<T>}

fn mul_div_valid_operation_check(d1: &(usize, usize), d2: &(usize, usize)) {
    if d1.1 != d2.0 {
        panic!("The matrix on the left of the operand does not have the same number of columns as \
        the number of rows in the matrix on the right of the operand.");
    }
    valid_operation_check(d1, d2);
}

use matrices::matrix_base::ROW_ALIGNED;
use matrices::matrix_base::COLUMN_ALIGNED;

impl<T, U> Mul<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Mul + Clone + From<i32> + From<<T as Mul<T>>::Output>,
        U: Into<T> + Clone,
        <T as Mul>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<U>) -> Self {
        mul_div_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if self.alignment != rhs.alignment {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[a][b] += (self[a][b].clone() * rhs[b][a].clone().into()).into();
                }
            }
            matr
        } else {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[a][b] += (self[(a, b)].clone() * rhs[(b, a)].clone().into()).into();
                }
            }
            matr
        }
    }
}

matrix_forward_ref_binop!{Mul, mul, Matrix<T>}

impl<T, U> Div<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + MulAssign + Clone + From<i32>
            + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>,
        Matrix<U>: MatrixInverse,
        <T as Mul>::Output: Into<T>, {
    type Output = Matrix<T>;
    fn div(self, rhs: Matrix<U>) -> Self {
        mul_div_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if let Ok(inv) = rhs.inverse() {
            if self.alignment != inv.alignment {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..rhs.rows {
                        matr[a][b] += (self[a][b].clone() + inv[b][a].clone().into()).into();
                    }
                }
                matr
            } else {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..rhs.rows {
                        matr[a][b] += (self[(a, b)].clone() + inv[(b, a)].clone().into()).into();
                    }
                }
                matr
            }
        } else {
            panic!("Unable to make inverse of divisor matrix!");
        }
    }
}

matrix_forward_ref_binop!{Div, div, Matrix<T>}

macro_rules! matrix_forward_ref_op_assign {
    ($imp:ident, $method:ident, $t:ty)  => {
        impl <'a, T, U> $imp<&'a Matrix<U>> for Matrix<T>
            where
                T: $imp + Clone,
                U: Into<T> + Clone, {
            fn $method(&mut self, rhs: &'a Matrix<U>) {
                $imp::method(self, rhs.clone());
            }
        }
    }
}

impl<T, U> AddAssign<Matrix<U>> for Matrix<T>
    where
        T: AddAssign + Clone,
        U: Into<T> + Clone, {
    fn add_assign(&mut self, rhs: Matrix<U>) {
        add_sub_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
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

matrix_forward_ref_op_assign!{AddAssign, add_assign, Matrix<T>}

impl<T, U> SubAssign<Matrix<U>> for Matrix<T>
    where
        T: SubAssign + From<U>,
        U: SubAssign<T> + Clone + SubAssign<U>, {
    fn sub_assign(&mut self, rhs: Matrix<U>) {
        add_sub_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if self.alignemnt == rhs.alignment {
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

matrix_forward_ref_op_assign!{SubAssign, sub_assign, Matrix<T>}

impl<T, U> MulAssign<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Mul + MulAssign + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>, {
    fn mul_assign(&mut self, rhs: Matrix<U>) {
        mul_div_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if self.alignment != rhs.alignment {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[a][b] += (self[a][b].clone() + rhs[b][a].clone().into()).into();
                }
            }
            *self = matr;
        } else {
            let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
            for a in 0..self.rows {
                for b in 0..rhs.rows {
                    matr[a][b] += (self[(a, b)].clone() + rhs[(b, a)].clone().into()).into();
                }
            }
            *self = matr;
        }
    }
}

matrix_forward_ref_op_assign!{MulAssign, mul_assign, Matrix<T>}

impl<T, U> DivAssign<Matrix<U>> for Matrix<T>
    where
        T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign + Clone + From<i32>
        + From<U> + From<<T as Mul<T>>::Output>,
        U: Mul<T> + Mul + Clone + Mul<U>, {
    fn div_assign(&mut self, rhs: Matrix<U>) {
        mul_div_valid_operation_check(&self.get_dimension(), &rhs.get_dimension());
        if let Ok(inv) = rhs.inverse() {
            if self.alignment != inv.alignment {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..rhs.rows {
                        matr[a][b] += (self[a][b].clone() + inv[b][a].clone().into()).into();
                    }
                }
                *self = matr;
            } else {
                let mut matr = Matrix::splat(&T::from(0), (self.rows, rhs.rows), false, ROW_ALIGNED);
                for a in 0..self.rows {
                    for b in 0..rhs.rows {
                        matr[a][b] += (self[(a, b)].clone() + inv[(b, a)].clone().into()).into();
                    }
                }
                *self = matr;
            }
        } else {
            panic!("Unable to make inverse of divisor matrix!");
        }
    }
}

matrix_forward_ref_op_assign!{DivAssign, div_assign, Matrix<T>}