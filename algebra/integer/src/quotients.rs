pub fn quotients(n: u64) -> Vec<u64> {
    let mut prefix = vec![];
    let mut suffix = vec![];
    for i in (1..).take_while(|i| i * i <= n) {
        prefix.push(i);
        if n / i != i {
            suffix.push(n / i);
        }
    }
    prefix.extend(suffix.iter().rev());
    prefix
}
