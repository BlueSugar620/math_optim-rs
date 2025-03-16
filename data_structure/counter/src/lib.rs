use std::hash::Hash;

pub struct Counter<T: Copy + Eq + Hash> {
    values: std::collections::HashMap<T, usize>,
    cnt: usize,
}

impl<T: Copy + Eq + Hash> Counter<T> {
    pub fn new(a: &[T]) -> Self {
        let mut values = std::collections::HashMap::new();
        for &a in a {
            *values.entry(a).or_insert(0) += 1;
        }
        Self {
            values,
            cnt: a.len(),
        }
    }

    pub fn add(&mut self, x: T, n: usize) -> usize {
        *self.values.entry(x).or_insert(0) += n;
        self.cnt += n;
        *self.values.get(&x).unwrap()
    }

    pub fn incr(&mut self, x: T) -> usize {
        *self.values.entry(x).or_insert(0) += 1;
        self.cnt += 1;
        *self.values.get(&x).unwrap()
    }

    pub fn reduce(&mut self, x: T, n: usize) -> usize {
        let a = *self.values.get(&x).unwrap_or(&0);
        if a <= n {
            self.values.remove(&x).unwrap();
            self.cnt -= a;
            0
        } else {
            self.values.insert(x, a - n);
            self.cnt -= n;
            a - n
        }
    }

    pub fn decr(&mut self, x: T) -> usize {
        let a = *self.values.get(&x).unwrap_or(&0);
        if a <= 1 {
            self.values.remove(&x).unwrap();
            self.cnt -= a;
            0
        } else {
            self.values.insert(x, a - 1);
            self.cnt -= 1;
            a - 1
        }
    }

    pub fn remove(&mut self, x: T) {
        let a = self.values.remove(&x).unwrap_or(0);
        self.cnt -= a;
    }

    pub fn variety(&self) -> usize {
        self.values.len()
    }

    pub fn total(&self) -> usize {
        self.cnt
    }
}
