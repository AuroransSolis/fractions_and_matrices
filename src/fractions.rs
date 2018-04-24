#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialOrd)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
    pub(crate) ud: bool
}

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

macro_rules! integer_into_frac {
    ($($t:ty)*) => ($(
        impl From<$t> for Fraction {
            fn from(num: $t) -> Self {
                Fraction {
                    num: num as i64,
                    den: 1,
                    ud: false
                }.simplify()
            }
        }
    )*)
}

integer_into_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize}

macro_rules! into_frac_float {
    ($($t:ty)*) => ($(
        impl From<$t> for Fraction {
            fn from(num: $t) -> Self {
                if num % 1.0 == 0.0 {
                    return Fraction::new(num as i64, 1);
                }
                let num_string = num.to_string();
                let p10: i64 = num_string.len() as i64 - 2;
                let mut decimal_ind: usize = 0;
                for (i, c) in num_string.chars().enumerate() {
                    if c == '.' {
                        decimal_ind = i + 1;
                        break;
                    }
                }
                let string = {
                    let mut num_string_tmp = num_string.into_bytes();
                    let (mut start, mut end) = num_string_tmp.split_at_mut(decimal_ind);
                    let mut start = start.to_vec();
                    start.pop();
                    start.extend_from_slice(end);
                    start
                };
                let final_num = string.iter().enumerate()
                    .map(|(i, &b)| ((b - '0' as u8) as i64) * 10i64.pow((p10 - i as i64) as u32))
                    .sum::<i64>();
                Fraction::new(final_num, 10i64.pow(p10 as u32))
            }
        }
    )*)
}

into_frac_float!{f32 f64}

macro_rules! from_frac {
    ($($t:ty)*) => ($(
        impl From<Fraction> for $t {
            fn from(num: Fraction) -> Self {
                num.num as $t / num.den as $t
            }
        }
    )*)
}

from_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64}

#[macro_export]
macro_rules! fraction {
    ($($num:ident), *) => {{
        Fraction::from($num)
    }};

    ($($num:expr), *) => {{
        Fraction::from($num)
    }};

    ($($num:ident, $den:ident)*) => {{
        Fraction {
            num: $num as i64,
            den: $den as i64,
            ud: false
        }
    }};

    ($($num:ident, $den:expr)*) => {{
        Fraction {
            num: $num as i64,
            den: $den as i64,
            ud: false
        }
    }};

    ($($num:expr, $den:ident)*) => {{
        Fraction {
            num: $num as i64,
            den: $den as i64,
            ud: false
        }
    }};

    ($($num:expr, $den:expr)*) => {{
        Fraction {
            num: $num as i64,
            den: $den as i64,
            ud: false
        }
    }};
}

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

// Implement T + Fraction
macro_rules! impl_add_with_frac {
    ($($target_type:ty)* ) => ($(
        impl Add<Fraction> for $target_type {
            type Output = $target_type;

            fn add(self, rhs: Fraction) -> $target_type {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return self;
                }
                Self::from(Fraction::from(self) + rhs)
            }
        }
    )*);
}

// Implement T += Fraction
macro_rules! impl_addassign_with_frac {
    ($($target_type:ty)* ) => ($(
        impl AddAssign<Fraction> for $target_type {
            fn add_assign(&mut self, rhs: Fraction) {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return;
                }
                *self = Self::from(Fraction::from(*self) + rhs);
            }
        }
    )*);
}

// Implement T - Fraction
macro_rules! impl_sub_with_frac {
    ($($target_type:ty)* ) => ($(
        impl Sub<Fraction> for $target_type {
            type Output = $target_type;

            fn sub(self, rhs: Fraction) -> $target_type {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return self;
                }
                Self::from(Fraction::from(self) - rhs)
            }
        }
    )*);
}

// Implement T -= Fraction
macro_rules! impl_subassign_with_frac {
    ($($target_type:ty)* ) => ($(
        impl SubAssign<Fraction> for $target_type {
            fn sub_assign(&mut self, rhs: Fraction) {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return;
                }
                *self = Self::from(Fraction::from(*self) - rhs);
            }
        }
    )*);
}

