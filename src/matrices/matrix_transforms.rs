use num::{One, Zero};

use std::ops::{AddAssign, SubAssign, MulAssign, Div, DivAssign, Rem, Range};
use std::cmp::{Eq, PartialEq};
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
        if self.is_row_aligned() {
            for b in 0..self.num_columns() {
                self[target][b] += self[tool][b];
            }
        } else {
            for b in 0..self.num_columns() {
                self((target, b)) += self((tool, b));
            }
        }
    }
}

impl<T: SubAssign> RowOpSub for Matrix<T> {
    fn row_op_sub(&mut self, target: usize, tool: usize) {
        if self.is_row_aligned() {
            for b in 0..self.num_columns() {
                self[target][b] -= self[tool][b];
            }
        } else {
            for b in 0..self.num_columns() {
                self((target, b)) -= self((tool, b));
            }
        }
    }
}

impl<T, U> RowOpMul<U> for Matrix<T>
    where T: MulAssign<U> {
    fn row_op_mul(&mut self, target: usize, tool: U) {
        if self.is_row_aligned() {
            for b in 0..self.num_columns() {
                self[target][b] *= tool;
            }
        } else {
            for b in 0..self.num_columns() {
                self((target, b)) *= tool;
            }
        }
    }
}

impl<T, U> RowOpDiv<U> for Matrix<T>
    where T: DivAssign<U> {
    fn row_op_div(&mut self, target: usize, tool: U) {
        if self.is_row_aligned() {
            for b in 0..self.num_columns() {
                self[target][b] /= tool;
            }
        } else {
            for b in 0..self.num_columns() {
                self((target, b)) /= tool;
            }
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
        let mut row_gcd = if self.is_row_aligned() {
            gcd(self[row][0], self[row][1])
        } else {
            gcd(self[(row, 0)], self[(row, 1)])
        };
        for i in 2..self.num_columns() {
            if self[(row, i)].is_zero() {
                continue;
            }
            row_gcd = if self.is_row_aligned() {
                gcd(row_gcd, self[row][i])
            } else {
                gcd(row_gcd, self[(row, i)])
            };
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
        let mut row_gcd = if self.is_row_aligned() {
            gcd(self[row][0], self[row][1])
        } else {
            gcd(self[(row, 0)], self[(row, 1)])
        };
        for i in 2..self.num_columns() {
            if self[(row, i)].is_zero() {
                continue;
            }
            row_gcd = if self.is_row_aligned() {
                gcd(row_gcd, self[row][i])
            } else {
                gcd(row_gcd, self[(row, i)])
            };
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
        let mut row_gcd = if self.is_row_aligned() {
            gcd(self[row][0], self[row][1])
        } else {
            gcd(self[(row, 0)], self[(row, 1)])
        };
        for i in 2..self.num_columns() {
            if self[(row, i)].is_zero() {
                continue;
            }
            row_gcd = if self.is_row_aligned() {
                gcd(row_gcd, self[row][i])
            } else {
                gcd(row_gcd, self[(row, i)])
            };
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

trait Inv {
    fn inverse(&self) -> Self;
}

impl<T: Div + From<i64>> Inv for T {
    fn inverse(&self) -> Self {
        1u64.into() / self
    }
}

trait REF {
    fn try_gaussian(&mut self, print_steps: bool) -> Result<(), MatrixError>;
    fn gaussian(&mut self, print_steps: bool);
    fn is_REF(&self) -> bool;
}

trait RREF {
    fn try_gauss_jordan(&mut self, print_steps: bool) -> Result<(), MatrixError>;
    fn gauss_jordan(&mut self, print_steps: bool);
    fn is_RREF(&self) -> bool;
}

pub(crate) trait Inverse {
    fn inverse(&self, print_steps: bool);
    fn try_inverse(&self, print_steps: bool);
}

pub(crate) trait InverseAssign {
    fn inverse_assign(&mut self, print_steps: bool);
    fn try_inverse_assign(&mut self, print_steps: bool);
}

macro_rules! REF_contents {
    ($fn_name:ident) => { }
}

macro_rules! try_REF_contents {
    ($fn_name:ident) => { }
}

impl<T> REF for Matrix<T>
    where Matrix<T>: RowOpAdd + RowOpSub + RowOpMul + RowOpDiv, {
    fn try_gaussian(&mut self, print_steps: bool) -> Result<(), MatrixError> {

    }

    REF_contents!{gaussian}

    fn is_REF<T: PartialEq>(&self) -> bool {
        for a in 0..self.num_rows() {
            for b in 0..a {
                if !(&self.matrix[a][b]).eq(&Frac::from(0)) {
                    return false;
                }
            }
            if !(&self.matrix[a][a]).eq(&Frac::from(1)) {
                return false;
            }
        }
        true
    }
}

impl<T> RREF for Matrix<T> {
    fn try_gauss_jordan(&mut self, print_steps: bool) -> Result<(), MatrixError> {

    }

    fn gauss_jordan(&mut self, print_steps: bool) {

    }

    fn is_RREF(&self) -> bool {
        for b in 1..self.num_rows() {
            for a in 0..b {
                if self.matrix[a][b] != Frac::from(0) {
                    return false;
                }
            }
        }
        true
    }
}

pub mod transforms {
    use std::cmp;
    use fracs::*;
    use matrix_base::FracMatrix;
    use matrix_base::format::*;
    use matrix_base::MatrixError;

    impl FracMatrix {
        pub fn row_echelon_form(&mut self, print_steps: bool) {
            if print_steps {
                println!("------- Starting REF -------\n");
            }
            let max = cmp::min(self.dimension.0, self.dimension.1);
            for a in 0..max {
                for b in 0..a + 1 { // Keep tested values "below" or on the diagonal line
                    let amt1 = self.matrix[a][b].clone(); // Current value
                    if b < a { // "Under" the diagonal line
                        if amt1.num == 0 {
                            continue;
                        }
                        let mut sign;
                        let mut neg = false;
                        match amt1.num > 0 {
                            true => {
                                self.row_ops_mul(b, amt1);
                                self.row_ops_sub(a, b);
                                self.row_ops_div(b, amt1);
                                sign = String::from("-");
                            },
                            false => {
                                let mut tmpamt = amt1;
                                tmpamt.num *= -1;
                                self.row_ops_mul(b, tmpamt);
                                self.row_ops_add(a, b);
                                self.row_ops_div(b, tmpamt);
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
                            }, b + 1, self);
                        }
                        continue;
                    }
                    if b == a { // On the diagonal line
                        if amt1.num == 0 {
                            let mut other: i32 = -1;
                            // Find row beneath current one with a value in the columnn that the current
                            // row's missing
                            for i in (b..max).filter(|&i| i != a) {
                                if self.matrix[i][b].clone().num != 0 {
                                    other = i as i32;
                                    break;
                                }
                            }
                            if other == -1 { // It's okay if there isn't one - just move on
                                continue;
                            }
                            let other = other as usize;
                            let mut add = true;
                            let amt2 = self.matrix[other][b].clone(); // Get second value
                            match amt2.num > 0 {
                                true => {
                                    self.row_ops_add(b, other); // Get value in zero element
                                }
                                false => {
                                    add = false;
                                    self.row_ops_sub(b, other); // Get value in zero element
                                }
                            }
                            let sign = match add {
                                true => String::from("+"),
                                false => String::from("-")
                            };
                            if print_steps {
                                print!("R{} {} R{} → R{0}\n{}\n\n", a + 1, sign, other + 1, self);
                            }
                            let amt1 = self.matrix[a][b].clone(); // Refresh current value
                            if amt1.num != 1 {
                                self.row_ops_div(a, amt1);
                                if print_steps {
                                    let foo = amt1.clone().inverse();
                                    print!("({}) * R{} → R{1}\n{}\n\n", foo, a + 1, self);
                                }
                            }
                            continue;
                        }
                        self.row_ops_div(a, amt1); // Divide by self
                        if print_steps {
                            let amt1 = amt1.inverse();
                            print!("({}) * R{} → R{1}\n{}\n\n", amt1, a + 1, self);
                        }
                        continue;
                    }
                }
            }
        }

        pub fn reduced_row_echelon_form(&mut self, print_steps: bool) {
            self.row_echelon_form(print_steps);
            if !self.check_ref() {
                return;
            }
            if print_steps {
                println!("------- Completed REF, starting RREF -------\n");
            }
            let max = cmp::min(self.dimension.0, self.dimension.1);
            for a in (0..max - 1).rev() {
                for b in (a + 1..max).rev() {
                    let amt = self.matrix[a][b].clone();
                    if !amt.eq(&Frac::from(0)) {
                        self.row_ops_mul(b, amt);
                        self.row_ops_sub(a, b);
                        self.row_ops_div(b, amt);
                        if print_steps {
                            print!("R{} - ({}) * R{} → R{0}\n{}\n\n", a + 1, amt, b + 1, self);
                        }
                    }
                }
            }
        }

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