pub mod disjoint_set_union_on_monoid {
    pub trait CommutativeMonoid {
        type Monoid: Copy;
        fn e() -> Self::Monoid;
        fn op(lhs: &Self::Monoid, rhs: &Self::Monoid) -> Self::Monoid;
    }

    pub struct DisjointSetUnion<M: CommutativeMonoid> {
        parents: Vec<isize>,
        values: Vec<M::Monoid>,
        cnt: usize,
    }

    impl<M: CommutativeMonoid> DisjointSetUnion<M> {
        pub fn new(n: usize) -> Self {
            Self {
                parents: vec![-1; n],
                values: vec![M::e(); n],
                cnt: n,
            }
        }

        pub fn root(&self, mut v: usize) -> usize {
            while self.parents[v] >= 0 {
                v = self.parents[v] as usize;
            }
            v
        }

        pub fn value(&self, v: usize) -> M::Monoid {
            self.values[self.root(v)]
        }

        pub fn update_value(&mut self, v: usize, value: M::Monoid) {
            let v = self.root(v);
            self.values[v] = value;
        }

        pub fn unite(&mut self, u: usize, v: usize) {
            let mut u = self.root(u);
            let mut v = self.root(v);
            if u == v {
                return;
            }
            if self.parents[u] > self.parents[v] {
                std::mem::swap(&mut u, &mut v);
            }
            self.parents[u] += self.parents[v];
            self.parents[v] = u as isize;
            self.values[u] = M::op(&self.values[u], &self.values[v]);
            self.cnt -= 1;
        }

        pub fn is_same(&self, u: usize, v: usize) -> bool {
            self.root(u) == self.root(v)
        }

        pub fn size(&self, v: usize) -> usize {
            -self.parents[self.root(v)] as usize
        }

        pub fn cnt(&self) -> usize {
            self.cnt
        }
    }
}
