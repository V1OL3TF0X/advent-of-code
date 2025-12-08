use std::collections::HashSet;

use crate::task_fns::SolveMode;

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        Universe::from(file).distances_sum(2).to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        Universe::from(file).distances_sum(1_000_000).to_string()
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl Universe {
    pub fn distances_sum(&self, expansion_rate: u128) -> u128 {
        let first_expanded_col = self.expanded_cols[0];
        let last_expanded_col = self.expanded_cols[self.expanded_cols.len() - 1];
        self.galaxies
            .iter()
            .enumerate()
            .map(|(row_num, g1)| {
                let mut expanded_x = 0;
                let mut last_x = g1.0;
                let mut last_ex_ind = 0;
                self.galaxies.iter().skip(row_num + 1).fold(0, |sum, g2| {
                    // because we're pushing galaxies iteratively, g1 row <= g2 row
                    let dist_x = g2.0 - g1.0;
                    if g2.0 > g1.0 {
                        let new_expanded = self
                            .expanded_rows
                            .iter()
                            .skip(last_ex_ind)
                            .take_while(|&&r| r < g2.0)
                            .count();
                        last_ex_ind += new_expanded;
                        expanded_x += new_expanded as u128;
                        last_x = g2.0;
                    }
                    let (start_y, end_y) = if g1.1 > g2.1 {
                        (g2.1, g1.1)
                    } else {
                        (g1.1, g2.1)
                    };
                    let dist_y = end_y - start_y;
                    let expanded_y = if g2.0 >= first_expanded_col && g1.0 <= last_expanded_col {
                        self.expanded_cols
                            .iter()
                            .filter(|y| (start_y..end_y).contains(y))
                            .count() as u128
                            * (expansion_rate - 1)
                    } else {
                        0
                    };

                    sum + dist_x as u128 + expanded_x + dist_y as u128 + expanded_y
                })
            })
            .sum()
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let mut galaxies = vec![];
        let mut rows_with_galaxy = HashSet::new();
        let mut cols_with_galaxy = HashSet::new();
        let mut col_num = 0;
        let mut row_num = 0;
        value.lines().enumerate().for_each(|(row, l)| {
            l.chars().enumerate().for_each(|(col, c)| {
                if c == '#' {
                    rows_with_galaxy.insert(row);
                    cols_with_galaxy.insert(col);
                    galaxies.push((row, col))
                }
                if row == 0 {
                    col_num += 1;
                }
            });
            row_num += 1;
        });
        let expanded_rows = (0..row_num)
            .filter(|n| !rows_with_galaxy.contains(n))
            .collect();
        let expanded_cols = (0..col_num)
            .filter(|n| !cols_with_galaxy.contains(n))
            .collect();
        Self {
            galaxies,
            expanded_rows,
            expanded_cols,
        }
    }
}
