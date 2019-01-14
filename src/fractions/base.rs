#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign};
use std::fmt;

use num::{Zero, One};

#[derive(Clone, Copy, Debug)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
    pub(crate) ud: bool
}

impl Zero for Fraction {
    fn zero() -> Self {
        Fraction::new(0, 1)
    }

    fn is_zero(&self) -> bool {
        *self == Fraction::new(0, 1)
    }
}

impl One for Fraction {
    fn one() -> Self {
        Fraction::new(1, 1)
    }

    fn is_one(&self) -> bool {
        *self == Fraction::new(1, 1)
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

integer_into_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize}

into_frac_float!{f32 f64}

from_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64}

impl_arithmetic_with_frac!{u8 i8 u16 i16 u32 i32 u64 i64 usize isize f32 f64}

impl Fraction {
    /// Makes a new `Fraction`.
    /// # Examples
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let foo = Fraction::new(1, 2);
    /// let bar = Fraction::new(-3, 4);
    /// ```
    /// # Panics
    /// ```should_panic
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let baz = Fraction::new(1, 0);
    /// ```
    pub fn new(num: i64, den: i64) -> Self {
        if num != 0 && den == 0 {
            panic!("Tried to create an undefined fraction (n / 0).");
        }
        Fraction {
            num: num,
            den: den,
            ud: false
        }
    }

    /// Gives the inverse of a `Fraction`. Returns `self` if `self` is undefined.
    /// # Examples
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let foo = Fraction::new(2, 3);
    /// assert_eq!(foo.inverse(), Fraction::new(3, 2));
    /// ```
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let mut foo = Fraction::new(2, 3);
    /// foo /= Fraction::from(0);
    /// assert!(foo.is_ud());
    /// assert_eq!(foo, foo.inverse());
    /// ```
    pub fn inverse(mut self) -> Fraction {
        if self.ud {
            return self;
        }
        let temp = self.num;
        self.num = self.den;
        self.den = temp;
        self
    }

    /// Does the same as `.inverse()`, except this method assigns the value to `self`.
    /// # Example
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let mut foo = Fraction::new(1, 2);
    /// foo.inverse_assign();
    /// assert_eq!(foo, Fraction::from(2));
    /// ```
    pub fn inverse_assign(&mut self) {
        if self.ud {
            return;
        }
        let temp = self.num;
        self.num = self.den;
        self.den = temp;
    }

    /// Simplifies a `Fraction` and assigns the simplified value. Also forces forces the negative into
    /// the numerator, and determines and sets whether a `Fraction` is undefined.
    ///  # Examples
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let mut four_tenths = Fraction::new(4, 10);
    /// four_tenths.simplify();
    /// assert_eq!(four_tenths, Fraction::new(2, 5));
    /// ```
    ///
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let mut neg_fourteen_thirty_fifths = Fraction::new(14, -35);
    /// neg_fourteen_thirty_fifths.simplify();
    /// assert_eq!(neg_fourteen_thirty_fifths, Fraction::new(-2, 5));
    /// ```
    pub fn simplify(&mut self) {
        // Probably the most overused function in this module :^)
        // 0/0 is a valid non-undefined state - for instance 1/2 - 1/2 produces 0/0. Set it to 0/1,
        // keep it defined, and move on.
        if self.num == 0 && self.den == 0  && !self.ud {
            self.den = 1;
            return;
        }
        if self.ud {
            if self.num != 0 {
                self.num = 0;
            }
            if self.den != 0 {
                self.den = 0;
            }
            return;
        }
        if self.num != 0 && self.den == 0 {
            self.ud = true;
            self.simplify();
        }
        if self.num == 0 {
            if self.den != 1 {
                self.den = 1;
            }
            return;
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
    }

    /// Checks to see whether or not a `Fraction` is undefined.
    /// # Example
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let mut foo = Fraction::new(1, 2);
    /// foo /= Fraction::from(0); // produces 1/0
    /// assert!(foo.is_ud());
    /// ```
    pub fn is_ud(&self) -> bool {
        self.ud
    }

    /// Converts the `Fraction` into an `(i64, i64)` tuple.
    /// # Examples
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let foo = Fraction::new(3, 4);
    /// assert_eq!(foo.split(), (3, 4));
    /// ```
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let foo = Fraction::from(5);
    /// assert_eq!(foo.split(), (5, 1));
    /// ```
    pub fn split(self) -> (i64, i64) {
        (self.num, self.den)
    }

    /// Returns an `Option<Fraction>` just in case either of the two `Fraction`s are undefined,
    /// which is the only case that should return `None`, unless the result is somehow UD.
    /// # Examples
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let foo = Fraction::new(1, 2);
    /// let foo = foo.try_add(Fraction::new(1, 2));
    /// assert_eq!(foo, Some(Fraction::from(1)));
    /// ```
    /// ```rust
    /// # extern crate fractions_and_matrices;
    /// # use fractions_and_matrices::fractions::base::Fraction;
    /// let mut foo = Fraction::from(1);
    /// foo /= Fraction::from(0);
    /// assert!(foo.try_add(Fraction::from(1)).is_none());
    /// ```
    pub fn try_add(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        if self.den == other.den || (0 - self.den == other.den && other.num < 0) {
            let mut s = Fraction::new(self.num + other.num, self.den);
            s.simplify();
            if s.ud {
                return None;
            }
            Some(s)
        } else {
            let lcm = get_lcm(self.den, other.den) as i64;
            let self_mult = lcm / self.den;
            let other_mult = lcm / other.den;
            let mut s = Fraction::new(self.num * self_mult + other.num * other_mult, lcm);
            s.simplify();
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
        let mut s = Fraction::new(self.num + other.into() * self.den, self.den);
        s.simplify();
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
            let mut s = Fraction::new(self.num - other.num, self.den);
            s.simplify();
            if s.ud {
                return None;
            }
            Some(s)
        } else {
            let lcm = get_lcm(self.den, other.den) as i64;
            let self_mult = lcm / self.den;
            let other_mult = lcm / other.den;
            let mut s = Fraction::new(self.num * self_mult - other.num * other_mult, lcm);
            s.simplify();
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
        let mut s = Fraction::new(self.num - other.into() * self.den, self.den);
        s.simplify();
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_mul(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        let mut s = Fraction::new(self.num * other.num, self.den * other.den);
        s.simplify();
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_mul_t<T: Into<i64>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let mut s = Fraction::new(self.num * other.into(), self.den);
        s.simplify();
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_div(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        let mut s = Fraction::new(self.num * other.den, self.den * other.num);
        s.simplify();
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_div_t<T: Into<i64>>(self, other: T) -> Option<Fraction> {
        if self.ud {
            return None;
        }
        let mut s = Fraction::new(self.num, self.den * other.into());
        s.simplify();
        if s.ud {
            return None;
        }
        Some(s)
    }

    pub fn try_rem(self, other: Fraction) -> Option<Fraction> {
        if self.ud || other.ud {
            return None;
        }
        let mut s = self % other;
        s.simplify();
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
        let mut s = self % r;
        s.simplify();
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