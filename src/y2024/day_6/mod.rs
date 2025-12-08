use crate::task_fns::SolveMode;
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn forward(&self, (x, y): &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Top => (*y > 0).then(|| (*x, y - 1)),
            Direction::Right => Some((x + 1, *y)),
            Direction::Bottom => Some((*x, y + 1)),
            Direction::Left => (*x > 0).then(|| (x - 1, *y)),
        }
    }
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        };
    }
}

struct Table {
    table: Vec<Vec<char>>,
    guard_pos: (usize, usize),
    guard_start_pos: (usize, usize),
    guard_start_dir: Direction,
    guard_dir: Direction,
}
impl Table {
    fn from_str(file: &str) -> Self {
        let mut guard_pos = (usize::MAX, usize::MAX);
        let mut guard_dir = Direction::Top;
        let table = file
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if match c {
                            '^' => {
                                guard_dir = Direction::Top;
                                true
                            }
                            'v' => {
                                guard_dir = Direction::Bottom;
                                true
                            }
                            '<' => {
                                guard_dir = Direction::Left;
                                true
                            }
                            '>' => {
                                guard_dir = Direction::Right;
                                true
                            }
                            _ => false,
                        } {
                            guard_pos = (x, y);
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            guard_pos,
            guard_start_pos: guard_pos,
            guard_start_dir: guard_dir,
            guard_dir,
            table,
        }
    }
    fn reset(&mut self) {
        self.guard_dir = self.guard_start_dir;
        self.guard_pos = self.guard_start_pos;
        self.table.iter_mut().for_each(|l| {
            l.iter_mut().for_each(|c| {
                if *c == 'X' {
                    *c = '.'
                }
            })
        });
    }
    fn is_oob(&self, pos: &(usize, usize)) -> bool {
        pos.0 >= self.table[0].len() || pos.1 >= self.table.len()
    }
    fn get_guard_cell_mut(&mut self) -> &mut char {
        &mut self.table[self.guard_pos.1][self.guard_pos.0]
    }
    fn get_cell_mut(&mut self, pos: &(usize, usize)) -> &mut char {
        &mut self.table[pos.1][pos.0]
    }
    fn get_cell(&self, pos: &(usize, usize)) -> char {
        self.table[pos.1][pos.0]
    }
    fn walk(
        &mut self,
        mut on_walk: impl FnMut(&mut Table),
        mut on_bump: impl FnMut((usize, usize), Direction) -> bool,
    ) {
        'outer: while !self.is_oob(&self.guard_pos) {
            on_walk(self);
            let current_cell = self.get_guard_cell_mut();
            if *current_cell != 'X' {
                *current_cell = 'X';
            }
            let next = self.guard_dir.forward(&self.guard_pos);
            let Some(mut next) = next else {
                break;
            };
            if self.is_oob(&next) {
                break;
            }
            while self.get_cell(&next) == '#' {
                if !on_bump(next, self.guard_dir) {
                    break 'outer;
                }
                self.guard_dir.turn_right();
                let Some(n) = self.guard_dir.forward(&self.guard_pos) else {
                    continue;
                };
                next = n;
            }
            self.guard_pos = next;
        }
    }
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        let mut table = Table::from_str(file);
        let mut path_count = 0;
        table.walk(
            |t| {
                if t.get_cell(&t.guard_pos) != 'X' {
                    path_count += 1;
                }
            },
            |_, _| true,
        );
        path_count.to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        let mut table = Table::from_str(file);
        let mut walked_pos = vec![];
        *table.get_guard_cell_mut() = 'X';
        table.walk(
            |t| {
                if t.get_cell(&t.guard_pos) != 'X' {
                    walked_pos.push(t.guard_pos);
                }
            },
            |_, _| true,
        );
        let mut found = HashSet::new();
        walked_pos
            .into_iter()
            .filter(|potential_obstruction| {
                if found.contains(potential_obstruction) {
                    return false;
                }
                table.reset();
                *table.get_cell_mut(potential_obstruction) = '#';
                let mut is_in_loop = false;
                let mut obstructions = HashSet::new();
                table.walk(
                    |_| {},
                    |bumped_obstruction, bump_from| {
                        if obstructions.insert((bumped_obstruction, bump_from)) {
                            true
                        } else {
                            is_in_loop = true;
                            false
                        }
                    },
                );
                *table.get_cell_mut(potential_obstruction) = '.';
                if is_in_loop {
                    found.insert(*potential_obstruction);
                }
                is_in_loop
            })
            .count()
            .to_string()
    }
}
