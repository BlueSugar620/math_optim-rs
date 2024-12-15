pub fn closest_pair(p: &[(f64, f64)]) -> (f64, (usize, usize)) {
    let mut p = p
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<Vec<_>>();
    p.sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());
    rec(&mut p.clone())
}

fn rec(p: &mut [((f64, f64), usize)]) -> (f64, (usize, usize)) {
    let n = p.len();
    if n < 2 {
        return (std::f64::MAX, (0, 0));
    }
    let m = n / 2;
    let t = p[m].0 .0;
    let l = rec(&mut p[..m]);
    let r = rec(&mut p[m..]);
    let mut dist;
    let mut pair;
    if l.0 < r.0 {
        (dist, pair) = l;
    } else {
        (dist, pair) = r;
    }
    inplace_merge(p, 0, m, n);
    let mut boundary = vec![];
    for &((x, y), i) in p.iter() {
        if (t - x).abs() >= dist {
            continue;
        }
        for &((z, w), j) in boundary.iter().rev() {
            if y - w >= dist {
                break;
            }
            let d = ((x - z) as f64).hypot(y - w);
            if d < dist {
                dist = d;
                pair = (i, j);
            }
        }
        boundary.push(((x, y), i));
    }
    (dist, pair)
}

fn inplace_merge(p: &mut [((f64, f64), usize)], x: usize, y: usize, z: usize) {
    let mut res = vec![];
    let mut l = x;
    let mut r = y;
    while l < y || r < z {
        if l == y {
            res.push(p[r]);
            r += 1;
        } else if r == z {
            res.push(p[l]);
            l += 1;
        } else {
            if p[l].0 .1 < p[r].0 .1 {
                res.push(p[l]);
                l += 1;
            } else {
                res.push(p[r]);
                r += 1;
            }
        }
    }
    p.clone_from_slice(&res);
}
