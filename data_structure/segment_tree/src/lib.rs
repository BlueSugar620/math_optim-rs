pub mod dual_segment_tree;
pub mod lazy_segment_tree;

pub trait Monoid {
    type Value: Copy;
    fn e() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

use std::ops::RangeBounds;
pub struct SegmentTree<T: Monoid> {
    values: Vec<T::Value>,
}
impl<T: Monoid> SegmentTree<T> {
    pub fn new(a: &[T::Value]) -> Self {
        let n = a.len().next_power_of_two();
        let mut values = vec![T::e(); 2 * n];
        values[n..n + a.len()].clone_from_slice(a);
        for i in (1..n).rev() {
            values[i] = T::op(&values[2 * i], &values[2 * i + 1]);
        }
        Self { values }
    }

    pub fn get_at(&self, idx: usize) -> T::Value {
        self.values[idx + self.values.len() / 2]
    }

    pub fn update_at(&mut self, i: usize, x: T::Value) {
        let mut i = i + self.values.len() / 2;
        self.values[i] = x;
        i >>= 1;
        while i > 0 {
            self.values[i] = T::op(&self.values[2 * i], &self.values[2 * i + 1]);
            i >>= 1;
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> T::Value {
        let (l, r) = unzip(range, self.values.len() / 2);
        let (mut l, mut r) = (l + self.values.len() / 2, r + self.values.len() / 2);
        let mut left = T::e();
        let mut right = T::e();
        while l < r {
            if l % 2 == 1 {
                left = T::op(&left, &self.values[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = T::op(&self.values[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&left, &right)
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
