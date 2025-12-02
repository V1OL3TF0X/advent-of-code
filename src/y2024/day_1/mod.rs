use std::collections::HashMap;

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let mut ids: (Vec<u32>, Vec<u32>) = file
            .lines()
            .map(|l| {
                let (f, s) = l
                    .split_once(char::is_whitespace)
                    .expect("two characters separated by whitespace in each line");
                let s = s.trim_start();
                (f.parse::<u32>().unwrap(), s.parse::<u32>().unwrap()) as (u32, u32)
            })
            .collect();

        ids.0.sort();
        ids.1.sort();
        ids.0
            .into_iter()
            .zip(ids.1)
            .fold(0, |sum, (a, b)| sum + a.abs_diff(b))
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let (left, right): (Vec<u32>, HashMap<u32, u32>) =
            file.lines()
                .fold((vec![], HashMap::new()), |(mut v, mut m), l| {
                    let (f, s) = l
                        .split_once(char::is_whitespace)
                        .expect("two characters separated by whitespace in each line");
                    let f = f.parse::<u32>().unwrap();
                    let s = s.trim_start().parse::<u32>().unwrap();
                    v.push(f);
                    m.entry(s).and_modify(|v| *v += 1).or_insert(1);
                    (v, m)
                });
        left.iter()
            .flat_map(|v| right.get(v).map(|occurences| occurences * v))
            .sum::<u32>()
            .to_string()
    }
}

