pub fn furthest_pair(p: &[(f64, f64)]) -> (f64, (usize, usize)) {
    let p = convex_hull(p);
    if p.len() == 1 {
        (0., (p[0].1, p[0].1))
    } else if p.len() == 2 {
        (
            (p[0].0 .0 - p[1].0 .0).hypot(p[0].0 .1 - p[1].0 .1),
            (p[0].1, p[1].1),
        )
    } else {
        let n = p.len();
        let mut i_start = (0..n)
            .min_by(|x, y| p[*x].0 .0.partial_cmp(&p[*y].0 .0).unwrap())
            .unwrap();
        let mut j_start = (0..n)
            .max_by(|x, y| p[*x].0 .0.partial_cmp(&p[*y].0 .0).unwrap())
            .unwrap();
        if i_start > j_start {
            std::mem::swap(&mut i_start, &mut j_start);
        }
        let mut i = i_start;
        let mut j = j_start;
        let mut dist = (p[i].0 .0 - p[j].0 .0).hypot(p[i].0 .1 - p[j].0 .1);
        let mut pair = (i, j);
        while i != i_start + n && j != j_start + n {
            if (p[i % n].0 .0 - p[(i + 1) % n].0 .0) * (p[j % n].0 .1 - p[(j + 1) % n].0 .1)
                > (p[j % n].0 .0 - p[(j + 1) % n].0 .0) * (p[i % n].0 .1 - p[(i + 1) % n].0 .1)
            {
                i += 1;
            } else {
                j += 1;
            }
            let d = (p[i % n].0 .0 - p[j % n].0 .0).hypot(p[i % n].0 .1 - p[j % n].0 .1);
            if d > dist {
                dist = d;
                pair = (i % n, j % n);
            }
        }
        (dist, (p[pair.0].1, p[pair.1].1))
    }
}

fn convex_hull(p: &[(f64, f64)]) -> Vec<((f64, f64), usize)> {
    let mut p = p
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<Vec<_>>();
    p.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
    p.dedup();
    if p.is_empty() {
        return Vec::new();
    } else if p.len() == 1 {
        return vec![p[0]];
    } else if p.len() == 2 {
        return vec![p[0], p[1]];
    } else {
        let mut res: Vec<((f64, f64), usize)> = Vec::new();
        for p in p.iter() {
            while res.len() > 1 && check(&res[res.len() - 2].0, &res[res.len() - 1].0, &p.0) {
                res.pop();
            }
            res.push(*p);
        }
        let len = res.len();
        for p in p.iter().rev().skip(1) {
            while len < res.len() && check(&res[res.len() - 2].0, &res[res.len() - 1].0, &p.0) {
                res.pop();
            }
            res.push(*p);
        }
        res.pop();
        res
    }
}

pub fn check(a: &(f64, f64), b: &(f64, f64), c: &(f64, f64)) -> bool {
    (b.0 - a.0) * (c.1 - a.1) - (c.0 - a.0) * (b.1 - a.1) >= 0.
}
