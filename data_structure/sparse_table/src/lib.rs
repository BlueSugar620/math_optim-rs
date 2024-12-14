pub mod sparse_table {
    use std::ops::RangeBounds;

    pub trait Band {
        type Band: Copy;
        fn op(lhs: &Self::Band, rhs: &Self::Band) -> Self::Band;
    }

    pub struct SparseTable<B: Band> {
        values: Vec<Vec<B::Band>>,
    }

    impl<B: Band> SparseTable<B> {
        pub fn new(a: &[B::Band]) -> Self {
            let mut a = a.to_vec();
            let mut values = vec![];
            let mut k = 1;
            while k < a.len() {
                let next = a
                    .iter()
                    .zip(a[k..].iter())
                    .map(|a| B::op(&a.0, &a.1))
                    .collect::<Vec<_>>();
                values.push(a);
                a = next;
                k <<= 1;
            }
            values.push(a);
            Self { values }
        }

        pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> B::Band {
            let (l, r) = unzip(range, self.values.first().unwrap().len());
            let k = (r - l).ilog2() as usize;
            let table = &self.values[k];
            B::op(&table[l], &table[r - (1 << k)])
        }

        pub fn get_at(&self, i: usize) -> B::Band {
            self.values[0][i]
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
}
