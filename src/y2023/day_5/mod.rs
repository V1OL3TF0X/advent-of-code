use std::ops::RangeInclusive;

use crate::task_fns::{SolveMode, TaskFns};

const MAPS: [&str; 7] = [
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:",
];
pub struct Solution;
impl TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        let mut lines = file.lines();
        let seeds = to_nums(
            lines
                .next()
                .unwrap()
                .split_once(": ")
                .expect("first line should be seeds: ({seed_no}) *")
                .1,
        );
        let seed_no = seeds.len();
        MAPS.iter()
            .fold(seeds, |mut prev, _| {
                let mut swapped = Vec::with_capacity(seed_no);
                lines
                    .by_ref()
                    .skip_while(|&l| !l.starts_with(|c: char| c.is_ascii_digit()))
                    .take_while(|&l| l.starts_with(|c: char| c.is_ascii_digit()))
                    .map(to_nums)
                    .for_each(|range| {
                        prev.retain(|prev| {
                            let v = *prev;
                            if range[1] <= v && v <= range[1] + range[2] {
                                swapped.push(v - range[1] + range[0]);
                                false
                            } else {
                                true
                            }
                        })
                    });
                swapped.extend(prev);
                swapped
            })
            .into_iter()
            .min()
            .unwrap()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        let mut lines = file.lines();
        let seed_ranges = to_num_ranges(
            lines
                .next()
                .unwrap()
                .split_once(": ")
                .expect("first line should be seeds: ({seed_no}) *")
                .1,
        );
        let seed_no = seed_ranges.len();
        MAPS.iter()
            .fold(seed_ranges, |mut prev, _| {
                let mut swapped = Vec::with_capacity(seed_no);
                lines
                    .by_ref()
                    // skip to start of map
                    .skip_while(|&l| !l.starts_with(|c: char| c.is_ascii_digit()))
                    // get all mapping ranges
                    .take_while(|&l| l.starts_with(|c: char| c.is_ascii_digit()))
                    .map(to_nums)
                    .map(|r| (r[1]..=(r[1] + r[2] - 1), r[0] as i128 - r[1] as i128))
                    .for_each(|(mapping_source, offset)| {
                        let mut partially_mapped = vec![];
                        prev.retain(|seed_range| {
                            // no intersection
                            if seed_range.start() > mapping_source.end()
                                || seed_range.end() < mapping_source.start()
                            {
                                return true;
                            }
                            let intersection = intersect(seed_range, &mapping_source);
                            // some seeds from this seed range were not mapped
                            if seed_range.start() < intersection.start() {
                                partially_mapped
                                    .push(*seed_range.start()..=(*intersection.start() - 1));
                            }
                            if intersection.end() < seed_range.end() {
                                partially_mapped
                                    .push((*intersection.end() + 1)..=*seed_range.end());
                            }
                            // push mapped range
                            let mapped_intersection = ((*intersection.start() as i128 + offset)
                                as u64)
                                ..=((*intersection.end() as i128 + offset) as u64);
                            swapped.push(mapped_intersection);
                            false
                        });
                        if !partially_mapped.is_empty() {
                            prev.extend(partially_mapped);
                        }
                        prev.sort_by(|r1, r2| {
                            r1.end()
                                .cmp(r2.end())
                                .then_with(|| r1.start().cmp(r2.start()))
                        })
                    });
                swapped.extend(prev);
                swapped
            })
            .iter()
            .map(RangeInclusive::<u64>::start)
            .min()
            .unwrap()
            .to_string()
    }
}

fn to_nums(num_str: &str) -> Vec<u64> {
    num_str
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn to_num_ranges(num_str: &str) -> Vec<RangeInclusive<u64>> {
    num_str
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .map_windows(|[s, e]| *s..=(s + e - 1))
        .step_by(2)
        .collect()
}

fn intersect(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    *a.start().max(b.start())..=*a.end().min(b.end())
}
