use crate::task_fns::SolveMode;
use std::str::FromStr;

use itertools::Itertools;
use rustc_hash::FxHashMap;

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        // SAFETY - valid input
        unsafe {
            BeamSplitter::from_str(file)
                .unwrap_unchecked()
                .simulate_timelines()
                .0
                .to_string()
        }
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        // SAFETY - valid input
        unsafe {
            BeamSplitter::from_str(file)
                .unwrap_unchecked()
                .simulate_timelines()
                .1
                .values()
                .sum::<usize>()
                .to_string()
        }
    }
}

struct BeamSplitter {
    start: usize,
    splitters: Vec<Vec<char>>,
}

impl FromStr for BeamSplitter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let start = lines
            .next()
            .ok_or("no lines in input")?
            .chars()
            .enumerate()
            .find(|(_, c)| *c == 'S')
            .ok_or("no start point in first line")?
            .0;
        Ok(Self {
            start,
            splitters: lines.map(|l| l.chars().collect_vec()).collect_vec(),
        })
    }
}

impl BeamSplitter {
    fn simulate_timelines(&self) -> (usize, FxHashMap<usize, usize>) {
        let mut split_number = 0;
        let mut ray_indices = FxHashMap::default();
        ray_indices.insert(self.start, 1);
        self.splitters.iter().for_each(|line| {
            ray_indices
                .keys()
                .filter(|i| line[**i] == '^')
                .copied()
                .collect_vec()
                .into_iter()
                .for_each(|i| {
                    split_number += 1;
                    // SAFETY - indices come from the hashmap itself
                    let timelines = unsafe { ray_indices.remove(&i).unwrap_unchecked() };
                    ray_indices
                        .entry(i - 1)
                        .and_modify(|v| *v += timelines)
                        .or_insert(timelines);
                    ray_indices
                        .entry(i + 1)
                        .and_modify(|v| *v += timelines)
                        .or_insert(timelines);
                });
        });
        (split_number, ray_indices)
    }
}
