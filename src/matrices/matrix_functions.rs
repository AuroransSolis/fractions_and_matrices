use std::cmp;

use matrix_base::*;

trait Inverse {
    fn inverse(&self, print_steps: bool);
    fn try_inverse(&self, print_steps: bool);
}

trait InverseAssign {
    fn inverse_assign(&mut self, print_steps: bool);
    fn try_inverse_assign(&mut self, print_steps: bool);
}

impl Matrix {
    pub fn determinant(&self) -> Result<i64, MatrixError> {
        if self.dimension.0 != self.dimension.1 {
            return MatrixError::FunctionError("Matrix is not square - cannot calculate determinant".to_string());
        } else if self.dimension.0 == 2 {
            return self.matrix[0][0].mul(self.matrix[1][1]).sub(self.matrix[0][1].mul(self.matrix[1][0]));
        } else if self.dimension.0 == 3 {
            return {
                let i = self.matrix[0][0].mul(
                    self.matrix[1][1].mul(self.matrix[2][2])
                        .sub(self.matrix[2][1].mul(self.matrix[1][2])));
                let j = self.matrix[0][1].mul(
                    self.matrix[1][0].mul(self.matrix[2][2])
                        .sub(self.matrix[2][0].mul(self.matrix[1][2])));
                let k = self.matrix[0][2].mul(
                    self.matrix[1][0].mul(self.matrix[2][1])
                        .sub(self.matrix[2][0].mul(self.matrix[1][1])));
                i - j + k
            };
        }
        let mut tmp = *self.clone();
        tmp.row_echelon_form(false);
        let mut vals = Vec::new();
        for i in 0..self.dimension.0 {
            for j in 0..self.dimension.0 - i {

            }
        }
    }

    pub fn is_linearly_independent(&self) -> bool {
        let mut tst = self.clone();
        tst.row_echelon_form(false);
        let max = cmp::min(self.dimension.0, self.dimension.1);
        for a in 0..max {
            for b in 0..a + 1 {
                if a != b && !tst.matrix[a][b].eq(&fracs::Frac::from(0)) {
                    return false;
                } else if a == b && !tst.matrix[a][b].eq(&fracs::Frac::from(1)) {
                    return false;
                }
            }
        }
        true
    }
}