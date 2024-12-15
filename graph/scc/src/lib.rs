pub fn scc(n: usize, e: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let _e = e;
    let mut e = vec![Vec::new(); n];
    let mut re = vec![Vec::new(); n];
    for &(v, w) in _e {
        e[v].push(w);
        re[w].push(v);
    }

    let mut flag1 = vec![false; n];
    let mut ord = Vec::with_capacity(n);
    for i in 0..n {
        if !flag1[i] {
            dfs1(i, &e, &mut flag1, &mut ord);
        }
    }

    let mut flag2 = vec![false; n];
    let mut comp = Vec::new();
    for &j in ord.iter().rev() {
        if !flag2[j] {
            let mut cmp_of = Vec::new();
            dfs2(j, &re, &mut flag2, &mut cmp_of);
            comp.push(cmp_of);
        }
    }

    comp
}

fn dfs1(i: usize, e: &Vec<Vec<usize>>, flag1: &mut [bool], ord: &mut Vec<usize>) {
    flag1[i] = true;
    for &j in &e[i] {
        if !flag1[j] {
            dfs1(j, e, flag1, ord);
        }
    }
    ord.push(i);
}

fn dfs2(j: usize, re: &Vec<Vec<usize>>, flag2: &mut [bool], cmp_of: &mut Vec<usize>) {
    flag2[j] = true;
    for &i in &re[j] {
        if !flag2[i] {
            dfs2(i, re, flag2, cmp_of);
        }
    }
    cmp_of.push(j);
}
