pub mod disjoint_set_union {
    pub struct DisjointSetUnion {
        parents: Vec<isize>,
        cnt: usize,
    }

    impl DisjointSetUnion {
        pub fn new(n: usize) -> Self {
            Self {
                parents: vec![-1; n],
                cnt: n,
            }
        }

        pub fn root(&self, mut v: usize) -> usize {
            while self.parents[v] >= 0 {
                v = self.parents[v] as usize;
            }
            v
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

pub mod disjoint_set_union_with_relation {
    pub trait ExIntegralDomain {
        type Value: Copy + PartialEq;
        fn zero() -> Self::Value;
        fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
        fn neg(val: &Self::Value) -> Self::Value;
        fn one() -> Self::Value;
        fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
        fn inv(val: &Self::Value) -> Option<Self::Value>;
        fn is_val(val: &Self::Value) -> bool;
    }

    pub struct DisjointSetUnion<I: ExIntegralDomain> {
        pub parents: Vec<isize>,
        pub relation: Vec<(I::Value, I::Value)>,
        pub values: Vec<Option<I::Value>>,
        cnt: usize,
    }

    impl<I: ExIntegralDomain> DisjointSetUnion<I> {
        pub fn new(n: usize) -> Self {
            Self {
                parents: vec![-1; n],
                relation: vec![(I::one(), I::zero()); n],
                values: vec![None; n],
                cnt: n,
            }
        }

        pub fn root(&self, mut v: usize) -> usize {
            while self.parents[v] >= 0 {
                v = self.parents[v] as usize;
            }
            v
        }

        pub fn relation_from_root(&self, mut v: usize) -> (usize, (I::Value, I::Value)) {
            let (mut a, mut b) = self.relation[v];
            while self.parents[v] >= 0 {
                v = self.parents[v] as usize;
                let (c, d) = &self.relation[v];
                (a, b) = (I::mul(&a, c), I::add(&I::mul(&a, d), &b));
            }
            (v, (a, b))
        }

        pub fn unite(&mut self, from: usize, to: usize, (a, b): (I::Value, I::Value)) -> bool {
            let (mut from, (af, bf)) = self.relation_from_root(from);
            let (mut to, (at, bt)) = self.relation_from_root(to);
            if from == to {
                if I::mul(&a, &af) == at {
                    return I::add(&I::mul(&a, &bf), &b) == bt;
                } else {
                    let val = I::mul(
                        &I::inv(&I::add(&at, &I::neg(&I::mul(&a, &af)))).unwrap(),
                        &I::add(&I::add(&I::mul(&a, &bf), &b), &I::neg(&bt)),
                    );
                    if !I::is_val(&val) {
                        return false;
                    }
                    if let Some(prev_val) = self.values[from] {
                        if prev_val == val {
                            return true;
                        } else {
                            return false;
                        }
                    }
                    self.values[from] = Some(val);
                    return true;
                }
            }
            let at_inv = I::inv(&at).unwrap();
            let (mut a, mut b) = (
                I::mul(&at_inv, &I::mul(&a, &af)),
                I::mul(
                    &at_inv,
                    &I::add(&I::add(&I::mul(&a, &bf), &b), &I::neg(&bt)),
                ),
            );
            if self.parents[from] > self.parents[to] {
                std::mem::swap(&mut from, &mut to);
                let a_inv = I::inv(&a).unwrap();
                (a, b) = (a_inv, I::neg(&I::mul(&a_inv, &b)));
            }
            if let Some(f_val) = &self.values[from] {
                if let Some(t_val) = &self.values[to] {
                    if *t_val != I::add(&I::mul(&a, f_val), &b) {
                        return false;
                    }
                }
            }
            if let Some(t_val) = &self.values[to] {
                let f_val = I::mul(&I::inv(&a).unwrap(), &I::add(t_val, &I::neg(&b)));
                if !I::is_val(&f_val) {
                    return false;
                }
                self.values[from] = Some(f_val);
                self.values[to] = None;
            }
            self.parents[from] += self.parents[to];
            self.parents[to] = from as isize;
            self.relation[to] = (a, b);
            true
        }

        pub fn relation(&self, from: usize, to: usize) -> Option<(I::Value, I::Value)> {
            let (from, (af, bf)) = self.relation_from_root(from);
            let (to, (at, bt)) = self.relation_from_root(to);
            if from == to {
                let af_inv = I::inv(&af).unwrap();
                Some((
                    I::mul(&at, &af_inv),
                    I::add(&I::neg(&I::mul(&I::mul(&at, &af_inv), &bf)), &bt),
                ))
            } else {
                None
            }
        }

        pub fn value(&self, v: usize) -> Option<I::Value> {
            let (v, (a, b)) = self.relation_from_root(v);
            if let Some(val) = &self.values[v] {
                Some(I::add(&I::mul(&a, val), &b))
            } else {
                None
            }
        }

        pub fn size(&self, v: usize) -> usize {
            -self.parents[self.root(v)] as usize
        }

        pub fn cnt(&self) -> usize {
            self.cnt
        }
    }
}
