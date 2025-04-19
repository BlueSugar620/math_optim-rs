use itertools::*;
use memoise::*;
use num::integer::{nth_root, sqrt};
use proconio::marker::*;
pub fn diameter(e: &[(usize, usize)]) -> (u64, (usize, usize)) {
    let n = e.len() + 1;
    let _e = e;
    let mut e = vec![Vec::new(); n];
    for &(u, v) in _e {
        e[u].push(v);
        e[v].push(u);
    }
    let mut dist0 = vec![!0; n];
    dist0[0] = 0;
    let mut stack0 = vec![0];
    while let Some(u) = stack0.pop() {
        for &v in &e[u] {
            if dist0[v] == !0 {
                dist0[v] = dist0[u] + 1;
                stack0.push(v);
            }
        }
    }
    let idx0 = (0..n).max_by_key(|&i| dist0[i]).unwrap();
    let mut dist1 = vec![!0; n];
    dist1[idx0] = 0;
    let mut stack1 = vec![idx0];
    while let Some(u) = stack1.pop() {
        for &v in &e[u] {
            if dist1[v] == !0 {
                dist1[v] = dist1[u] + 1;
                stack1.push(v);
            }
        }
    }
    let idx1 = (0..n).max_by_key(|&i| dist1[i]).unwrap();
    (dist1[idx1], (idx0, idx1))
}
