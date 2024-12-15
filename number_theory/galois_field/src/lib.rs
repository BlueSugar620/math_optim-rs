pub mod galois_field {
    use std::{
        iter::{Product, Sum},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct GaloisField<const MOD: u64> {
        value: u64,
    }

    impl<const MOD: u64> GaloisField<MOD> {
        pub fn new(value: u64) -> Self {
            Self { value: value % MOD }
        }

        pub fn value(&self) -> u64 {
            self.value
        }

        pub fn pow(&self, mut exp: u64) -> Self {
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

    macro_rules! impl_from_signed {
        ($($t:ty),*) => {
            $(
                impl<const MOD: u64> From<$t> for GaloisField<MOD> {
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
    macro_rules! impl_from_unsigned {
        ($($t:ty),*) => {
            $(
                impl<const MOD: u64> From<$t> for GaloisField<MOD> {
                    fn from(x: $t) -> Self {
                        Self::new(x as u64)
                    }
                }
            )*
        };
    }
    impl_from_signed!(i8, i16, i32, i64, i128, isize);
    impl_from_unsigned!(u8, u16, u32, u64, u128, usize);
    #[macro_export]
    macro_rules! gf {
        ($value:expr) => {
            $crate::GaloisField::from($value)
        };
        ($value:expr; mod $p:expr) => {
            $crate::GaloisField::<$p>::from($value)
        };
    }

    impl<const MOD: u64> AddAssign<GaloisField<MOD>> for GaloisField<MOD> {
        fn add_assign(&mut self, rhs: GaloisField<MOD>) {
            self.value += rhs.value;
            if self.value >= MOD {
                self.value -= MOD;
            }
        }
    }
    impl<const MOD: u64> SubAssign<GaloisField<MOD>> for GaloisField<MOD> {
        fn sub_assign(&mut self, rhs: GaloisField<MOD>) {
            if self.value < rhs.value {
                self.value += MOD;
            }
            self.value -= rhs.value;
        }
    }
    impl<const MOD: u64> MulAssign<GaloisField<MOD>> for GaloisField<MOD> {
        fn mul_assign(&mut self, rhs: GaloisField<MOD>) {
            self.value *= rhs.value;
            self.value %= MOD;
        }
    }
    impl<const MOD: u64> DivAssign<GaloisField<MOD>> for GaloisField<MOD> {
        fn div_assign(&mut self, rhs: GaloisField<MOD>) {
            self.value *= rhs.inv().value;
            self.value %= MOD;
        }
    }
    macro_rules! gf_forward_ops {
        ($(
                $trait:ident,
                $trait_assign:ident,
                $fn:ident,
                $fn_assign:ident,
        )*) => {$(
            impl<const MOD: u64> $trait_assign<&GaloisField<MOD>> for GaloisField<MOD> {
                fn $fn_assign(&mut self, rhs: &GaloisField<MOD>) {
                    self.$fn_assign(*rhs);
                }
            }
            impl<const MOD: u64, T: Into<GaloisField<MOD>>> $trait<T> for GaloisField<MOD> {
                type Output = GaloisField<MOD>;
                fn $fn(mut self, rhs: T) -> Self::Output {
                    self.$fn_assign(rhs.into());
                    self
                }
            }
            impl<const MOD: u64> $trait<&GaloisField<MOD>> for GaloisField<MOD> {
                type Output = GaloisField<MOD>;
                fn $fn(self, rhs: &GaloisField<MOD>) -> Self::Output {
                    self.$fn(*rhs)
                }
            }
            impl<const MOD: u64, T: Into<GaloisField<MOD>>> $trait<T> for &GaloisField<MOD> {
                type Output = GaloisField<MOD>;
                fn $fn(self, rhs: T) -> Self::Output {
                    (*self).$fn(rhs.into())
                }
            }
            impl<const MOD: u64> $trait<&GaloisField<MOD>> for &GaloisField<MOD> {
                type Output = GaloisField<MOD>;
                fn $fn(self, rhs: &GaloisField<MOD>) -> Self::Output {
                    (*self).$fn(*rhs)
                }
            }
        )*};
    }
    gf_forward_ops! {
        Add, AddAssign, add, add_assign,
        Sub, SubAssign, sub, sub_assign,
        Mul, MulAssign, mul, mul_assign,
        Div, DivAssign, div, div_assign,
    }
    impl<const MOD: u64> Neg for GaloisField<MOD> {
        type Output = Self;
        fn neg(mut self) -> Self::Output {
            if self.value > 0 {
                self.value = MOD - self.value;
            }
            self
        }
    }
    impl<const MOD: u64> Sum for GaloisField<MOD> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(0), |acc, x| acc + x)
        }
    }
    impl<'a, const MOD: u64> Sum<&'a Self> for GaloisField<MOD> {
        fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.copied().sum()
        }
    }
    impl<const MOD: u64> Product for GaloisField<MOD> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(1), |acc, x| acc * x)
        }
    }
    impl<'a, const MOD: u64> Product<&'a Self> for GaloisField<MOD> {
        fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.copied().product()
        }
    }
}
