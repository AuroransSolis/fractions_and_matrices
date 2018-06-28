use num::{One, Zero};

use std::ops::{AddAssign, SubAssign, MulAssign, Neg, Div, DivAssign, Rem, Range};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{Debug, Display};
use std::mem::swap;
use std::marker::Sized;

use matrices::matrix_base::{Matrix, AugmentedMatrix, MatrixError, Unit};

pub trait RowOpAdd {
    fn row_op_add(&mut self, target: usize, tool: usize);
}

pub trait RowOpSub {
    fn row_op_sub(&mut self, target: usize, tool: usize);
}

pub trait RowOpMul<Scalar> {
    fn row_op_mul(&mut self, target: usize, tool: Scalar);
}

pub trait RowOpDiv<Scalar> {
    fn row_op_div(&mut self, target: usize, tool: Scalar);
}

pub trait Gcd: Rem + PartialEq + Sized {}
impl<T: Rem + PartialEq> Gcd for T {}

fn gcd<T: Gcd + Zero + Clone>(a: T, b: T) -> T
    where <T as Rem>::Output: Into<T> {
    if b.is_zero() {
        return a;
    } else {
        gcd(b.clone(), (a % b).into())
    }
}

pub trait Simplify {
    fn simplify_row(&mut self, row: usize);
    fn simplify_rows(&mut self, rows: Range<usize>);
    fn simplify_matrix(&mut self);
}

pub trait SimplifyGetStepsDisplay {
    fn simplify_row_get_steps_ds(&mut self, row: usize) -> Option<String>;
    fn simplify_rows_get_steps_ds(&mut self, rows: Range<usize>) -> Option<Vec<Option<String>>>;
    fn simplify_matrix_get_steps_ds(&mut self) -> Option<Vec<Option<String>>>;
}

pub trait SimplifyGetStepsDebug {
    fn simplify_row_get_steps_db(&mut self, row: usize) -> Option<String>;
    fn simplify_rows_get_steps_db(&mut self, rows: Range<usize>) -> Option<Vec<Option<String>>>;
    fn simplify_matrix_get_steps_db(&mut self) -> Option<Vec<Option<String>>>;
}

pub trait SimplifyTraits: Div + DivAssign + Gcd + Zero + One + PartialEq {}
impl<T: Div + DivAssign + Gcd + Zero + One + PartialEq> SimplifyTraits for T {}

pub trait REF {
    fn gaussian_elim(&mut self);
    fn is_row_reduced(&self) -> bool;
}

pub trait REFDisplay {
    fn gaussian_elim_display(&mut self) -> Option<Vec<String>>;
}

pub trait REFDebug {
    fn gaussian_elim_debug(&mut self) -> Option<Vec<String>>;
}

pub trait RREF {
    fn gauss_jordan(&mut self);
    fn is_gauss_jordan(&self) -> bool;
}

pub trait RREFDisplay {
    fn gauss_jordan_display(&mut self) -> Option<Vec<String>>;
}

pub trait RREFDebug {
    fn gauss_jordan_debug(&mut self) -> Option<Vec<String>>;
}

pub trait Inverse where Self: Sized {
    fn inverse(&self) -> Self;
    fn try_inverse(&self) -> Result<Self, MatrixError>;
}

pub trait InverseDisplay where Self: Sized {
    fn inverse_display(&self) -> (Self, Option<Vec<String>>);
    fn try_inverse_display(&self) -> Result<(Self, Option<Vec<String>>), MatrixError>;
}

pub trait InverseDebug where Self: Sized {
    fn inverse_debug(&self) -> (Self, Option<Vec<String>>);
    fn try_inverse_debug(&self) -> Result<(Self, Option<Vec<String>>), MatrixError>;
}

pub trait InverseAssign where Self: Sized {
    fn inverse_assign(&mut self);
    fn try_inverse_assign(&mut self) -> Result<(), MatrixError>;
}

pub trait InverseAssignDisplay where Self: Sized {
    fn inverse_assign_display(&mut self) -> Option<Vec<String>>;
    fn try_inverse_assign_display(&mut self) -> Result<Option<Vec<String>>, MatrixError>;
}

