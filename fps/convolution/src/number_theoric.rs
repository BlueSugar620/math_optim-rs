use super::Convolution;
use galois_field::GF;

const SUM_E: [u32; 22] = [
    911660635, 509520358, 369330050, 332049552, 983190778, 123842337, 238493703, 975955924,
    603855026, 856644456, 131300601, 842657263, 730768835, 942482514, 806263778, 151565301,
    510815449, 503497456, 743006876, 741047443, 56250497, 867605899,
];
const SUM_IE: [u32; 22] = [
    86583718, 372528824, 373294451, 645684063, 112220581, 692852209, 155456985, 797128860,
    90816748, 860285882, 927414960, 354738543, 109331171, 293255632, 535113200, 308540755,
    121186627, 608385704, 438932459, 359477183, 824071951, 103369235,
];

pub enum NumberTheoric998244353 {}
impl Convolution for NumberTheoric998244353 {
    type Value = GF<998_244_353>;
    fn e() -> Self::Value {
        GF::new(0)
    }
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        lhs * rhs
    }
    fn fourier_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let b = n.trailing_zeros() as usize;
        for k in (0..b).rev() {
            let k = 1 << k;
            let mut coef = GF::new(1);
            for (i, a) in a.chunks_exact_mut(2 * k).enumerate() {
                let (x, y) = a.split_at_mut(k);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    (*x, *y) = (*x + *y * coef, *x - *y * coef);
                }
                coef *= GF::new(SUM_E[(!i).trailing_zeros() as usize]);
            }
        }
    }
    fn inverse_transform(a: &mut [Self::Value]) {
        let n = a.len();
        let b = n.trailing_zeros() as usize;
        for k in 0..b {
            let k = 1 << k;
            let mut coef = GF::new(1);
            for (i, a) in a.chunks_exact_mut(2 * k).enumerate() {
                let (x, y) = a.split_at_mut(k);
                for (x, y) in x.iter_mut().zip(y.iter_mut()) {
                    (*x, *y) = (*x + *y, (*x - *y) * coef);
                }
                coef *= GF::new(SUM_IE[(!i).trailing_zeros() as usize]);
            }
        }
        let coef = GF::new(2).inv().pow(b as u32);
        for a in a.iter_mut() {
            *a *= coef;
        }
    }
}
