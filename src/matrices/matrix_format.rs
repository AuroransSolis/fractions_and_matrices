use std::fmt;
use matrices::matrix_base::{AugmentedMatrix, Matrix};

pub enum Separator {
    Plus,
    Minus,
    Times,
    Divide,
    Space
}

impl fmt::Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Separator::Plus => write!(f, "+"),
            &Separator::Minus => write!(f, "-"),
            &Separator::Times => write!(f, "*"),
            &Separator::Divide => write!(f, "/"),
            &Separator::Space => write!(f, " ")
        }
    }
}

macro_rules! add_mat_to_string {
        {{$matrix_string:expr, $matr:ident, $separator:expr}} => {{
        {
            let mut lines_vec = $matrix_string.lines().map(|line| String::from(line))
                .collect::<Vec<String>>();
            let mut mat_vec = $matr.to_string().lines().map(|line| String::from(line))
                .collect::<Vec<String>>();
            let mut top_gap = 0;
            let height_comp_state;
            if lines_vec.len() > mat_vec.len() {
                height_comp_state = -1;
            } else if lines_vec.len() == mat_vec.len() {
                height_comp_state = 0;
            } else {
                height_comp_state = 1;
            }
            if height_comp_state == 0 {
                for i in 0..lines_vec.len() {
                    if i == lines_vec.len() / 2 {
                        lines_vec[i] = format!("{} {} {}", lines_vec[i], $separator, mat_vec[i]);
                    } else {
                        lines_vec[i] = format!("{}   {}", lines_vec[i], mat_vec[i]);
                    }
                }
                let mut ret = String::from("");
                for i in 0..lines_vec.len() - 1 {
                    ret = format!("{}{}\n", ret, lines_vec[i]);
                }
                ret = format!("{}{}", ret, lines_vec[lines_vec.len() - 1]);
                return ret;
            }
            let ws_max;
            let max;
            if height_comp_state == -1 {
                ws_max = lines_vec.len() - mat_vec.len();
                max = lines_vec.len();
            } else {
                ws_max = mat_vec.len() - lines_vec.len();
                max = mat_vec.len();
            }
            for _a in (0..ws_max).filter(|&a| a & 1 == 0) {
                top_gap += 1;
            }
            let new_lines: Vec<String>;
            match height_comp_state > 0 {
                true => {
                    for i in top_gap..max {
                        if i == mat_vec.len() / 2 {
                            mat_vec[i] = format!("{} {} {}", mat_vec[i], $separator, lines_vec[i - top_gap]);
                        } else {
                            mat_vec[i] = format!("{}   {}", mat_vec[i], lines_vec[i - top_gap]);
                        }
                    }
                    new_lines = mat_vec;
                },
                false => {
                    for i in top_gap..max {
                        if i == lines_vec.len() / 2 {
                            lines_vec[i] = format!("{} {} {}", lines_vec[i], $separator, mat_vec[i - top_gap]);
                        } else {
                            lines_vec[i] = format!("{}   {}", lines_vec[i], mat_vec[i - top_gap]);
                        }
                    }
                    new_lines = lines_vec;
                }
            }
            let mut ret = String::from("");
            for i in 0..new_lines.len() - 1 {
                ret = format!("{}{}\n", ret, new_lines[i]);
            }
            ret = format!("{}{}", ret, new_lines[new_lines.len() - 1]);
            ret
        }
    }}
}