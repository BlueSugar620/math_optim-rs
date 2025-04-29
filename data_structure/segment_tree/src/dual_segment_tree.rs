pub trait DualSegmentTreeOp {
    type Value: Copy;
    type Map;
    fn is_commutative() -> bool;
    fn id() -> Self::Map;
    fn comp(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map;
    fn act(val: &mut Self::Value, map: &Self::Map);
}

use std::ops::RangeBounds;
pub struct DualSegmentTree<T: DualSegmentTreeOp> {
    values: Vec<T::Value>,
    maps: Vec<T::Map>,
}

impl<T: DualSegmentTreeOp> DualSegmentTree<T> {
    pub fn new(values: &[T::Value]) -> Self {
        let n = values.len().next_power_of_two();
        Self {
            values: values.to_vec(),
            maps: (0..2 * n).map(|_| T::id()).collect(),
        }
    }

    pub fn get_at(&self, idx: usize) -> T::Value {
        let mut value = self.values[idx];
        let mut idx = idx + self.maps.len() / 2;
        while idx > 0 {
            T::act(&mut value, &self.maps[idx]);
            idx >>= 1;
        }
        value
    }

    pub fn act(&mut self, range: impl RangeBounds<usize>, x: T::Map) {
        let n = self.maps.len() / 2;
        let (l, r) = unzip(range, n);
        let (mut l, mut r) = (l + n, r + n);

        if !T::is_commutative() {
            for i in (1..=n.trailing_zeros()).rev() {
                if (l >> i) << i != l {
                    self.sink(l >> i);
                }
                if (r >> i) << i != r {
                    self.sink((r - 1) >> i);
                }
            }
        }

        while l < r {
            if l & 1 == 1 {
                self.maps[l] = T::comp(&self.maps[l], &x);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.maps[r] = T::comp(&self.maps[r], &x);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    fn sink(&mut self, i: usize) {
        let a = std::mem::replace(&mut self.maps[i], T::id());
        self.maps[2 * i] = T::comp(&self.maps[2 * i], &a);
        self.maps[2 * i + 1] = T::comp(&self.maps[2 * i + 1], &a);
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
