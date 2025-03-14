pub mod bitwise;
pub mod number_theoric;

pub trait Convolution {
    type Value: Copy;
    fn e() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn convolution(lhs: &[Self::Value], rhs: &[Self::Value]) -> Vec<Self::Value> {
        let size = (lhs.len() + rhs.len() - 1).next_power_of_two();
        let mut f = vec![Self::e(); size];
        let mut g = vec![Self::e(); size];
        f[..lhs.len()].copy_from_slice(lhs);
        g[..rhs.len()].copy_from_slice(rhs);

        Self::fourier_transform(&mut f);
        Self::fourier_transform(&mut g);

        let mut h = f
            .iter()
            .zip(g.iter())
            .map(|(f, g)| Self::mul(f, g))
            .collect::<Vec<_>>();

        Self::inverse_transform(&mut h);

        h.truncate(lhs.len() + rhs.len() - 1);
        h
    }
    fn fourier_transform(a: &mut [Self::Value]);
    fn inverse_transform(a: &mut [Self::Value]);
}