// Implement T * Fraction
macro_rules! impl_mul_with_frac {
    ($($target_type:ty)* ) => ($(
        impl Mul<Fraction> for $target_type {
            type Output = $target_type;

            fn mul(self, rhs: Fraction) -> $target_type {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return self;
                }
                Self::from(Fraction::from(self) * rhs)
            }
        }
    )*);
}

// Implement T *= Fraction
macro_rules! impl_mulassign_with_frac {
    ($($target_type:ty)* ) => ($(
        impl MulAssign<Fraction> for $target_type {
            fn mul_assign(&mut self, rhs: Fraction) {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return;
                }
                *self = Self::from(Fraction::from(*self) * rhs);
            }
        }
    )*);
}

// Implement T / Fraction
macro_rules! impl_div_with_frac {
    ($($target_type:ty)* ) => ($(
        impl Div<Fraction> for $target_type {
            type Output = $target_type;

            fn div(self, rhs: Fraction) -> $target_type {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return self;
                }
                Self::from(Fraction::from(self) / rhs)
            }
        }
    )*);
}

// Implement T /= Fraction
macro_rules! impl_divassign_with_frac {
    ($($target_type:ty)* ) => ($(
        impl DivAssign<Fraction> for $target_type {
            fn div_assign(&mut self, rhs: Fraction) {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return;
                }
                *self = Self::from(Fraction::from(*self) / rhs);
            }
        }
    )*);
}

// Implement T % Fraction
macro_rules! impl_rem_with_frac {
    ($($target_type:ty)* ) => ($(
        impl Rem<Fraction> for $target_type {
            type Output = $target_type;

            fn rem(self, rhs: Fraction) -> $target_type {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return self;
                }
                Self::from(Fraction::from(self) % rhs)
            }
        }
    )*);
}

// Implement T %= Fraction
macro_rules! impl_remassign_with_frac {
    ($($target_type:ty)* ) => ($(
        impl RemAssign<Fraction> for $target_type {
            fn rem_assign(&mut self, rhs: Fraction) {
                debug_assert!(rhs.ud);
                if rhs.ud {
                    return;
                }
                *self = Self::from(Fraction::from(*self) % rhs);
            }
        }
    )*);
}

// Implement all non-assign operator overloads from above
macro_rules! impl_nonassign_arithmetic_with_frac {
    ($($target_type:ty)* ) => ($(
        impl_add_with_frac!{$target_type}
        impl_sub_with_frac!{$target_type}
        impl_mul_with_frac!{$target_type}
        impl_div_with_frac!{$target_type}
        impl_rem_with_frac!{$target_type}
    )*);
}

// Implement all assign operator overloads from above
macro_rules! impl_assign_arithmetic_with_frac {
    ($($target_type:ty)* ) => ($(
        impl_addassign_with_frac!{$target_type}
        impl_subassign_with_frac!{$target_type}
        impl_mulassign_with_frac!{$target_type}
        impl_divassign_with_frac!{$target_type}
        impl_remassign_with_frac!{$target_type}
    )*);
}

// Implement all operator overloads from above
macro_rules! impl_arithmetic_with_frac {
    ($($target_type:ty)* ) => ($(
        impl_nonassign_arithmetic_with_frac!{$target_type}
        impl_assign_arithmetic_with_frac!{$target_type}
    )*);
}

impl_arithmetic_with_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64}

// Start of operator overloads for Fraction
impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        if self.ud {
            return self;
        }
        let mut s = self.clone();
        s.num *= -1;
        s
    }
}

impl<T: Into<Fraction> + From<Fraction>> Add<T> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: T) -> Fraction {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return self;
        }
        if self.den == r.den || (0 - self.den == r.den && r.num < 0) {
            let mut s = self.clone();
            s.num += r.num;
            s.simplify()
        } else {
            let mut s = self.clone();
            let lcm = get_lcm(s.den, r.den) as i64;
            let self_mult = lcm / s.den;
            let r_mult = lcm / r.den;
            s.num *= self_mult;
            s.num += r.num * r_mult;
            s.den = lcm;
            s.simplify()
        }
    }
}

