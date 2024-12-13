pub mod lazy_segment_tree {
    use std::ops::{Bound, RangeBounds};

    pub trait MonoidAct {
        type Monoid: Copy;
        fn e() -> Self::Monoid;
        fn op(lhs: &Self::Monoid, rhs: &Self::Monoid) -> Self::Monoid;
        type Act: Copy;
        fn id() -> Self::Act;
        fn comp(lhs: &Self::Act, rhs: &Self::Act) -> Self::Act;
        fn act(val: &Self::Monoid, action: &Self::Act) -> Self::Monoid;
    }
    pub struct LazySegmentTree<M: MonoidAct> {
        pub values: Vec<M::Monoid>,
        pub actions: Vec<M::Act>,
    }

    impl<M: MonoidAct> LazySegmentTree<M> {
        pub fn new(a: &[M::Monoid]) -> Self {
            let n = a.len().next_power_of_two();
            let mut values = vec![M::e(); 2 * n];
            values[n..n + a.len()].copy_from_slice(a);
            for i in (1..n).rev() {
                values[i] = M::op(&values[2 * i], &values[2 * i + 1]);
            }
            Self {
                values,
                actions: vec![M::id(); 2 * n],
            }
        }

        pub fn update_at(&mut self, i: usize, val: M::Monoid) {
            let n = self.values.len() / 2;
            let i = i + n;
            for k in (1..=n.trailing_zeros()).rev() {
                let f = std::mem::replace(&mut self.actions[i >> k], M::id());
                self.values[2 * (i >> k)] = M::act(&self.values[2 * (i >> k)], &f);
                self.actions[2 * (i >> k)] = M::comp(&self.actions[2 * (i >> k)], &f);
                self.values[2 * (i >> k) + 1] = M::act(&self.values[2 * (i >> k) + 1], &f);
                self.actions[2 * (i >> k) + 1] = M::comp(&self.actions[2 * (i >> k) + 1], &f);
            }

            self.values[i] = val;
            for k in 1..=n.trailing_zeros() {
                self.values[i >> k] =
                    M::op(&self.values[2 * (i >> k)], &self.values[2 * (i >> k) + 1]);
            }
        }

        pub fn fold<R: RangeBounds<usize>>(&mut self, range: R) -> M::Monoid {
            let n = self.values.len() / 2;
            let (l, r) = unzip(range, n);
            let (mut l, mut r) = (l + n, r + n);

            for k in (1..=n.trailing_zeros()).rev() {
                if (l >> k) << k != l {
                    let f = std::mem::replace(&mut self.actions[l >> k], M::id());
                    self.values[2 * (l >> k)] = M::act(&self.values[2 * (l >> k)], &f);
                    self.values[2 * (l >> k) + 1] = M::act(&self.values[2 * (l >> k) + 1], &f);
                    self.actions[2 * (l >> k)] = M::comp(&self.actions[2 * (l >> k)], &f);
                    self.actions[2 * (l >> k) + 1] = M::comp(&self.actions[2 * (l >> k) + 1], &f);
                }
                if (r >> k) << k != r {
                    let f = std::mem::replace(&mut self.actions[(r - 1) >> k], M::id());
                    self.values[2 * ((r - 1) >> k)] = M::act(&self.values[2 * ((r - 1) >> k)], &f);
                    self.values[2 * ((r - 1) >> k) + 1] =
                        M::act(&self.values[2 * ((r - 1) >> k) + 1], &f);
                    self.actions[2 * ((r - 1) >> k)] =
                        M::comp(&self.actions[2 * ((r - 1) >> k)], &f);
                    self.actions[2 * ((r - 1) >> k) + 1] =
                        M::comp(&self.actions[2 * ((r - 1) >> k) + 1], &f);
                }
            }

            let mut left = M::e();
            let mut right = M::e();
            while l < r {
                if l & 1 == 1 {
                    left = M::op(&left, &self.values[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    right = M::op(&self.values[r], &right);
                }
                l >>= 1;
                r >>= 1;
            }
            M::op(&left, &right)
        }

        pub fn act<R: RangeBounds<usize>>(&mut self, range: R, f: M::Act) {
            let n = self.values.len() / 2;
            let (l, r) = unzip(range, n);
            let (l, r) = (l + n, r + n);

            for k in (1..=n.trailing_zeros()).rev() {
                if (l >> k) << k != l {
                    let f = std::mem::replace(&mut self.actions[l >> k], M::id());
                    self.values[2 * (l >> k)] = M::act(&self.values[2 * (l >> k)], &f);
                    self.values[2 * (l >> k) + 1] = M::act(&self.values[2 * (l >> k) + 1], &f);
                    self.actions[2 * (l >> k)] = M::comp(&self.actions[2 * (l >> k)], &f);
                    self.actions[2 * (l >> k) + 1] = M::comp(&self.actions[2 * (l >> k) + 1], &f);
                }
                if (r >> k) << k != r {
                    let f = std::mem::replace(&mut self.actions[(r - 1) >> k], M::id());
                    self.values[2 * ((r - 1) >> k)] = M::act(&self.values[2 * ((r - 1) >> k)], &f);
                    self.values[2 * ((r - 1) >> k) + 1] =
                        M::act(&self.values[2 * ((r - 1) >> k) + 1], &f);
                    self.actions[2 * ((r - 1) >> k)] =
                        M::comp(&self.actions[2 * ((r - 1) >> k)], &f);
                    self.actions[2 * ((r - 1) >> k) + 1] =
                        M::comp(&self.actions[2 * ((r - 1) >> k) + 1], &f);
                }
            }

            {
                let mut l = l;
                let mut r = r;
                while l < r {
                    if l & 1 == 1 {
                        self.values[l] = M::act(&self.values[l], &f);
                        self.actions[l] = M::comp(&self.actions[l], &f);
                        l += 1;
                    }
                    if r & 1 == 1 {
                        r -= 1;
                        self.values[r] = M::act(&self.values[r], &f);
                        self.actions[r] = M::comp(&self.actions[r], &f);
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }

            for k in 1..=n.trailing_zeros() {
                if (l >> k) << k != l {
                    self.values[l >> k] =
                        M::op(&self.values[2 * (l >> k)], &self.values[2 * (l >> k) + 1]);
                }
                if (r >> k) << k != r {
                    self.values[(r - 1) >> k] = M::op(
                        &self.values[2 * ((r - 1) >> k)],
                        &self.values[2 * ((r - 1) >> k) + 1],
                    );
                }
            }
        }
    }

    fn unzip<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
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
