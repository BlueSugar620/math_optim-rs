use std::collections::BinaryHeap;

const MAX: i64 = std::i64::MAX / 2;
const MIN: i64 = std::i64::MIN / 2;

pub struct SlopeTrick {
    min: i64,
    l: BinaryHeap<i64>,
    r: BinaryHeap<i64>,
}

impl SlopeTrick {
    pub fn new() -> Self {
        let mut l = BinaryHeap::new();
        let mut r = BinaryHeap::new();
        l.push(MIN);
        r.push(-MAX);
        SlopeTrick { min: 0, l, r }
    }

    pub fn min(&self) -> i64 {
        self.min
    }

    pub fn add_const(&mut self, a: i64) {
        self.min += a;
    }

    pub fn add_plus(&mut self, a: i64) {
        self.l.push(a);
        let x = self.l.pop().unwrap();
        self.min += x - a;
        self.r.push(-x);
    }

    pub fn add_minus(&mut self, a: i64) {
        self.r.push(-a);
        let x = -self.r.pop().unwrap();
        self.min += a - x;
        self.l.push(x);
    }

    pub fn add_absolute(&mut self, a: i64) {
        self.add_plus(a);
        self.add_minus(a);
    }
}
