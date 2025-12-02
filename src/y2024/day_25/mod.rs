use itertools::Itertools;

fn get_locks_and_keys(file: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, u8) {
    let mut locks = vec![];
    let mut keys = vec![];
    let mut height = 0;
    file.split("\n\n").for_each(|chunk| {
        let mut lines = chunk.lines();
        let rep = lines.next().unwrap();
        let (to, mut definition) = if rep.starts_with('#') {
            (&mut locks, vec![1; rep.len()])
        } else {
            (&mut keys, vec![0; rep.len()])
        };
        height = lines
            .inspect(|l| {
                l.chars()
                    .positions(|el| el == '#')
                    .for_each(|p| definition[p] += 1);
            })
            .count();
        definition.iter_mut().for_each(|v| *v -= 1);
        to.push(definition);
    });
    (locks, keys, (height - 1) as u8)
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let (locks, keys, height) = get_locks_and_keys(file);
        println!("{locks:?}\n{keys:?}\n height: {height}");
        locks
            .iter()
            .cartesian_product(keys.iter())
            .filter(|(l, k)| l.iter().zip(k.iter()).all(|(a, b)| a + b <= height))
            .count()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        todo!("{file}")
    }
}
