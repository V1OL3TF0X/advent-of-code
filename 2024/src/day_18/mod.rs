use std::str::Lines;

use pathfinding::matrix::directions::DIRECTIONS_4;
use rustc_hash::{FxHashMap, FxHashSet};

struct Ram {
    size: isize,
    corrupted: Vec<(isize, isize)>,
    steps: usize,
    edges: FxHashMap<(isize, isize), Vec<((isize, isize), usize)>>,
}

impl Ram {
    fn from_file(s: &str, walls_taken: Option<usize>) -> Result<Self, String> {
        let (size, walls_taken_default, lines) = s.split_once('\n').map_or(
            Err("Ram has to have at least two lines".to_string()),
            |(s, l)| {
                let (size, walls_taken_def) = s
                    .split_once(',')
                    .ok_or(format!("{l}: coordinates must be separated by a comma"))?;
                let size = size
                    .trim_end()
                    .parse()
                    .map_err(|_| format!("{size} is not a valid number"))?;
                let walls_taken_def = walls_taken_def
                    .trim_end()
                    .parse()
                    .map_err(|_| format!("{walls_taken_def} is not a valid number"))?;
                Ok::<(isize, usize, Lines<'_>), String>((size, walls_taken_def, l.lines()))
            },
        )?;
        let corrupted = lines
            .map(|l| {
                let (x, y) = l
                    .split_once(',')
                    .ok_or(format!("{l}: coordinates must be separated by a comma"))?;
                let x = x
                    .parse()
                    .map_err(|_| format!("{x} is not a valid number"))?;
                let y = y
                    .parse()
                    .map_err(|_| format!("{y} is not a valid number"))?;
                Ok::<(isize, isize), String>((x, y))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let walls: FxHashSet<_> = corrupted
            .iter()
            .take(walls_taken.unwrap_or(walls_taken_default))
            .collect();
        let mut edges = FxHashMap::default();
        for y in 0..=size {
            for x in 0..=size {
                for d in DIRECTIONS_4 {
                    let cell = (x + d.0, y + d.1);
                    if cell.0 < 0 || cell.0 > size || cell.1 < 0 || cell.1 > size {
                        continue;
                    }

                    if walls.contains(&cell) {
                        continue;
                    }
                    edges
                        .entry((x, y))
                        .and_modify(|e: &mut Vec<_>| e.push((cell, 1)))
                        .or_insert(vec![(cell, 1)]);
                }
            }
        }
        for y in 0..=size {
            for x in 0..=size {
                if walls.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!()
        }
        Ok(Self {
            size,
            edges,
            corrupted,
            steps: walls_taken.unwrap_or(walls_taken_default) - 1,
        })
    }
    fn find_path_length(&self) -> Option<usize> {
        pathfinding::prelude::dijkstra(
            &(0, 0),
            |n| self.edges.get(n).cloned().unwrap_or(vec![]),
            |n| *n == (self.size, self.size),
        )
        .map(|(_, c)| c)
    }
    fn make_step(&mut self) {
        self.steps += 1;
        let next_corrupted_pos = self.corrupted[self.steps];
        let Some(next_corrupted) = self.edges.remove(&next_corrupted_pos) else {
            return;
        };
        next_corrupted.into_iter().for_each(|(connected, _)| {
            let Some(e) = self.edges.get_mut(&connected) else {
                return;
            };
            e.retain(|(n, _)| *n != next_corrupted_pos);
        });
    }
}

pub fn task_1(file: &str) -> String {
    Ram::from_file(file, None)
        .unwrap()
        .find_path_length()
        .unwrap()
        .to_string()
}

pub fn task_2(file: &str) -> String {
    let mut ram = Ram::from_file(file, None).unwrap();
    while ram.find_path_length().is_some() {
        ram.make_step();
    }

    let blocking = ram.corrupted[ram.steps];
    format!("{},{}", blocking.0, blocking.1)
}
