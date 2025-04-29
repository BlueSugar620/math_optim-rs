pub mod inclusive_scan_2d;

use std::ops::{Bound, RangeBounds};

pub trait InclusiveScanOp {
    type Value: Clone;
    fn e() -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(val: &Self::Value) -> Self::Value;
}

pub struct InclusiveScan<T: InclusiveScanOp> {
    scan: Vec<T::Value>,
}

impl<T: InclusiveScanOp> InclusiveScan<T> {
    pub fn new(a: &[T::Value]) -> Self {
        Self {
            scan: std::iter::once(T::e())
                .chain(a.to_vec())
                .scan(T::e(), |acc, a| {
                    *acc = T::mul(acc, &a);
                    Some(acc.clone())
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Value {
        let (l, r) = unzip(range, self.scan.len() - 1);
        T::mul(&T::inv(&self.scan[l]), &self.scan[r])
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
