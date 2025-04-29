pub trait ValuedDSUOp {
    type Value: Copy;
    fn e() -> Self::Value;
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct ValuedDSU<T: ValuedDSUOp> {
    parents: Vec<isize>,
    values: Vec<T::Value>,
    cnt: usize,
}

impl<T: ValuedDSUOp> ValuedDSU<T> {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            values: vec![T::e(); n],
            cnt: n,
        }
    }

    pub fn root(&self, mut v: usize) -> usize {
        while self.parents[v] >= 0 {
            v = self.parents[v] as usize;
        }
        v
    }

    pub fn value(&self, v: usize) -> T::Value {
        self.values[self.root(v)]
    }

    pub fn update_at(&mut self, v: usize, value: T::Value) {
        let v = self.root(v);
        self.values[v] = value;
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let mut u = self.root(u);
        let mut v = self.root(v);
        if u == v {
            return;
        }
        if self.parents[u] > self.parents[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.parents[u] += self.parents[v];
        self.parents[v] = u as isize;
        self.values[u] = T::op(&self.values[u], &self.values[v]);
        self.values[v] = T::e();
        self.cnt -= 1;
    }

    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    pub fn size(&self, v: usize) -> usize {
        -self.parents[self.root(v)] as usize
    }

    pub fn cnt(&self) -> usize {
        self.cnt
    }
}
