pub struct Solution;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinding::prelude::dijkstra;
use regex::Regex;

use crate::task_fns::SolveMode;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        unsafe {
            file.lines()
                .map(Machine::from_str)
                .map(|r| r.unwrap_unchecked())
                .map(Machine::shortest_turn_on_cons)
                .sum::<usize>()
                .to_string()
        }
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        unsafe {
            file.lines()
                .map(Machine::from_str)
                .map(|r| r.unwrap_unchecked())
                .map(|m| m.solve_lp())
                .sum::<i64>()
                .to_string()
        }
    }
}

lazy_static! {
    // SAFETY - valid regex
    static ref MACHINE_REGEX: Regex =
        unsafe { Regex::new(r"\[(?<lights>[.#]+)\]\s*(?<buttons>(?:\s*\(\d+(?:,\s*\d+)*\))+)\s*\{(?<requirements>\d+(?:,\s*\d+)*)\}").unwrap_unchecked() };
}

#[derive(Debug)]
struct Machine {
    diagram: Vec<Light>,
    button_wiring: Vec<u32>,
    requirements: Vec<i64>,
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = MACHINE_REGEX.captures(s).ok_or("invalid input passed")?;
        let diagram: Vec<Light> = res["lights"].chars().map(Light::try_from).try_collect()?;
        Ok(Self {
            diagram,
            button_wiring: res["buttons"]
                .split_whitespace()
                .map(|s| to_vec_of_numbers(&s[1..(s.len() - 1)]))
                .try_collect()?,
            requirements: res["requirements"]
                .split(',')
                .map(str::parse::<i64>)
                .try_collect()
                .map_err(|_| "couldn't parse joltage")?,
        })
    }
}

impl Machine {
    fn shortest_turn_on_cons(self) -> usize {
        self.shortest_turn_on()
    }
    fn shortest_turn_on(&self) -> usize {
        let goal = self
            .diagram
            .iter()
            .rev()
            .fold(0, |acc, n| acc * 2 + u32::from(n));
        unsafe {
            dijkstra(
                &0,
                |v| {
                    self.button_wiring
                        .iter()
                        .map(|button| ((*v ^ button), 1))
                        .collect_vec()
                },
                |p| *p == goal,
            )
            .unwrap_unchecked()
            .1
        }
    }
    fn solve_lp(&self) -> i64 {
        let num_goals = self.diagram.len();
        let num_buttons = self.button_wiring.len();

        let rows = 2 * num_goals + num_buttons;
        let cols = num_buttons + 1;

        let mut matrix = vec![vec![0.0; cols]; rows];

        for j in 0..num_buttons {
            let row_idx = rows - 1 - j;
            matrix[row_idx][j] = -1.0;
        }

        for (j, &mask) in self.button_wiring.iter().enumerate() {
            for i in 0..num_goals {
                if (mask >> i) & 1 == 1 {
                    matrix[i][j] = 1.0;
                    matrix[i + num_goals][j] = -1.0;
                }
            }
        }

        for i in 0..num_goals {
            let val = self.requirements[i] as f64;
            matrix[i][cols - 1] = val;
            matrix[i + num_goals][cols - 1] = -val;
        }

        let obj_coeffs = vec![1.0; num_buttons];
        solve_ilp_bnb(matrix, &obj_coeffs)
    }
}

fn to_vec_of_numbers(s: &str) -> Result<u32, &'static str> {
    s.split(',')
        .map(str::parse::<u32>)
        .try_fold(0u32, |acc, n| {
            Ok(acc | (1 << n.map_err(|_| "number parsing error")?))
        })
}

#[derive(Debug, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

impl From<u32> for Light {
    fn from(value: u32) -> Self {
        match value {
            0 => Light::Off,
            1 => Light::On,
            _ => panic!(),
        }
    }
}

impl From<&Light> for u32 {
    fn from(value: &Light) -> Self {
        match value {
            Light::On => 1,
            Light::Off => 0,
        }
    }
}

impl TryFrom<char> for Light {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Off),
            '#' => Ok(Self::On),
            _ => Err("wrong symbol for light: only '.' and '#' are valid"),
        }
    }
}

impl FromStr for Light {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.chars().next().ok_or("no character found")?)
    }
}

const EPS: f64 = 1e-9;

fn solve_ilp_bnb(initial_a: Vec<Vec<f64>>, obj_coeffs: &[f64]) -> i64 {
    let mut best_val = f64::INFINITY;
    let mut stack = Vec::new();
    stack.push(initial_a);

    while let Some(current_a) = stack.pop() {
        let (val, x_opt) = simplex(&current_a, obj_coeffs);

        if val == -f64::INFINITY || val >= best_val - EPS {
            continue;
        }

        let mut fractional_idx = None;
        let mut fractional_val = 0.0;

        if let Some(x) = x_opt {
            for (i, &xv) in x.iter().enumerate() {
                if (xv - xv.round()).abs() > EPS {
                    fractional_idx = Some(i);
                    fractional_val = xv;
                    break;
                }
            }

            if let Some(idx) = fractional_idx {
                let floor_v = fractional_val.floor();
                let n_cols = current_a[0].len();

                let mut row1 = vec![0.0; n_cols];
                row1[idx] = 1.0;
                row1[n_cols - 1] = floor_v;
                let mut a1 = current_a.clone();
                a1.push(row1);
                stack.push(a1);

                let ceil_v = fractional_val.ceil();
                let mut row2 = vec![0.0; n_cols];
                row2[idx] = -1.0;
                row2[n_cols - 1] = -ceil_v;
                let mut a2 = current_a.clone();
                a2.push(row2);
                stack.push(a2);
            } else if val < best_val {
                best_val = val;
            }
        }
    }

    if best_val == f64::INFINITY {
        0
    } else {
        best_val.round() as i64
    }
}

