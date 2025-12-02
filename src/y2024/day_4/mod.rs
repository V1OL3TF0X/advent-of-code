use std::cmp::min;
pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let table: Vec<Vec<_>> = file
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();
        let mut search_xmas = vec![];
        let mut search_samx = vec![];
        let mut found = vec![];
        // horizontal
        table.iter().enumerate().for_each(|(i, row)| {
            search_xmas.clear();
            search_samx.clear();
            row.iter().enumerate().for_each(|(j, l)| {
                find_step(l, &mut search_xmas, &mut search_samx, &mut found, i, j);
            });
        });
        search_xmas.clear();
        search_samx.clear();
        // vertical
        for j in 0..table[0].len() {
            search_xmas.clear();
            search_samx.clear();
            (0..table.len()).for_each(|i| {
                find_step(
                    &table[i][j],
                    &mut search_xmas,
                    &mut search_samx,
                    &mut found,
                    i,
                    j,
                );
            });
        }
        if table.len() < 4 || table[0].len() < 4 {
            return (found.len() / 4).to_string();
        }
        // diagonal to BL
        for j in 3..table[0].len() {
            search_xmas.clear();
            search_samx.clear();
            (0..min(j + 1, table.len())).for_each(|i| {
                find_step(
                    &table[i][j - i],
                    &mut search_xmas,
                    &mut search_samx,
                    &mut found,
                    i,
                    j - i,
                );
            });
        }
        for i in 1..(table.len() - 3) {
            search_xmas.clear();
            search_samx.clear();
            for j in 0..min(table[0].len(), table.len() - i) {
                let x = i + j;
                let y = table[0].len() - 1 - j;
                find_step(
                    &table[x][y],
                    &mut search_xmas,
                    &mut search_samx,
                    &mut found,
                    x,
                    y,
                );
            }
        }
        // diagonal to BR
        for j in 3..table[0].len() {
            search_xmas.clear();
            search_samx.clear();
            for i in 0..min(j + 1, table.len()) {
                let y = table[0].len() - j - 1 + i;
                find_step(
                    &table[i][y],
                    &mut search_xmas,
                    &mut search_samx,
                    &mut found,
                    i,
                    y,
                );
            }
        }
        for i in 1..(table.len() - 3) {
            search_xmas.clear();
            search_samx.clear();
            for j in 0..min(table[0].len(), table.len() - i) {
                find_step(
                    &table[i + j][j],
                    &mut search_xmas,
                    &mut search_samx,
                    &mut found,
                    i + j,
                    j,
                );
            }
        }
        (found.len() / 4).to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let table: Vec<Vec<_>> = file
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect();
        (1..(table.len() - 1))
            .map(|i| {
                (1..table[0].len() - 1).fold(0, |sum, j| {
                    if table[i][j] != 'A' {
                        return sum;
                    }
                    if !matches!(
                        (table[i + 1][j + 1], table[i - 1][j - 1]),
                        ('M', 'S') | ('S', 'M')
                    ) || !matches!(
                        (table[i + 1][j - 1], table[i - 1][j + 1]),
                        ('M', 'S') | ('S', 'M')
                    ) {
                        return sum;
                    }
                    sum + 1
                })
            })
            .sum::<u32>()
            .to_string()
    }
}

fn is_next_letter(l: &char, current: usize) -> bool {
    matches!((l, current), ('X', 0) | ('M', 1) | ('A', 2) | ('S', 3))
}
fn is_prev_letter(l: &char, current: usize) -> bool {
    matches!((l, current), ('S', 0) | ('A', 1) | ('M', 2) | ('X', 3))
}

fn find_step(
    l: &char,
    front: &mut Vec<(usize, usize)>,
    back: &mut Vec<(usize, usize)>,
    found: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
) {
    if is_next_letter(l, front.len()) {
        front.push((x, y));
        if front.len() == 4 {
            found.extend(front.iter());
            front.clear();
        }
    } else {
        front.clear();
        if is_next_letter(l, 0) {
            front.push((x, y));
        }
    }
    if is_prev_letter(l, back.len()) {
        back.push((x, y));
        if back.len() == 4 {
            found.extend(back.iter());
            back.clear();
        }
    } else {
        back.clear();
        if is_prev_letter(l, 0) {
            back.push((x, y))
        }
    }
}
