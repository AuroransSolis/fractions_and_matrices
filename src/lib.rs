#![allow(unused_macros)]

#[macro_use] pub mod fractions;
pub mod matrices;

use fractions::*;
use matrices::*;

#[cfg(test)]
mod tests {
    extern crate rand;
    use rand::{Rng, thread_rng, Distribution, Range};

    use fractions::*;
    use matrix_base::*;
    //use mat_extras::*;

    #[test]
    fn make_nbyn_from_i32_vecs() {
        for i in 2..1000 {
            let mut matr = Vec::new();
            for j in 0..i {
                let mut row = Vec::new();
                for k in 0..i {
                    row.push(i * j + k);
                }
                matr.push(row);
            }
            assert!(Matrix::from_vecs(matr, false, NOTRY_NOPRINT)
                .is_ok())
        }
    }

    #[test]
    fn make_nbyn_from_frac_vecs() {
        for i in 2..1000 {
            let mut matr = Vec::new();
            for j in 0..i {
                let mut row = Vec::new();
                for k in 0..i {
                    let tmp = if j & 1 == 0 {
                        j + 1
                    } else {
                        (j + 1) * -1
                    };
                    row.push(fraction!(tmp, k + 1 ))
                }
                matr.push(row);
            }
            assert!(Matrix::from_vecs(matr, false, NOTRY_NOPRINT).is_ok());
        }
    }

    #[test]
    fn make_nbyn_from_i32_vec() {
        for n in 2..1000 {
            assert!(Matrix::from_i32_vec(n as usize, (0..n * n).collect::<Vec<i32>>(), false,
                                         NOTRY_NOPRINT).is_ok());
        }
    }

    /*#[test]
    fn matrix_addition() {
        let mut rng = rand::thread_rng();
        let range = Range::new(-100i32, 100i32);

    }*/
}

