pub mod prime {
    pub fn is_prime(n: u64) -> bool {
        if n == 0 || n == 1 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        let r = (n - 1).trailing_zeros();
        let d = (n - 1) >> r;

        let miller_rabin = |a: u64| -> bool {
            let n = n as u128;
            let mut base = a as u128;
            let mut pow = 1;
            let mut d = d;
            while d > 0 {
                if d & 1 == 1 {
                    pow *= base;
                    pow %= n;
                }
                base *= base;
                base %= n;
                d >>= 1;
            }
            if pow == 1 || pow == n - 1 {
                return true;
            }
            for _ in 1..r {
                pow *= pow;
                pow %= n;
                if pow == n - 1 {
                    return true;
                }
            }
            false
        };

        let a = if n < 4_759_123_141 {
            vec![2, 7, 61]
        } else {
            vec![2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022]
        };

        a.iter().filter(|&&a| a < n).all(|&a| miller_rabin(a))
    }

    pub fn prime_counting(n: usize) -> usize {
        let m = (1..).find(|i| i * i > n).unwrap() - 1;
        let mut linear = (0..=m)
            .map(|i| if i == 0 { 0 } else { i - 1 })
            .collect::<Vec<_>>();
        let mut inverse = (0..=m)
            .map(|i| if i == 0 { 0 } else { n / i - 1 })
            .collect::<Vec<_>>();
        let mut skip = vec![false; m + 1];
        let mut pos = (1..=m).collect::<Vec<_>>();
        for p in 2..=m {
            if skip[p] {
                continue;
            }
            skip[p] = true;
            for j in ((p * p)..=m).step_by(p) {
                skip[j] = true;
            }
            let q = n / p;
            let qq = q / p;
            pos.retain(|x| *x <= qq && !skip[*x]);
            let a = linear[p - 1];
            for &x in pos.iter() {
                inverse[x] -= if x * p <= m {
                    inverse[x * p]
                } else {
                    linear[q / x]
                } - a;
            }
            for j in (p..(m / p + 1)).rev() {
                let b = linear[j] - a;
                let s = j * p;
                let t = (s + p).min(m + 1);
                linear[s..t].iter_mut().for_each(|v| *v -= b);
            }
        }
        inverse[1]
    }
}
