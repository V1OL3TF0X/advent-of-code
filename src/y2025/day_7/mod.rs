use itertools::Itertools;
use rustc_hash::FxHashMap;

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        unsafe {
            BeamSplitter::from_lines(file)
                .simulate_timelines()
                .0
                .to_string()
        }
    }

    fn task_2(&self, file: &str) -> String {
        unsafe {
            BeamSplitter::from_lines(file)
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

impl BeamSplitter {
    unsafe fn from_lines(file: &str) -> Self {
        let mut lines = file.lines();
        let start = unsafe {
            lines
                .next()
                .unwrap_unchecked()
                .chars()
                .enumerate()
                .find(|(_, c)| *c == 'S')
                .unwrap_unchecked()
                .0
        };
        BeamSplitter {
            start,
            splitters: lines.map(|l| l.chars().collect_vec()).collect_vec(),
        }
    }
    fn simulate_timelines(&self) -> (usize, FxHashMap<usize, usize>) {
        let mut split_number = 0;
        let mut ray_indices = FxHashMap::default();
        ray_indices.insert(self.start, 1);
        for line in &self.splitters {
            ray_indices
                .keys()
                .filter(|i| line[**i] == '^')
                .copied()
                .collect_vec()
                .into_iter()
                .for_each(|i| {
                    split_number += 1;
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
        }
        (split_number, ray_indices)
    }
}
