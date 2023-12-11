use std::collections::HashSet;

pub fn task_1(file: &str) -> String {
    Universe::from(file).distances_sum(2).to_string()
}

pub fn task_2(file: &str) -> String {
    Universe::from(file).distances_sum(1_000_000).to_string()
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

impl Universe {
    pub fn distances_sum(&self, expansion_rate: u128) -> u128 {
        let mut pairs = 0;
        self.galaxies
            .iter()
            .enumerate()
            .map(|(i, g1)| {
                self.galaxies.iter().skip(i + 1).fold(0, |sum, g2| {
                    pairs += 1;
                    let (start_x, end_x) = if g1.0 > g2.0 {
                        (g2.0, g1.0)
                    } else {
                        (g1.0, g2.0)
                    };
                    let expanded_x = self
                        .expanded_rows
                        .iter()
                        .filter(|x| (start_x..end_x).contains(x))
                        .count() as u128
                        * (expansion_rate - 1);
                    let dist_x = end_x - start_x;
                    let (start_y, end_y) = if g1.1 > g2.1 {
                        (g2.1, g1.1)
                    } else {
                        (g1.1, g2.1)
                    };
                    let expanded_y = self
                        .expanded_cols
                        .iter()
                        .filter(|y| (start_y..end_y).contains(y))
                        .count() as u128
                        * (expansion_rate - 1);
                    let dist_y = end_y - start_y;
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
