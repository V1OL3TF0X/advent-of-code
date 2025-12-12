pub struct Solution;
use itertools::Itertools;
use split_paragraphs::SplitParagraphs;

use crate::task_fns::SolveMode;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        let mut sections = file.paragraphs();
        let presents = sections
            .take_while_ref(|l| {
                l.lines()
                    .peekable()
                    .peek()
                    .map(|l| l.trim_end().ends_with(':'))
                    .unwrap_or(false)
            })
            .map(|l| {
                let mut size = 0;
                Present {
                    shape: l
                        .lines()
                        .skip(1)
                        .inspect(|l| size += l.chars().filter(|&c| c == '#').count())
                        .collect_vec(),
                    size,
                }
            })
            .collect_vec();
        sections
            .flat_map(|s| {
                // SAFETY - valid input
                s.lines().map(|l| unsafe {
                    let (size, counts) = l.split_once(": ").unwrap_unchecked();
                    let (x, y) = size.split_once('x').unwrap_unchecked();
                    GiftArea {
                        w: x.parse().unwrap_unchecked(),
                        h: y.parse().unwrap_unchecked(),
                        present_count: counts
                            .split_whitespace()
                            .map(str::parse)
                            .try_collect()
                            .unwrap_unchecked(),
                    }
                })
            })
            .filter(|a| a.is_big_enough(&presents))
            .count()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        todo!("{file}")
    }
}

struct Present<'a> {
    shape: Vec<&'a str>,
    size: usize,
}

struct GiftArea {
    w: usize,
    h: usize,
    present_count: Vec<usize>,
}

impl GiftArea {
    // let's pray it works
    fn is_big_enough(&self, presents: &[Present]) -> bool {
        self.present_count
            .iter()
            .enumerate()
            .map(|(p, count)| presents[p].size * count)
            .sum::<usize>() as f64
            <= (self.w * self.h) as f64
    }
}
