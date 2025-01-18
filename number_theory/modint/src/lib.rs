pub fn pow_mod(a: u64, mut n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut base = a;
    while n > 0 {
        if n & 1 == 1 {
            res *= base;
            res %= m;
        }
        base *= base;
        base %= m;
        n >>= 1;
    }
    res
}
