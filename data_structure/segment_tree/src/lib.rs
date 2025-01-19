pub trait Monoid {
    type Monoid: Copy;
    fn e() -> Self::Monoid;
    fn op(lhs: &Self::Monoid, rhs: &Self::Monoid) -> Self::Monoid;
}

use std::ops::RangeBounds;
pub struct SegmentTree<M: Monoid> {
    values: Vec<M::Monoid>,
}
impl<M: Monoid> SegmentTree<M> {
    pub fn new(a: &[M::Monoid]) -> Self {
        let n = a.len().next_power_of_two();
        let mut values = vec![M::e(); 2 * n];
        values[n..n + a.len()].clone_from_slice(a);
        for i in (1..n).rev() {
            values[i] = M::op(&values[2 * i], &values[2 * i + 1]);
        }
        Self { values }
    }

    pub fn get_at(&self, idx: usize) -> M::Monoid {
        self.values[idx + self.values.len() / 2]
    }

    pub fn update_at(&mut self, i: usize, x: M::Monoid) {
        let mut i = i + self.values.len() / 2;
        self.values[i] = x;
        i >>= 1;
        while i > 0 {
            self.values[i] = M::op(&self.values[2 * i], &self.values[2 * i + 1]);
            i >>= 1;
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> M::Monoid {
        let (l, r) = unzip(range, self.values.len() / 2);
        let (mut l, mut r) = (l + self.values.len() / 2, r + self.values.len() / 2);
        let mut left = M::e();
        let mut right = M::e();
        while l < r {
            if l % 2 == 1 {
                left = M::op(&left, &self.values[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = M::op(&self.values[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&left, &right)
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
