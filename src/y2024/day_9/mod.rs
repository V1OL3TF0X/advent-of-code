use crate::task_fns::SolveMode;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Section {
    length: u64,
    file_no: u64,
}

impl Section {
    fn new(length: u64, file_no: u64) -> Self {
        Self { length, file_no }
    }
}

fn compress_task_1(mut files: Vec<Section>, gaps: Vec<u64>) -> Vec<Section> {
    let mut i = 0;
    let mut compressed = vec![];
    while i < files.len() {
        compressed.push(files[i].clone());
        files[i].length = 0;
        let mut needed_bytes = gaps[i];
        i += 1;
        while needed_bytes > 0 && i < files.len() {
            let file_to_move = files.last_mut().unwrap();
            let available_bytes = file_to_move.length;
            if available_bytes > needed_bytes {
                file_to_move.length -= needed_bytes;
                compressed.push(Section::new(needed_bytes, file_to_move.file_no));
                break;
            }
            needed_bytes -= file_to_move.length;
            compressed.push(files.pop().unwrap());
        }
    }
    if i >= files.len() {
        compressed.extend(files.into_iter().filter(|f| f.length != 0));
    }
    compressed
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        solve(file, compress_task_1)
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        solve(file, compress_task_2)
    }
}

const GAP_FILE_NO: u64 = u64::MAX;
fn compress_task_2(files: Vec<Section>, gaps: Vec<u64>) -> Vec<Section> {
    let mut compressed = files
        .into_iter()
        .interleave(gaps.into_iter().map(|g| Section::new(g, GAP_FILE_NO)))
        .collect::<Vec<_>>();
    let mut i = compressed.len();
    while i >= 1 {
        if compressed[i - 1].file_no == GAP_FILE_NO {
            i -= 1;
            continue;
        }
        let file_to_fit = &compressed[i - 1];
        let Some(fitting_gap) = compressed
            .iter()
            .take(i - 1)
            .position(|s| s.file_no == GAP_FILE_NO && s.length >= file_to_fit.length)
        else {
            i -= 1;
            continue;
        };
        match file_to_fit.length.cmp(&compressed[fitting_gap].length) {
            std::cmp::Ordering::Less => {
                i += 1;
                let file_to_fit = file_to_fit.clone();
                compressed[fitting_gap].length -= file_to_fit.length;
                compressed.insert(fitting_gap, file_to_fit);
                compressed[i - 1].file_no = GAP_FILE_NO;
            }
            std::cmp::Ordering::Equal => {
                compressed[fitting_gap].file_no = file_to_fit.file_no;
                compressed[i - 1].file_no = GAP_FILE_NO;
            }
            std::cmp::Ordering::Greater => {}
        }
        i -= 1;
    }
    compressed
}

fn solve(file: &str, compress: impl FnOnce(Vec<Section>, Vec<u64>) -> Vec<Section>) -> String {
    let (files, gaps) = file
        .chars()
        .flat_map(|c| c.to_digit(10).map(|d| d as u64))
        .enumerate()
        .fold((vec![], vec![]), |mut acc, (i, d)| {
            if i % 2 == 0 {
                acc.0.push(Section::new(d, acc.0.len() as u64));
            } else {
                acc.1.push(d);
            }
            acc
        });
    let compressed = compress(files, gaps);
    calculate_checksum(&compressed).to_string()
}

fn sum_range(from: u64, to: u64) -> u64 {
    (from + to) * (to + 1 - from) / 2
}

fn calculate_checksum(disk: &[Section]) -> u64 {
    disk.iter()
        .fold((0, 0), |mut acc, s| {
            if s.file_no == GAP_FILE_NO {
                acc.1 += s.length;
            } else {
                let new_length = acc.1 + s.length;
                acc.0 += sum_range(acc.1, new_length - 1) * s.file_no;
                acc.1 = new_length;
            }
            acc
        })
        .0
}
