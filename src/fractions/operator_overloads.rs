use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg};

use fractions::base::{Fraction, get_lcm};

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
            s.simplify();
            s
        } else {
            let mut s = self.clone();
            let lcm = get_lcm(s.den, r.den) as i64;
            let self_mult = lcm / s.den;
            let r_mult = lcm / r.den;
            s.num *= self_mult;
            s.num += r.num * r_mult;
            s.den = lcm;
            s.simplify();
            s
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
            s.simplify();
            s
        } else {
            let mut s = self.clone();
            let lcm = get_lcm(s.den, r.den) as i64;
            let self_mult = lcm / s.den;
            let r_mult = lcm / r.den;
            s.num *= self_mult;
            s.num += r.num * r_mult;
            s.den = lcm;
            s.simplify();
            s
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