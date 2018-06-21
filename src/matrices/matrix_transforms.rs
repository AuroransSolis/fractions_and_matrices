use num::{One, Zero};

use std::ops::{AddAssign, SubAssign, MulAssign, Neg, Div, DivAssign, Rem, Range};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{Debug, Display};

use matrices::matrix_base::*;

pub trait RowOpAdd {
    fn row_op_add(&mut self, target: usize, tool: usize);
}

pub trait RowOpSub {
    fn row_op_sub(&mut self, target: usize, tool: usize);
}

pub trait RowOpMul<Scalar = usize> {
    fn row_op_mul(&mut self, target: usize, tool: Scalar);
}

pub trait RowOpDiv<scalar = usize> {
    fn row_op_div(&mut self, target: usize, tool: scalar);
}

trait Gcd: Rem + PartialEq {}
impl<T: Rem + PartialEq> Gcd for T {}

fn gcd<T: Gcd>(a: T, b: T) {
    if b == 0 {
        return a;
    } else {
        gcd(b, a % b)
    }
}

pub trait Simplify {
    fn simplify_row(&mut self, row: usize);
    fn simplify_rows(&mut self, rows: Range<usize>);
    fn simplify_matrix(&mut self);
}

pub trait SimplifyGetStepsDisplay {
    fn simplify_row_get_steps_ds(&mut self, row: usize) -> Option<String>;
    fn simplify_rows_get_steps_ds(&mut self, row: usize) -> Option<Vec<Option<String>>>;
    fn simplify_matrix_get_steps_ds(&mut self) -> Option<Vec<Option<String>>>;
}

pub trait SimplifyGetStepsDebug {
    fn simplify_row_get_steps_db(&mut self, row: usize) -> Option<String>;
    fn simplify_rows_get_steps_db(&mut self, row: usize) -> Option<String>;
    fn simplify_matrix_get_steps_db(&mut self) -> Option<String>;
}

trait SimplifyTraits: Div + DivAssign + Gcd + Zero + One + PartialEq {}
impl<T: Div + DivAssign + Gcd + Zero + One + PartialEq> SimplifyTraits for T {}

pub trait REF {
    fn gaussian(&mut self);
    fn is_REF(&self) -> bool;
}

pub trait REFDisplay {
    fn gaussian_display(&mut self) -> Option<Vec<String>>;
}

pub trait REFDebug {
    fn gaussian_debug(&mut self);
}

pub trait RREF {
    fn gauss_jordan(&mut self);
    fn is_RREF(&self) -> bool;
}

pub trait RREFDisplay {
    fn gauss_jordan_display(&mut self) -> Option<Vec<String>>;
}

pub trait RREFDebug {
    fn gauss_jordan_debug(&mut self) -> Option<Vec<String>>;
}

pub trait Inverse {
    fn inverse(&self) -> Self;
    fn try_inverse(&self) -> Result<Self, MatrixError>;
}

pub trait InverseDisplay {
    fn inverse_display(&self) -> (Self, Option<Vec<String>>);
    fn try_inverse_display(&self) -> Result<(Self, Option<Vec<String>>), MatrixError>;
}

pub trait InverseDebug {
    fn inverse_debug(&self) -> (Self, Option<Vec<String>>);
    fn try_inverse_debug(&self) -> Result<(Self, Option<Vec<String>>), MatrixError>;
}

pub trait InverseAssign {
    fn inverse_assign(&mut self);
    fn try_inverse_assign(&mut self) -> Result<(), MatrixError>;
}

pub trait InverseAssignDisplay {
    fn inverse_assign_display(&mut self) -> Option<Vec<String>>;
    fn try_inverse_assign_display(&mut self) -> Result<Option<Vec<String>>, MatrixError>;
}

pub trait InverseAssignDebug {
    fn inverse_assign_debug(&mut self) -> Option<Vec<String>>;
    fn try_inverse_assign_debug(&mut self) -> Result<Option<Vec<String>>, MatrixError>;
}

