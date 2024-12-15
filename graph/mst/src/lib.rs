pub fn kruskal(n: usize, e: &[(usize, usize, i64)]) -> Option<(i64, Vec<usize>)> {
    use unionfind::Unionfind;
    let mut e = e.iter().copied().enumerate().collect::<Vec<_>>();
    e.sort_unstable_by_key(|(_, (_, _, c))| *c);

    let mut uf = Unionfind::new(n);
    let mut edge = vec![];
    let mut cost = 0;

    for &(i, (u, v, c)) in &e {
        if !uf.is_same(u, v) {
            uf.unite(u, v);
            edge.push(i);
            cost += c;
        }
    }

    if edge.len() < n - 1 {
        return None;
    } else {
        return Some((cost, edge));
    }
}

pub fn prim(n: usize, e: &[(usize, usize, i64)]) -> Option<(i64, Vec<usize>)> {
    let _e = e;
    let mut e = vec![Vec::new(); n];
    for (i, &(u, v, c)) in _e.iter().enumerate() {
        e[u].push((v, c, i));
        e[v].push((u, c, i));
    }

    let mut heap = std::collections::BinaryHeap::new();
    let mut cost = 0;
    let mut edge = Vec::new();
    let mut flag = vec![false; n];

    for &(v, c, i) in &e[0] {
        heap.push((!c, v, i));
    }
    flag[0] = true;

    while let Some((c, u, k)) = heap.pop() {
        let c = !c;
        if flag[u] {
            continue;
        }
        cost += c;
        edge.push(k);
        flag[u] = true;
        for &(v, c, k) in &e[u] {
            if !flag[v] {
                heap.push((!c, v, k));
            }
        }
    }

    if edge.len() < n - 1 {
        return None;
    } else {
        return Some((cost, edge));
    }
}
