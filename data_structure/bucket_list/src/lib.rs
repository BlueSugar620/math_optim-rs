const BUCKET_RATIO: usize = 16;
const SPLIT_RATIO: usize = 24;

pub struct BucketList<T: Copy> {
    values: Vec<Vec<T>>,
}

impl<T: Copy> BucketList<T> {
    pub fn new(a: &[T]) -> Self {
        Self {
            values: a
                .chunks(BUCKET_RATIO)
                .into_iter()
                .map(|a| a.to_vec())
                .collect::<Vec<_>>(),
        }
    }

    pub fn get_at(&self, mut i: usize) -> T {
        for a in self.values.iter() {
            if i < a.len() {
                return a[i];
            }
            i -= a.len();
        }
        panic!()
    }

    pub fn insert(&mut self, mut i: usize, x: T) {
        if self.values.len() == 0 {
            self.values = vec![vec![x]];
            return;
        }
        for j in 0..self.values.len() {
            if i <= self.values[j].len() {
                self.values[j].insert(i, x);
                if self.values[j].len() > self.values.len() * SPLIT_RATIO {
                    let o = self.values[j].len() >> 1;
                    let (s, t) = (self.values[j][..o].to_vec(), self.values[j][o..].to_vec());
                    self.values[j] = t;
                    self.values.insert(j, s);
                }
                return;
            }
            i -= self.values[j].len();
        }
    }
}
