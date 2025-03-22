pub fn levenshtein<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let mut dp = vec![!0; b.len() + 1];
    for j in 0..=b.len() {
        dp[j] = j;
    }
    for (i, a) in a.iter().enumerate() {
        let mut ndp = vec![!0; b.len() + 1];
        ndp[0] = i + 1;
        for (j, b) in b.iter().enumerate() {
            ndp[j + 1] = (dp[j + 1] + 1)
                .min(ndp[j] + 1)
                .min(dp[j] + if a == b { 0 } else { 1 });
        }
        dp = ndp;
    }
    dp[b.len()]
}
