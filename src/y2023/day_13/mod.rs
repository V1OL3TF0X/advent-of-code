use std::{fmt::Debug, slice::Iter, str::Lines};

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        solve(file, 0)
    }

    fn task_2(&self, file: &str) -> String {
        solve(file, 1)
    }
}

fn solve(file: &str, max_diff: usize) -> String {
    parse_input(file)
        .into_iter()
        .fold(0, |reflections, pattern| {
            let row_candidates = pattern.mirr_row(max_diff);
            let mut sum = 0;
            if !row_candidates.is_empty() {
                if let Some((v, _)) = row_candidates.into_iter().find(|(r, d)| {
                    if d > &max_diff {
                        return false;
                    }
                    let mut diff = *d;
                    for (t, b) in pattern.row_pairs(*r) {
                        if let Some(d) = lines_match(t, b, max_diff) {
                            diff += d;
                            if diff > max_diff {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    diff == max_diff
                }) {
                    sum = 100 * (v + 1);
                }
            }
            if sum == 0 {
                let (v, _) = pattern
                    .mirr_col(max_diff)
                    .find(|(c, d)| {
                        if d > &max_diff {
                            return false;
                        }
                        let mut diff = *d;
                        for (l, r) in pattern.col_pairs(*c) {
                            if let Some(d) = lines_match(l, r, max_diff) {
                                diff += d;
                                if diff > max_diff {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                        diff == max_diff
                    })
                    .unwrap(); // it has to exist
                sum = v + 1 // indexed from 0, so number of cols before is index of first mirror of pair + 1
            }
            reflections + sum
        })
        .to_string()
}

fn parse_input(input: &str) -> Vec<Pattern> {
    let mut lines = input.lines();
    let mut res = vec![];
    loop {
        match Pattern::try_from(lines.by_ref()) {
            Ok(p) => res.push(p),
            Err(_) => break res,
        }
    }
}

struct Pattern {
    patt: Vec<Vec<Surface>>,
}

fn lines_match<'a>(
    s1: impl Iterator<Item = &'a Surface>,
    s2: impl Iterator<Item = &'a Surface>,
    max_diff: usize,
) -> Option<usize> {
    let mut diff = 0;
    for (c1, c2) in s1.zip(s2) {
        if c1 != c2 {
            diff += 1;
        }
        if diff > max_diff {
            return None;
        }
    }
    Some(diff)
}

impl Pattern {
    /// find indices of pairs of rows that have the same pattern
    fn mirr_row(&self, max_diff: usize) -> Vec<(usize, usize)> {
        self.patt
            .windows(2)
            .enumerate()
            .filter_map(|(i, vs)| lines_match(vs[0].iter(), vs[1].iter(), max_diff).map(|d| (i, d)))
            .collect()
    }
    /// find indices of pairs of cols that have the same pattern
    fn mirr_col(&self, max_diff: usize) -> Box<dyn Iterator<Item = (usize, usize)> + '_> {
        let row_len = self.patt.len();
        if row_len == 0 || self.patt[0].is_empty() {
            return Box::new(std::iter::empty());
        }
        let col_pair_no = self.patt[0].len() - 1;
        Box::new((0..col_pair_no).filter_map(move |c| {
            lines_match(self.col_iter(c), self.col_iter(c + 1), max_diff).map(|d| (c, d))
        }))
    }

    fn row_pairs(&self, index: usize) -> RowPairIter<'_> {
        RowPairIter {
            p: self,
            top: index,
            bottom: index + 1,
        }
    }

    fn col_pairs(&self, index: usize) -> ColPairIter<'_> {
        ColPairIter {
            p: self,
            left: index,
            right: index + 1,
        }
    }

    fn col_iter(&self, col: usize) -> ColIter<'_> {
        ColIter::new(self, col)
    }
}

struct ColIter<'p> {
    p: &'p Pattern,
    col: usize,
    index: usize,
}

impl Debug for ColIter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.p
            .col_iter(self.col)
            .try_for_each(|c| write!(f, "{c:?}"))
    }
}

impl<'p> ColIter<'p> {
    fn new(p: &'p Pattern, col: usize) -> Self {
        Self { p, col, index: 0 }
    }
}

impl<'p> Iterator for ColIter<'p> {
    type Item = &'p Surface;

    fn next(&mut self) -> Option<Self::Item> {
        let v = &self.p.patt.get(self.index)?[self.col];
        self.index += 1;
        Some(v)
    }
}

struct RowPairIter<'p> {
    p: &'p Pattern,
    bottom: usize,
    top: usize,
}

impl<'p> Iterator for RowPairIter<'p> {
    type Item = (Iter<'p, Surface>, Iter<'p, Surface>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.top == 0 || self.bottom == self.p.patt.len() - 1 {
            return None;
        }
        self.top -= 1;
        self.bottom += 1;
        Some((
            self.p.patt[self.top].iter(),
            self.p.patt[self.bottom].iter(),
        ))
    }
}
struct ColPairIter<'p> {
    p: &'p Pattern,
    left: usize,
    right: usize,
}

impl<'p> Iterator for ColPairIter<'p> {
    type Item = (ColIter<'p>, ColIter<'p>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.left == 0 || self.right == self.p.patt[0].len() - 1 {
            return None;
        }
        self.left -= 1;
        self.right += 1;
        Some((self.p.col_iter(self.left), self.p.col_iter(self.right)))
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ").and_then(|_| {
            (0..self.patt[0].len())
                .try_for_each(|i| write!(f, "{i:X}"))
                .and_then(|_| writeln!(f))
                .and_then(|_| {
                    self.patt.iter().enumerate().try_for_each(|(i, r)| {
                        write!(f, "{i:0>2} ").and_then(|_| {
                            r.iter()
                                .try_for_each(|c| write!(f, "{c:?}"))
                                .and_then(|_| writeln!(f))
                        })
                    })
                })
        })
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Self::try_from(&mut value.lines()).unwrap()
    }
}

impl TryFrom<&mut Lines<'_>> for Pattern {
    type Error = &'static str;

    fn try_from(value: &mut Lines<'_>) -> Result<Self, Self::Error> {
        let patt: Vec<_> = value
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().map(Surface::from).collect())
            .collect();
        if patt.is_empty() {
            Err("No pattern detected!")
        } else {
            Ok(Self { patt })
        }
    }
}

#[derive(PartialEq, Eq)]
enum Surface {
    Rock,
    Ash,
}

impl Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Ash => write!(f, "."),
        }
    }
}

impl From<char> for Surface {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Rock,
            '.' => Self::Ash,
            c => unreachable!("{c} is not a valid surface character"),
        }
    }
}
