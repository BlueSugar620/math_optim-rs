use std::ops::{Add, Index, IndexMut, Neg, Sub};

use algebra::{Field, One, Zero};

#[derive(Clone)]
pub struct Matrix<T: Copy> {
    h: usize,
    w: usize,
    value: Box<[T]>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(h: usize, w: usize, a: &[T]) -> Self {
        Self {
            h,
            w,
            value: a.to_vec().into_boxed_slice(),
        }
    }

    pub fn new_from_2d(a: &Vec<Vec<T>>) -> Self {
        let h = a.len();
        let w = a[0].len();
        let mut value = Vec::with_capacity(h * w);
        for a in a.iter() {
            value.extend(a.iter());
        }
        Self {
            h,
            w,
            value: value.into_boxed_slice(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        self.value.chunks_exact(self.w)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.value.chunks_exact_mut(self.w)
    }
}

impl<T: Copy + Add<Output = T>> Matrix<T> {
    pub fn add(&self, rhs: Self) -> Self {
        let mut res = self.clone();
        for (res, rhs) in res.value.iter_mut().zip(rhs.value.iter()) {
            *res = *res + *rhs;
        }
        res
    }
}

impl<T: Copy + Sub<Output = T>> Matrix<T> {
    pub fn sub(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        for (res, rhs) in res.value.iter_mut().zip(rhs.value.iter()) {
            *res = *res - *rhs;
        }
        res
    }
}

impl<T: Copy + Neg<Output = T>> Matrix<T> {
    pub fn neg(&self) -> Self {
        Self {
            h: self.h,
            w: self.w,
            value: self.value.iter().map(|v| -*v).collect::<Box<_>>(),
        }
    }
}

impl<T: Copy + Zero + One> Matrix<T> {
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut value = Vec::with_capacity(self.h * rhs.w);
        for (res_row, lhs_row) in value.chunks_exact_mut(self.h).zip(self.iter()) {
            for (lhs_val, rhs_row) in lhs_row.iter().zip(rhs.iter()) {
                for (res, rhs_val) in res_row.iter_mut().zip(rhs_row.iter()) {
                    *res = *res + *lhs_val * *rhs_val;
                }
            }
        }
        Self {
            h: self.h,
            w: rhs.w,
            value: value.into_boxed_slice(),
        }
    }

    pub fn zero(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            value: vec![T::zero(); h * w].into_boxed_slice(),
        }
    }

    pub fn e(n: usize) -> Self {
        let mut value = vec![T::zero(); n * n];
        for i in 0..n {
            value[i * (n + 1)] = T::one();
        }
        Self {
            h: n,
            w: n,
            value: value.into_boxed_slice(),
        }
    }

    pub fn pow(&self, mut k: u64) -> Self {
        let n = self.h;
        let mut res = Self::e(n);
        let mut value = self.clone();
        while k > 0 {
            if k & 1 == 1 {
                res = Self::mul(&res, &value);
            }
            value = Self::mul(&value, &value);
            k >>= 1;
        }
        res
    }
}

impl<T: Copy + Field> Matrix<T> {
    pub fn determinant(&self) -> T {
        let n = self.h;
        let mut value = self.value.clone();
        let mut res = T::one();
        for i in (0..n).rev() {
            if let Some(k) = value
                .chunks_exact(n)
                .take(i + 1)
                .rposition(|v| !v[i].is_zero())
            {
                let (upper, lower) = value.split_at_mut(i * n);
                if k != i {
                    res = -res;
                    upper[k * n..(k + 1) * n].swap_with_slice(&mut lower[..n]);
                }
                res = res * lower[i];
                let inv = T::one() / lower[i];
                for r in lower[..i].iter_mut() {
                    *r = *r * inv;
                }
                for c in upper.chunks_exact_mut(n) {
                    let p = c[i];
                    for (v, r) in c.iter_mut().zip(lower.iter()) {
                        *v = *v - p * *r;
                    }
                }
            } else {
                return T::zero();
            }
        }
        res
    }

    pub fn gaussian_elimination(&mut self) -> usize {
        let h = self.h;
        let w = self.w;
        let mut x = 0;
        let mut tmp = Vec::with_capacity(w);
        for y in 0..w {
            if let Some(k) = (x..h).find(|k| !self[*k][y].is_zero()) {
                for j in 0..w {
                    self.value.swap(x * w + j, k * w + j);
                }
                let inv = T::one() / self[x][y];
                for v in self[x].iter_mut().skip(y) {
                    *v = *v * inv;
                }
                tmp.clear();
                tmp.extend_from_slice(&self[x]);
                for (i, r) in self.iter_mut().enumerate() {
                    if i == x {
                        continue;
                    }
                    let p = r[y];
                    for (v, tmp) in r[y..].iter_mut().zip(tmp[y..].iter()) {
                        *v = *v - p * *tmp;
                    }
                }
                x += 1;
            }
        }
        x
    }
}

impl<T: Copy> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index * self.w..(index + 1) * self.w]
    }
}

impl<T: Copy> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index * self.w..(index + 1) * self.w]
    }
}
