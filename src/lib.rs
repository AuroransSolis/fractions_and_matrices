#![allow(unused_macros)]
#![allow(unused_imports)]
pub extern crate num;

#[macro_use] pub mod fractions;
#[macro_use] pub mod matrices;

#[cfg(test)]
mod tests {
    use fractions::base::Fraction;
    use matrices::base::{Matrix, AugmentedMatrix, Alignment::{ColumnAligned, RowAligned}};
    use matrices::extras::*;
    
    #[test]
    fn index_methods_test() {
        let mut foo: Matrix<i32> = matrix![
             0  1  2  3;
             4  5  6  7;
             8  9 10 11;
            12 13 14 15
        ];
        assert_eq!(foo[0], [0, 1, 2, 3]);
        assert_eq!(foo[0..2], [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(foo[0][0], 0);
        assert_eq!(foo[0][3], 3);
        assert_eq!(foo[3][3], 15);
        assert_eq!(foo[3][0], 12);
        assert_eq!(foo[(0, 0)], 0);
        assert_eq!(foo[(0, 3)], 3);
        assert_eq!(foo[(3, 3)], 15);
        assert_eq!(foo[(3, 0)], 12);
        foo.column_align();
        assert_eq!(foo[0], [0, 4, 8, 12]);
        assert_eq!(foo[0..2], [0, 4, 8, 12, 1, 5, 9, 13]);
        assert_eq!(foo[0][0], 0);
        assert_eq!(foo[0][3], 12);
        assert_eq!(foo[3][3], 15);
        assert_eq!(foo[3][0], 3);
        assert_eq!(foo[(0, 0)], 0);
        assert_eq!(foo[(0, 3)], 3);
        assert_eq!(foo[(3, 3)], 15);
        assert_eq!(foo[(3, 0)], 12);
    }

    #[test]
    fn partial_eq_test() {
        let foo = matrix![
            1 2 3;
            4 5 6;
            7 8 9
        ];
        let bar = matrix![
            1 2;
            3 4
        ];
        assert_ne!(foo, bar);
    }

    #[test]
    fn alignment_test() {
        let foo: Matrix<i32> = Matrix::new((1, 3), RowAligned);
        assert!(foo.is_row_aligned());
        let bar: Matrix<i32> = Matrix::new((1, 3), ColumnAligned);
        assert!(bar.is_column_aligned());
    }

    #[test]
    fn rows_test() {
        let foo: Matrix<i32> = Matrix::new((1, 3), RowAligned);
        assert_eq!(foo.num_rows(), 1);
        let bar: Matrix<i32> = Matrix::new((1, 3), ColumnAligned);
        assert_eq!(bar.num_rows(), 1);
    }

    #[test]
    fn columns_test() {
        let foo: Matrix<i32> = Matrix::new((1, 3), RowAligned);
        assert_eq!(foo.num_columns(), 3);
        let bar: Matrix<i32> = Matrix::new((1, 3), ColumnAligned);
        assert_eq!(bar.num_columns(), 3);
    }

    #[test]
    fn row_aligned_index_test() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let foo: Matrix<i32> = Matrix::new_from_vec((3, 3), vec.clone(), RowAligned).unwrap();
        assert_eq!(vec[2], foo[(0, 2)]);
        assert_eq!(vec[4], foo[(1, 1)]);
        assert_eq!(vec[6], foo[(2, 0)]);
    }

    #[test]
    fn column_aligned_index_test() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let tmp = vec![0, 3, 6, 1, 4, 7, 2, 5, 8];
        let foo: Matrix<i32> = Matrix::new_from_vec((3, 3), tmp, ColumnAligned).unwrap();
        assert_eq!(vec[2], foo[(0, 2)]);
        assert_eq!(vec[4], foo[(1, 1)]);
        assert_eq!(vec[6], foo[(2, 0)]);
    }

    #[test]
    fn matrix_macro_test() {
        let foo = matrix![
        1 2 3;
        4 5 6;
        7 8 9i32
        ];
        let bar: Matrix<i32> = Matrix::new_from_vec((3, 3), vec![1, 4, 7, 2, 5, 8, 3, 6, 9],
                                                    ColumnAligned).unwrap();
        assert_eq!(foo, bar);
    }

    #[test]
    fn column_align_eq_test() {
        let foo = matrix![
        1  2  3  4;
        5  6  7  8;
        9 10 11 12i32
        ];
        let mut bar = foo.clone();
        bar.column_align();
        assert_eq!(foo, bar);
    }

    #[test]
    fn row_align_eq_test() {
        let foo: Matrix<i32> = Matrix::new_from_vec((3, 4),
                                                    vec![1, 5, 9, 2, 6, 10, 3, 7, 11, 4, 8, 12],
                                                    ColumnAligned).unwrap();
        let mut bar = foo.clone();
        bar.row_align();
        assert_eq!(foo, bar);
    }

    #[test]
    fn there_and_back() {
        let foo: Matrix<i32> = Matrix::new_from_vec((4, 4),
                                                    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
                                                         13, 14, 15], RowAligned).unwrap();
        let mut bar = foo.clone();
        bar.in_place_transpose();
        assert_eq!(foo, bar);
        bar.in_place_transpose();
        assert_eq!(foo, bar);
    }

    #[test]
    fn push_row_ra() {
        let mut foo = matrix![
            1 2 3;
            4 5 6
        ];
        foo.push_row([7, 8, 9]);
        assert_eq!(foo, matrix![1 2 3; 4 5 6; 7 8 9]);
    }

    #[test]
    fn push_row_ca() {
        let mut foo = matrix![
            1 2 3;
            4 5 6
        ];
        foo.column_align();
        foo.push_row([7, 8, 9]);
        assert_eq!(foo, matrix![1 2 3; 4 5 6; 7 8 9]);
    }

    #[test]
    fn try_push_row_ra_ok() {
        let mut foo = matrix![
            1 2 3;
            4 5 6
        ];
        assert!(foo.try_push_row([7, 8, 9]).is_ok());
        assert_eq!(foo, matrix![1 2 3; 4 5 6; 7 8 9]);
    }

    #[test]
    fn try_push_row_ca_ok() {
        let mut foo = matrix![
            1 2 3;
            4 5 6
        ];
        foo.column_align();
        assert!(foo.try_push_row([7, 8, 9]).is_ok());
        assert_eq!(foo, matrix![1 2 3; 4 5 6; 7 8 9]);
    }

    #[test]
    fn try_push_row_ra_err() {
        let mut foo = matrix![
            1 2 3;
            4 5 6
        ];
        assert!(foo.try_push_row([7, 8]).is_err());
    }

    #[test]
    fn try_push_row_ca_err() {
        let mut foo = matrix![
            1 2 3;
            4 5 6
        ];
        foo.column_align();
        assert!(foo.try_push_row([7, 8]).is_err());
    }

    #[test]
    fn push_rows_ra() {
        let mut foo = matrix![1 2 3];
        foo.push_rows([4, 5, 6, 7, 8, 9]);
        assert_eq!(foo, matrix![1 2 3; 4 5 6; 7 8 9]);
    }

    #[test]
    fn push_rows_ca() {
        let mut foo = matrix![1 2 3];
        foo.column_align();
        foo.push_rows([4, 5, 6, 7, 8, 9]);
        assert_eq!(foo, matrix![1 2 3; 4 5 6; 7 8 9]);
    }
}
