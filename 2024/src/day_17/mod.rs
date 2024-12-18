use core::panic;
use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
}

impl FromStr for Computer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .and_then(|(_, n)| n.parse().ok())
            .ok_or_else(|| "Not a valid line for A register".to_string())?;
        let b = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .and_then(|(_, n)| n.parse().ok())
            .ok_or_else(|| "Not a valid line for B register".to_string())?;
        let c = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .and_then(|(_, n)| n.parse().ok())
            .ok_or_else(|| "Not a valid line for C register".to_string())?;
        let _ = lines.next();
        let program = lines
            .next()
            .and_then(|l| l.split_once(": "))
            .and_then(|(_, p)| {
                p.split(',')
                    .map(|d| d.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .ok_or_else(|| "Not a valid line for program input".to_string())?;

        Ok(Self { a, b, c, program })
    }
}

struct Output(Vec<u64>);

impl Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..(self.0.len() - 1) {
            write!(f, "{},", self.0[i])?;
        }
        write!(f, "{}", self.0.last().unwrap())
    }
}

impl Computer {
    fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.a = a;
        self.b = b;
        self.c = c;
    }
    fn combo(&self, literal_value: u64) -> u64 {
        match literal_value {
            v if v < 4 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid value to get combo op"),
        }
    }
    fn a_div_pow_2(&self, literal_value: u64) -> u64 {
        let exp = self.combo(literal_value);
        self.a / 2_u64.pow(exp as u32)
    }
    fn run(&mut self) -> Output {
        let mut output = vec![];

        let mut pos = 0;
        while pos < self.program.len() - 1 {
            let literal_value = self.program[pos + 1];
            let mut next = pos + 2;
            match self.program[pos] {
                0 => self.a = self.a_div_pow_2(literal_value),
                1 => self.b ^= literal_value,
                2 => {
                    self.b = self.combo(literal_value) % 8;
                }
                3 => {
                    if self.a != 0 {
                        next = literal_value as usize;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    let v = self.combo(literal_value);
                    output.push(v % 8);
                }
                6 => self.b = self.a_div_pow_2(literal_value),
                7 => self.c = self.a_div_pow_2(literal_value),
                _ => {
                    panic!("invalid instruction op");
                }
            }
            pos = next;
        }
        Output(output)
    }
    fn find_min_self(&mut self) -> usize {
        let mut q = vec![(0, self.program.len() - 1)];
        while let Some((a, offset)) = q.pop() {
            'inner: for i in 0..8 {
                let next_a = a * 8 + i;
                self.reset(next_a, 0, 0);
                let Output(nums) = self.run();
                if nums != self.program[offset..] {
                    continue 'inner;
                }
                if offset == 0 {
                    return next_a as usize;
                }
                println!("found a potential solution: {next_a} -> {nums:?} for offset {offset}");
                q.push((next_a, offset - 1));
            }
        }
        panic!("solution doesn't exist!")
    }
}

pub fn task_1(file: &str) -> String {
    format!("{:?}", Computer::from_str(file).unwrap().run())
}

pub fn task_2(file: &str) -> String {
    Computer::from_str(file)
        .unwrap()
        .find_min_self()
        .to_string()
}
