pub trait Op {
    type Value;
    fn zero() -> Self::Value;
    fn one() -> Self::Value;
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

pub struct PascalTriangle<O: Op> {
    values: Vec<Vec<O::Value>>,
}

impl<O: Op> PascalTriangle<O>
where
    O::Value: Copy,
{
    pub fn new(size: usize) -> Self {
        let mut values = vec![vec![O::one()]];
        for n in 1..=size {
            let mut tmp = vec![];
            for r in 0..=n {
                if r == 0 {
                    tmp.push(O::add(&O::zero(), &values.last().unwrap()[0]));
                } else if r == n {
                    tmp.push(O::add(&values.last().unwrap()[r - 1], &O::zero()));
                } else {
                    tmp.push(O::add(
                        &values.last().unwrap()[r - 1],
                        &values.last().unwrap()[r],
                    ));
                }
            }
            values.push(tmp);
        }
        Self { values }
    }

    pub fn binom(&self, n: usize, r: usize) -> O::Value {
        self.values[n][r]
    }

    pub fn hom(&self, n: usize, r: usize) -> O::Value {
        self.values[n + r - 1][r]
    }
}
