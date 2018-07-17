use matrices::base::Alignment::{ColumnAligned, RowAligned};

#[macro_export]
macro_rules! matrix {
    ($($($val:expr) *);*) => {{
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        $(
            let rc = [$($val),*];
            for l in &lens {
                if rc.len() != *l {
                    panic!("A row/column had an incorrect number of elements.");
                }
            }
            lens.push(rc.len());
            matr.extend_from_slice(&rc);
        )*
        let mut res = Matrix::new((matr.len() / lens[0], lens[0]), RowAligned);
        res.set_matrix(matr);
        res
    }};
}

#[macro_export]
macro_rules! augmented_matrix {
    ($($($val:expr) * => $sol_val:expr);*) => {{
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        let mut solution_column = Vec::new();
        $(
            let rc = [$($val),*];
            solution_column.push($sol_val);
            for l in &lens {
                if rc.len() != *l {
                    panic!("A row/column had an incorrect number of elements.");
                }
            }
            lens.push(rc.len());
            matr.extend_from_slice(&rc);
        )*
        if solution_column.len() != matr.len() / lens[0] {
            panic!("Solution column had an incorrect number of elements.");
        } else {
            matr.push(solution_column[solution_column.len() - 1]);
            for r in (0..solution_column.len()).rev().skip(1) {
                matr.insert((r + 1) * lens[0], solution_column[r]);
            }
            let mut res = AugmentedMatrix::new((solution_column.len(), lens[0] + 1), RowAligned);
            res.set_matrix(matr);
            res
        }
    }};
}