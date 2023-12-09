#[derive(Debug)]
pub struct TrieNode<T> {
    end_value: Option<T>,
    children: Vec<(char, TrieNode<T>)>,
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
        self.children
            .iter()
            .find(|(k, _v)| k == key)
            .map(|(_k, v)| v)
    }

    pub fn get_or_insert_new(&mut self, key: char) -> &mut TrieNode<T> {
        let pos = self.children.iter_mut().position(|(k, _v)| k == &key);
        if let Some(i) = pos {
            &mut self.children[i].1
        } else {
            let n = TrieNode::default();
            self.children.push((key, n));
            &mut self.children.last_mut().unwrap().1
        }
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
        self.root.has_child(key)
    }

    pub fn insert(&mut self, word: &str, value: T) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.get_or_insert_new(c);
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
