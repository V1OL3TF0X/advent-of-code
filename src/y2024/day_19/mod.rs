use crate::task_fns::SolveMode;
use rustc_hash::FxHashMap;

fn ways_is_possible_to_make<'a>(
    available_patterns: &[&'a str],
    towel: &'a str,
    cache: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if let Some(res) = cache.get(towel) {
        return *res;
    }
    let towel_ways = available_patterns
        .iter()
        .filter(|&&pat| towel.starts_with(pat))
        .map(|pat| ways_is_possible_to_make(available_patterns, &towel[pat.len()..], cache))
        .sum();
    cache.insert(towel, towel_ways);
    towel_ways
}

fn make_possible_iter(file: &str) -> impl Iterator<Item = u64> + '_ {
    let mut lines = file.lines();
    let available_patterns = (&mut lines)
        .take_while(|l| !l.is_empty())
        .flat_map(|l| l.split(", "))
        .collect::<Vec<_>>();
    let mut cache = FxHashMap::from_iter([("", 1)]);
    lines
        .map(|l| l.trim())
        .map(move |model| ways_is_possible_to_make(&available_patterns, model, &mut cache))
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        make_possible_iter(file)
            .filter(|ways_to_make| *ways_to_make > 0)
            .count()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        make_possible_iter(file).sum::<u64>().to_string()
    }
}
