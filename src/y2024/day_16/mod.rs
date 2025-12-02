use std::{iter::once, str::FromStr};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::plane::{Direction, Point};

type EdgeListWithDirectionAndWeight =
    FxHashMap<(Point, Direction), Vec<((Point, Direction), usize)>>;

struct Labitynth {
    edges: EdgeListWithDirectionAndWeight,
    start: Point,
    end: Point,
    map: Vec<Vec<char>>,
}

impl Labitynth {
    fn add_edges(&mut self, p: Point) {
        [Direction::Right, Direction::Bottom]
            .into_iter()
            .for_each(|d| {
                let peek = &p + d;
                match self.map[peek.1 as usize][peek.0 as usize] {
                    '#' => {}
                    '.' | 'S' | 'E' => {
                        //> >, > ^ and > v
                        for w in d
                            .one_turn_away()
                            .into_iter()
                            .map(|d| (d, 1000))
                            .chain(once((d, 0)))
                            .combinations_with_replacement(2)
                        {
                            let (from, from_cost) = w[0];
                            let (to, to_cost) = w[1];
                            self.edges
                                .entry((p, from))
                                .and_modify(|v| v.push(((peek, to), from_cost + to_cost + 1)))
                                .or_insert(vec![((peek, to), from_cost + to_cost + 1)]);
                        }
                        //> >, > ^ and > v
                        for w in d
                            .one_turn_away()
                            .into_iter()
                            .map(|d| (d, 1000))
                            .chain(once((d.rev(), 0)))
                            .combinations_with_replacement(2)
                        {
                            let (from, from_cost) = w[0];
                            let (to, to_cost) = w[1];
                            self.edges
                                .entry((peek, from))
                                .and_modify(|v| v.push(((p, to), from_cost + to_cost + 1)))
                                .or_insert(vec![((p, to), from_cost + to_cost + 1)]);
                        }
                    }
                    v => panic!("map shouldn't have invalid cell contents! {v}"),
                }
            });
    }
    fn solve_dijkstra(&self) -> usize {
        pathfinding::prelude::dijkstra(
            &(self.start, Direction::Right),
            |n| self.edges.get(n).cloned().unwrap_or(vec![]),
            |(n, _)| *n == self.end,
        )
        .map(|(_, c)| c)
        .unwrap()
    }
    fn solve_all_shortest(&self) -> usize {
        // love me a good brute force
        let shortest = pathfinding::prelude::yen(
            &(self.start, Direction::Right),
            |n| self.edges.get(n).cloned().unwrap_or(vec![]),
            |(n, _)| *n == self.end,
            100,
        );
        let s_cost = shortest[0].1;
        let unique = shortest
            .into_iter()
            .take_while(|(_, c)| *c == s_cost)
            .flat_map(|(p, _)| p.into_iter().map(|(n, _)| n))
            .collect::<FxHashSet<_>>();
        for (y, l) in self.map.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if unique.contains(&Point::new(x, y)) {
                    print!("O");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
        unique.len()
    }
}

impl FromStr for Labitynth {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut s = Self {
            edges: FxHashMap::default(),
            start: Point(-1, -1),
            end: Point(-1, -1),
            map: string.lines().map(|l| l.chars().collect()).collect(),
        };
        for y in 0..s.map.len() {
            for x in 0..s.map[y].len() {
                let c = s.map[y][x];
                match c {
                    '#' => {}
                    'S' => {
                        s.start = Point::new(x, y);
                        s.add_edges(Point::new(x, y))
                    }
                    'E' => {
                        s.end = Point::new(x, y);
                        s.add_edges(Point::new(x, y))
                    }
                    '.' => s.add_edges(Point::new(x, y)),
                    v => return Err(format!("invalid value passed as cell: {v}")),
                }
            }
        }
        if s.start.0 < 0 || s.start.1 < 0 {
            return Err(String::from("Start point not found!"));
        }
        if s.end.0 < 0 || s.end.1 < 0 {
            return Err(String::from("End point not found!"));
        }
        Ok(s)
    }
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        Labitynth::from_str(file)
            .unwrap()
            .solve_dijkstra()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        Labitynth::from_str(file)
            .unwrap()
            .solve_all_shortest()
            .to_string()
    }
}