fn simplex(lhs: &[Vec<f64>], c: &[f64]) -> (f64, Option<Vec<f64>>) {
    let m = lhs.len();
    let n = lhs[0].len() - 1;

    let mut n_indices: Vec<i32> = (0..n as i32).collect();
    n_indices.push(-1);

    let mut b_indices: Vec<i32> = (n as i32..(n + m) as i32).collect();

    let mut d = vec![vec![0.0; n + 2]; m + 2];

    for i in 0..m {
        d[i][..=n].copy_from_slice(&lhs[i]);
        d[i][n + 1] = -1.0;
    }

    for i in 0..m {
        d[i].swap(n, n + 1);
    }

    d[m][..n].copy_from_slice(&c[..n]);
    d[m + 1][n] = 1.0;

    let pivot =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, r: usize, s: usize| {
            let k = 1.0 / d[r][s];

            for i in 0..m + 2 {
                if i == r {
                    continue;
                }
                for j in 0..n + 2 {
                    if j != s {
                        d[i][j] -= d[r][j] * d[i][s] * k;
                    }
                }
            }

            for i in 0..n + 2 {
                d[r][i] *= k;
            }
            for i in 0..m + 2 {
                d[i][s] *= -k;
            }
            d[r][s] = k;

            std::mem::swap(&mut b_idx[r], &mut n_idx[s]);
        };

    let find =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, p_idx: usize| -> bool {
            loop {
                let mut best_s = usize::MAX;
                let mut best_val = (f64::INFINITY, i32::MAX);

                for i in 0..=n {
                    if p_idx != 0 || n_idx[i] != -1 {
                        let val = d[m + p_idx][i];
                        let key = (val, n_idx[i]);
                        if best_s == usize::MAX
                            || key.0 < best_val.0 - EPS
                            || ((key.0 - best_val.0).abs() <= EPS && key.1 < best_val.1)
                        {
                            best_s = i;
                            best_val = key;
                        }
                    }
                }
                let s = best_s;

                if d[m + p_idx][s] > -EPS {
                    return true;
                }

                let mut best_r = usize::MAX;
                let mut best_r_key = (f64::INFINITY, i32::MAX);

                for i in 0..m {
                    if d[i][s] > EPS {
                        let ratio = d[i][n + 1] / d[i][s];
                        let key = (ratio, b_idx[i]);
                        if best_r == usize::MAX
                            || key.0 < best_r_key.0 - EPS
                            || ((key.0 - best_r_key.0).abs() <= EPS && key.1 < best_r_key.1)
                        {
                            best_r = i;
                            best_r_key = key;
                        }
                    }
                }
                let r = best_r;

                if r == usize::MAX {
                    return false;
                }

                pivot(d, b_idx, n_idx, r, s);
            }
        };

    let mut split_r = 0;
    let mut min_val = d[0][n + 1];
    for i in 1..m {
        if d[i][n + 1] < min_val {
            min_val = d[i][n + 1];
            split_r = i;
        }
    }

    if d[split_r][n + 1] < -EPS {
        pivot(&mut d, &mut b_indices, &mut n_indices, split_r, n);
        if !find(&mut d, &mut b_indices, &mut n_indices, 1) || d[m + 1][n + 1] < -EPS {
            return (-f64::INFINITY, None);
        }
        for i in 0..m {
            if b_indices[i] == -1 {
                let mut best_s = 0;
                let mut best_key = (d[i][0], n_indices[0]);
                for j in 1..n {
                    let key = (d[i][j], n_indices[j]);
                    if key.0 < best_key.0 - EPS
                        || ((key.0 - best_key.0).abs() <= EPS && key.1 < best_key.1)
                    {
                        best_s = j;
                        best_key = key;
                    }
                }
                pivot(&mut d, &mut b_indices, &mut n_indices, i, best_s);
            }
        }
    }

    if find(&mut d, &mut b_indices, &mut n_indices, 0) {
        let mut x = vec![0.0; n];
        for i in 0..m {
            if b_indices[i] >= 0 && (b_indices[i] as usize) < n {
                x[b_indices[i] as usize] = d[i][n + 1];
            }
        }
        let mut sum_val = 0.0;
        for i in 0..n {
            sum_val += c[i] * x[i];
        }
        return (sum_val, Some(x));
    }

    (-f64::INFINITY, None)
}
