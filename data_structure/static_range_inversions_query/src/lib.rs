pub mod static_range_inversions_query {
    use std::{
        cmp::Reverse,
        ops::{Bound, RangeBounds},
    };

    pub struct StaticRangeInversionsQuery<T: Copy + PartialOrd> {
        n: usize,
        chunk_size: usize,
        left_block: Vec<Vec<usize>>,
        right_block: Vec<Vec<usize>>,
        sorted_chunks: Vec<Vec<(usize, T)>>,
    }

    impl<T: Copy + PartialOrd> StaticRangeInversionsQuery<T> {
        pub fn new(a: &[T], inf: T) -> Self {
            let n = a.len();
            let chunk_size = (n as f64).sqrt() as usize;
            let a = {
                let mut a = a.to_vec();
                for _ in 0..(chunk_size - n % chunk_size) % chunk_size {
                    a.push(inf)
                }
                a
            };
            let left_block = Self::calc_block(&a, chunk_size);
            let mut right_block = Self::calc_block(
                &a.iter().rev().map(|a| Reverse(a)).collect::<Vec<_>>(),
                chunk_size,
            );
            right_block.reverse();
            let sorted_chunks = a
                .clone()
                .chunks(chunk_size)
                .map(|a| {
                    let mut a = a.iter().copied().enumerate().collect::<Vec<_>>();
                    a.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    a
                })
                .collect::<Vec<_>>();
            Self {
                n,
                chunk_size,
                left_block,
                right_block,
                sorted_chunks,
            }
        }

        fn calc_block<S: Copy + PartialOrd>(a: &[S], chunk_size: usize) -> Vec<Vec<usize>> {
            let mut res: Vec<Vec<usize>> = vec![];
            let mut sorted_block = vec![];
            for (chunk_idx, chunk) in a.chunks(chunk_size).enumerate().rev() {
                let sorted_chunk = {
                    let mut sorted_chunk = (0..chunk_size).collect::<Vec<_>>();
                    sorted_chunk.sort_by(|i, j| chunk[*j].partial_cmp(&chunk[*i]).unwrap());
                    sorted_chunk
                };
                let c = {
                    let mut c = vec![0; sorted_block.len()];
                    let mut idx = 0;
                    for &k in sorted_block.iter() {
                        while idx < chunk_size
                            && chunk[sorted_chunk[idx]] > a[chunk_size * (chunk_idx + 1) + k]
                        {
                            idx += 1;
                        }
                        c[k] = idx;
                    }
                    c
                };
                sorted_block = Self::merge_sort(
                    &a[chunk_size * (chunk_idx + 1)..],
                    &chunk,
                    &sorted_block,
                    &sorted_chunk,
                    chunk_size,
                );
                let mut res_i = vec![0];
                for (j, aj) in chunk.iter().enumerate() {
                    res_i.push(
                        res_i.last().unwrap() + chunk[..j].iter().filter(|al| *al > aj).count(),
                    );
                }
                for (k, c) in c.iter().enumerate() {
                    res_i.push(
                        res_i.last().unwrap() + res.last().unwrap()[k + 1] + c
                            - res.last().unwrap()[k],
                    );
                }
                res.push(res_i);
            }
            res.reverse();
            res
        }

        fn merge_sort<S: Copy + PartialOrd>(
            a: &[S],
            b: &[S],
            x: &[usize],
            y: &[usize],
            d: usize,
        ) -> Vec<usize> {
            let mut res = vec![0; x.len() + y.len()];
            let mut x_idx = 0;
            let mut y_idx = 0;
            while x_idx < x.len() && y_idx < y.len() {
                if a[x[x_idx]] >= b[y[y_idx]] {
                    res[x_idx + y_idx] = x[x_idx] + d;
                    x_idx += 1;
                } else {
                    res[x_idx + y_idx] = y[y_idx];
                    y_idx += 1;
                }
            }
            if x_idx < x.len() {
                res[x_idx + y_idx..]
                    .clone_from_slice(&x[x_idx..].iter().map(|x| x + d).collect::<Vec<_>>());
            } else if y_idx < y.len() {
                res[x_idx + y_idx..].clone_from_slice(&y[y_idx..]);
            }
            res
        }

        pub fn calc_inversions<R: RangeBounds<usize>>(&self, range: R) -> usize {
            let l = match range.start_bound() {
                Bound::Unbounded => 0,
                Bound::Included(&x) => x,
                Bound::Excluded(&x) => x + 1,
            };
            let r = match range.end_bound() {
                Bound::Unbounded => self.n,
                Bound::Included(&x) => x + 1,
                Bound::Excluded(&x) => x,
            };
            let ll = l - l % self.chunk_size;
            let rr = r + (self.chunk_size - r % self.chunk_size) % self.chunk_size;
            self.left_block[ll / self.chunk_size][r - ll]
                + self.right_block[rr / self.chunk_size - 1][rr - l]
                + self.calc_ends(
                    l % self.chunk_size,
                    r % self.chunk_size,
                    ll / self.chunk_size,
                    rr / self.chunk_size,
                )
                - self.left_block[ll / self.chunk_size][rr - ll]
        }

        fn calc_ends(&self, l: usize, r: usize, ll: usize, rr: usize) -> usize {
            if r == 0 {
                return 0;
            }
            let mut res = 0;
            let mut l_idx = 0;
            let mut cnt = 0;
            for &(r_idx, r_val) in self.sorted_chunks[rr - 1].iter() {
                while l_idx < self.chunk_size && self.sorted_chunks[ll][l_idx].1 > r_val {
                    if l > self.sorted_chunks[ll][l_idx].0 {
                        cnt += 1;
                    }
                    l_idx += 1;
                }
                if r_idx >= r {
                    res += cnt;
                }
            }
            res
        }
    }
}
