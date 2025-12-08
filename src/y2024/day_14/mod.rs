use crate::task_fns::SolveMode;
use std::fmt::Debug;

use regex::Regex;
use rustc_hash::FxHashMap;

const ROBOT_REGEX_STR: &str = r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)";
struct Bathroom {
    width: i64,
    height: i64,
    q1: usize,
    q2: usize,
    q3: usize,
    q4: usize,
}

impl Bathroom {
    fn new(width: i64, height: i64) -> Self {
        Self {
            width,
            height,
            q1: 0,
            q2: 0,
            q3: 0,
            q4: 0,
        }
    }
    fn insert_after_sec(mut self, x: i64, y: i64, vx: i64, vy: i64, time: i64) -> Self {
        let mut pos_x = (x + (vx * time) % self.width) % self.width;
        let mut pos_y = (y + (vy * time) % self.height) % self.height;
        if pos_x < 0 {
            pos_x += self.width;
        }
        if pos_y < 0 {
            pos_y += self.height;
        }

        match (pos_x.cmp(&(self.width / 2)), pos_y.cmp(&(self.height / 2))) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => self.q1 += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => self.q3 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => self.q2 += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => self.q4 += 1,
            _ => {}
        };
        self
    }

    fn safety_factor(&self) -> usize {
        self.q1 * self.q2 * self.q3 * self.q4
    }
}

fn n(num: &str) -> i64 {
    num.parse().unwrap()
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        let robot_regex = Regex::new(ROBOT_REGEX_STR).unwrap();
        let mut lines = file.lines();
        let (width, height) = lines
            .next()
            .unwrap()
            .split_once(' ')
            .map(|(w, h)| (w.parse().unwrap(), h.parse().unwrap()))
            .unwrap();
        lines
            .fold(Bathroom::new(width, height), |bathroom, l| {
                let cap = robot_regex.captures(l).unwrap();
                bathroom.insert_after_sec(
                    n(&cap["px"]),
                    n(&cap["py"]),
                    n(&cap["vx"]),
                    n(&cap["vy"]),
                    100,
                )
            })
            .safety_factor()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        let robot_regex = Regex::new(ROBOT_REGEX_STR).unwrap();
        let mut lines = file.lines();
        let (width, height) = lines
            .next()
            .unwrap()
            .split_once(' ')
            .map(|(w, h)| (w.parse().unwrap(), h.parse().unwrap()))
            .unwrap();
        let mut bs = BathroomSnapshot::new(
            width,
            height,
            lines.map(|l| {
                let cap = robot_regex.captures(l).unwrap();
                (n(&cap["px"]), n(&cap["py"]), n(&cap["vx"]), n(&cap["vy"]))
            }),
        );
        let mut i = 0;
        loop {
            println!("{bs:?}");
            if bs.robots.values().all(|v| v.len() == 1) {
                return i.to_string();
            }
            i += 1;
            bs.progress();
        }
    }
}

struct BathroomSnapshot {
    robots: FxHashMap<(i64, i64), Vec<(i64, i64)>>,
    width: i64,
    height: i64,
}

impl BathroomSnapshot {
    fn new(
        width: i64,
        height: i64,
        robots: impl IntoIterator<Item = (i64, i64, i64, i64)>,
    ) -> Self {
        Self {
            robots: robots
                .into_iter()
                .fold(FxHashMap::default(), |mut r, (x, y, vx, vy)| {
                    r.entry((x, y))
                        .and_modify(|v| v.push((vx, vy)))
                        .or_insert(vec![(vx, vy)]);
                    r
                }),
            width,
            height,
        }
    }
    fn progress(&mut self) {
        let mut new_s = FxHashMap::default();
        std::mem::swap(&mut new_s, &mut self.robots);
        new_s.into_iter().for_each(|((x, y), v_list)| {
            v_list.into_iter().for_each(|(vx, vy)| {
                let mut new_x = x + vx;
                let mut new_y = y + vy;
                if new_x < 0 {
                    new_x += self.width;
                } else if new_x >= self.width {
                    new_x -= self.width;
                }
                if new_y < 0 {
                    new_y += self.height;
                } else if new_y >= self.height {
                    new_y -= self.height;
                }
                self.robots
                    .entry((new_x, new_y))
                    .and_modify(|v| v.push((vx, vy)))
                    .or_insert(vec![(vx, vy)]);
            })
        });
    }
}

impl Debug for BathroomSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(v) = self.robots.get(&(x, y)) {
                    write!(f, "{:X}", v.len())?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
