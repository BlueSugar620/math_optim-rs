pub struct LCA {
    idx: Vec<usize>,
    sparse_table: Vec<Vec<(usize, usize)>>,
    dist: Vec<usize>,
}

impl LCA {
    pub fn new(r: usize, e: &[(usize, usize)]) -> Self {
        let n = e.len() + 1;
        let _e = e;
        let mut e = vec![vec![]; n];
        for &(u, v) in _e {
            e[u].push(v);
            e[v].push(u);
        }
        let mut dist = vec![!0; n];
        let mut eular_tour = Vec::with_capacity(2 * n - 1);
        Self::dfs(r, 0, &e, &mut eular_tour, &mut dist);
        let mut idx = vec![!0; n];
        for (i, &(_, u)) in eular_tour.iter().enumerate() {
            if idx[u] == !0 {
                idx[u] = i;
            }
        }
        let mut sparse_table = vec![eular_tour];
        let mut i = 1;
        while i < n {
            let prev = sparse_table.last().unwrap();
            let crnt = prev
                .iter()
                .zip(&prev[i..])
                .map(|(x, y)| *x.min(y))
                .collect::<Vec<_>>();
            sparse_table.push(crnt);
            i <<= 1;
        }
        Self {
            idx,
            sparse_table,
            dist,
        }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        let mut pu = self.idx[u];
        let mut pv = self.idx[v];
        if pu > pv {
            std::mem::swap(&mut pu, &mut pv);
        }
        let x = (pv + 1 - pu).ilog2() as usize;
        self.sparse_table[x][pu]
            .min(self.sparse_table[x][pv + 1 - (1 << x)])
            .1
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        let mut pu = self.idx[u];
        let mut pv = self.idx[v];
        if pu > pv {
            std::mem::swap(&mut pu, &mut pv);
        }
        let x = (pv + 1 - pu).ilog2() as usize;
        let lca_dist = self.sparse_table[x][pu]
            .min(self.sparse_table[x][pv + 1 - (1 << x)])
            .0;
        self.dist[u] + self.dist[v] - 2 * lca_dist
    }

    fn dfs(
        u: usize,
        d: usize,
        e: &[Vec<usize>],
        eular_tour: &mut Vec<(usize, usize)>,
        dist: &mut [usize],
    ) {
        dist[u] = d;
        for &v in &e[u] {
            if dist[v] == !0 {
                eular_tour.push((d, u));
                Self::dfs(v, d + 1, e, eular_tour, dist);
            }
        }
        eular_tour.push((d, u));
    }
}

pub struct LCAwithDoubling {
    doubling: Vec<Vec<usize>>,
    dist: Vec<usize>,
}

impl LCAwithDoubling {
    pub fn new(r: usize, e: &[(usize, usize)]) -> Self {
        let n = e.len() + 1;
        let _e = e;
        let mut e = vec![vec![]; n];
        for &(u, v) in _e {
            e[u].push(v);
            e[v].push(u);
        }
        let mut parent = vec![!0; n];
        let mut dist = vec![!0; n];
        let mut stack = vec![r];
        parent[r] = r;
        dist[r] = 0;
        while let Some(u) = stack.pop() {
            for &v in &e[u] {
                if dist[v] == !0 {
                    parent[v] = u;
                    dist[v] = dist[u] + 1;
                    stack.push(v);
                }
            }
        }
        let mut doubling = vec![parent];
        let mut i = 1;
        while i < n {
            let prev = doubling.last().unwrap();
            let mut crnt = Vec::with_capacity(n);
            for u in 0..n {
                crnt.push(prev[prev[u]]);
            }
            doubling.push(crnt);
            i <<= 1;
        }
        Self { doubling, dist }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.dist[u] > self.dist[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let d = self.dist[v] - self.dist[u];
        for (i, parent) in self.doubling.iter().enumerate() {
            if (d >> i) & 1 == 1 {
                v = parent[v];
            }
        }
        if u == v {
            return u;
        }
        for parent in self.doubling.iter().rev() {
            if parent[u] == parent[v] {
                continue;
            }
            u = parent[u];
            v = parent[v];
        }
        self.doubling[0][u]
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        let lca = self.lca(u, v);
        self.dist[u] + self.dist[v] - 2 * self.dist[lca]
    }
}
