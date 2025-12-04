use itertools::Itertools;

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let map = get_map(file);
        get_removable(&map).len().to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let mut map = get_map(file);
        let mut total = 0;
        let mut removable = get_removable(&map);
        while !removable.is_empty() {
            total += removable.len();
            removable.into_iter().for_each(|(y, x)| map[y][x] = 0);
            removable = get_removable(&map);
        }
        total.to_string()
    }
}

fn get_map(file: &str) -> Vec<Vec<u8>> {
    file.lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect_vec()
        })
        .collect_vec()
}

fn get_removable(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 0..map.len() {
        let col_min = if i == 0 { 0 } else { i - 1 };
        let col_max = (i + 1).min(map.len() - 1);
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                continue;
            }
            let row_min = if j == 0 { 0 } else { j - 1 };
            let row_max = (j + 1).min(map[0].len() - 1);
            let adjacent_rolls = (col_min..=col_max)
                .map(|col| (row_min..=row_max).map(|row| map[col][row]).sum::<u8>())
                .sum::<u8>()
                - map[i][j];
            if adjacent_rolls < 4 {
                res.push((i, j));
            }
        }
    }
    res
}
