const MOD: u64 = 998_244_353;
const SUM_E: [u64; 30] = [
    911660635, 509520358, 369330050, 332049552, 983190778, 123842337, 238493703, 975955924,
    603855026, 856644456, 131300601, 842657263, 730768835, 942482514, 806263778, 151565301,
    510815449, 503497456, 743006876, 741047443, 56250497, 867605899, 0, 0, 0, 0, 0, 0, 0, 0,
];
const SUM_IE: [u64; 30] = [
    86583718, 372528824, 373294451, 645684063, 112220581, 692852209, 155456985, 797128860,
    90816748, 860285882, 927414960, 354738543, 109331171, 293255632, 535113200, 308540755,
    121186627, 608385704, 438932459, 359477183, 824071951, 103369235, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub fn convolution998_244_353(a: &[GF<MOD>], b: &[GF<MOD>]) -> Vec<GF<MOD>> {
    let size = (a.len() + b.len() - 1).next_power_of_two();
    let mut f = vec![gf!(0); size];
    let mut g = vec![gf!(0); size];
    f[..a.len()].copy_from_slice(a);
    g[..b.len()].copy_from_slice(b);
    ntt(&mut f);
    ntt(&mut g);
    f.iter_mut().zip(g.iter()).for_each(|(f, g)| *f *= g);
    intt(&mut f);
    f.truncate(a.len() + b.len() - 1);
    f
}

fn ntt(f: &mut [GF<MOD>]) {
    let n = f.len();
    let len = n.trailing_zeros() as usize;
    for ph in 1..=len {
        let p = 1 << (len - ph);
        let mut now = gf!(1);
        for (i, f) in f.chunks_exact_mut(2 * p).enumerate() {
            let (x, y) = f.split_at_mut(p);
            for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                let l = *x;
                let r = *y * now;
                *x = l + r;
                *y = l - r;
            }
            now *= gf!(SUM_E[(!i).trailing_zeros() as usize]);
        }
    }
}

fn intt(f: &mut [GF<MOD>]) {
    let n = f.len();
    let len = n.trailing_zeros();
    for ph in (1..=len).rev() {
        let p = 1 << (len - ph);
        let mut inow = gf!(1);
        for (i, f) in f.chunks_exact_mut(2 * p).enumerate() {
            let (x, y) = f.split_at_mut(p);
            for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                let l = *x;
                let r = *y;
                *x = l + r;
                *y = (l - r) * inow;
            }
            inow *= gf!(SUM_IE[(!i).trailing_zeros() as usize]);
        }
    }
    let ik = gf!(2).inv().pow(len as u64);
    for f in f.iter_mut() {
        *f *= ik;
    }
}

use std::{
    fmt::Display,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct GF<const MOD: u64> {
    value: u64,
}

impl<const MOD: u64> GF<MOD> {
    pub fn new(value: u64) -> Self {
        Self { value: value % MOD }
    }

    pub fn pow(&self, mut exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = *self;
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
            impl<const MOD: u64> From<$t> for GF<MOD> {
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
            impl<const MOD: u64> From<$t> for GF<MOD> {
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
        $crate::GF::from($value)
    };
    ($value:expr; mod $p:expr) => {
        $crate::GF::<$p>::from($value)
    };
}

impl<const MOD: u64> Display for GF<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: u64> AddAssign<GF<MOD>> for GF<MOD> {
    fn add_assign(&mut self, rhs: GF<MOD>) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}
impl<const MOD: u64> SubAssign<GF<MOD>> for GF<MOD> {
    fn sub_assign(&mut self, rhs: GF<MOD>) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}
impl<const MOD: u64> MulAssign<GF<MOD>> for GF<MOD> {
    fn mul_assign(&mut self, rhs: GF<MOD>) {
        self.value *= rhs.value;
        self.value %= MOD;
    }
}
impl<const MOD: u64> DivAssign<GF<MOD>> for GF<MOD> {
    fn div_assign(&mut self, rhs: GF<MOD>) {
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
        impl<const MOD: u64> $trait_assign<&GF<MOD>> for GF<MOD> {
            fn $fn_assign(&mut self, rhs: &GF<MOD>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const MOD: u64, T: Into<GF<MOD>>> $trait<T> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const MOD: u64> $trait<&GF<MOD>> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const MOD: u64, T: Into<GF<MOD>>> $trait<T> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const MOD: u64> $trait<&GF<MOD>> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
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
impl<const MOD: u64> Neg for GF<MOD> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = MOD - self.value;
        }
        self
    }
}

impl<const MOD: u64> Sum for GF<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, x| acc + x)
    }
}
impl<'a, const MOD: u64> Sum<&'a Self> for GF<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl<const MOD: u64> Product for GF<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, x| acc * x)
    }
}
impl<'a, const MOD: u64> Product<&'a Self> for GF<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
