use super::GF;

pub struct Combinatorics<const MOD: u32> {
    factorials: Vec<GF<MOD>>,
    inv_factorials: Vec<GF<MOD>>,
}

impl<const MOD: u32> Combinatorics<MOD> {
    pub fn new(n: usize) -> Self {
        let mut factorials = Vec::with_capacity(n);
        factorials.push(GF::<MOD>::new(1));
        for i in 1..=n as u32 {
            factorials.push(factorials.last().unwrap() * GF::<MOD>::new(i));
        }
        let mut inv_factorials = Vec::with_capacity(n);
        inv_factorials.push(factorials.last().unwrap().inv());
        for i in (1..=n as u32).rev() {
            inv_factorials.push(inv_factorials.last().unwrap() * GF::<MOD>::new(i));
        }
        inv_factorials.reverse();

        Self {
            factorials,
            inv_factorials,
        }
    }

    pub fn binom(&self, n: usize, r: usize) -> GF<MOD> {
        self.factorials[n] * self.inv_factorials[r] * self.inv_factorials[n - r]
    }
}
