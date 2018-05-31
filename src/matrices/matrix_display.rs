use matrices::matrix_base::{AugmentedMatrix, Matrix, MatrixError};

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns());
        for _ in 0..self.num_columns() {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() {
                if format!("{:?}", self[(a, b)]).len() > longest_in_column[b] {
                    longest_in_column[b] = format!("{:?}", self[(a, b)]).len();
                }
            }
        }
        for a in 0..self.num_rows() {
            let mut line = format!("Row {}: ", a); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.num_columns() {
                let mut spacer_left = String::from("");
                let elem_string = format!("{:?}", self[(a, b)]);
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.num_columns() - 1 {
                    line = format!("{}{}{} ", line, spacer_left, elem_string);
                } else {
                    line = format!("{}{}{}, ", line, spacer_left, elem_string);
                }
            }
            // Add line to matrix string, add newline if it's not the last line
            if a < self.num_rows() {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", format!("Dimension: ({}, {})\n{}", self.num_rows(),
                                self.num_columns(), matr))
    }
}

impl<T: fmt::Debug> fmt::Debug for AugmentedMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns());
        for _ in 0..self.num_columns() {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() {
                if format!("{:?}", self[(a, b)]).len() > longest_in_column[b] {
                    longest_in_column[b] = format!("{:?}", self[(a, b)]).len();
                }
            }
        }
        for a in 0..self.num_rows() {
            let mut line = format!("Row {}: ", a); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.num_columns() {
                let mut spacer_left = String::from("");
                let elem_string = format!("{:?}", self[(a, b)]);
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.num_columns() - 1 && self.augmented {
                    line = format!("{}| {}{}", line, spacer_left, elem_string);
                } else if b == self.num_columns() - 2 {
                    line = format!("{}{}{} ", line, spacer_left, elem_string);
                } else {
                    line = format!("{}{}{}, ", line, spacer_left, elem_string);
                }
            }
            // Add line to matrix string, add newline if it's not the last line
            if a < self.num_rows() {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", format!("Dimension: ({}, {})\n{}", self.num_rows(),
                                self.num_columns(), matr))
    }
}

impl<T: fmt::Display> fmt::Display for AugmentedMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns());
        for _ in 0..self.num_columns() {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() {
                if self.get_element_ref(a, b).to_string().len() > longest_in_column[b] {
                    longest_in_column[b] = self.get_element_ref(a, b).to_string().len();
                }
            }
        }
        for a in 0..self.num_rows() {
            let mut line = String::from(""); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            if a == 0 {
                line = format!("⎡ {}", line);
            } else if a == self.num_rows() - 1 {
                line = format!("⎣ {}", line);
            } else {
                line = format!("⎢ {}", line);
            }
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.num_columns() {
                let mut spacer_left = String::from("");
                let elem_string = self.get_element_ref(a, b).to_string();
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.num_columns() - 1 && self.augmented {
                    line = format!("{}| {}{}", line, spacer_left, elem_string);
                } else if b == self.num_columns() - 2 {
                    line = format!("{}{}{} ", line, spacer_left, elem_string);
                } else {
                    line = format!("{}{}{}  ", line, spacer_left, elem_string);
                }
            }
            // Append appropriate end symbol for bracket section at the end of each line
            if a == 0 {
                line = format!("{} ⎤", line);
            } else if a == self.num_rows() - 1 {
                line = format!("{} ⎦", line);
            } else {
                line = format!("{} ⎥", line);
            }
            // Add line to matrix string, add newline if it's not the last line
            if a == self.num_rows() - 1 {
                matr = format!("{}{}", matr, line);
            } else {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", matr)
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns());
        for _ in 0..self.num_columns() {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() {
                if self.get_element_ref(a, b).to_string().len() > longest_in_column[b] {
                    longest_in_column[b] = self.get_element_ref(a, b).to_string().len();
                }
            }
        }
        for a in 0..self.num_rows() {
            let mut line = String::from(""); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            if a == 0 {
                line = format!("⎡ {}", line);
            } else if a == self.num_rows() - 1 {
                line = format!("⎣ {}", line);
            } else {
                line = format!("⎢ {}", line);
            }
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.num_columns() {
                let mut spacer_left = String::from("");
                let elem_string = self.get_element_ref(a, b).to_string();
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.num_columns() - 1 {
                    line = format!("{}{}{} ", line, spacer_left, elem_string);
                } else {
                    line = format!("{}{}{}  ", line, spacer_left, elem_string);
                }
            }
            // Append appropriate end symbol for bracket section at the end of each line
            if a == 0 {
                line = format!("{} ⎤", line);
            } else if a == self.num_rows() - 1 {
                line = format!("{} ⎦", line);
            } else {
                line = format!("{} ⎥", line);
            }
            // Add line to matrix string, add newline if it's not the last line
            if a == self.num_rows() - 1 {
                matr = format!("{}{}", matr, line);
            } else {
                matr = format!("{}{}\n", matr, line);
            }
        }
        write!(f, "{}", matr)
    }
}

impl<T> Matrix<T> {
    pub fn print_augmented_solution(&self, variable_names: &Vec<&str>) -> Result<(), MatrixError> {
        if !self.augmented {
            return Err(MatrixError::FunctionError(
                "Attempted to print solution for a non-augmented matrix as though it were."
                    .to_string()
            ));
        }
        if !self.check_ref() {
            return Err(MatrixError::FunctionError("Matrix is not in REF form.".to_string()));
        }
        if !self.check_rref() {
            println!("Matrix was not in RREF form. Attempting to solve RREF.");
            let mut tmp = self.clone();
            tmp.reduced_row_echelon_form(false);
            if tmp.check_rref() {
                return Err(MatrixError::FunctionError("Matrix unsolveable for RREF.".to_string()));
            }
            return tmp.print_augmented_solution(variable_names);
        }
        let mut res = format!("{} = {}", variable_names[0], self.matrix[0][self.dimension.1 - 1]);
        for a in 1..variable_names.len() {
            res = format!("{}\n{} = {}",
                          res, variable_names[a], self.matrix[a][self.dimension.1 - 1]);
        }
        println!("{}", res);
        Ok(())
    }
}