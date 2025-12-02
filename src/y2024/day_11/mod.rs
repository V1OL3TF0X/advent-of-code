use rustc_hash::FxHashMap;

fn solve(stone: u64, blinks: u32, cache: &mut FxHashMap<(u64, u32), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(c) = cache.get(&(stone, blinks)) {
        return *c;
    }

    let digit_num = (stone as f64).log10().floor() as u32 + 1;
    let c = if stone == 0 {
        solve(1, blinks - 1, cache)
    } else if digit_num % 2 == 0 {
        let half = 10_u64.pow(digit_num / 2);
        solve(stone / half, blinks - 1, cache) + solve(stone % half, blinks - 1, cache)
    } else {
        solve(stone * 2024, blinks - 1, cache)
    };
    cache.insert((stone, blinks), c);
    c
}
pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let mut cache = FxHashMap::default();
        file.split_whitespace()
            .map(|c| c.parse::<u64>().unwrap())
            .map(|s| solve(s, 25, &mut cache))
            .sum::<usize>()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let mut cache = FxHashMap::default();
        file.split_whitespace()
            .map(|c| c.parse::<u64>().unwrap())
            .map(|s| solve(s, 75, &mut cache))
            .sum::<usize>()
            .to_string()
    }
}
