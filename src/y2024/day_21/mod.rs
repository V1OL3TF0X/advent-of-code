use crate::task_fns::SolveMode;
use core::panic;
use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    iter::once,
};

use cached::proc_macro::{cached, once};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum KeypadType {
    Numeric,
    Arrows,
}

struct Keypad(FxHashMap<Key, Vec<(Key, Key)>>);

impl FromIterator<(Key, Vec<(Key, Key)>)> for Keypad {
    fn from_iter<T: IntoIterator<Item = (Key, Vec<(Key, Key)>)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Keypad {
    fn make_shortest_paths(self) -> FxHashMap<(Key, Key), Vec<Vec<Key>>> {
        self.0
            .keys()
            .cartesian_product(self.0.keys())
            .map(|(&a, &b)| ((a, b), shortest_paths(&self.0, a, b)))
            .collect()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
#[rustfmt::skip]
enum Key { UpArrow, DownArrow, LeftArrow, RightArrow, Accept, Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine }

impl From<char> for Key {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::UpArrow,
            'v' => Self::DownArrow,
            '<' => Self::LeftArrow,
            '>' => Self::RightArrow,
            'A' => Self::Accept,
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            v => panic!("{v} is not a proper key"),
        }
    }
}
impl From<&Key> for char {
    fn from(value: &Key) -> Self {
        match value {
            Key::UpArrow => '^',
            Key::DownArrow => 'v',
            Key::LeftArrow => '<',
            Key::RightArrow => '>',
            Key::Accept => 'A',
            Key::Zero => '0',
            Key::One => '1',
            Key::Two => '2',
            Key::Three => '3',
            Key::Four => '4',
            Key::Five => '5',
            Key::Six => '6',
            Key::Seven => '7',
            Key::Eight => '8',
            Key::Nine => '9',
        }
    }
}
impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}
impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[once]
#[rustfmt::skip]
fn numeric_paths() -> FxHashMap<(Key, Key), Vec<Vec<Key>>> {
    Keypad::from_iter([
        (Key::Seven, vec![(Key::Four, Key::DownArrow), (Key::Eight, Key::RightArrow)]),
        (Key::Eight, vec![ (Key::Five, Key::DownArrow), (Key::Nine, Key::RightArrow), (Key::Seven, Key::LeftArrow)]),
        (Key::Nine, vec![(Key::Six, Key::DownArrow), (Key::Eight, Key::LeftArrow)]),
        (Key::Four, vec![ (Key::One, Key::DownArrow), (Key::Five, Key::RightArrow), (Key::Seven, Key::UpArrow)]),
        (Key::Five, vec![ (Key::Two, Key::DownArrow), (Key::Six, Key::RightArrow), (Key::Four, Key::LeftArrow), (Key::Eight, Key::UpArrow)]),
        (Key::Six, vec![ (Key::Three, Key::DownArrow), (Key::Five, Key::LeftArrow), (Key::Nine, Key::UpArrow)]),
        (Key::One, vec![(Key::Two, Key::RightArrow), (Key::Four, Key::UpArrow)]),
        (Key::Two, vec![ (Key::Three, Key::RightArrow), (Key::Five, Key::UpArrow), (Key::One, Key::LeftArrow), (Key::Zero, Key::DownArrow)]),
        (Key::Zero, vec![(Key::Two, Key::UpArrow), (Key::Accept, Key::RightArrow)]),
        (Key::Three, vec![ (Key::Six, Key::UpArrow), (Key::Two, Key::LeftArrow), (Key::Accept, Key::DownArrow)]),
        (Key::Accept, vec![(Key::Zero, Key::LeftArrow), (Key::Three, Key::UpArrow)]),
    ])
    .make_shortest_paths()
}

#[once]
#[rustfmt::skip]
fn direction_paths() -> FxHashMap<(Key, Key), Vec<Vec<Key>>> {
    Keypad::from_iter(vec![
        (Key::UpArrow, vec![ (Key::Accept, Key::RightArrow), (Key::DownArrow, Key::DownArrow)]),
        (Key::Accept, vec![ (Key::UpArrow, Key::LeftArrow), (Key::RightArrow, Key::DownArrow)]),
        (Key::RightArrow, vec![ (Key::Accept, Key::UpArrow), (Key::DownArrow, Key::LeftArrow)]),
        (Key::LeftArrow, vec![(Key::DownArrow, Key::RightArrow)]),
        (Key::DownArrow, vec![ (Key::LeftArrow, Key::LeftArrow), (Key::UpArrow, Key::UpArrow), (Key::RightArrow, Key::RightArrow)]),
    ])
    .make_shortest_paths()
}

fn shortest_paths(
    neighbour_map: &FxHashMap<Key, Vec<(Key, Key)>>,
    from: Key,
    to: Key,
) -> Vec<Vec<Key>> {
    let mut queue = VecDeque::new();
    queue.push_back((from, Vec::new(), FxHashSet::default()));
    let mut paths = Vec::new();
    let mut lowest = usize::MAX;
    while let Some((node, path, mut visited)) = queue.pop_front() {
        if node == to {
            if path.len() <= lowest {
                lowest = path.len();
                paths.push(path.clone());
            }
            continue;
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for (next, dir) in neighbour_map.get(&node).unwrap() {
            let mut path = path.clone();
            path.push(*dir);
            queue.push_back((*next, path, visited.clone()));
        }
    }
    paths
}

#[cached]
fn find_shortest_sequence(sequence: Vec<Key>, depth: usize, keypad_type: KeypadType) -> usize {
    let paths = match keypad_type {
        KeypadType::Numeric => numeric_paths(),
        KeypadType::Arrows => direction_paths(),
    };
    std::iter::once(Key::Accept)
        .chain(sequence)
        .tuple_windows()
        .map(|key| paths.get(&key).unwrap())
        .map(|shortest_paths| {
            if depth == 0 {
                return shortest_paths[0].len() + 1;
            }
            shortest_paths
                .iter()
                .map(|path| path.iter().cloned().chain(once(Key::Accept)))
                .map(|path| find_shortest_sequence(path.collect(), depth - 1, KeypadType::Arrows))
                .min()
                .unwrap()
        })
        .sum::<usize>()
}
fn solve(file: &str, depth: usize) -> String {
    file.lines()
        .map(|line| (line.chars().map(Key::from).collect(), line))
        .map(|(start, line)| (start, line.trim_end_matches('A').parse::<usize>().unwrap()))
        .map(|(start, value)| find_shortest_sequence(start, depth, KeypadType::Numeric) * value)
        .sum::<usize>()
        .to_string()
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        solve(file, 2)
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        solve(file, 25)
    }
}
