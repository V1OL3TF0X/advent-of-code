use crate::task_fns::SolveMode;
use std::{fmt::Debug, marker::PhantomData, str::FromStr};

trait OperationChain: Debug {
    fn as_next(op: &Operation) -> Operation;
}

#[derive(Debug)]
struct OperationTask1;
#[derive(Debug)]
struct OperationTask2;

impl OperationChain for OperationTask1 {
    fn as_next(op: &Operation) -> Operation {
        match op {
            Operation::Add => Operation::Multiply,
            Operation::Multiply => Operation::Add,
            v => panic!("invalid value: {v:?}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Operation {
    Add,
    Concat,
    Multiply,
}
impl Operation {
    fn compute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concat => a * 10_u64.pow(b.to_string().len() as u32) + b,
        }
    }
}

impl OperationChain for OperationTask2 {
    fn as_next(op: &Operation) -> Operation {
        match op {
            Operation::Add => Operation::Multiply,
            Operation::Multiply => Operation::Concat,
            Operation::Concat => Operation::Add,
        }
    }
}

#[derive(Debug)]
struct Equation {
    sum: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((sum, nums)) = s.split_once(':') else {
            return Err("invalid input - couldn't find ':'");
        };
        let sum = sum
            .parse::<u64>()
            .map_err(|_| "invalid input - couldn't parse sum")?;
        let numbers = nums
            .split_whitespace()
            .flat_map(str::parse::<u64>)
            .collect::<Vec<_>>();
        Ok(Self { sum, numbers })
    }
}

impl Equation {
    fn iter<Op: OperationChain>(&self) -> EquationIter<'_, Op> {
        EquationIter {
            equation: self,
            state: vec![Operation::Add; self.numbers.len() - 1],
            last_switched: self.numbers.len() - 2,
            checked_last_state: false,
            _type: PhantomData,
        }
    }
}

#[derive(Debug)]
struct EquationIter<'a, Op: OperationChain> {
    equation: &'a Equation,
    state: Vec<Operation>,
    last_switched: usize,
    checked_last_state: bool,
    _type: PhantomData<Op>,
}

impl<'a, Op: OperationChain> EquationIter<'a, Op> {
    fn compute_value(&self) -> u64 {
        self.state
            .iter()
            .enumerate()
            .fold(self.equation.numbers[0], |res, (i, op)| {
                op.compute(res, self.equation.numbers[i + 1])
            })
    }
    fn advance_operation_permutation(&mut self) {
        if self.checked_last_state {
            return;
        }
        let next_for_rightmost = Op::as_next(&self.state[self.last_switched]);
        self.state[self.last_switched] = next_for_rightmost;
        if next_for_rightmost != Operation::Add {
            return;
        }
        // reset "bits" before
        ((self.last_switched + 1)..self.state.len()).for_each(|i| self.state[i] = Operation::Add);
        if self.last_switched == 0 {
            self.checked_last_state = true;
            return;
        }
        self.last_switched -= 1;
        loop {
            let next = Op::as_next(&self.state[self.last_switched]);
            self.state[self.last_switched] = next;
            if next != Operation::Add {
                self.last_switched = self.state.len() - 1;
                return;
            }
            if self.last_switched == 0 {
                self.checked_last_state = true;
                return;
            }
            self.last_switched -= 1;
        }
    }
}

impl<'a, Op: OperationChain> Iterator for EquationIter<'a, Op> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.checked_last_state {
            return None;
        }
        let next = self.compute_value();
        if !self.checked_last_state {
            self.advance_operation_permutation();
            return Some(next);
        }
        None
    }
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        file.lines()
            .flat_map(Equation::from_str)
            .filter_map(|e| {
                e.iter::<OperationTask1>()
                    .find(|potential_sum| *potential_sum == e.sum)
            })
            .sum::<u64>()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        file.lines()
            .flat_map(Equation::from_str)
            .filter_map(|e| {
                e.iter::<OperationTask2>()
                    .find(|potential_sum| *potential_sum == e.sum)
            })
            .sum::<u64>()
            .to_string()
    }
}
