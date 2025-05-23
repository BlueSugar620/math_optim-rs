pub mod combinatorics;

use algebra::{Abelian, Field, Group, One, Ring, Zero};

use std::{
    fmt::{Debug, Display},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GF<const MOD: u32> {
    value: u32,
}

impl<const MOD: u32> GF<MOD> {
    pub fn new(value: u32) -> Self {
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

    pub fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }
}

#[macro_export]
macro_rules! gf {
    ($value:expr) => {
        $crate::GF::from($value)
    };
    ($value:expr; mod $p:expr) => {
        $crate::GF::<$p>::from($value)
    };
}
macro_rules! gf_new_from_signed {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u32> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    if x < 0 {
                        - Self::new((MOD as i64 - x as i64) as u32)
                    } else {
                        Self::new(x as u32)
                    }
                }
            }
        )*
    };
}
gf_new_from_signed!(i8, i16, i32, i64, i128, isize);
macro_rules! gf_new_from_unsigned {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u32> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(x as u32)
                }
            }
        )*
    };
}
gf_new_from_unsigned!(u8, u16, u32, u64, u128, usize);
impl<const MOD: u32> Debug for GF<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<const MOD: u32> Display for GF<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: u32> Neg for GF<MOD> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = MOD - self.value;
        }
        self
    }
}
impl<const MOD: u32> Zero for GF<MOD> {
    fn zero() -> Self {
        GF::new(0)
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
}
impl<const MOD: u32> One for GF<MOD> {
    fn one() -> Self {
        GF::new(1)
    }
    fn is_one(&self) -> bool {
        self.value == 1
    }
}
impl<const MOD: u32> Group for GF<MOD> {}
impl<const MOD: u32> Abelian for GF<MOD> {}
impl<const MOD: u32> Ring for GF<MOD> {}
impl<const MOD: u32> Field for GF<MOD> {}

impl<const MOD: u32> AddAssign<GF<MOD>> for GF<MOD> {
    fn add_assign(&mut self, rhs: GF<MOD>) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}
impl<const MOD: u32> SubAssign<GF<MOD>> for GF<MOD> {
    fn sub_assign(&mut self, rhs: GF<MOD>) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}
impl<const MOD: u32> MulAssign<GF<MOD>> for GF<MOD> {
    fn mul_assign(&mut self, rhs: GF<MOD>) {
        self.value = ((self.value as u64 * rhs.value as u64) % MOD as u64) as u32;
    }
}
impl<const MOD: u32> DivAssign<GF<MOD>> for GF<MOD> {
    fn div_assign(&mut self, rhs: GF<MOD>) {
        self.value = ((self.value as u64 * rhs.inv().value as u64) % MOD as u64) as u32;
    }
}
macro_rules! gf_ops {
    ($(
            $trait:ident,
            $trait_assign:ident,
            $fn:ident,
            $fn_assign:ident,
    )*) => {$(
        impl<const MOD: u32> $trait_assign<&GF<MOD>> for GF<MOD> {
            fn $fn_assign(&mut self, rhs: &GF<MOD>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const MOD: u32, T: Into<GF<MOD>>> $trait<T> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const MOD: u32> $trait<&GF<MOD>> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const MOD: u32, T: Into<GF<MOD>>> $trait<T> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const MOD: u32> $trait<&GF<MOD>> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}
gf_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}

impl<const MOD: u32> Sum for GF<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, a| acc + a)
    }
}
impl<'a, const MOD: u32> Sum<&'a Self> for GF<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl<const MOD: u32> Product for GF<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, a| acc * a)
    }
}
impl<'a, const MOD: u32> Product<&'a Self> for GF<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
