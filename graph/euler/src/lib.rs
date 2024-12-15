pub mod directed {
    pub fn hierholzer(n: usize, e: &[(usize, usize)]) -> Option<(Vec<usize>, Vec<usize>)> {
        if e.is_empty() {
            return Some((vec![0], vec![]));
        }
        let edge_cnt = e.len();
        let _e = e;
        let mut e = vec![Vec::new(); n];
        let mut deg = vec![0i32; n];
        for (i, &(u, v)) in _e.iter().enumerate() {
            e[u].push((v, i));
            deg[u] += 1;
            deg[v] -= 1;
        }
        if deg.iter().map(|deg| deg.abs()).sum::<i32>() > 2 {
            return None;
        }
        let s = (0..n).max_by_key(|&v| (deg[v], e[v].len())).unwrap();

        let mut vertex = vec![];
        let mut edge = vec![];
        while let Some((v0, k0)) = e[s].pop() {
            dfs((v0, k0), &mut e, &mut vertex, &mut edge);
        }

        if edge.len() != edge_cnt {
            return None;
        }
        vertex.push(s);
        vertex.reverse();
        edge.reverse();
        Some((vertex, edge))
    }

    fn dfs(
        (u, k): (usize, usize),
        e: &mut Vec<Vec<(usize, usize)>>,
        vertex: &mut Vec<usize>,
        edge: &mut Vec<usize>,
    ) {
        while let Some((v, l)) = e[u].pop() {
            dfs((v, l), e, vertex, edge);
        }
        vertex.push(u);
        edge.push(k);
    }
}

pub mod undirected {
    pub fn hierholzer(n: usize, e: &[(usize, usize)]) -> Option<(Vec<usize>, Vec<usize>)> {
        if e.is_empty() {
            return Some((vec![0], vec![]));
        }
        let edge_cnt = e.len();
        let _e = e;
        let mut e = vec![Vec::new(); n];
        for (i, &(u, v)) in _e.iter().enumerate() {
            e[u].push((v, i));
            e[v].push((u, i));
        }
        if e.iter().map(|e| e.len() % 2).sum::<usize>() > 2 {
            return None;
        }
        let s = (0..n)
            .max_by_key(|&v| (e[v].len() % 2, e[v].len()))
            .unwrap();

        let mut vertex = vec![];
        let mut edge = vec![];
        let mut dfs = vec![];
        let mut flag = vec![false; edge_cnt];
        let mut u = s;
        loop {
            if let Some((v, l)) = e[u].pop() {
                if !flag[l] {
                    flag[l] = true;
                    dfs.push((u, l));
                    u = v;
                }
            } else if let Some((w, j)) = dfs.pop() {
                vertex.push(u);
                edge.push(j);
                u = w;
            } else {
                break;
            }
        }

        if edge.len() != edge_cnt {
            return None;
        }
        vertex.push(s);
        vertex.reverse();
        edge.reverse();
        Some((vertex, edge))
    }
}
