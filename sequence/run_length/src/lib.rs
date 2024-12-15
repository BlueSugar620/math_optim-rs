pub mod run_length {
    pub fn run_length_enconding<T: Copy + Eq>(a: &[T]) -> Vec<(T, usize)> {
        let mut a = a.iter().copied().map(|a| (a, 1)).collect::<Vec<_>>();
        a.dedup_by(|a, b| {
            a.0 == b.0 && {
                b.1 += a.1;
                true
            }
        });
        a
    }

    pub fn run_length_decoding<T: Copy + Eq>(a: &[(T, usize)]) -> Vec<T> {
        a.iter()
            .map(|a| std::iter::repeat(a.0).take(a.1))
            .flatten()
            .collect::<Vec<_>>()
    }
}
