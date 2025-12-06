use itertools::Itertools;
use std::str::FromStr;

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        unsafe {
            OperationList::from_str(file)
                .unwrap_unchecked()
                .0
                .into_iter()
                .map(Operation::solve_cons)
                .sum::<u64>()
                .to_string()
        }
    }

    fn task_2(&self, file: &str) -> String {
        unsafe {
            CephalopodOperationList::from_str(file)
                .unwrap_unchecked()
                .0
                .into_iter()
                .map(Operation::solve_cons)
                .sum::<u64>()
                .to_string()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mult,
}

struct Operation {
    sign: Op,
    numbers: Vec<u64>,
}

impl Operation {
    fn solve(&self) -> u64 {
        match self.sign {
            Op::Add => self.numbers.iter().sum::<u64>(),
            Op::Mult => self.numbers.iter().product::<u64>(),
        }
    }
    fn solve_cons(self) -> u64 {
        self.solve()
    }
}

impl TryFrom<char> for Op {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '*' => Ok(Op::Mult),
            '+' => Ok(Op::Add),
            _ => Err("invalid sign"),
        }
    }
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .ok_or("no chars to make Op")
            .and_then(Op::try_from)
    }
}

struct OperationList(Vec<Operation>);
struct CephalopodOperationList(Vec<Operation>);

impl FromStr for CephalopodOperationList {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let numbers_vec = lines
            .next()
            .ok_or("no lines passed")
            .map(|lines| lines.split_whitespace().map(|d| vec![d]).collect_vec())?;
        let numbers_vec: Vec<Vec<&str>> = lines
            .take_while_ref(|l| {
                let first_not_whitespace = l.trim_start();
                !first_not_whitespace.is_empty()
                    && unsafe {
                        first_not_whitespace
                            .chars()
                            .next()
                            .unwrap_unchecked()
                            .is_ascii_digit()
                    }
            })
            .map(|l| l.split_whitespace())
            .fold(numbers_vec, |mut num, split| {
                split.enumerate().for_each(|(i, d)| {
                    num[i].push(d);
                });
                num
            });
        let signs = lines
            .next()
            .ok_or("no signs at last line")?
            .split_whitespace()
            .map(Op::from_str)
            .collect::<Result<Vec<Op>, Self::Err>>()?;
        Ok(Self(
            numbers_vec
                .into_iter()
                .map(|numbers| todo!("cephalopod mapping"))
                .zip(signs)
                .map(|(numbers, sign)| Operation { numbers, sign })
                .collect_vec(),
        ))
    }
}

impl FromStr for OperationList {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let numbers_vec = lines.next().ok_or("no lines passed").and_then(|lines| {
            lines
                .split_whitespace()
                .map(|d| d.parse().map(|d| vec![d]))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "not a number in number line")
        })?;
        let numbers_vec = lines
            .take_while_ref(|l| {
                let first_not_whitespace = l.trim_start();
                !first_not_whitespace.is_empty()
                    && unsafe {
                        first_not_whitespace
                            .chars()
                            .next()
                            .unwrap_unchecked()
                            .is_ascii_digit()
                    }
            })
            .map(|l| l.split_whitespace())
            .try_fold(
                numbers_vec,
                |mut num, split| -> Result<Vec<Vec<u64>>, &'static str> {
                    split
                        .map(|d| d.parse())
                        .enumerate()
                        .try_for_each(|(i, d)| match d {
                            Ok(d) => {
                                num[i].push(d);
                                Ok(())
                            }
                            Err(_) => Err(()),
                        })
                        .map_err(|_| "couldn't parse a number")?;
                    Ok(num)
                },
            )?;
        let signs = lines
            .next()
            .ok_or("no signs at last line")?
            .split_whitespace()
            .map(Op::from_str)
            .collect::<Result<Vec<Op>, Self::Err>>()?;
        Ok(Self(
            numbers_vec
                .into_iter()
                .zip(signs)
                .map(|(numbers, sign)| Operation { numbers, sign })
                .collect_vec(),
        ))
    }
}