impl<T: Into<Fraction> + From<Fraction>> AddAssign<T> for Fraction {
    fn add_assign(&mut self, rhs: T) {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return;
        }
        if self.den == r.den || (0 - self.den == r.den && r.num < 0) {
            self.num += r.num;
            self.simplify();
        } else {
            let lcm = get_lcm(self.den, r.den) as i64;
            let self_mult = lcm / self.den;
            let r_mult = lcm / r.den;
            self.num *= self_mult;
            self.num += r.num * r_mult;
            self.den = lcm;
            self.simplify();
        }
    }
}

impl<T: Into<Fraction> + From<Fraction>> Sub<T> for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: T) -> Fraction {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return self;
        }
        if self.den == r.den || (0 - self.den == r.den && r.num < 0) {
            let mut s = self.clone();
            s.num += r.num;
            s.simplify()
        } else {
            let mut s = self.clone();
            let lcm = get_lcm(s.den, r.den) as i64;
            let self_mult = lcm / s.den;
            let r_mult = lcm / r.den;
            s.num *= self_mult;
            s.num += r.num * r_mult;
            s.den = lcm;
            s.simplify()
        }
    }
}

impl<T: Into<Fraction> + From<Fraction>> SubAssign<T> for Fraction {
    fn sub_assign(&mut self, rhs: T) {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return;
        }
        if self.den == r.den || (0 - self.den == r.den && r.num < 0) {
            self.num -= r.num;
            self.simplify();
        } else {
            let lcm = get_lcm(self.den, r.den) as i64;
            let self_mult = lcm / self.den;
            let r_mult = lcm / r.den;
            self.num *= self_mult;
            self.num -= r.num * r_mult;
            self.den = lcm;
            self.simplify();
        }
    }
}

impl<T: Into<Fraction> + From<Fraction>> Mul<T> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: T) -> Fraction {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return self;
        }
        Fraction::new(self.num * r.num, self.den * r.den)
    }
}

impl<T: Into<Fraction> + From<Fraction>> MulAssign<T> for Fraction {
    fn mul_assign(&mut self, rhs: T) {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return;
        }
        self.num *= r.num;
        self.den *= r.den;
        self.simplify();
    }
}

impl<T: Into<Fraction> + From<Fraction>> Div<T> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: T) -> Fraction {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return self;
        }
        Fraction::new(self.num * r.den, self.den * r.num)
    }
}

impl<T: Into<Fraction> + From<Fraction>> DivAssign<T> for Fraction {
    fn div_assign(&mut self, rhs: T) {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return;
        }
        self.num *= r.den;
        self.den *= r.num;
        self.simplify();
    }
}

impl<T: Into<Fraction> + From<Fraction>> Rem<T> for Fraction {
    type Output = Fraction;

    fn rem(self, rhs: T) -> Self::Output {
        debug_assert!(!self.ud);
        let r = rhs.into(); 
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return self;
        }
        if self.den == r.den || (0 - self.den == r.den && r.num < 0) {
            Fraction::new(self.num % r.den, self.den)
        } else {
            let lcm = get_lcm(self.den, r.den) as i64;
            let self_mult = lcm / self.den;
            let r_mult = lcm / r.den;
            Fraction::new((self.num * self_mult) % (r.num * r_mult), lcm)
        }
    }
}

impl<T: Into<Fraction> + From<Fraction>> RemAssign<T> for Fraction {
    fn rem_assign(&mut self, rhs: T) {
        debug_assert!(!self.ud);
        let r = rhs.into();
        debug_assert!(!r.ud);
        if self.ud || r.ud {
            return;
        }
        if self.den == r.den || (0 - self.den == r.den && r.num < 0) {
            self.num %= r.num;
            self.simplify();
        } else {
            let lcm = get_lcm(self.den, r.den) as i64;
            let self_mult = lcm / self.den;
            let r_mult = lcm / r.den;
            self.num *= self_mult;
            self.den = lcm;
            self.num %= r * r_mult;
            self.simplify();
        }
    }
}