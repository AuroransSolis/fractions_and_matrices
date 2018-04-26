use std::cmp::Ordering;

use fractions::fractions::{Fraction, get_lcm};

impl Eq for Fraction {}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        let lcm = get_lcm(self.den, other.den) as i64;
        let self_mult = lcm / self.den;
        let other_mult = lcm / other.den;
        let s_num = self.num * self_mult;
        let o_num = other.num * other_mult;
        s_num == o_num
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.den == other.den {
            return self.num.cmp(&other.num);
        }
        // Compare numerators for equal denominators
        let lcm = get_lcm(self.den, other.den);
        let self_lcm = self.num * lcm / self.den;
        let other_lcm = other.num * lcm / other.den;
        self_lcm.cmp(&other_lcm)
    }
}