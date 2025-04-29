use super::Convolution;
use galois_field::GF;

const MOD: u32 = 998_244_353;

pub enum Xor {}
impl BitConv for Xor {
    fn fourier_matrix(lhs: &mut GF<MOD>, rhs: &mut GF<MOD>) {
        (*lhs, *rhs) = (*lhs + *rhs, *lhs - *rhs);
    }
    fn inverse_matrix(lhs: &mut GF<MOD>, rhs: &mut GF<MOD>) {
        (*lhs, *rhs) = (*lhs + *rhs, *lhs - *rhs);
    }
}

use std::marker::PhantomData;
pub trait BitConv {
    fn fourier_matrix(lhs: &mut GF<MOD>, rhs: &mut GF<MOD>);
    fn inverse_matrix(lhs: &mut GF<MOD>, rhs: &mut GF<MOD>);
}

struct Bitwise<T: BitConv> {
    phantom: PhantomData<T>,
}
impl<T: BitConv> Convolution for Bitwise<T> {
    type Value = GF<MOD>;
    fn e() -> Self::Value {
        GF::new(0)
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs * rhs
    }
    fn fourier_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let b = n.trailing_zeros() as usize;
        for k in 0..b {
            let k = 1 << k;
            for a in a.chunks_exact_mut(2 * k) {
                let (x, y) = a.split_at_mut(k);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    T::fourier_matrix(x, y);
                }
            }
        }
    }
    fn inverse_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let b = n.trailing_zeros() as usize;
        for k in 0..b {
            let k = 1 << k;
            for a in a.chunks_exact_mut(2 * k) {
                let (x, y) = a.split_at_mut(k);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    T::inverse_matrix(x, y);
                }
            }
        }

        let coef = GF::new(2).inv().pow(b as u32);
        for a in a.iter_mut() {
            *a *= coef;
        }
    }
}
