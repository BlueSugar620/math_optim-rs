pub mod inclusive_scan {
    use std::ops::{Bound, RangeBounds};
    pub trait Abelian {
        type Abelian: Clone;
        fn id() -> Self::Abelian;
        fn add(lhs: &Self::Abelian, rhs: &Self::Abelian) -> Self::Abelian;
        fn inv(val: &Self::Abelian) -> Self::Abelian;
    }

    pub struct InclusiveScan<T: Abelian> {
        scan: Box<[T::Abelian]>,
    }

    impl<T: Abelian> InclusiveScan<T> {
        pub fn new(a: &[T::Abelian]) -> Self {
            Self {
                scan: std::iter::once(T::id())
                    .chain(a.to_vec())
                    .scan(T::id(), |acc, a| {
                        *acc = T::add(acc, &a);
                        Some(acc.clone())
                    })
                    .collect::<Box<_>>(),
            }
        }

        pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Abelian {
            let n = self.scan.len() - 1;
            let (l, r) = unzip(range, n);
            T::add(&self.scan[r], &T::inv(&self.scan[l]))
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
