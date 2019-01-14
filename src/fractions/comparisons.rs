use std::cmp::{PartialEq, PartialOrd, Ordering};

use fractions::base::{Fraction, get_lcm};

impl Eq for Fraction {}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        if self.ud || other.ud {
            return false;
        }
        let lcm = get_lcm(self.den, other.den);
        let self_mul = lcm / self.den;
        let other_mul = lcm / other.den;
        self.num * self_mul == other.num * other_mul
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<Ordering> {
        if self.ud || other.ud {
            return None;
        }
        if self == other {
            return Some(Ordering::Equal);
        }
        let lcm = get_lcm(self.den, other.den);
        let self_mul = lcm / self.den;
        let other_mul = lcm / other.den;
        if self.num * self_mul < other.num * other_mul {
            Some(Ordering::Less)
        } else if self.num * self_mul > other.num * other_mul {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}