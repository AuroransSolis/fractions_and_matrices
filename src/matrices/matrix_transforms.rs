use matrices::matrix_base::*;

pub(crate) struct TSOpts {
    try: bool,
    print: bool
}

pub const NOTRY_NOPRINT: TSOpts = TSOpts { try: false, print: false };

pub const TRY_NOPRINT: TSOpts = TSOpts { try: true, print: false };

pub const TRY_PRINT: TSOpts = TSOpts { try: true, print: true };

trait RowOpAdd {
    fn row_op_add(&mut self, target: usize, tool: usize);
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

trait RowOpSub {
    fn row_op_sub(&mut self, target: usize, tool: usize);
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

trait RowOpMul<scalar = usize> {
    fn row_op_mul(&mut self, target: usize, tool: scalar);
}

impl<T, U> RowOpMul for Matrix<T>
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

trait RowOpDiv<scalar = usize> {
    fn row_op_div(&mut self, target: usize, tool: scalar);
}

impl<T, U> RowOpDiv for Matrix<T>
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

trait TrySimplify {
    fn try_simplify(&mut self, print_steps: bool) -> Result<(), MatrixError>;
}

trait Simplify {
    fn simplify(&mut self, print_steps: bool);
}

trait Inv {
    fn inverse(&self) -> Self;
}

impl<T: Div + From<i64>> Inv for T {
    fn inverse(&self) -> Self {
        1u64.into() / self
    }
}

macro_rules! REF_contents {
    ($fn_name:ident) => { }
}

macro_rules! try_REF_contents {
    ($fn_name:ident) => { }
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

impl<T: > REF for Matrix<T> {
    fn try_gaussian(&mut self, print_steps: bool) -> Result<(), MatrixError> {

    }

    REF_contents!{gaussian}

    fn is_REF(&self) -> bool {
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

trait Inverse {
    fn inverse(&self, print_steps: bool);
    fn try_inverse(&self, print_steps: bool);
}

trait InverseAssign {
    fn inverse_assign(&mut self, print_steps: bool);
    fn try_inverse_assign(&mut self, print_steps: bool);
}

pub mod transforms {
    use std::cmp;
    use fracs::*;
    use matrix_base::FracMatrix;
    use matrix_base::format::*;
    use matrix_base::MatrixError;

    impl FracMatrix {
        pub fn try_simplify_matrix(&mut self, print_steps: bool) {
            println!("Attempting to simplify matrix.\n");
            let mut did_simplification = false;
            for row in 0..self.matrix.len() {
                match self.try_simplify(row, print_steps) {
                    Ok(true) => did_simplification = true,
                    Ok(false) => {},
                    Err(e) => println!("Error in simplifying row {}: {}", row, e),
                }
            }
            if did_simplification {
                println!("\nWas able to simplify. New matrix:\n\n{}\n", self);
            }
            if !did_simplification {
                println!("\nWas unable to simplify.\n");
            }
        }

        pub fn try_simplify(&mut self, row: usize, print_steps: bool)
                            -> Result<bool, String> {
            if self.matrix[row].len() > 1 {
                let row_vec: &Vec<Frac> = &self.clone().matrix[row];
                let (mut plus_minus_one_count, mut neg_count, mut non_zeros) = (0, 0, Vec::new());
                for i in 0..row_vec.len() {
                    let tst = row_vec[i];
                    match tst.cmp(&Frac::from(0)) {
                        Ok(CmpRes::Eq) => {},
                        Ok(CmpRes::Lt) => {
                            neg_count += 1;
                            match tst.eq(&Frac::from(-1)) {
                                true => plus_minus_one_count += 1,
                                false => non_zeros.push(tst)
                            }
                        },
                        Ok(CmpRes::Gt) => {
                            match tst.eq(&Frac::from(1)) {
                                true => plus_minus_one_count += 1,
                                false => non_zeros.push(tst)
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }
                if neg_count == self.matrix[row].len() {
                    for b in 0..neg_count {
                        self.matrix[row][b] = self.matrix[row][b].negative();
                    }
                    if print_steps {
                        print!("(-1) * R{} → R{0}\n{}\n\n", row, self);
                    }
                }
                if non_zeros.len() > 1 {
                    if plus_minus_one_count == 0 {
                        let first_frac = non_zeros[0];
                        let second_frac = non_zeros[1];
                        let mut num_gcd = get_gcd(first_frac.num as u32, second_frac.num as u32);
                        let mut den_gcd = get_gcd(first_frac.den as u32, second_frac.den as u32);
                        if num_gcd == 1 && den_gcd == 1 {
                            return Ok(false);
                        }
                        if non_zeros.len() > 2 {
                            for i in 2..non_zeros.len() {
                                let next = non_zeros[i];
                                num_gcd = get_gcd(num_gcd, next.num as u32);
                                den_gcd = get_gcd(den_gcd, next.den as u32);
                                if num_gcd == 1 && den_gcd == 1 {
                                    return Ok(false);
                                }
                            }
                        }
                        let sorta_gcd = Frac::new(num_gcd as i32, den_gcd as i32);
                        for b in 0..row_vec.len() {
                            self.matrix[row][b] = self.matrix[row][b].div(sorta_gcd);
                        }
                        if print_steps {
                            print!("({}) * R{} → R{1}\n{}\n\n", sorta_gcd.inverse(), row, self);
                        }
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }

        pub fn try_simplify_matrix_unchecked(&mut self, print_steps: bool) {
            println!("Attempting to simplify matrix.\n");
            let mut did_simplification = false;
            for row in 0..self.matrix.len() {
                match self.try_simplify_unchecked(row, print_steps) {
                    true => did_simplification = true,
                    _ => {}
                }
            }
            if did_simplification {
                println!("\nWas able to simplify. New matrix:\n\n{}\n", self);
            }
            if !did_simplification {
                println!("\nWas unable to simplify.\n");
            }
        }

        pub fn try_simplify_unchecked(&mut self, row: usize, print_steps: bool) -> bool {
            if self.matrix[row].len() > 1 {
                let row_vec: &Vec<Frac> = &self.clone().matrix[row];
                let (mut plus_minus_one_count, mut neg_count, mut non_zeros) = (0, 0, Vec::new());
                for i in 0..row_vec.len() {
                    let tst = row_vec[i];
                    match tst.cmp_unchecked(&Frac::from(0)) {
                        CmpRes::Eq => {},
                        CmpRes::Lt => {
                            neg_count += 1;
                            match tst.eq(&Frac::from(-1)) {
                                true => plus_minus_one_count += 1,
                                false => non_zeros.push(tst)
                            }
                        }
                        CmpRes::Gt => {
                            match tst.eq(&Frac::from(1)) {
                                true => plus_minus_one_count += 1,
                                false => non_zeros.push(tst)
                            }
                        }
                    }
                }
                if neg_count == self.matrix[row].len() {
                    for b in 0..neg_count {
                        self.matrix[row][b] = self.matrix[row][b].negative();
                    }
                    if print_steps {
                        print!("(-1) * R{} → R{0}\n{}\n\n", row, self);
                    }
                }
                if non_zeros.len() > 1 {
                    if plus_minus_one_count == 0 {
                        let first_frac = non_zeros[0];
                        let second_frac = non_zeros[1];
                        let mut num_gcd = get_gcd(first_frac.num as u32, second_frac.num as u32);
                        let mut den_gcd = get_gcd(first_frac.den as u32, second_frac.den as u32);
                        if num_gcd == 1 && den_gcd == 1 {
                            return false;
                        }
                        if non_zeros.len() > 2 {
                            for i in 2..non_zeros.len() {
                                let next = non_zeros[i];
                                num_gcd = get_gcd(num_gcd, next.num as u32);
                                den_gcd = get_gcd(den_gcd, next.den as u32);
                                if num_gcd == 1 && den_gcd == 1 {
                                    return false;
                                }
                            }
                        }
                        let sorta_gcd = Frac::new(num_gcd as i32, den_gcd as i32);
                        for b in 0..row_vec.len() {
                            self.matrix[row][b] = self.matrix[row][b].div(sorta_gcd);
                        }
                        if print_steps {
                            print!("({}) * R{} → R{1}\n{}\n\n", sorta_gcd.inverse(), row, self);
                        }
                        return true;
                    }
                }
            }
            false
        }

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