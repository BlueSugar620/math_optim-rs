const BUCKET_RATIO: usize = 2048;
const SPLIT_RATIO: usize = 8;

pub struct SortedSet<T: Copy + PartialOrd> {
    values: Vec<Vec<T>>,
    size: usize,
}

impl<T: Copy + PartialOrd> SortedSet<T> {
    pub fn new(a: &[T]) -> Self {
        if a.windows(2).all(|a| a[0] < a[1]) {
            Self {
                values: a
                    .chunks(BUCKET_RATIO)
                    .into_iter()
                    .map(|a| a.to_vec())
                    .collect::<Vec<_>>(),
                size: a.len(),
            }
        } else {
            let mut a = a.to_vec();
            a.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            a.dedup();
            Self {
                values: a
                    .chunks(BUCKET_RATIO)
                    .into_iter()
                    .map(|a| a.to_vec())
                    .collect::<Vec<_>>(),
                size: a.len(),
            }
        }
    }

    fn _position(&self, x: &T) -> (usize, usize) {
        if let Some(block) = self.values.iter().position(|v| x <= v.last().unwrap()) {
            if x <= &self.values[block][0] {
                (block, 0)
            } else {
                let mut l = 0;
                let mut r = self.values[block].len();
                while r - l > 1 {
                    let o = (r + l) / 2;
                    if x <= &self.values[block][o] {
                        r = o;
                    } else {
                        l = o;
                    }
                }
                (block, r)
            }
        } else {
            (self.values.len() - 1, self.values.last().unwrap().len())
        }
    }

    pub fn contains(&self, x: &T) -> bool {
        if self.size == 0 {
            false
        } else {
            let (block, idx) = self._position(x);
            idx < self.values[block].len() && self.values[block][idx] == *x
        }
    }

    pub fn insert(&mut self, x: T) -> bool {
        if self.size == 0 {
            self.values = vec![vec![x]];
            self.size = 1;
            true
        } else {
            let (block, idx) = self._position(&x);
            if idx == self.values[block].len() {
                self.values[block].push(x);
                if self.values[block].len() > self.values.len() * SPLIT_RATIO {
                    let o = self.values[block].len() >> 1;
                    self.values
                        .insert(block + 1, self.values[block][o..].to_vec());
                    self.values[block] = self.values[block][..o].to_vec();
                }
                self.size += 1;
                true
            } else {
                if self.values[block][idx] == x {
                    false
                } else {
                    self.values[block].insert(idx, x);
                    if self.values[block].len() > self.values.len() * SPLIT_RATIO {
                        let o = self.values[block].len() >> 1;
                        self.values
                            .insert(block + 1, self.values[block][o..].to_vec());
                        self.values[block] = self.values[block][..o].to_vec();
                    }
                    self.size += 1;
                    true
                }
            }
        }
    }

    pub fn discard(&mut self, x: T) -> bool {
        if self.size == 0 {
            false
        } else {
            let (block, idx) = self._position(&x);
            if idx == self.values[block].len() {
                false
            } else {
                if self.values[block][idx] == x {
                    self.values[block].remove(idx);
                    if self.values[block].len() == 0 {
                        self.values.remove(block);
                    }
                    self.size -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn nth(&self, mut i: usize) -> Option<T> {
        for block in &self.values {
            if block.len() <= i {
                i -= block.len();
            } else {
                return Some(block[i]);
            }
        }
        None
    }

    pub fn index(&self, x: &T) -> Option<usize> {
        let mut res = 0;
        for block in &self.values {
            if block.last().unwrap() < x {
                res += block.len();
            } else {
                for (i, v) in block.iter().enumerate() {
                    if x == v {
                        return Some(res + i);
                    }
                }
            }
        }
        None
    }

    pub fn less_than_equal(&self, x: &T) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let (block, idx) = self._position(x);
            if idx == self.values[block].len() {
                Some(*self.values[block].last().unwrap())
            } else {
                if x == &self.values[block][idx] {
                    Some(*x)
                } else {
                    if idx == 0 {
                        if block == 0 {
                            None
                        } else {
                            Some(*self.values[block - 1].last().unwrap())
                        }
                    } else {
                        Some(self.values[block][idx - 1])
                    }
                }
            }
        }
    }

    pub fn less_than(&self, x: &T) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let (block, idx) = self._position(x);
            if idx == 0 {
                if block == 0 {
                    None
                } else {
                    Some(*self.values[block - 1].last().unwrap())
                }
            } else {
                Some(self.values[block][idx - 1])
            }
        }
    }

    pub fn greater_than_equal(&self, x: &T) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let (block, idx) = self._position(x);
            if idx == self.values[block].len() {
                None
            } else {
                Some(self.values[block][idx])
            }
        }
    }

    pub fn greater_than(&self, x: &T) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let (block, idx) = self._position(x);
            if idx == self.values[block].len() {
                None
            } else if x == &self.values[block][idx] {
                if idx + 1 == self.values[block].len() {
                    if block + 1 == self.values.len() {
                        None
                    } else {
                        Some(self.values[block + 1][0])
                    }
                } else {
                    Some(self.values[block][idx + 1])
                }
            } else {
                Some(self.values[block][idx])
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}
