use std::fmt::{Display, Debug, Formatter, Result};

use matrices::base::{AugmentedMatrix, Matrix, MatrixError};

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
        write!(f, "{}", format!("Dimension: ({}, {}), alignment: {:?}\n{}", self.num_rows(),
                                self.num_columns(), self.alignment, matr))
    }
}

impl<T: Debug> Debug for AugmentedMatrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns() + 1);
        for _ in 0..self.num_columns() + 1 {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() + 1 {
                if format!("{:?}", self[(a, b)]).len() > longest_in_column[b] {
                    longest_in_column[b] = format!("{:?}", self[(a, b)]).len();
                }
            }
        }
        for a in 0..self.num_rows() {
            let mut line = format!("Row {}: ", a); // String for each individual line
            // Add the appropriate character for the section of the bracket at the start of each line
            // Add spacing to line up the right side of the numbers in each column
            for b in 0..self.num_columns() + 1 {
                let mut spacer_left = String::from("");
                let elem_string = format!("{:?}", self[(a, b)]);
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.num_columns() {
                    line = format!("{}| {}{}", line, spacer_left, elem_string);
                } else if b == self.num_columns() - 1 {
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
        write!(f, "{}", format!("Dimension: ({}, {}), alignment: {:?}\n{}", self.num_rows(),
                                self.num_columns() + 1, self.alignment, matr))
    }
}

impl<T: Display> Display for AugmentedMatrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns() + 1);
        for _ in 0..self.num_columns() + 1 {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() + 1 {
                if self[(a, b)].to_string().len() > longest_in_column[b] {
                    longest_in_column[b] = self[(a, b)].to_string().len();
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
            for b in 0..self.num_columns() + 1 {
                let mut spacer_left = String::from("");
                let elem_string = self[(a, b)].to_string();
                for _ in 0..longest_in_column[b] - elem_string.len() {
                    spacer_left = format!("{}{}", spacer_left, " ");
                }
                if b == self.num_columns() {
                    line = format!("{}| {}{}", line, spacer_left, elem_string);
                } else if b == self.num_columns() - 1 {
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

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut matr = String::from(""); // Will contain string for entire matrix
        let mut longest_in_column: Vec<usize> = Vec::with_capacity(self.num_columns());
        for _ in 0..self.num_columns() {
            longest_in_column.push(0);
        }
        for a in 0..self.num_rows() {
            for b in 0..self.num_columns() {
                if self[(a, b)].to_string().len() > longest_in_column[b] {
                    longest_in_column[b] = self[(a, b)].to_string().len();
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
                let elem_string = self[(a, b)].to_string();
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