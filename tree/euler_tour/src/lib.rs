use std::ops::{Range, RangeInclusive};

pub struct EulerTour {
    tot_time: usize,
    in_time: Box<[usize]>,
    out_time: Box<[usize]>,
}

impl EulerTour {
    pub fn new(root: usize, e: &Vec<Vec<usize>>) -> Self {
        let n = e.len() + 1;
        let mut in_time = vec![!0; n];
        let mut out_time = vec![!0; n];
        let mut stack = vec![root];
        let mut t = 0usize;
        while let Some(u) = stack.pop() {
            if in_time[u] == !0 {
                in_time[u] = t;
                t += 1;
                stack.push(u);
                for &v in &e[u] {
                    if in_time[v] == !0 {
                        stack.push(v);
                    }
                }
            } else if out_time[u] == !0 {
                out_time[u] = t;
                t += 1;
            }
        }
        Self {
            tot_time: 2 * n,
            in_time: in_time.into_boxed_slice(),
            out_time: out_time.into_boxed_slice(),
        }
    }

    pub fn in_time(&self, u: usize) -> usize {
        self.in_time[u]
    }

    pub fn out_time(&self, u: usize) -> usize {
        self.out_time[u]
    }

    pub fn tot_time(&self) -> usize {
        self.tot_time
    }

    pub fn subtree(&self, u: usize) -> Range<usize> {
        self.in_time[u]..self.out_time[u]
    }

    pub fn path(&self, u: usize, v: usize) -> RangeInclusive<usize> {
        self.in_time[u]..=self.in_time[v]
    }
}
