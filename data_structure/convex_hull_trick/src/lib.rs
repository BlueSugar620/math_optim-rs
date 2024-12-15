use std::collections::BTreeMap;

pub trait Op {
    type Value: Clone + Copy + Ord;
    fn inf() -> Self::Value;
    fn add(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn inv(elm: &Self::Value) -> Self::Value;
    fn mul(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn intersect(
        lhs: (&Self::Value, &Self::Value),
        rhs: (&Self::Value, &Self::Value),
    ) -> Self::Value;
}

pub struct ConvexHullTrick<O: Op> {
    ab: BTreeMap<O::Value, O::Value>,
    ax: BTreeMap<O::Value, O::Value>,
    xa: BTreeMap<O::Value, O::Value>,
}

impl<O: Op> ConvexHullTrick<O> {
    pub fn new() -> Self {
        Self {
            ab: BTreeMap::new(),
            ax: BTreeMap::new(),
            xa: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, (a, b): (O::Value, O::Value)) {
        if self.ab.is_empty() {
            self.ab.insert(a, b);
            self.ax.insert(a, O::inf());
            self.xa.insert(O::inf(), a);
            return;
        }
        if self.skipped((a, b)) {
            return;
        }

        if self.ab.contains_key(&a) {
            self.ab.remove(&a);
            let x = self.ax.remove(&a);
            self.xa.remove(&x.unwrap());
        }
        let mut rm = vec![];
        for (ll, (&al, &bl)) in self.ab.range(a..).skip(1).zip(self.ab.range(a..)) {
            if O::intersect(ll, (&al, &bl)) >= O::intersect((&al, &bl), (&a, &b)) {
                rm.push(al);
            } else {
                break;
            }
        }
        for (rr, (&ar, &br)) in self
            .ab
            .range(..a)
            .rev()
            .skip(1)
            .zip(self.ab.range(..a).rev())
        {
            if O::intersect((&a, &b), (&ar, &br)) >= O::intersect((&ar, &br), rr) {
                rm.push(ar);
            } else {
                break;
            }
        }
        for ar in &rm {
            self.ab.remove(ar);
            let x = self.ax.remove(ar);
            self.xa.remove(&x.unwrap());
        }

        if let Some((al, bl)) = self.ab.range(a..).next() {
            let xr = self.ax.remove(al);
            self.xa.remove(&xr.unwrap());
            let x = O::intersect((&a, &b), (al, bl));
            self.ax.insert(*al, x);
            self.xa.insert(x, *al);
        }
        if let Some((ar, br)) = self.ab.range(..a).next_back() {
            let x = O::intersect((ar, br), (&a, &b));
            self.ax.insert(a, x);
            self.xa.insert(x, a);
        } else {
            self.ax.insert(a, O::inf());
            self.xa.insert(O::inf(), a);
        }
        self.ab.insert(a, b);
    }

    pub fn min(&self, x: O::Value) -> Option<O::Value> {
        if self.ab.len() != self.xa.len() {
            return Some(O::inf());
        }
        let a = *self.xa.range(x..).next()?.1;
        let b = *self.ab.get(&a).unwrap();
        return Some(O::add(&O::mul(&a, &x), &b));
    }

    fn skipped(&self, (a, b): (O::Value, O::Value)) -> bool {
        let l = match self.ab.range(a..).next() {
            Some((&al, &bl)) if al == a => return bl <= b,
            Some(ab) => ab,
            None => return false,
        };
        let r = match self.ab.range(..a).next_back() {
            Some(ab) => ab,
            None => return false,
        };
        O::intersect(l, (&a, &b)) >= O::intersect((&a, &b), r)
    }
}
