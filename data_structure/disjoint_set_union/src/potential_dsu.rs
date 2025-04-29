pub trait PotentialDSUOp {
    type Value: Clone + PartialEq;
    fn e() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct PotentialDSU<T: PotentialDSUOp> {
    parents: Vec<i32>,
    potentials: Vec<T::Value>,
    cnt: usize,
}

impl<T: PotentialDSUOp> PotentialDSU<T> {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            potentials: vec![T::e(); n],
            cnt: n,
        }
    }

    pub fn root(&self, mut v: usize) -> (usize, T::Value) {
        let mut potential = self.potentials[v].clone();
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
            potential = T::mul(&self.potentials[v], &potential);
        }
        (v, potential)
    }

    pub fn unite(&mut self, from: usize, to: usize, d: T::Value) -> bool {
        let (mut from, p_from) = self.root(from);
        let (mut to, p_to) = self.root(to);
        if from == to {
            T::mul(&p_from, &d) == p_to
        } else {
            let mut d = T::mul(&T::mul(&p_from, &d), &T::inv(&p_to));
            if self.parents[from] > self.parents[to] {
                std::mem::swap(&mut from, &mut to);
                d = T::inv(&d);
            }
            self.parents[from] += self.parents[to];
            self.parents[to] = from as i32;
            self.potentials[to] = d;
            self.cnt -= 1;
            true
        }
    }

    pub fn poteintial(&self, from: usize, to: usize) -> Option<T::Value> {
        let (from, p_from) = self.root(from);
        let (to, p_to) = self.root(to);
        if from == to {
            Some(T::mul(&T::inv(&p_from), &p_to))
        } else {
            None
        }
    }

    pub fn size(&self, u: usize) -> usize {
        -self.parents[self.root(u).0] as usize
    }

    pub fn cnt(&self) -> usize {
        self.cnt
    }
}
