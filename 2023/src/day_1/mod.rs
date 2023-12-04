use crate::trie::Trie;
use once_cell::sync::Lazy;
pub fn task_1(file: &str) -> String {
    let sum: u32 = file.lines().map(combine_first_last_digit).sum();
    sum.to_string()
}

fn combine_first_last_digit(line: &str) -> u32 {
    let first = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|d| d.to_digit(10))
        .unwrap();
    let last = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .and_then(|d| d.to_digit(10))
        .unwrap();
    first * 10 + last
}

static NUMBER_TRIE: Lazy<Trie<u32>> = Lazy::new(|| {
    Trie::from(vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

static REV_NUMBER_TRIE: Lazy<Trie<u32>> = Lazy::new(|| {
    Trie::from(vec![
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ])
});

pub fn task_2(file: &str) -> String {
    let sum: u32 = file.lines().map(find_first_last_digit_or_word).sum();
    sum.to_string()
}

fn find_first_last_digit_or_word(line: &str) -> u32 {
    let first = find_first(line.chars(), &NUMBER_TRIE);
    let last = find_first(line.chars().rev(), &REV_NUMBER_TRIE);
    let v = first * 10 + last;
    println!("{line} {v}");
    v
}

fn find_first(chars: impl Iterator<Item = char>, trie: &Lazy<Trie<u32>>) -> u32 {
    let mut candidates = vec![];
    for c in chars {
        let mut found = u32::MAX;
        if let Some(v) = c.to_digit(10) {
            return v;
        }
        candidates.retain_mut(|candidate: &mut &crate::trie::TrieNode<u32>| {
            if found != u32::MAX {
                return false;
            }
            if let Some(child) = candidate.has_child(&c) {
                *candidate = child;
                if let Some(v) = candidate.get_value() {
                    found = v;
                    return false;
                }
                true
            } else {
                false
            }
        });
        if found != u32::MAX {
            return found;
        }
        if let Some(b) = trie.has_branch(&c) {
            candidates.push(b);
            if let Some(v) = b.get_value() {
                return v;
            }
        }
    }
    unreachable!()
}
