use ahash::AHashMap;

#[derive(Debug)]
struct Node<T> {
    children: AHashMap<String, Node<T>>,
    val: Option<T>,
}
impl <T> Node<T> {
    pub fn new() -> Self {
        Node {
            children: AHashMap::new(),
            val: None
        }
    }
}
#[derive(Debug)]
pub struct WordTrie<T> {
    root: Node<T>,
}
fn moving<T>(t: T) -> T { t }
impl <T> WordTrie<T>{
    /// Creates a new trie
    pub fn new() -> Self {
        WordTrie { root: Node::new() }
    }
    /// insert a phrase into the trie
    pub fn insert(&mut self, key: String, val: T) {
        let mut node = &mut self.root;
        for word in key.split_whitespace() {
            node = moving(node).children
                .entry(word.to_string())
                .or_insert(Node::new());
            node.children.shrink_to_fit();
        }
        node.val = Some(val);
    }
    /// search for a phrase in the trie
    pub fn get(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;
        for word in key.split_whitespace() {
            match node.children.get(&word.to_string()) {
                None => {
                    return None;
                },
                Some(val) => {
                    node = val;
                }
            }
        }
        match &node.val {
            None => { return None; },
            Some(val) => {
                return Some(&val);
            }
        }
    }
}
