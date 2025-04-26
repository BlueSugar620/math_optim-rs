pub struct FunctionalGraph {
    _n: usize,
    _to: Vec<usize>,
    components: Vec<(Vec<usize>, Vec<usize>)>,
}

impl FunctionalGraph {
    pub fn from_edges(n: usize, e: &[(usize, usize)]) -> Self {
        let mut underlying = vec![vec![]; n];
        for &(u, v) in e {
            underlying[u].push(v);
            underlying[v].push(u);
        }
        let mut connected = vec![];
        let mut flag = vec![false; n];
        for i in 0..n {
            if !flag[i] {
                let mut comp = vec![i];
                let mut stack = vec![i];
                while let Some(u) = stack.pop() {
                    for &v in &underlying[u] {
                        if !flag[v] {
                            comp.push(v);
                            stack.push(v);
                            flag[v] = true;
                        }
                    }
                }
                connected.push(comp);
            }
        }
        let mut to = vec![0; n];
        for &(u, v) in e {
            to[u] = v;
        }
        let mut in_degree = vec![0; n];
        for &(_, v) in e {
            in_degree[v] += 1;
        }
        let mut components = vec![];
        let mut flag = vec![false; n];
        for connected in &connected {
            let mut stack = vec![];
            let mut comp = vec![];
            for &u in connected {
                if in_degree[u] == 0 {
                    stack.push(u);
                    comp.push(u);
                    flag[u] = true;
                }
            }
            while let Some(u) = stack.pop() {
                let v = to[u];
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    stack.push(v);
                    comp.push(v);
                    flag[v] = true;
                }
            }
            let mut p = *comp.last().unwrap_or(&connected[0]);
            let mut cycle = vec![p];
            while to[p] != cycle[0] {
                p = to[p];
                cycle.push(p);
            }
            components.push((comp, cycle));
        }
        Self {
            _n: n,
            _to: to,
            components,
        }
    }

    pub fn from_to(n: usize, to: &[usize]) -> Self {
        let mut underlying = vec![vec![]; n];
        for (i, &t) in to.iter().enumerate() {
            underlying[i].push(t);
            underlying[t].push(i);
        }
        let mut connected = vec![];
        let mut flag = vec![false; n];
        for i in 0..n {
            if !flag[i] {
                let mut comp = vec![i];
                let mut stack = vec![i];
                flag[i] = true;
                while let Some(u) = stack.pop() {
                    for &v in &underlying[u] {
                        if !flag[v] {
                            comp.push(v);
                            stack.push(v);
                            flag[v] = true;
                        }
                    }
                }
                connected.push(comp);
            }
        }
        let mut in_degree = vec![0; n];
        for &t in to.iter() {
            in_degree[t] += 1;
        }
        let mut components = vec![];
        let mut flag = vec![false; n];
        for connected in &connected {
            let mut stack = vec![];
            let mut comp = vec![];
            for &u in connected {
                if in_degree[u] == 0 {
                    stack.push(u);
                    comp.push(u);
                    flag[u] = true;
                }
            }
            while let Some(u) = stack.pop() {
                let v = to[u];
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    stack.push(v);
                    comp.push(v);
                    flag[v] = true;
                }
            }
            let mut p = to[*comp.last().unwrap_or(&connected[0])];
            let mut cycle = vec![p];
            flag[p] = true;
            while !flag[to[p]] {
                p = to[p];
                cycle.push(p);
                flag[p] = true;
            }
            components.push((comp, cycle));
        }
        Self {
            _n: n,
            _to: to.to_vec(),
            components,
        }
    }

    pub fn components(&self) -> Vec<(Vec<usize>, Vec<usize>)> {
        self.components.clone()
    }
}
