pub struct Solution;

use itertools::Itertools;
use pathfinding::prelude::count_paths;
use rustc_hash::FxHashMap;

use crate::task_fns::SolveMode;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        let paths = FxHashMap::from_iter(
            file.lines()
                .map(|l| unsafe { l.split_once(':').unwrap_unchecked() })
                .map(|(from, to)| (from, to.split_whitespace().collect_vec())),
        );
        count_paths(
            &"you",
            |from| unsafe { paths.get(*from).unwrap_unchecked() },
            |node| **node == "out",
        )
        .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        let mut paths = FxHashMap::from_iter(
            file.lines()
                .map(|l| unsafe { l.split_once(':').unwrap_unchecked() })
                .map(|(from, to)| (from, to.split_whitespace().collect_vec())),
        );
        paths.insert("out", vec![]);
        count_paths(
            ("svr", false, false),
            |from| unsafe {
                let (from, has_fft, has_dac) = *from;
                paths
                    .get(from)
                    .unwrap_unchecked()
                    .iter()
                    .map(move |&el| match el {
                        "fft" => (el, true, has_dac),
                        "dac" => (el, has_fft, true),
                        other => (other, has_fft, has_dac),
                    })
            },
            |node| node.0 == "out" && node.1 && node.2,
        )
        .to_string()
    }
}
