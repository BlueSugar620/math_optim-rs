pub mod fenwick_tree_on_monoid {
    pub trait CommutativeMonoid {
        type Monoid: Copy;
        fn e() -> Self::Monoid;
        fn op(lhs: &Self::Monoid, rhs: &Self::Monoid) -> Self::Monoid;
    }

    pub struct FenwickTree<M: CommutativeMonoid> {
        values: Vec<M::Monoid>,
    }

    impl<M: CommutativeMonoid> FenwickTree<M> {
        pub fn new(a: &[M::Monoid]) -> Self {
            let mut values = vec![M::e(); a.len() + 1];
            for (i, a) in a.iter().enumerate() {
                let i = i + 1;
                values[i] = M::op(&values[i], a);
                let lsb = i & i.wrapping_neg();
                if i + lsb < values.len() {
                    values[i + lsb] = M::op(&values[i + lsb], &values[i]);
                }
            }
            Self { values }
        }

        pub fn push(&mut self, mut x: M::Monoid) {
            let n = self.values.len();
            let lsb = n & n.wrapping_neg();
            let mut d = 1;
            while d < lsb {
                x = M::op(&x, &self.values[n - d]);
                d *= 2;
            }
            self.values.push(x);
        }

        pub fn op_at(&mut self, mut i: usize, x: M::Monoid) {
            i += 1;
            while i < self.values.len() {
                self.values[i] = M::op(&self.values[i], &x);
                i += i & i.wrapping_neg();
            }
        }

        pub fn prefix_sum(&self, mut end: usize) -> M::Monoid {
            let mut res = M::e();
            while end > 0 {
                res = M::op(&res, &self.values[end]);
                end -= end & end.wrapping_neg();
            }
            res
        }
    }
}

pub mod fenwick_tree_on_abelian {
    use std::ops::RangeBounds;

    pub trait Abelian {
        type Group: Copy;
        fn e() -> Self::Group;
        fn op(lhs: &Self::Group, rhs: &Self::Group) -> Self::Group;
        fn inv(val: &Self::Group) -> Self::Group;
    }

    pub struct FenwickTree<G: Abelian> {
        values: Vec<G::Group>,
    }

    impl<G: Abelian> FenwickTree<G> {
        pub fn new(a: &[G::Group]) -> Self {
            let mut values = vec![G::e(); a.len() + 1];
            for (i, a) in a.iter().enumerate() {
                let i = i + 1;
                values[i] = G::op(&values[i], a);
                let lsb = i & i.wrapping_neg();
                if i + lsb < values.len() {
                    values[i + lsb] = G::op(&values[i + lsb], &values[i]);
                }
            }
            Self { values }
        }

        pub fn push(&mut self, mut x: G::Group) {
            let n = self.values.len();
            let lsb = n & n.wrapping_neg();
            let mut d = 1;
            while d < lsb {
                x = G::op(&x, &self.values[n - d]);
                d *= 2;
            }
            self.values.push(x);
        }

        pub fn op_at(&mut self, mut i: usize, x: G::Group) {
            i += 1;
            while i < self.values.len() {
                self.values[i] = G::op(&self.values[i], &x);
                i += i & i.wrapping_neg();
            }
        }

        pub fn update_at(&mut self, i: usize, x: G::Group) {
            let a = self.fold(i..=i);
            self.op_at(i, G::op(&G::inv(&a), &x));
        }

        pub fn prefix_sum(&self, mut end: usize) -> G::Group {
            let mut res = G::e();
            while end > 0 {
                res = G::op(&res, &self.values[end]);
                end -= end & end.wrapping_neg();
            }
            res
        }

        pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> G::Group {
            let (l, r) = unzip(range, self.values.len() - 1);
            G::op(&self.prefix_sum(r), &G::inv(&self.prefix_sum(l)))
        }
    }

    fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
        use std::ops::Bound;
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => n,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };
        (start, end)
    }
}
