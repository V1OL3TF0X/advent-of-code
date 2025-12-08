use crate::task_fns::SolveMode;
use std::{collections::VecDeque, num::ParseIntError};

use rustc_hash::{FxHashMap, FxHashSet};

fn next_secret_num(mut num: u64) -> u64 {
    num ^= num << 6;
    num %= 16_777_216;
    num ^= num >> 5;
    num %= 16_777_216;
    num ^= num << 11;
    num %= 16_777_216;
    num
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        file.lines()
            .map(|l| l.parse::<u64>())
            .try_fold(0, |sum, n| {
                Ok::<_, ParseIntError>(sum + (0..2000).fold(n?, |prev, _| next_secret_num(prev)))
            })
            .unwrap()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        file.lines()
            .map(|l| l.parse::<u64>())
            .try_fold(FxHashMap::default(), |mut diff_cache, n| {
                let first = n?;
                let mut diff_hold = VecDeque::with_capacity(4);
                let mut found: FxHashSet<[i64; 4]> = FxHashSet::default();
                let _ = (0..2000).fold((first, first % 10), |(secret, price), _| {
                    let new_secret = next_secret_num(secret);
                    let new_price = new_secret % 10;
                    let price_diff = new_price as i64 - price as i64;
                    diff_hold.push_back(price_diff);
                    if diff_hold.len() == 4 {
                        let key = [diff_hold[0], diff_hold[1], diff_hold[2], diff_hold[3]];
                        if !found.contains(&key) {
                            diff_cache
                                .entry(key)
                                .and_modify(|v| *v += new_price)
                                .or_insert(new_price);
                            found.insert(key);
                        }
                        diff_hold.pop_front();
                    }
                    (new_secret, new_price)
                });
                Ok::<_, ParseIntError>(diff_cache)
            })
            .unwrap()
            .into_values()
            .max()
            .unwrap()
            .to_string()
    }
}
