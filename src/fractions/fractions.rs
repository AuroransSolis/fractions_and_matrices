#![allow(dead_code)]

use std::fmt;

#[macro_use] use fractions::fraction_macros::*;
use fractions::fraction_operator_overloads;
use fractions::fraction_comparisons;

#[derive(Clone, Copy, PartialOrd, Debug)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
    pub(crate) ud: bool
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.ud {
            return write!(f, "UD");
        } else if self.den == 1 {
            return write!(f, "{}", self.num);
        }
        write!(f, "{} / {}", self.num, self.den)
    }
}

integer_into_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize}

into_frac_float!{f32 f64}

from_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64}

impl_arithmetic_with_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        if num != 0 && den == 0 {
            panic!("Tried to create an undefined fraction (n / 0).");
        }
        Fraction {
            num: num,
            den: den,
            ud: false
        }.simplify()
    }

    pub fn inverse(mut self) -> Fraction {
        if self.ud {
            return self;
        }
        let temp = self.num;
        self.num = self.den;
        self.den = temp;
        self.simplify()
    }

    // Probably the most overused function in this module :^)
    pub fn simplify(mut self) -> Fraction {
        // 0/0 is a valid non-undefined state - for instance 1/2 - 1/2 produces 0/0. Set it to 0/1,
        // keep it defined, and move on.
        if self.num == 0 && self.den == 0  && !self.ud {
            self.den = 1;
            return self;
        }
        if self.ud {
            if self.num != 0 {
                self.num = 0;
            }
            if self.den != 0 {
                self.den = 0;
            }
            return self;
        }
        if self.num != 0 && self.den == 0 {
            self.ud = true;
            return self.simplify();
        }
        if self.num == 0 {
            if self.den != 1 {
                self.den = 1;
            }
            return self;
        }
        if self.den < 0 && self.num >= 0 { // Keep the negative in the numerator
            self.den *= -1;
            self.num *= -1;
        }
        if self.num < 0 && self.den < 0 { // Simplify to positives
            self.num *= -1;
            self.den *= -1;
        }
        if self.num % self.den == 0 {
            self.num /= self.den;
            self.den = 1;
        }
        // Test if the numerator and denominator are coprime; if not, divide by gcd
        let a = match self.num < 0 {
            true => (0 - self.num) as u64,
            false => self.num as u64
        };
        let b = match self.den < 0 {
            true => (0 - self.den) as u64,
            false => self.den as u64
        };
        let test_gcd = get_gcd(a, b) as i64;
        if test_gcd > 1 {
            self.num /= test_gcd;
            self.den /= test_gcd;
        }
        self
    }
    
    pub fn split(self) -> (i64, i64) {
        (self.num, self.den)
    }

    pub fn try_add(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        if self.den == other.den || (0 - self.den == other.den && other.num < 0) {
            let s = Fraction::new(self.num + other.num, self.den);
            if s.ud {
                return None;
            }
            Some(s)
        } else {
            let lcm = get_lcm(self.den, other.den) as i64;
            let self_mult = lcm / self.den;
            let other_mult = lcm / other.den;
            let s = Fraction::new(self.num * self_mult + other.num * other_mult, lcm);
            if s.ud {
                return None;
            }
            Some(s)
        }
    }

    pub fn try_add_t<T: Into<i64>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let s = Fraction::new(self.num + other.into() * self.den, self.den);
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_sub(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        if self.den == other.den || (0 - self.den == other.den && other.num < 0) {
            let s = Fraction::new(self.num - other.num, self.den);
            if s.ud {
                return None;
            }
            Some(s)
        } else {
            let lcm = get_lcm(self.den, other.den) as i64;
            let self_mult = lcm / self.den;
            let other_mult = lcm / other.den;
            let s = Fraction::new(self.num * self_mult - other.num * other_mult, lcm);
            if s.ud {
                return None;
            }
            Some(s)
        }
    }

    pub fn try_sub_t<T: Into<i64>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let s = Fraction::new(self.num - other.into() * self.den, self.den);
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_mul(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        let s = Fraction::new(self.num * other.num, self.den * other.den);
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_mul_t<T: Into<i64>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let s = Fraction::new(self.num * other.into(), self.den);
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_div(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        let s = Fraction::new(self.num * other.den, self.den * other.num);
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_div_t<T: Into<i64>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let s = Fraction::new(self.num, self.den * other.into());
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_rem(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        let s = self % other;
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_rem_t<T: Into<Fraction>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let r = other.into();
        if r.ud {
            return None;
        }
        let s = self % r;
        if s.ud {
            return None;
        }
        Some(s)
    }
}

// Not using Euclid's Algorithm anymore because it's really slow >:v
pub fn get_gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if b == 0 {
            return a;
        } else {
            let c = b;
            b = a % b;
            a = c;
        }
    }
}

// Neat trick here: lcm = a * b / gcd
pub fn get_lcm(a: i64, b: i64) -> i64 {
    let ayy = match a < 0 {
        true => (0 - a) as u64,
        false => a as u64
    };
    let bee = match b < 0 {
        true => (0 - b) as u64,
        false => b as u64
    };
    let gcd = get_gcd(ayy, bee);
    (ayy * bee / gcd) as i64
}