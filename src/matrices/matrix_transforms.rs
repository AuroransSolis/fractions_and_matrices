use num::{One, Zero};

use std::ops::{AddAssign, SubAssign, MulAssign, Div, DivAssign, Rem, Range};
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{Debug, Display};

use matrices::matrix_base::*;

trait RowOpAdd {
    fn row_op_add(&mut self, target: usize, tool: usize);
}

trait RowOpSub {
    fn row_op_sub(&mut self, target: usize, tool: usize);
}

trait RowOpMul<Scalar = usize> {
    fn row_op_mul(&mut self, target: usize, tool: Scalar);
}

trait RowOpDiv<scalar = usize> {
    fn row_op_div(&mut self, target: usize, tool: scalar);
}

impl<T: AddAssign> RowOpAdd for Matrix<T> {
    fn row_op_add(&mut self, target: usize, tool: usize) {
        for b in 0..self.num_columns() {
            self[(target, b)] += self[(tool, b)];
        }
    }
}

impl<T: SubAssign> RowOpSub for Matrix<T> {
    fn row_op_sub(&mut self, target: usize, tool: usize) {
        for b in 0..self.num_columns() {
            self[(target, b)] -= self[(tool, b)];
        }
    }
}

impl<T, U> RowOpMul<U> for Matrix<T>
    where T: MulAssign<U> {
    fn row_op_mul(&mut self, target: usize, tool: U) {
        for b in 0..self.num_columns() {
            self[(target, b)] *= tool;
        }
    }
}

