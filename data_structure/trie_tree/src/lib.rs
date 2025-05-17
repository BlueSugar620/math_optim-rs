pub struct TrieNode<K, V> {
    value: V,
    children: std::collections::BTreeMap<K, TrieNode<K, V>>,
}

pub struct TrieTree<K, V> {
    tree: TrieNode<K, V>,
}

impl<K, V> TrieTree<K, V> {}
