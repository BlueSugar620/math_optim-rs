pub trait FenwickTreeOp {
    type Value: Clone;
    fn e() -> Self::Value;
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct FenwickTree<T: FenwickTreeOp> {
    values: Vec<T::Value>,
}

impl<T: FenwickTreeOp> FenwickTree<T> {
    pub fn new(a: &[T::Value]) -> Self {
        let n = a.len();
        let mut values = vec![T::e(); n + 1];
        for (i, a) in a.iter().enumerate() {
            let i = i + 1;
            values[i] = T::add(&values[i], &a);
            let lsb = i & i.wrapping_neg();
            if i + lsb < n + 1 {
                values[i + lsb] = T::add(&values[i + lsb], &values[i]);
            }
        }
        Self { values }
    }

    pub fn push(&mut self, mut x: T::Value) {
        let n = self.values.len();
        let lsb = n & n.wrapping_neg();
        let mut d = 1;
        while d < lsb {
            x = T::add(&x, &self.values[n - d]);
            d *= 2;
        }
        self.values.push(x);
    }

    pub fn add_at(&mut self, mut i: usize, x: T::Value) {
        i += 1;
        while i < self.values.len() {
            self.values[i] = T::add(&self.values[i], &x);
            i += i & i.wrapping_neg();
        }
    }

    pub fn fold(&self, mut r: usize) -> T::Value {
        let mut res = T::e();
        while r > 0 {
            res = T::add(&res, &self.values[r]);
            r -= r & r.wrapping_neg();
        }
        res
    }
}
