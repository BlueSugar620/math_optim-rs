pub fn diameter(e: &[(usize, usize, u64)]) -> (u64, Vec<usize>) {
    let n = e.len() + 1;
    let _e = e;
    let mut e = vec![Vec::new(); n];
    for &(u, v, w) in _e {
        e[u].push((v, w));
        e[v].push((u, w));
    }
    let mut d = vec![!0; n];
    d[0] = 0;
    let mut stack = vec![0];
    while let Some(u) = stack.pop() {
        for &(v, w) in &e[u] {
            if d[v] == !0 {
                d[v] = d[u] + w;
                stack.push(v);
            }
        }
    }
    let idx = (0..n).max_by_key(|&i| d[i]).unwrap();
    let mut dist = vec![!0; n];
    dist[idx] = 0;
    let mut prev = vec![None; n];
    let mut stack = vec![idx];
    while let Some(u) = stack.pop() {
        for &(v, w) in &e[u] {
            if dist[v] == !0 {
                dist[v] = dist[u] + w;
                prev[v] = Some(u);
                stack.push(v);
            }
        }
    }
    let idx = (0..n).max_by_key(|&i| dist[i]).unwrap();
    let mut path = vec![idx];
    while let Some(next) = prev[*path.last().unwrap()] {
        path.push(next);
    }
    path.reverse();
    (dist[idx], path)
}
