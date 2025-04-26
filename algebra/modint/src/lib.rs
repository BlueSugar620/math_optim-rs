use algebra::{Abelian, One, Ring, Zero};

use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModInt<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> ModInt<MOD> {
    pub fn new(value: u64) -> Self {
        Self { value: value % MOD }
    }

    pub fn pow(&self, mut exp: u32) -> Self {
        let mut res = Self::new(1);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    }
}

#[macro_export]
macro_rules! modint {
    ($value:expr) => {
        $crate::ModInt::from($value)
    };
    ($value:expr; mod $p:expr) => {
        $crate::ModInt::<$p>::from($value)
    };
}
macro_rules! modint_new_from_signed {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u64> From<$t> for ModInt<MOD> {
                fn from(x: $t) -> Self {
                    if x < 0 {
                        - Self::new((MOD as i64 - x as i64) as u64)
                    } else {
                        Self::new(x as u64)
                    }
                }
            }
        )*
    };
}
modint_new_from_signed!(i8, i16, i32, i64, i128, isize);
macro_rules! modint_new_from_unsigned {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u64> From<$t> for ModInt<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(x as u64)
                }
            }
        )*
    };
}
modint_new_from_unsigned!(u8, u16, u32, u64, u128, usize);
impl<const MOD: u64> Debug for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<const MOD: u64> Display for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: u64> Neg for ModInt<MOD> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = MOD - self.value;
        }
        self
    }
}
impl<const MOD: u64> Zero for ModInt<MOD> {
    fn zero() -> Self {
        ModInt::new(0)
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
}
impl<const MOD: u64> One for ModInt<MOD> {
    fn one() -> Self {
        ModInt::new(1)
    }
    fn is_one(&self) -> bool {
        if MOD == 0 {
            self.value == 0
        } else {
            self.value == 1
        }
    }
}
impl<const MOD: u64> Abelian for ModInt<MOD> {}
impl<const MOD: u64> Ring for ModInt<MOD> {}

impl<const MOD: u64> AddAssign<ModInt<MOD>> for ModInt<MOD> {
    fn add_assign(&mut self, rhs: ModInt<MOD>) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}
impl<const MOD: u64> SubAssign<ModInt<MOD>> for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: ModInt<MOD>) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}
impl<const MOD: u64> MulAssign<ModInt<MOD>> for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: ModInt<MOD>) {
        self.value = (self.value as u128 * rhs.value as u128 % MOD as u128) as u64;
    }
}
macro_rules! modint_ops {
    ($(
            $trait:ident,
            $trait_assign:ident,
            $fn:ident,
            $fn_assign:ident,
    )*) => {$(
        impl<const MOD: u64> $trait_assign<&ModInt<MOD>> for ModInt<MOD> {
            fn $fn_assign(&mut self, rhs: &ModInt<MOD>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const MOD: u64, T: Into<ModInt<MOD>>> $trait<T> for ModInt<MOD> {
            type Output = ModInt<MOD>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const MOD: u64> $trait<&ModInt<MOD>> for ModInt<MOD> {
            type Output = ModInt<MOD>;
            fn $fn(self, rhs: &ModInt<MOD>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const MOD: u64, T: Into<ModInt<MOD>>> $trait<T> for &ModInt<MOD> {
            type Output = ModInt<MOD>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const MOD: u64> $trait<&ModInt<MOD>> for &ModInt<MOD> {
            type Output = ModInt<MOD>;
            fn $fn(self, rhs: &ModInt<MOD>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}
modint_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
}

impl<const MOD: u64> Sum for ModInt<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, a| acc + a)
    }
}
impl<'a, const MOD: u64> Sum<&'a Self> for ModInt<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl<const MOD: u64> Product for ModInt<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, a| acc * a)
    }
}
impl<'a, const MOD: u64> Product<&'a Self> for ModInt<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
