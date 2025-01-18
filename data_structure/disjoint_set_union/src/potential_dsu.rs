pub mod potential_dsu {
    pub trait Abelian {
        type Abelian: Clone + PartialEq;
        fn id() -> Self::Abelian;
        fn add(lhs: &Self::Abelian, rhs: &Self::Abelian) -> Self::Abelian;
        fn inv(val: &Self::Abelian) -> Self::Abelian;
    }

    pub struct PotentialDSU<T: Abelian> {
        parents: Vec<isize>,
        potentials: Vec<T::Abelian>,
        cnt: usize,
    }

    impl<T: Abelian> PotentialDSU<T> {
        pub fn new(n: usize) -> Self {
            Self {
                parents: vec![-1; n],
                potentials: vec![T::id(); n],
                cnt: n,
            }
        }

        pub fn root(&self, mut v: usize) -> (usize, T::Abelian) {
            let mut potential = self.potentials[v].clone();
            while self.parents[v] >= 0 {
                v = self.parents[v] as usize;
                potential = T::add(&potential, &self.potentials[v]);
            }
            (v, potential)
        }

        pub fn unite(&mut self, from: usize, to: usize, d: T::Abelian) -> bool {
            let (mut from, p_from) = self.root(from);
            let (mut to, p_to) = self.root(to);
            if from == to {
                T::add(&p_to, &T::inv(&p_from)) == d
            } else {
                let mut d = T::add(&T::add(&d, &p_from), &T::inv(&p_to));
                if self.parents[from] > self.parents[to] {
                    std::mem::swap(&mut from, &mut to);
                    d = T::inv(&d);
                }
                self.parents[from] += self.parents[to];
                self.parents[to] = from as isize;
                self.potentials[to] = d;
                self.cnt -= 1;
                true
            }
        }

        pub fn poteintial(&self, from: usize, to: usize) -> Option<T::Abelian> {
            let (from, p_from) = self.root(from);
            let (to, p_to) = self.root(to);
            if from == to {
                Some(T::add(&p_to, &T::inv(&p_from)))
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
}
