use crate::task_fns::SolveMode;
use std::str::FromStr;

use pathfinding::matrix::directions::DIRECTIONS_4;
use rustc_hash::FxHashMap;

struct Racetrack {
    track_in_order: Vec<(isize, isize)>,
}

impl FromStr for Racetrack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let map: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                let mut x = 0;
                l.chars()
                    .inspect(|c| {
                        match c {
                            'S' => start = Some((x as isize, y as isize)),
                            'E' => end = Some((x as isize, y as isize)),
                            _ => {}
                        }
                        x += 1;
                    })
                    .collect()
            })
            .collect();
        let start = start.ok_or("no start tile found!")?;
        let end = end.ok_or("no end tile found!")?;
        let mut pos = start;
        let mut from = (0, 0);
        let mut track_in_order = vec![start];

        while pos != end {
            for dir in DIRECTIONS_4 {
                if from.0 + dir.0 == 0 && from.1 + dir.1 == 0 {
                    continue;
                }
                let checked = (pos.0 + dir.0, pos.1 + dir.1);
                match map[checked.1 as usize][checked.0 as usize] {
                    '.' | 'E' => {
                        from = dir;
                        pos = checked;
                        track_in_order.push(checked);
                    }
                    '#' => {}
                    v => {
                        return Err(format!("{v} shouldn't be a part of the track!"));
                    }
                }
            }
        }
        Ok(Self { track_in_order })
    }
}
impl Racetrack {
    fn find_cheats_in_rad(&self, radius: usize, min_save: usize) -> FxHashMap<usize, usize> {
        let mut cheats = FxHashMap::default();

        for (ord1, el1) in self.track_in_order.iter().enumerate() {
            for (ord2, el2) in self.track_in_order.iter().enumerate().skip(min_save + ord1) {
                let possible_save = ord1.abs_diff(ord2);
                let d = el1.0.abs_diff(el2.0) + el1.1.abs_diff(el2.1);
                if d > radius {
                    continue;
                }
                let saved = possible_save - d;
                if saved >= min_save {
                    cheats.entry(saved).and_modify(|c| *c += 1).or_insert(1);
                }
            }
        }
        cheats
    }
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        Racetrack::from_str(file)
            .unwrap()
            .find_cheats_in_rad(2, 100)
            .into_iter()
            .filter_map(|(size, count)| (size >= 100).then_some(count))
            .sum::<usize>()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        Racetrack::from_str(file)
            .unwrap()
            .find_cheats_in_rad(20, 100)
            .into_values()
            .sum::<usize>()
            .to_string()
    }
}