impl<T, U> RowOpDiv<U> for Matrix<T>
    where T: DivAssign<U> {
    fn row_op_div(&mut self, target: usize, tool: U) {
        for b in 0..self.num_columns() {
            self[(target, b)] /= tool;
        }
    }
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

trait Simplify {
    fn simplify_row(&mut self, row: usize);
    fn simplify_rows(&mut self, rows: Range<usize>);
    fn simplify_matrix(&mut self);
}

trait SimplifyGetStepsDisplay {
    fn simplify_row_get_steps_ds(&mut self, row: usize) -> Option<String>;
    fn simplify_rows_get_steps_ds(&mut self, row: usize) -> Option<Vec<Option<String>>>;
    fn simplify_matrix_get_steps_ds(&mut self) -> Option<Vec<Option<String>>>;
}

trait SimplifyGetStepsDebug {
    fn simplify_row_get_steps_db(&mut self, row: usize) -> Option<String>;
    fn simplify_rows_get_steps_db(&mut self, row: usize) -> Option<String>;
    fn simplify_matrix_get_steps_db(&mut self) -> Option<String>;
}

trait SimplifyTraits: Div + DivAssign + Gcd + Zero + One + PartialEq {}
impl<T: Div + DivAssign + Gcd + Zero + One + PartialEq> SimplifyTraits for T {}

impl<T: SimplifyTraits> Simplify for Matrix<T> {
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

impl<T: SimplifyTraits + Display> SimplifyGetStepsDisplay for Matrix<T> {
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

    fn simplify_rows_get_steps_ds(&mut self, rows: Range<usize>) -> Option<Vec<Option<String>>> {
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

impl<T: SimplifyTraits + Debug> SimplifyGetStepsDebug for Matrix<T> {
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

    fn simplify_rows_get_steps_db(&mut self, rows: Range<usize>) -> Option<Vec<Option<String>>> {
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

trait REF {
    fn gaussian(&mut self);
    fn is_REF(&self) -> bool;
}

impl<T: PartialOrd + PartialEq + Zero + One> REF for Matrix<T>
    where Matrix<T>: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
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
                    // (c, c).is_zero(), just continue since doing more work on (r, c) right now
                    // is pointless
                    if self[(c, c)].is_zero() {
                        continue;
                    }
                    let selfcc = self[(c, c)];
                    self.row_op_mul(c, (amt1 / selfcc));
                    self.row_op_sub(r, c);
                    self.row_op_div(c, (amt1 / selfcc));
                } else if c == r { // If the value's on the major diagonal
                    if amt1.is_one() { // Continue if it's already what it should be
                        continue;
                    } else if !amt1.is_zero() { // If it's not zero...
                        self.row_op_div(r, amt1); // ...divide by itself to make it one
                    } if self.is_zero() { // If it is zero...
                        if self[(c, c)].is_zero() { // ...and the best tool is zlso zero, continue
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

trait REFDisplay {
    fn gaussian_display(&mut self) -> Option<Vec<String>>;
}

impl<T: Div + PartialOrd + PartialEq + Zero + One + Display> REFDisplay for Matrix<T>
    where Matrix<T>: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
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
                    let selfcc = self[(c, c)];
                    self.row_op_mul(c, (amt1 / selfcc));
                    self.row_op_sub(r, c);
                    self.row_op_div(c, (amt1 / selfcc));
                    steps.push(format!("R{} - ({}) * R{} → R{0}", r, amt1 / selfcc, c));
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


trait REFDebug {
    fn gaussian_debug(&mut self);
}

impl<T: Div + PartialOrd + PartialEq + Zero + One + Debug> REFDebug for Matrix<T>
    where Matrix<T>: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
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
                    let selfcc = self[(c, c)];
                    self.row_op_mul(c, (amt1 / selfcc));
                    self.row_op_sub(r, c);
                    self.row_op_div(c, (amt1 / selfcc));
                    steps.push(format!("Step {}: R{} - ({:?}) * R{} → R{0}", steps.len(), r,
                                       amt1 / selfcc, c));
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

trait RREF {
    fn gauss_jordan(&mut self);
    fn is_RREF(&self) -> bool;
}

impl<T: PartialEz + Zero + One> RREF for Matrix<T> where Matrix<T>: REF, {
    fn gauss_jordan(&mut self) {
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

trait RREFDisplay {
    fn gauss_jordan_display(&mut self) -> Option<Vec<String>>;
}

impl<T: PartialEz + Zero + One + Display> RREFDisplay for Matrix<T> where Matrix<T>: REF, {
    fn gauss_jordan_display(&mut self) -> Option<Vec<String>> {
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

trait RREFDebug {
    fn gauss_jordan_debug(&mut self) -> Option<Vec<String>>;
}

impl<T: PartialEz + Zero + One + Debug> RREFDebug for Matrix<T> where Matrix<T>: REF, {
    fn gauss_jordan_debug(&mut self) -> Option<Vec<String>> {
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
                steps.push(format!("R{} - ({:?}) * R{} → R{0}", r, self_rc, c));
            }
        }
        Some(steps)
    }
}

pub(crate) trait Inverse {
    fn inverse(&self, print_steps: bool);
}

pub(crate) trait InverseAssign {
    fn inverse_assign(&mut self, print_steps: bool);
}

pub mod transforms {
    use std::cmp;
    use fracs::*;
    use matrix_base::FracMatrix;
    use matrix_base::format::*;
    use matrix_base::MatrixError;

    impl FracMatrix {
        // The inverse can be achieved by taking a matrix and transforming it into a unit matrix (RREF
        // form) and applying the transformations to a unit matrix. The resulting non-unit matrix is the
        // inverse of the original. This function combines the REF and RREF functions above and applies
        // each transformation to a unit matrix.
        pub fn invert(&mut self, print_steps: bool) -> Result<(), MatrixError> {
            if self.dimension.0 != self.dimension.1 {
                return Err(MatrixError::OpError(
                    "Matrix must be square in dimension to calculate the inverse.".to_string()
                ));
            }
            let mut unit = match FracMatrix::from_dimension((self.dimension.0, self.dimension.1),
                                                            false) {
                Ok(matr) => matr,
                Err(e) => return Err(e),
            };
            for a in 0..unit.dimension.0 {
                unit.matrix[a][a] = Frac::from(1);
            }
            if print_steps {
                print!("Setup at start of inverse calculation:\n{}\n\n", add_mat_to_string(self.to_string(), &unit, Separator::Space));
            }
            let max = cmp::min(self.dimension.0, self.dimension.1);
            for a in 0..max {
                for b in 0..a + 1 { // Keep tested values "below" or on the diagonal line
                    let amt1 = self.matrix[a][b].clone(); // Current value
                    if b < a { // "Under" the diagonal line
                        if amt1.num == 0 {
                            continue;
                        }
                        let sign;
                        let mut neg = false;
                        match amt1.num > 0 {
                            true => {
                                self.row_ops_mul(b, amt1);
                                unit.row_ops_mul(b, amt1);
                                self.row_ops_sub(a, b);
                                unit.row_ops_sub(a, b);
                                self.row_ops_div(b, amt1);
                                unit.row_ops_div(b, amt1);
                                sign = String::from("-");
                            },
                            false => {
                                let mut tmpamt = amt1;
                                tmpamt.num *= -1;
                                self.row_ops_mul(b, tmpamt);
                                unit.row_ops_mul(b, tmpamt);
                                self.row_ops_add(a, b);
                                unit.row_ops_add(a, b);
                                self.row_ops_div(b, tmpamt);
                                unit.row_ops_div(b, tmpamt);
                                sign = String::from("+");
                                neg = true;
                            }
                        }
                        if print_steps {
                            print!("R{} {} ({}) * R{} → R{0}\n{}\n\n", a + 1, sign, {
                                if neg {
                                    amt1.negative().try_simplify()
                                } else {
                                    amt1
                                }
                            }, b + 1, add_mat_to_string(self.to_string(), &unit, Separator::Space));
                        }
                        continue;
                    }
                    if b == a { // On the diagonal line
                        if amt1.num == 0 {
                            let mut other: i32 = -1;
                            for i in (b..max).filter(|&i| i != a) {
                                if self.matrix[i][b].clone().num != 0 {
                                    other = i as i32;
                                    break;
                                }
                            }
                            if other == -1 {
                                continue;
                            }
                            let other = other as usize;
                            let mut add = true;
                            let amt2 = self.matrix[other][b].clone();
                            match amt2.num > 0 {
                                true => {
                                    self.row_ops_add(b, other);
                                    unit.row_ops_add(b, other);
                                }
                                false => {
                                    add = false;
                                    self.row_ops_sub(b, other);
                                    unit.row_ops_sub(b, other);
                                }
                            }
                            let sign = match add {
                                true => String::from("+"),
                                false => String::from("-")
                            };
                            if print_steps {
                                print!("R{} {} R{} → R{0}\n{}\n\n", a + 1, sign, other + 1,
                                       add_mat_to_string(self.to_string(), &unit, Separator::Space));
                            }
                            let amt1 = self.matrix[a][b].clone();
                            if amt1.num != 1 {
                                self.row_ops_div(a, amt1);
                                if print_steps {
                                    let foo = amt1.clone().inverse();
                                    print!("({}) * R{} → R{1}\n{}\n\n", foo, a + 1,
                                           add_mat_to_string(self.to_string(), &unit, Separator::Space));
                                }
                            }
                            continue;
                        }
                        self.row_ops_div(a, amt1); // Divide by self
                        unit.row_ops_div(a, amt1);
                        if print_steps {
                            let amt1 = amt1.inverse();
                            print!("({}) * R{} → R{1}\n{}\n\n", amt1, a + 1,
                                   add_mat_to_string(self.to_string(), &unit, Separator::Space));
                        }
                        continue;
                    }
                }
            }
            for a in (0..max - 1).rev() {
                for b in (a + 1..max).rev() {
                    let amt = self.matrix[a][b].clone();
                    if !amt.eq(&Frac::from(0)) {
                        self.row_ops_mul(b, amt);
                        unit.row_ops_mul(b, amt);
                        self.row_ops_sub(a, b);
                        unit.row_ops_sub(a, b);
                        self.row_ops_div(b, amt);
                        unit.row_ops_div(b, amt);
                        if print_steps {
                            print!("R{} - ({}) * R{} → R{0}\n{}\n\n", a + 1, amt, b + 1,
                                   add_mat_to_string(self.to_string(), &unit, Separator::Space))
                        }
                    }
                }
            }
            for a in 0..max { // Check to see if the original matrix is now a unit matrix
                for b in 0..max {
                    if a != b && !self.matrix[b][a].clone().eq(&Frac::from(0)) {
                        return Err(MatrixError::OpError(
                            "Unable to convert matrix into unit matrix to make the inverse."
                                .to_string()
                        ));
                    }
                    if a == b && !self.matrix[b][a].clone().eq(&Frac::from(1)) {
                        return Err(MatrixError::OpError(
                            "Unable to convert matrix into unit matrix to make the inverse."
                                .to_string()
                        ));
                    }
                }
            }
            Ok(())
        }

        pub fn inverse(&self, print_steps: bool) -> Result<FracMatrix, MatrixError> {
            let mut tmp = self.clone();
            match tmp.invert(print_steps) {
                Err(e) => Err(e),
                Ok(_) => Ok(tmp)
            }
        }
    }
}