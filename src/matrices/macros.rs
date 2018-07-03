use matrices::base::Alignment::{ColumnAligned, RowAligned};

#[macro_export]
macro_rules! matrix {
    ($($($val:expr) *);*) => {{
        let mut matr = Vec::new();
        let mut lens = Vec::new();
            $(
            let mut rc = Vec::new();
            $(
                rc.push($val);
            )*
            for l in &lens {
                if rc.len() != *l {
                    panic!("A row/column had an incorrect number of elements.");
                }
            }
            lens.push(rc.len());
            matr.append(&mut rc);
        )*
        Matrix {
            rows: matr.len() / lens[0],
            columns: lens[0],
            matrix: matr,
            alignment: RowAligned
        }
    }};
}

#[macro_export]
macro_rules! augmented_matrix {
    ([$($($val:expr) * => $sol_val:expr);* => $($sol_val:expr),*], $alignment:expr) => {
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        let mut solution_column = Vec::new();
        $(
            let mut rc = Vec::new();
            $(
                rc.push($val);
            )*
            for l in &lens {
                if rc.len() != *l {
                    panic!("A row/column had an incorrect number of elements.");
                }
            }
            lens.push(rc.len());
            matr.append(&mut rc);
        )*
        $(
            solution_column.push($sol_val);
        )*
        if solution_column.len() != matr.len() / lens[0] {
            panic!("Solution column had an incorrect number of elements.");
        } else {
            if $alignment == RowAligned {
                for r in (0..solution_column.len()).rev() {
                    matr.insert(r * lens[0], solution_column[r]);
                }
                Matrix {
                    rows: solution_column.len(),
                    columns: lens[0],
                    matrix: matr,
                    alignment: $alignment
                }
            } else {
                matr.append(solution_column);
                Matrix {
                    rows: lens[0],
                    columns: solution_column.len(),
                    matrix: matr,
                    alignment: $alignment
                }
            }
        }
    }
}