pub mod fenwick_tree {
    use std::ops::RangeBounds;

    pub trait Abelian {
        type Abelian: Clone;
        fn e() -> Self::Abelian;
        fn add(lhs: &Self::Abelian, rhs: &Self::Abelian) -> Self::Abelian;
        fn inv(val: &Self::Abelian) -> Self::Abelian;
    }

    pub struct FenwickTree<T: Abelian> {
        values: Vec<T::Abelian>,
    }

    impl<T: Abelian> FenwickTree<T> {
        pub fn new(a: &[T::Abelian]) -> Self {
            let mut values = vec![T::e(); a.len() + 1];
            for (i, a) in a.iter().enumerate() {
                let i = i + 1;
                values[i] = T::add(&values[i], a);
                let lsb = i & i.wrapping_neg();
                if i + lsb < values.len() {
                    values[i + lsb] = T::add(&values[i + lsb], &values[i]);
                }
            }
            Self { values }
        }

        pub fn push(&mut self, mut x: T::Abelian) {
            let n = self.values.len();
            let lsb = n & n.wrapping_neg();
            let mut d = 1;
            while d < lsb {
                x = T::add(&x, &self.values[n - d]);
                d *= 2;
            }
            self.values.push(x);
        }

        pub fn add_at(&mut self, mut i: usize, x: T::Abelian) {
            i += 1;
            while i < self.values.len() {
                self.values[i] = T::add(&self.values[i], &x);
                i += i & i.wrapping_neg();
            }
        }

        pub fn update_at(&mut self, i: usize, x: T::Abelian) {
            let a = self.fold(i..=i);
            self.add_at(i, T::add(&T::inv(&a), &x));
        }

        fn _fold(&self, mut r: usize) -> T::Abelian {
            let mut res = T::e();
            while r > 0 {
                res = T::add(&res, &self.values[r]);
                r -= r & r.wrapping_neg();
            }
            res
        }

        pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Abelian {
            let (l, r) = unzip(range, self.values.len() - 1);
            T::add(&self._fold(r), &T::inv(&self._fold(l)))
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
