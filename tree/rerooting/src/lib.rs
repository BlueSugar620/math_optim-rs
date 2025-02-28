pub trait ValMonoid {
    type Monoid: Copy;
    fn e() -> Self::Monoid;
    fn op(lhs: &Self::Monoid, rhs: &Self::Monoid) -> Self::Monoid;

    type Value: Copy;
    type Edge: Copy;
    type Vertex: Copy;
    fn monoidize(value: &Self::Value, edge: &Self::Edge) -> Self::Monoid;
    fn valueize(monoid: &Self::Monoid, vertex: &Self::Vertex) -> Self::Value;
}

pub fn rerooting<T: ValMonoid>(
    n: usize,
    vertex: &[T::Vertex],
    edge: &[(usize, usize, T::Edge, T::Edge)],
) -> Vec<T::Value> {
    if n == 1 {
        return vec![T::valueize(&T::e(), &vertex[0])];
    }
    let mut child = vec![vec![]; n];
    for &(u, v, u2v, v2u) in edge {
        child[u].push((v, u2v));
        child[v].push((u, v2u));
    }
    let mut sorted = vec![];
    let sample_edge = edge[0].2;
    let mut parent = vec![(!0, sample_edge); n];
    parent[0] = (0, sample_edge);

    let mut stack = vec![0];
    while let Some(u) = stack.pop() {
        sorted.push(u);
        child[u].retain(|v| parent[u].0 != v.0);
        for &(v, e) in &child[u] {
            parent[v] = (u, e);
            stack.push(v);
        }
    }

    let mut lower = vec![T::e(); n];
    let mut upper = vec![T::e(); n];
    let sample_value = T::valueize(&T::e(), &vertex[0]);
    let mut partial = vec![sample_value; n];
    for &u in sorted.iter().rev() {
        lower[u] = child[u]
            .iter()
            .map(|(v, e)| T::monoidize(&partial[*v], e))
            .fold(lower[u], |acc, a| T::op(&acc, &a));
        partial[u] = T::valueize(&lower[u], &vertex[u]);
    }

    for &u in &sorted {
        let (p, pe) = parent[u];
        let mut suffix = if u == p {
            T::e()
        } else {
            T::monoidize(&T::valueize(&upper[u], &vertex[p]), &pe)
        };
        for &(v, e) in child[u].iter().rev() {
            upper[v] = T::op(&upper[v], &suffix);
            suffix = T::op(&suffix, &T::monoidize(&partial[v], &e));
        }
        let mut prefix = T::e();
        for &(v, e) in &child[u] {
            upper[v] = T::op(&upper[v], &prefix);
            prefix = T::op(&prefix, &T::monoidize(&partial[v], &e));
        }
    }

    (0..n)
        .map(|i| {
            let (p, e) = parent[i];
            let x = if i == p {
                T::e()
            } else {
                T::monoidize(&T::valueize(&upper[i], &vertex[p]), &e)
            };
            T::valueize(&T::op(&lower[i], &x), &vertex[i])
        })
        .collect::<Vec<_>>()
}
