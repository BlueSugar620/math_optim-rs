pub fn divisors(n: u64) -> Vec<u64> {
    let mut prefix = vec![];
    let mut suffix = vec![];
    for i in (1..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            prefix.push(i);
            if i * i != n {
                suffix.push(n / i);
            }
        }
    }
    prefix.extend(suffix.iter().rev());
    prefix
}