pub trait InverseAssignDebug where Self: Sized {
    fn inverse_assign_debug(&mut self) -> Option<Vec<String>>;
    fn try_inverse_assign_debug(&mut self) -> Result<Option<Vec<String>>, MatrixError>;
}

macro_rules! transforms_impl {
    ($($target_type:ty: $name:ident),*) => ($(
        impl<T: AddAssign + Clone> RowOpAdd for $target_type {
            fn row_op_add(&mut self, target: usize, tool: usize) {
                for b in 0..self.num_columns() {
                    self[(target, b)] += self[(tool, b)].clone();
                }
            }
        }

        impl<T: SubAssign + Clone> RowOpSub for $target_type {
            fn row_op_sub(&mut self, target: usize, tool: usize) {
                for b in 0..self.num_columns() {
                    self[(target, b)] -= self[(tool, b)].clone();
                }
            }
        }

        impl<T: MulAssign + Clone> RowOpMul<T> for $target_type {
            fn row_op_mul(&mut self, target: usize, tool: T) {
                for b in 0..self.num_columns() {
                    self[(target, b)] *= tool.clone();
                }
            }
        }

        impl<T: DivAssign + Clone> RowOpDiv<T> for $target_type {
            fn row_op_div(&mut self, target: usize, tool: T) {
                for b in 0..self.num_columns() {
                    self[(target, b)] /= tool.clone();
                }
            }
        }

        impl<T: SimplifyTraits + Clone> Simplify for $target_type
            where <T as Rem>::Output: Into<T> {
            fn simplify_row(&mut self, row: usize) {
                if self.num_columns() < 2 {
                    return;
                }
                let mut row_gcd = gcd(self[(row, 0)].clone(), self[(row, 1)].clone());
                for i in 2..self.num_columns() {
                    if self[(row, i)].is_zero() {
                        continue;
                    }
                    row_gcd = gcd(row_gcd, self[(row, i)].clone());
                    if row_gcd.is_one() {
                        return;
                    }
                }
                if self.is_row_aligned() {
                    for e in self[row].iter_mut() {
                        *e /= row_gcd.clone();
                    }
                } else {
                    for i in (row * self.num_columns())..((row + 1) * self.num_columns()) {
                        self[(row, i)] /= row_gcd.clone();
                    }
                }
            }

            fn simplify_rows(&mut self, rows: Range<usize>) {
                for r in rows {
                    self.simplify_row(r);
                }
            }

            fn simplify_matrix(&mut self) {
                let end = self.num_rows();
                self.simplify_rows(0..end);
            }
        }

        impl<T: SimplifyTraits + Display + Clone + Zero + One> SimplifyGetStepsDisplay for $target_type
            where <T as Rem>::Output: Into<T> {
            fn simplify_row_get_steps_ds(&mut self, row: usize) -> Option<String> {
                if self.num_columns() < 2 {
                    return None;
                }
                let mut row_gcd = gcd(self[(row, 0)].clone(), self[(row, 1)].clone());
                for i in 2..self.num_columns() {
                    if self[(row, i)].is_zero() {
                        continue;
                    }
                    row_gcd = gcd(row_gcd, self[(row, i)].clone());
                    if row_gcd == T::one() {
                        return None;
                    }
                }
                if self.is_row_aligned() {
                    for e in self[row].iter_mut() {
                        *e /= row_gcd.clone();
                    }
                } else {
                    for i in (row * self.num_columns())..((row + 1) * self.num_columns()) {
                        self[(row, i)] /= row_gcd.clone();
                    }
                }
                Some(format!("R{} / {} → R{0}", row, row_gcd))
            }

            fn simplify_rows_get_steps_ds(&mut self, rows: Range<usize>)
                -> Option<Vec<Option<String>>> {
                if self.num_columns() < 2 {
                    return None;
                }
                Some(rows.map(|r| self.simplify_row_get_steps_ds(r))
                    .collect::<Vec<Option<String>>>())
            }

            fn simplify_matrix_get_steps_ds(&mut self) -> Option<Vec<Option<String>>> {
                if self.num_columns() < 2 {
                    return None;
                }
                let end = self.num_rows();
                self.simplify_rows_get_steps_ds(0..end)
            }
        }

        impl<T: SimplifyTraits + Debug + Clone + Zero + One> SimplifyGetStepsDebug for $target_type
            where <T as Rem>::Output: Into<T> {
            fn simplify_row_get_steps_db(&mut self, row: usize) -> Option<String> {
                if self.num_columns() < 2 {
                    return None;
                }
                let mut row_gcd = gcd(self[(row, 0)].clone(), self[(row, 1)].clone());
                for i in 2..self.num_columns() {
                    if self[(row, i)].is_zero() {
                        continue;
                    }
                    row_gcd = gcd(row_gcd, self[(row, i)].clone());
                    if row_gcd.is_one() {
                        return None;
                    }
                }
                if self.is_row_aligned() {
                    for e in self[row].iter_mut() {
                        *e /= row_gcd.clone();
                    }
                } else {
                    for i in (row * self.num_columns())..((row + 1) * self.num_columns()) {
                        self[(row, i)] /= row_gcd.clone();
                    }
                }
                Some(format!("R{} / {:?} → R{0}", row, row_gcd))
            }

            fn simplify_rows_get_steps_db(&mut self, rows: Range<usize>)
                -> Option<Vec<Option<String>>> {
                if self.num_columns() < 2 {
                    return None;
                }
                Some(rows.map(|r| self.simplify_row_get_steps_db(r))
                    .collect::<Vec<Option<String>>>())
            }

            fn simplify_matrix_get_steps_db(&mut self) -> Option<Vec<Option<String>>> {
                if self.num_columns() < 2 {
                    return None;
                }
                let end = self.num_rows();
                self.simplify_rows_get_steps_db(0..end)
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Clone> REF for $target_type
            where
                $target_type: RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T>,
                 <T as Div>::Output: Into<T> {
            fn gaussian_elim(&mut self) {
                if self.is_row_reduced() {
                    return;
                }
                for r in 0..self.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = self[(r, c)].clone();
                        if c < r { // If the value's under the major diagonal
                            if amt1.is_zero() { // Continue if it's already what it should be
                                continue;
                            }
                            // The value at (c, c) can be used to make (r, c) zero for REF - if
                            // (c, c).is_zero(), just continue since doing more work on (r, c) right
                            // now is pointless
                            if self[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / self[(c, c)].clone()).into();
                            (*self).row_op_mul(c, amt2.clone());
                            (*self).row_op_sub(r, c);
                            (*self).row_op_div(c, amt2);
                        } else if c == r { // If the value's on the major diagonal
                            if amt1.is_one() { // Continue if it's already what it should be
                                continue;
                            } else if !amt1.is_zero() { // If it's not zero...
                                (*self).row_op_div(r, amt1); // ...divide by itself to make it one
                            }
                            if self[(r, c)].is_zero() { // If it is zero...
                                // ...and the best tool is also zero, continue
                                if self[(c, c)].is_zero() {
                                    continue;
                                }
                                (*self).row_op_add(r, c); // Otherwise, add said tool to row 'r'
                                if !self[(r, c)].is_one() { // If that tool somehow wasn't 1...
                                    let src = self[(r, c)].clone();
                                    (*self).row_op_div(r, src); // ...divide by itself
                                }
                            }
                        }
                    }
                }
            }

            fn is_row_reduced(&self) -> bool {
                for a in 0..self.num_rows() {
                    for b in 0..a {
                        if !self[(a, b)].is_zero() {
                            return false;
                        }
                    }
                    if !self[(a, a)].is_one() {
                        return false;
                    }
                }
                true
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Display + Clone> REFDisplay for $target_type
            where
                $target_type: RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T>,
                 <T as Div>::Output: Into<T> {
            fn gaussian_elim_display(&mut self) -> Option<Vec<String>> {
                if self.is_row_reduced() {
                    return None;
                }
                let mut steps = Vec::new();
                steps.push("------- REF -------".to_string());
                for r in 0..self.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = self[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if self[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / self[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                            (*self).row_op_mul(c, amt2.clone());
                            (*self).row_op_sub(r, c);
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                (*self).row_op_div(r, amt1);
                            } else if self[(r, c)].is_zero() {
                                if self[(c, c)].is_zero() {
                                    continue;
                                }
                                (*self).row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !self[(r, c)].is_one() {
                                    let src = self[(r, c)].clone();
                                    steps.push(format!("R{} / ({}) → R{0}", r, src));
                                    (*self).row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                Some(steps)
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Debug + Clone> REFDebug for $target_type
            where
                $target_type: RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T>,
                 <T as Div>::Output: Into<T> {
            fn gaussian_elim_debug(&mut self) -> Option<Vec<String>> {
                if self.is_row_reduced() {
                    return None;
                }
                let mut steps = Vec::new();
                steps.push("------- REF -------".to_string());
                for r in 0..self.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = self[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if self[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / self[(c, c)].clone()).into();
                            let step_no = steps.len();
                            steps.push(format!("Step {}: R{} - ({:?}) * R{} → R{0}", step_no, r,
                                               amt2, c));
                            (*self).row_op_mul(c, amt2.clone());
                            (*self).row_op_sub(r, c);
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                let step_no = steps.len();
                                steps.push(format!("Step {}: R{} / ({:?}) → R{0}", step_no, r,
                                                   amt1));
                                (*self).row_op_div(r, amt1);
                            } else if self[(r, c)].is_zero() {
                                if self[(c, c)].is_zero() {
                                    continue;
                                }
                                (*self).row_op_add(r, c);
                                let step_no = steps.len();
                                steps.push(format!("Step {}: R{} + R{} → R{0}", step_no, r, c));
                                if !self[(r, c)].is_one() {
                                    let src = self[(r, c)].clone();
                                    let step_no = steps.len();
                                    steps.push(format!("Step {}: R{} / ({:?}) → R{0}", step_no,
                                                       r, src));
                                    (*self).row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                Some(steps)
            }
        }

        impl<T: Div + PartialEq + Zero + One + Clone> RREF for $target_type
            where
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T>,
                 <T as Div>::Output: Into<T> {
            fn gauss_jordan(&mut self) {
                if self.is_gauss_jordan() {
                    return;
                }
                if !self.is_row_reduced() {
                    (*self).gaussian_elim();
                }
                if !self.is_row_reduced() {
                    return;
                }
                for c in (1..self.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if self[(r, c)].is_zero() {
                            continue;
                        }
                        let self_rc = self[(r, c)].clone();
                        (*self).row_op_mul(c, self_rc.clone());
                        (*self).row_op_sub(r, c);
                        (*self).row_op_div(c, self_rc);
                    }
                }
            }

            fn is_gauss_jordan(&self) -> bool {
                if !self.is_row_reduced() {
                    return false;
                }
                for b in 1..self.num_rows() {
                    for a in 0..b {
                        if !self[(a, b)].is_zero() {
                            return false;
                        }
                    }
                }
                true
            }
        }

        impl<T: Div + PartialEq + Zero + One + Display + Clone> RREFDisplay for $target_type
            where
                $target_type: REF + REFDisplay + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T>,
                 <T as Div>::Output: Into<T> {
            fn gauss_jordan_display(&mut self) -> Option<Vec<String>> {
                if self.is_gauss_jordan() {
                    return None;
                }
                let mut steps = if !self.is_row_reduced() {
                    (*self).gaussian_elim_display().unwrap()
                } else {
                    Vec::new()
                };
                if !self.is_row_reduced() && steps.len() == 0 {
                    return None;
                }
                steps.push("------- RREF -------".to_string());
                for c in (1..self.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if self[(r, c)].is_zero() {
                            continue;
                        }
                        let self_rc = self[(r, c)].clone();
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, self_rc, c));
                        (*self).row_op_mul(c, self_rc.clone());
                        (*self).row_op_sub(r, c);
                        (*self).row_op_div(c, self_rc);
                    }
                }
                Some(steps)
            }
        }

        impl<T: Div + PartialEq + Zero + One + Debug + Clone> RREFDebug for $target_type
            where
                $target_type: REF + REFDebug + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T>,
                 <T as Div>::Output: Into<T> {
            fn gauss_jordan_debug(&mut self) -> Option<Vec<String>> {
                if self.is_gauss_jordan() {
                    return None;
                }
                let mut steps = if !self.is_row_reduced() {
                    (*self).gaussian_elim_debug().unwrap()
                } else {
                    Vec::new()
                };
                if !self.is_row_reduced() && steps.len() == 0 {
                    return None;
                }
                steps.push("------- RREF -------".to_string());
                for c in (1..self.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if self[(r, c)].is_zero() {
                            continue;
                        }
                        let self_rc = self[(r, c)].clone();
                        let step_no = steps.len();
                        steps.push(format!("Step {}: R{} - ({:?}) * R{} → R{0}", step_no, r,
                                           self_rc, c));
                        (*self).row_op_mul(c, self_rc.clone());
                        (*self).row_op_sub(r, c);
                        (*self).row_op_div(c, self_rc);
                    }
                }
                Some(steps)
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Clone> Inverse for $target_type
            where
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T> + Unit,
                 <T as Div>::Output: Into<T> {
            fn inverse(&self) -> Self {
                assert!(self.is_unit_dimension());
                let mut s = self.clone();
                let mut unit = $name::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            s.row_op_mul(c, amt2.clone());
                            unit.row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1.clone());
                                unit.row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    unit.row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        s.row_op_mul(c, src.clone());
                        unit.row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        unit.row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
                unit
            }

            fn try_inverse(&self) -> Result<Self, MatrixError> {
                if !(*self).is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut s = self.clone();
                let mut unit = $name::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            s.row_op_mul(c, amt2.clone());
                            unit.row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1.clone());
                                unit.row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    unit.row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                if !self.is_row_reduced() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string()));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        s.row_op_mul(c, src.clone());
                        unit.row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        unit.row_op_div(c, src);
                    }
                }
                if s.is_unit() {
                    Ok(unit)
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T> InverseDisplay for $target_type
            where
                T: Div + PartialOrd + PartialEq + Display + Zero + One + Clone,
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T> + Unit,
                 <T as Div>::Output: Into<T> {
            fn inverse_display(&self) -> (Self, Option<Vec<String>>) {
                assert!(self.is_unit_dimension());
                if (*self).is_unit() {
                    return (self.clone(), None);
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = $name::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            unit.row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                unit.row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({}) → R{0}", r, src));
                                    unit.row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        unit.row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        unit.row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
                (unit, Some(steps))
            }

            fn try_inverse_display(&self) -> Result<(Self, Option<Vec<String>>), MatrixError> {
                if !(*self).is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = $name::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            unit.row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            unit.row_op_div(c, amt2);
                            } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                unit.row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({}) → R{0}", r, src));
                                    unit.row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                if !self.is_row_reduced() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string()));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        unit.row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        unit.row_op_div(c, src);
                    }
                }
                if s.is_unit() {
                    Ok((unit, Some(steps)))
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T> InverseDebug for $target_type
            where
                T: Div + PartialOrd + PartialEq + Debug + Zero + One + Clone,
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T> + Unit,
                 <T as Div>::Output: Into<T> {
            fn inverse_debug(&self) -> (Self, Option<Vec<String>>) {
                assert!(self.is_unit_dimension());
                if self.is_unit() {
                    return (self.clone(), None);
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = $name::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            unit.row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                unit.row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, src));
                                    unit.row_op_div(r, src.clone());
                                    s.row_op_div(r, src.clone());
                                }
                            }
                        }
                    }
                }
                assert!(s.is_row_reduced());
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        unit.row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        unit.row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
                (unit, Some(steps))
            }

            fn try_inverse_debug(&self) -> Result<(Self, Option<Vec<String>>), MatrixError> {
                if !(*self).is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = $name::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            unit.row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                unit.row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, src));
                                    unit.row_op_div(r, src.clone());
                                    s.row_op_div(r, src.clone());
                                }
                            }
                        }
                    }
                }
                if !self.is_row_reduced() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string()));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        unit.row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        unit.row_op_div(c, src);
                    }
                }
                if s.is_unit() {
                    Ok((unit, Some(steps)))
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Clone> InverseAssign for $target_type
            where
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T> + Unit,
                 <T as Div>::Output: Into<T> {
            fn inverse_assign(&mut self) {
                assert!(self.is_unit_dimension());
                let mut s = $name::unit(self.rows);
                swap(&mut s, self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            s.row_op_mul(c, amt2.clone());
                            (*self).row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            (*self).row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1.clone());
                                (*self).row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                (*self).row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    (*self).row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                assert!(s.is_row_reduced());
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        s.row_op_mul(c, src.clone());
                        (*self).row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        (*self).row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        (*self).row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
            }

            fn try_inverse_assign(&mut self) -> Result<(), MatrixError> {
                if !(*self).is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut s = $name::unit(self.rows);
                swap(&mut s, self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            s.row_op_mul(c, amt2.clone());
                            (*self).row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            (*self).row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1.clone());
                                (*self).row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                (*self).row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    (*self).row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                if !self.is_row_reduced() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string()));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        s.row_op_mul(c, src.clone());
                        (*self).row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        (*self).row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        (*self).row_op_div(c, src);
                    }
                }
                if s.is_unit() {
                    Ok(())
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T> InverseAssignDisplay for $target_type
            where
                T: Div + PartialOrd + PartialEq + Display + Zero + One + Clone,
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T> + Unit,
                 <T as Div>::Output: Into<T> {
            fn inverse_assign_display(&mut self) -> Option<Vec<String>> {
                assert!(self.is_unit_dimension());
                if (*self).is_unit() {
                    return None;
                }
                let mut steps = Vec::new();
                let mut s = $name::unit(self.rows);
                swap(&mut s, self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            (*self).row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            (*self).row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                (*self).row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                (*self).row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({}) → R{0}", r, src));
                                    (*self).row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        (*self).row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        (*self).row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        (*self).row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
                Some(steps)
            }

            fn try_inverse_assign_display(&mut self) -> Result<Option<Vec<String>>, MatrixError> {
                if !(*self).is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                if (*self).is_unit() {
                    return Ok(None);
                }
                let mut steps = Vec::new();
                let mut s = $name::unit(self.rows);
                swap(&mut s, self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            (*self).row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            (*self).row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                (*self).row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                (*self).row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({}) → R{0}", r, src));
                                    (*self).row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                if !self.is_row_reduced() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string()));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        (*self).row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        (*self).row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        (*self).row_op_div(c, src);
                    }
                }
                if s.is_unit() {
                    Ok(Some(steps))
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Debug + Clone> InverseAssignDebug for $target_type
            where
                $target_type: REF + RowOpAdd + RowOpSub + RowOpMul<T> + RowOpDiv<T> + Unit,
                <T as Div>::Output: Into<T> {
            fn inverse_assign_debug(&mut self) -> Option<Vec<String>> {
                assert!(self.is_unit_dimension());
                if (*self).is_unit() {
                    return None;
                }
                let mut steps = Vec::new();
                let mut s = $name::unit(self.rows);
                swap(&mut s, self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            (*self).row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            (*self).row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                (*self).row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                (*self).row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, src));
                                    (*self).row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        (*self).row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        (*self).row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        (*self).row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
                Some(steps)
            }

            fn try_inverse_assign_debug(&mut self) -> Result<Option<Vec<String>>, MatrixError> {
                if !(*self).is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                if (*self).is_unit() {
                    return Ok(None);
                }
                let mut steps = Vec::new();
                let mut s = $name::unit(self.rows);
                swap(&mut s, self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)].clone();
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = (amt1 / s[(c, c)].clone()).into();
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, amt2, c));
                            s.row_op_mul(c, amt2.clone());
                            (*self).row_op_mul(c, amt2.clone());
                            s.row_op_sub(r, c);
                            (*self).row_op_sub(r, c);
                            s.row_op_div(c, amt2.clone());
                            (*self).row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1.clone());
                                (*self).row_op_div(r, amt1);
                            } else if amt1.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                (*self).row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    let src = s[(r, c)].clone();
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, src));
                                    (*self).row_op_div(r, src.clone());
                                    s.row_op_div(r, src);
                                }
                            }
                        }
                    }
                }
                if !self.is_row_reduced() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string()));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        if s[(r, c)].is_zero() {
                            continue;
                        }
                        let src = s[(r, c)].clone();
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                        s.row_op_mul(c, src.clone());
                        (*self).row_op_mul(c, src.clone());
                        s.row_op_sub(r, c);
                        (*self).row_op_sub(r, c);
                        s.row_op_div(c, src.clone());
                        (*self).row_op_div(c, src);
                    }
                }
                if s.is_unit() {
                    Ok(Some(steps))
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }
    )*)
}

transforms_impl!{Matrix<T>: Matrix, AugmentedMatrix<T>: AugmentedMatrix}