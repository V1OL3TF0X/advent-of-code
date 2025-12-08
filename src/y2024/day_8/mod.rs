use crate::task_fns::SolveMode;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

struct Antennas {
    locations: HashMap<char, Vec<Point>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Antennas {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locations = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        s.lines().enumerate().for_each(|(y, l)| {
            height += 1;
            l.chars()
                .enumerate()
                .inspect(|_| {
                    width += 1;
                })
                .filter(|(_, c)| *c != '.')
                .for_each(|(x, c)| {
                    locations
                        .entry(c)
                        .and_modify(|antennas: &mut Vec<Point>| antennas.push(Point { x, y }))
                        .or_insert(vec![Point { x, y }]);
                });
        });
        width /= height;
        Ok(Self {
            locations,
            width,
            height,
        })
    }
}

impl Antennas {
    fn get_antinodes_1(&self) -> HashSet<Point> {
        self.locations
            .values()
            .flat_map(|v| {
                v.iter()
                    .enumerate()
                    .flat_map(|(i, p1)| v.iter().skip(i + 1).map(move |p2| (p1, p2)))
            })
            .flat_map(|(a1, a2)| {
                let diff_x = a1.x.abs_diff(a2.x);
                let diff_y = a1.y.abs_diff(a2.y);
                let mut res = vec![];
                match (a1.x.cmp(&a2.x), a1.y.cmp(&a2.y)) {
                    (Ordering::Greater, Ordering::Greater) => {
                        if a1.y + diff_y < self.height && a1.x + diff_x < self.width {
                            res.push(Point::new(a1.x + diff_x, a1.y + diff_y));
                        }
                        if a2.y >= diff_y && a2.x >= diff_x {
                            res.push(Point::new(a2.x - diff_x, a2.y - diff_y));
                        }
                    }
                    (Ordering::Less, Ordering::Less) => {
                        if a2.y + diff_y < self.height && a2.x + diff_x < self.width {
                            res.push(Point::new(a2.x + diff_x, a2.y + diff_y));
                        }
                        if a1.y >= diff_y && a1.x >= diff_x {
                            res.push(Point::new(a1.x - diff_x, a1.y - diff_y));
                        }
                    }
                    (Ordering::Less, Ordering::Equal) => {
                        if a1.x >= diff_x {
                            res.push(Point::new(a1.x - diff_x, a1.y));
                        }
                        if a2.x + diff_x < self.width {
                            res.push(Point::new(a2.x + diff_x, a2.y));
                        }
                    }
                    (Ordering::Greater, Ordering::Equal) => {
                        if a2.x >= diff_x {
                            res.push(Point::new(a2.x - diff_x, a2.y));
                        }
                        if a1.x + diff_x < self.width {
                            res.push(Point::new(a1.x + diff_x, a1.y));
                        }
                    }
                    (Ordering::Equal, Ordering::Less) => {
                        if a1.y >= diff_y {
                            res.push(Point::new(a1.x, a1.y - diff_y));
                        }
                        if a2.y + diff_y < self.height {
                            res.push(Point::new(a2.x, a2.y + diff_y));
                        }
                    }
                    (Ordering::Equal, Ordering::Greater) => {
                        if a2.y >= diff_y {
                            res.push(Point::new(a2.x, a2.y - diff_y));
                        }
                        if a1.y + diff_y < self.height {
                            res.push(Point::new(a1.x, a1.y + diff_y));
                        }
                    }
                    (Ordering::Less, Ordering::Greater) => {
                        if a1.x >= diff_x && a1.y + diff_y < self.height {
                            res.push(Point::new(a1.x - diff_x, a1.y + diff_y));
                        }
                        if a2.x + diff_x < self.width && a2.y >= diff_y {
                            res.push(Point::new(a2.x + diff_x, a2.y - diff_y));
                        }
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        if a2.x >= diff_x && a2.y + diff_y < self.height {
                            res.push(Point::new(a2.x - diff_x, a2.y + diff_y));
                        }
                        if a1.x + diff_x < self.width && a1.y >= diff_y {
                            res.push(Point::new(a1.x + diff_x, a1.y - diff_y));
                        }
                    }
                    (Ordering::Equal, Ordering::Equal) => {}
                };
                res
            })
            .collect()
    }
    fn get_antinodes_2(&self) -> HashSet<Point> {
        self.locations
            .values()
            .flat_map(|v| {
                v.iter()
                    .enumerate()
                    .flat_map(|(i, p1)| v.iter().skip(i + 1).map(move |p2| (p1, p2)))
            })
            .flat_map(|(a1, a2)| {
                let diff_x = a1.x.abs_diff(a2.x);
                let diff_y = a1.y.abs_diff(a2.y);
                let mut res = vec![];
                let mut start_a1x = a1.x;
                let mut start_a2x = a2.x;
                let mut start_a1y = a1.y;
                let mut start_a2y = a2.y;
                match (a1.x.cmp(&a2.x), a1.y.cmp(&a2.y)) {
                    (Ordering::Greater, Ordering::Greater) => {
                        while start_a1y + diff_y < self.height && start_a1x + diff_x < self.width {
                            start_a1y += diff_y;
                            start_a1x += diff_x;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                        while start_a2y >= diff_y && start_a2x >= diff_x {
                            start_a2x -= diff_x;
                            start_a2y -= diff_y;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                    }
                    (Ordering::Less, Ordering::Less) => {
                        while start_a2y + diff_y < self.height && start_a2x + diff_x < self.width {
                            start_a2y += diff_y;
                            start_a2x += diff_x;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                        while start_a1y >= diff_y && start_a1x >= diff_x {
                            start_a1x -= diff_x;
                            start_a1y -= diff_y;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                    }
                    (Ordering::Less, Ordering::Equal) => {
                        while start_a1x >= diff_x {
                            start_a1x -= diff_x;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                        if start_a2x + diff_x < self.width {
                            start_a2x += diff_x;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                    }
                    (Ordering::Greater, Ordering::Equal) => {
                        while start_a2x >= diff_x {
                            start_a2x -= diff_x;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                        while start_a1x + diff_x < self.width {
                            start_a1x += diff_x;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                    }
                    (Ordering::Equal, Ordering::Less) => {
                        while start_a1y >= diff_y {
                            start_a1y -= diff_y;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                        while start_a2y + diff_y < self.height {
                            start_a2y += diff_y;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                    }
                    (Ordering::Equal, Ordering::Greater) => {
                        while start_a2y >= diff_y {
                            start_a2y -= diff_y;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                        while start_a1y + diff_y < self.height {
                            start_a1y += diff_y;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                    }
                    (Ordering::Less, Ordering::Greater) => {
                        while start_a1x >= diff_x && start_a1y + diff_y < self.height {
                            start_a1x -= diff_x;
                            start_a1y += diff_y;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                        while start_a2x + diff_x < self.width && start_a2y >= diff_y {
                            start_a2x += diff_x;
                            start_a2y -= diff_y;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                    }
                    (Ordering::Greater, Ordering::Less) => {
                        while start_a2x >= diff_x && start_a2y + diff_y < self.height {
                            start_a2x -= diff_x;
                            start_a2y += diff_y;
                            res.push(Point::new(start_a2x, start_a2y));
                        }
                        while start_a1x + diff_x < self.width && start_a1y >= diff_y {
                            start_a1x += diff_x;
                            start_a1y -= diff_y;
                            res.push(Point::new(start_a1x, start_a1y));
                        }
                    }
                    (Ordering::Equal, Ordering::Equal) => {}
                };
                res
            })
            .chain(self.locations.values().flatten().copied())
            .collect()
    }
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        Antennas::from_str(file)
            .unwrap()
            .get_antinodes_1()
            .len()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        Antennas::from_str(file)
            .unwrap()
            .get_antinodes_2()
            .len()
            .to_string()
    }
}
