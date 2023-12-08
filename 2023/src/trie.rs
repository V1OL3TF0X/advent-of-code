use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode<T> {
    end_value: Option<T>,
    children: HashMap<char, TrieNode<T>>,
}

impl<T> Default for TrieNode<T> {
    fn default() -> Self {
        Self {
            end_value: None,
            children: Default::default(),
        }
    }
}

impl<T: Copy> TrieNode<T> {
    pub fn has_child(&self, key: &char) -> Option<&TrieNode<T>> {
        self.children.get(key)
    }

    pub fn get_value(&self) -> Option<T> {
        self.end_value
    }
}

#[derive(Default, Debug)]
pub struct Trie<T> {
    root: TrieNode<T>,
}

impl<T: Copy> Trie<T> {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }
    pub fn has_branch(&self, key: &char) -> Option<&TrieNode<T>> {
        self.root.children.get(key)
    }

    pub fn insert(&mut self, word: &str, value: T) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.end_value = Some(value);
    }
}

impl<T: Sized + Copy> From<Vec<(&str, T)>> for Trie<T> {
    fn from(value: Vec<(&str, T)>) -> Self {
        let mut t = Trie::new();
        value
            .iter()
            .for_each(|(word, value)| t.insert(word, *value));
        t
    }
}
