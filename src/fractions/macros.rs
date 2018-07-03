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

macro_rules! from_frac {
    ($($t:ty)*) => ($(
        impl From<Fraction> for $t {
            fn from(num: Fraction) -> Self {
                num.num as $t / num.den as $t
            }
        }
    )*)
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