pub mod quotients {
    pub fn enumerate_quotients(n: u64) -> Vec<u64> {
        let mut prefix = vec![];
        let mut suffix = vec![];
        for i in (1..).take_while(|i| i * i <= n) {
            prefix.push(i);
            if i == n / i {
                break;
            }
            suffix.push(n / i);
        }
        prefix.extend(suffix.iter().rev());
        prefix
    }

    pub fn enumerate_quotients_with_range(n: usize) -> Vec<(usize, (usize, usize))> {
        let mut prefix = vec![];
        let mut suffix = vec![];
        for i in (1..).take_while(|i| i * i <= n) {
            prefix.push((n / i, (n / (n / i + 1), n / (n / i))));
            if i == n / i {
                break;
            }
            suffix.push((i, (n / (i + 1), n / i)));
        }
        prefix.extend(suffix.iter().rev());
        prefix
    }
}
