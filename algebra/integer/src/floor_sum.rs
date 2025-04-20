pub fn floor_sum(mut n: i64, mut m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut res = 0;
    while m > 0 {
        if a >= m {
            res += a / m * n * (n - 1) / 2;
            a %= m;
        }
        if b >= m {
            res += b / m * n;
            b %= m;
        }
        let k = a * n + b;
        if k < m {
            break;
        }
        n = k / m;
        b = k % m;
        std::mem::swap(&mut a, &mut m);
    }
    res
}
