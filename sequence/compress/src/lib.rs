pub fn compress<T: Copy + PartialOrd>(a: &[T]) -> Vec<usize> {
    let mut val = a.to_vec();
    val.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    val.dedup();
    let mut res = Vec::with_capacity(a.len());
    for ai in a {
        res.push(val.partition_point(|x| x < ai));
    }
    res
}
