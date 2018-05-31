use matrices::matrix_base::{AugmentedMatrix, Matrix};

#[macro_export]
macro_rules! matrix {
    ($([$([($a:expr), *]); *] E $alignment:expr)*, ) => ($(
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        ($(
            let mut rc = Vec::new();
            ($(
                rc.push($a);
            )*)
            lens.push(rc.len());
            matr.append(&mut rc);
        )*)
        for i in 0..lens.len() - 1 {
            for j in i + 1..lens.len() {
                if lens[i] != lens[j] {
                    panic!("Invalid matrix - rows do not all have the same length.");
                }
            }
        }
        Matrix {
            rows: lens.len(),
            columns: lens[0],
            matrix: matr,
            alignment: $alignment
        }
    )*);
    ($([$([($a:expr),* | $val:expr]); *] E $alignment:expr),* ) => (*(
        let mut matr = Vec::new();
        let mut lens = Vec::new();
        let mut
        ($(
            let mut rc = Vec::new();
            ($(
                rc.push($a);
            )*)
            lens.push(rc.len());
            rc.push($val);
            matr.append(&mut rc);
        )*)
        for i in 0..lens.len() - 1 {
            for j in i + 1..lens.len() {
                if lens[i] != lens[j] {
                    panic!("Invalid matrix - rows do not all have the same length.");
                }
            }
        }
        AugmentedMatrix {
            rows: lens.len(),
            columns: lens[0],
            matrix: matr,
            alignment: $alignment
        }
    )$);
}