macro_rules! transforms_impl {
    ($($target_type:ty),*) => ($(
        impl<T: AddAssign> RowOpAdd for $target_type {
            fn row_op_add(&mut self, target: usize, tool: usize) {
                for b in 0..self.num_columns() {
                    self[(target, b)] += self[(tool, b)];
                }
            }
        }

        impl<T: SubAssign> RowOpSub for $target_type {
            fn row_op_sub(&mut self, target: usize, tool: usize) {
                for b in 0..self.num_columns() {
                    self[(target, b)] -= self[(tool, b)];
                }
            }
        }

        impl<T, U> RowOpMul<U> for $target_type
            where T: MulAssign<U> {
            fn row_op_mul(&mut self, target: usize, tool: U) {
                for b in 0..self.num_columns() {
                    self[(target, b)] *= tool;
                }
            }
        }

        impl<T, U> RowOpDiv<U> for $target_type
            where T: DivAssign<U> {
            fn row_op_div(&mut self, target: usize, tool: U) {
                for b in 0..self.num_columns() {
                    self[(target, b)] /= tool;
                }
            }
        }

        impl<T: SimplifyTraits> Simplify for $target_type {
            fn simplify_row(&mut self, row: usize) {
                if self.num_columns() < 2 {
                    return;
                }
                let mut row_gcd = gcd(self[(row, 0)], self[(row, 1)]);
                for i in 2..self.num_columns() {
                    if self[(row, i)].is_zero() {
                        continue;
                    }
                    row_gcd = gcd(row_gcd, self[(row, i)]);
                    if row_gcd == T::one() {
                        return;
                    }
                }
                if self.is_row_aligned() {
                    for e in self[row].iter_mut() {
                        e /= row_gcd;
                    }
                } else {
                    for i in (r * self.num_columns())..((r + 1) * self.num_columns()) {
                        self[(row, i)] /= row_gcd;
                    }
                }
            }

            fn simplify_rows(&mut self, rows: Range<usize>) {
                for r in rows {
                    self.simplify_row(r);
                }
            }

            fn simplify_matrix(&mut self) {
                self.simplify_rows((0..self.num_rows()));
            }
        }

        impl<T: SimplifyTraits + Display> SimplifyGetStepsDisplay for $target_type {
            fn simplify_row_get_steps_ds(&mut self, row: usize) -> Option<String> {
                if self.num_columns() < 2 {
                    return None;
                }
                let mut row_gcd = gcd(self[(row, 0)], self[(row, 1)]);
                for i in 2..self.num_columns() {
                    if self[(row, i)].is_zero() {
                        continue;
                    }
                    row_gcd = gcd(row_gcd, self[(row, i)]);
                    if row_gcd == T::one() {
                        return None;
                    }
                }
                if self.is_row_aligned() {
                    for e in self[row].iter_mut() {
                        e /= row_gcd;
                    }
                } else {
                    for i in (r * self.num_columns())..((r + 1) * self.num_columns()) {
                        self[(row, i)] /= row_gcd;
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
                self.simplify_rows_get_steps_ds((0..self.num_rows()))
            }
        }

        impl<T: SimplifyTraits + Debug> SimplifyGetStepsDebug for $target_type {
            fn simplify_row_get_steps_db(&mut self, row: usize) -> Option<String> {
                if self.num_columns() < 2 {
                    return None;
                }
                let mut row_gcd = gcd(self[(row, 0)], self[(row, 1)]);
                for i in 2..self.num_columns() {
                    if self[(row, i)].is_zero() {
                        continue;
                    }
                    row_gcd = gcd(row_gcd, self[(row, i)]);
                    if row_gcd == T::one() {
                        return None;
                    }
                }
                if self.is_row_aligned() {
                    for e in self[row].iter_mut() {
                        e /= row_gcd;
                    }
                } else {
                    for i in (r * self.num_columns())..((r + 1) * self.num_columns()) {
                        self[(row, i)] /= row_gcd;
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
                self.simplify_rows_get_steps_db((0..self.num_rows()))
            }
        }

        impl<T: PartialOrd + PartialEq + Zero + One> REF for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn gaussian(&mut self) {
                if self.is_REF() {
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
                            let amt2 = amt1 / self[(c, c)];
                            self.row_op_mul(c, amt2);
                            self.row_op_sub(r, c);
                            self.row_op_div(c, amt2);
                        } else if c == r { // If the value's on the major diagonal
                            if amt1.is_one() { // Continue if it's already what it should be
                                continue;
                            } else if !amt1.is_zero() { // If it's not zero...
                                self.row_op_div(r, amt1); // ...divide by itself to make it one
                            } if self.is_zero() { // If it is zero...
                                // ...and the best tool is also zero, continue
                                if self[(c, c)].is_zero() {
                                    continue;
                                }
                                self.row_op_add(r, c); // Otherwise, add said tool to row 'r'
                                if !self[(r, c)].is_one() { // If that tool somehow wasn't 1...
                                    self.row_op_div(r, self[(r, c)]); // ...divide by itself
                                }
                            }
                        }
                    }
                }
            }

            fn is_REF(&self) -> bool {
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

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Display> REFDisplay for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn gaussian_display(&mut self) -> Option<Vec<String>> {
                if self.is_REF() {
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
                            let amt2 = amt1 / self[(c, c)];
                            self.row_op_mul(c, amt2);
                            self.row_op_sub(r, c);
                            self.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                self.row_op_div(r, amt1);
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                            } if self.is_zero() {
                                if self[(c, c)].is_zero() {
                                    continue;
                                }
                                self.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !self[(r, c)].is_one() {
                                    self.row_op_div(r, self[(r, c)]);
                                    steps.push(format!("R{} / ({}) → R{0}", r, self[(r, c)]));
                                }
                            }
                        }
                    }
                }
                Some(steps)
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Zero + One + Debug> REFDebug for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn gaussian_debug(&mut self) -> Option<Vec<String>> {
                if self.is_REF() {
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
                            let amt2 = amt1 / self[(c, c)];
                            self.row_op_mul(c, amt2);
                            self.row_op_sub(r, c);
                            self.row_op_div(c, amt2);
                            steps.push(format!("Step {}: R{} - ({:?}) * R{} → R{0}", steps.len(), r,
                                               amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                self.row_op_div(r, amt1);
                                steps.push(format!("Step {}: R{} / ({:?}) → R{0}", steps.len(), r,
                                                   amt1));
                            } if self.is_zero() {
                                if self[(c, c)].is_zero() {
                                    continue;
                                }
                                self.row_op_add(r, c);
                                steps.push(format!("Step {}: R{} + R{} → R{0}", steps.len(), r, c));
                                if !self[(r, c)].is_one() {
                                    self.row_op_div(r, self[(r, c)]);
                                    steps.push(format!("Step {}: R{} / ({:?}) → R{0}", steps.len(),
                                                       r, self[(r, c)]));
                                }
                            }
                        }
                    }
                }
                Some(steps)
            }
        }

        impl<T: PartialEq + Zero + One> RREF for $target_type where $target_type: REF, {
            fn gauss_jordan(&mut self) {
                if self.is_RREF() {
                    return;
                }
                if !self.is_REF() {
                    self.gaussian();
                }
                if !self.is_REF() {
                    return;
                }
                for c in (1..self.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let self_rc = self[(r, c)];
                        if self_rc.is_zero() {
                            continue;
                        }
                        self.row_op_mul(c, self_rc);
                        self.row_op_sub(r, c);
                        self.row_op_div(c, self_rc);
                    }
                }
            }

            fn is_RREF(&self) -> bool {
                if !self.is_REF() {
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

        impl<T: Neg + PartialEq + PartialOrd + Zero + One + Display> RREFDisplay for $target_type
            where $target_type: REF, {
            fn gauss_jordan_display(&mut self) -> Option<Vec<String>> {
                if self.is_RREF() {
                    return None;
                }
                let mut steps = if !self.is_REF() {
                    self.gaussian_display().unwrap()
                } else {
                    Vec::new()
                };
                if !self.is_REF() && steps.len() == 0 {
                    return None;
                }
                steps.push("------- RREF -------".to_string());
                for c in (1..self.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let self_rc = self[(r, c)];
                        if self_rc.is_zero() {
                            continue;
                        }
                        self.row_op_mul(c, self_rc);
                        self.row_op_sub(r, c);
                        self.row_op_div(c, self_rc);
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, self_rc, c));
                    }
                }
                Some(steps)
            }
        }

        impl<T: Neg + PartialEq + PartialOrd + Zero + One + Debug> RREFDebug for $target_type
            where $target_type: REF, {
            fn gauss_jordan_debug(&mut self) -> Option<Vec<String>> {
                if self.is_RREF() {
                    return None;
                }
                let mut steps = if !self.is_REF() {
                    self.gaussian_display().unwrap()
                } else {
                    Vec::new()
                };
                if !self.is_REF() && steps.len() == 0 {
                    return None;
                }
                steps.push("------- RREF -------".to_string());
                for c in (1..self.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let self_rc = self[(r, c)];
                        if self_rc.is_zero() {
                            continue;
                        }
                        self.row_op_mul(c, self_rc);
                        self.row_op_sub(r, c);
                        self.row_op_div(c, self_rc);
                        steps.push(format!("Step {}: R{} - ({:?}) * R{} → R{0}", steps.len(), r,
                                           self_rc, c));
                    }
                }
                Some(steps)
            }
        }

        impl<T: PartialOrd + PartialEq + Unit> Inverse for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn inverse(&self) -> Self {
                assert!(self.is_unit_dimension());
                let mut s = self.clone();
                let mut unit = Matrix::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            unit.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1);
                                unit.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    unit.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        unit.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        unit.row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
                unit
            }

            fn try_inverse(&self) -> Result<Self, MatrixError> {
                if !self.is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut s = self.clone();
                let mut unit = Matrix::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            unit.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            unit.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1);
                                unit.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    unit.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                if !self.is_REF() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        unit.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src);
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

        impl<T: PartialOrd + PartialEq + Unit + Display> InverseDisplay for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn inverse_display(&self) -> (Self, Option<Vec<String>>) {
                assert!(self.is_unit_dimension());
                if self.is_unit() {
                    return (self, None);
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = Matrix::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            unit.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            unit.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                unit.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({}) → R{0}", r, s[(r, c)]));
                                    unit.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        unit.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        unit.row_op_div(c, src);
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                    }
                }
                assert!(s.is_unit());
                (unit, Some(steps))
            }

            fn try_inverse_display(&self) -> Result<(Self, Option<Vec<String>>), MatrixError> {
                if !self.is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = Matrix::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            unit.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            unit.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                unit.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({}) → R{0}", r, s[(r, c)]));
                                    unit.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                if !self.is_REF() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        unit.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        unit.row_op_div(c, src);
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                    }
                }
                if s.is_unit() {
                    Ok((unit, steps))
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T: PartialOrd + PartialEq + Unit + Debug> InverseDebug for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn inverse_debug(&self) -> (Self, Option<Vec<String>>) {
                assert!(self.is_unit_dimension());
                if self.is_unit() {
                    return (self, None);
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = Matrix::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            unit.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            unit.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                unit.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, s[(r, c)]));
                                    unit.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        unit.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        unit.row_op_div(c, src);
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                    }
                }
                assert!(s.is_unit());
                (unit, Some(steps))
            }

            fn try_inverse_debug(&self) -> Result<(Self, Option<Vec<String>>), MatrixError> {
                if !self.is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut steps = Vec::new();
                let mut s = self.clone();
                let mut unit = Matrix::unit(self.rows);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            unit.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            unit.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            unit.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                unit.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                unit.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, s[(r, c)]));
                                    unit.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                if !self.is_REF() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        unit.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        unit.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        unit.row_op_div(c, src);
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                    }
                }
                if s.is_unit() {
                    Ok((unit, steps))
                } else {
                    Err(MatrixError::TransformError("Was unable to make an inverse - unable to put \
                    original matrix in RREF form.".to_string()))
                }
            }
        }

        impl<T: Div + PartialOrd + PartialEq + Unit> InverseAssign for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn inverse_assign(&mut self) {
                assert!(self.is_unit_dimension());
                let mut s = Matrix::unit(self.rows);
                std::mem::swap(&mut s, &mut self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / scc;
                            s.row_op_mul(c, amt2);
                            self.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            self.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            self.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1);
                                self.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                self.row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    self.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        self.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        self.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        self.row_op_div(c, src);
                    }
                }
                assert!(s.is_unit());
            }

            fn try_inverse_assign(&mut self) -> Result<(), MatrixError> {
                if !self.is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                let mut s = Matrix::unit(self.rows);
                std::mem::swap(&mut s, &mut self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            self.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            self.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            self.row_op_div(c, amt2);
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                s.row_op_div(r, amt1);
                                self.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                self.row_op_add(r, c);
                                if !s[(r, c)].is_one() {
                                    self.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                if !self.is_REF() {
                    return Err(MatrixError::TransformError("Was unable to make an inverse - unable \
                    to put original matrix in REF form.".to_string));
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        self.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        self.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        self.row_op_div(c, src);
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

        impl<T: PartialOrd + PartialEq + Unit + Display> InverseAssignDisplay for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn inverse_assign_display(&mut self) -> Option<Vec<String>> {
                assert!(self.is_unit_dimension());
                if self.is_unit() {
                    return None;
                }
                let mut steps = Vec::new();
                let mut s = Matrix::unit(self.rows);
                std::mem::swap(&mut s, &mut self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            self.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            self.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            self.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                self.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                self.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({}) → R{0}", r, s[(r, c)]));
                                    self.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        self.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        self.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        self.row_op_div(c, src);
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
                    }
                }
                assert!(s.is_unit());
                Some(steps)
            }

            fn try_inverse_assign_display(&mut self) -> Result<Option<Vec<String>>, MatrixError> {
                if !self.is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                if self.is_unit() {
                    return Ok(None);
                }
                let mut steps = Vec::new();
                let mut s = Matrix::unit(self.rows);
                std::mem::swap(&mut s, &mut self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let scc = s[(c, c)];
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            self.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            self.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            self.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt2, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                self.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                self.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({}) → R{0}", r, s[(r, c)]));
                                    self.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        self.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        self.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        self.row_op_div(c, src);
                        steps.push(format!("R{} - ({}) * R{} → R{0}", r, src, c));
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

        impl<T: Div + PartialOrd + PartialEq + Unit + Debug> InverseAssignDebug for $target_type
            where $target_type: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
            fn inverse_assign_debug(&mut self) -> Option<Vec<String>> {
                assert!(self.is_unit_dimension());
                if self.is_unit() {
                    return None;
                }
                let mut steps = Vec::new();
                let mut s = Matrix::unit(self.rows);
                std::mem::swap(&mut s, &mut self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            self.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            self.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            self.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, scc, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                self.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                self.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, s[(r, c)]));
                                    self.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        self.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        self.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        self.row_op_div(c, src);
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
                    }
                }
                assert!(s.is_unit());
                Some(steps)
            }

            fn try_inverse_assign_debug(&mut self)
                -> Result<(Self, Option<Vec<String>>), MatrixError> {
                if !self.is_unit_dimension() {
                    return Err(MatrixError::InitError("Matrix does not have the same number of \
                    rows and columns - unable to make inverse.".to_string()));
                }
                if self.is_unit() {
                    return Ok(None);
                }
                let mut steps = Vec::new();
                let mut s = Matrix::unit(self.rows);
                std::mem::swap(&mut s, &mut self);
                for r in 0..s.num_rows() {
                    for c in 0..r + 1 {
                        let amt1 = s[(r, c)];
                        if c < r {
                            if amt1.is_zero() {
                                continue;
                            }
                            if s[(c, c)].is_zero() {
                                continue;
                            }
                            let amt2 = amt1 / s[(c, c)];
                            s.row_op_mul(c, amt2);
                            self.row_opp_mul(c, amt2);
                            s.row_op_sub(r, c);
                            self.row_op_sub(r, c);
                            s.row_op_div(c, amt2);
                            self.row_op_div(c, amt2);
                            steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, scc, c));
                        } else if c == r {
                            if amt1.is_one() {
                                continue;
                            } else if !amt1.is_zero() {
                                steps.push(format!("R{} / ({:?}) → R{0}", r, amt1));
                                s.row_op_div(r, amt1);
                                self.row_op_div(r, amt1);
                            }
                            if s.is_zero() {
                                if s[(c, c)].is_zero() {
                                    continue;
                                }
                                s.row_op_add(r, c);
                                self.row_op_add(r, c);
                                steps.push(format!("R{} + R{} → R{0}", r, c));
                                if !s[(r, c)].is_one() {
                                    steps.push(format!("R{} / ({:?}) → R{0}", r, s[(r, c)]));
                                    self.row_op_div(r, s[(r, c)]);
                                    s.row_op_div(r, s[(r, c)]);
                                }
                            }
                        }
                    }
                }
                for c in (1..s.num_columns()).rev() {
                    for r in (0..c).rev() {
                        let src = s[(r, c)];
                        if src.is_zero() {
                            continue;
                        }
                        s.row_op_mul(c, src);
                        self.row_op_mul(c, src);
                        s.row_op_sub(r, c);
                        self.row_op_sub(r, c);
                        s.row_op_div(c, src);
                        self.row_op_div(c, src);
                        steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, src, c));
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

transforms_impl!{Matrix<T>, AugmentedMatrix<T>}