pub trait MonoidAct2Monoid {
    type Value: Copy;
    type Map: Copy;
    fn e() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn id() -> Self::Map;
    fn comp(lhs: &Self::Map, rhs: &Self::Map) -> Self::Map;
    fn act(val: &mut Self::Value, map: &Self::Map);
}

use std::ops::RangeBounds;
pub struct LazySegmentTree<T: MonoidAct2Monoid> {
    values: Vec<T::Value>,
    maps: Vec<T::Map>,
}

impl<T: MonoidAct2Monoid> LazySegmentTree<T> {
    pub fn new(a: &[T::Value]) -> Self {
        let n = a.len().next_power_of_two();
        let mut values = vec![T::e(); 2 * n];
        values[n..n + a.len()].clone_from_slice(a);
        for i in (1..n).rev() {
            values[i] = T::op(&values[2 * i], &values[2 * i + 1]);
        }
        Self {
            values,
            maps: vec![T::id(); 2 * n],
        }
    }

    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> T::Value {
        let n = self.values.len() / 2;
        let (l, r) = unzip(range, n);
        let (mut l, mut r) = (l + n, r + n);

        for i in (1..=n.trailing_zeros()).rev() {
            if (l >> i) << i != l {
                self.sink_map(l >> i);
            }
            if (r >> i) << i != r {
                self.sink_map((r - 1) >> i);
            }
        }
        let mut left = T::e();
        let mut right = T::e();
        while l < r {
            if l & 1 == 1 {
                left = T::op(&left, &self.values[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = T::op(&self.values[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&left, &right)
    }

    pub fn act(&mut self, range: impl RangeBounds<usize>, x: T::Map) {
        let n = self.values.len() / 2;
        let (l, r) = unzip(range, n);
        let (l, r) = (l + n, r + n);

        for i in (1..=n.trailing_zeros()).rev() {
            if (l >> i) << i != l {
                self.sink_map(l >> i);
            }
            if (r >> i) << i != r {
                self.sink_map((r - 1) >> i);
            }
        }
        {
            let (mut l, mut r) = (l, r);
            while l < r {
                if l & 1 == 1 {
                    self.apply(l, &x);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.apply(r, &x);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        for i in 1..=n.trailing_zeros() {
            if (l >> i) << i != l {
                self.float_value(l >> i);
            }
            if (r >> i) << i != r {
                self.float_value((r - 1) >> i);
            }
        }
    }

    fn apply(&mut self, i: usize, x: &T::Map) {
        T::act(&mut self.values[i], x);
        self.maps[i] = T::comp(&self.maps[i], x);
    }

    fn float_value(&mut self, i: usize) {
        self.values[i] = T::op(&self.values[2 * i], &self.values[2 * i + 1]);
    }

    fn sink_map(&mut self, i: usize) {
        let a = std::mem::replace(&mut self.maps[i], T::id());
        self.apply(2 * i, &a);
        self.apply(2 * i + 1, &a);
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
