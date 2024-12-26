use core::panic;
use std::{
    iter::once,
    ops::{BitAnd, BitOr, BitXor},
};

use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Xor,
    And,
    Or,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            v => panic!("{v} is not a valid Op type!"),
        }
    }
}

impl Op {
    fn execute<T>(&self, a: T, b: T) -> T
    where
        T: BitXor<Output = T>,
        T: BitAnd<Output = T>,
        T: BitOr<Output = T>,
    {
        match self {
            Op::Xor => a ^ b,
            Op::And => a & b,
            Op::Or => a | b,
        }
    }
}

struct Equation<'a>(&'a str, Op, &'a str);

impl<'a> Equation<'a> {
    fn contains_start_node(&self) -> bool {
        self.inputs().any(is_start_node)
    }
    fn op(&self) -> &Op {
        &self.1
    }
    fn inputs(&self) -> impl Iterator<Item = &'a str> {
        once(self.0).chain(once(self.2))
    }
}

fn is_start_node(n: &str) -> bool {
    n.starts_with("x") || n.starts_with("y")
}
impl<'a> From<&'a str> for Equation<'a> {
    fn from(value: &'a str) -> Self {
        value
            .split_whitespace()
            .tuples()
            .next()
            .map(|(l, op, r)| Self(l, op.into(), r))
            .unwrap()
    }
}

type Values = FxHashMap<String, usize>;
type Equations<'a> = FxHashMap<String, Equation<'a>>;

fn parse<'a, EqType: FromIterator<(String, Equation<'a>)>>(file: &'a str) -> (Values, EqType) {
    let (v_str, e_str) = file.split_once("\n\n").unwrap();
    (
        v_str
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .map(|(n, v)| (n.into(), v.parse().unwrap()))
            .collect(),
        e_str
            .lines()
            .map(|l| l.split_once(" -> ").unwrap())
            .map(|(e, n)| (n.into(), Equation::from(e)))
            .collect(),
    )
}

fn value_of(key: &str, values: &mut Values, equations: &Equations<'_>) -> usize {
    if let Some(v) = values.get(key) {
        return *v;
    }
    let Equation(left, op, right) = equations.get(key).unwrap();
    let res = op.execute(
        value_of(left, values, equations),
        value_of(right, values, equations),
    );
    values.insert(key.into(), res);
    res
}

fn extract_value(prefix: &str, values: &mut Values, equations: &Equations<'_>) -> usize {
    let mut ind = 0;
    let mut key = format!("{prefix}{ind:02}");
    let mut res = 0;
    while equations.contains_key(&key) {
        res += value_of(&key, values, equations) << ind;
        ind += 1;
        key = format!("{prefix}{ind:02}");
    }
    res
}

pub fn task_1(file: &str) -> String {
    let (mut v, eq) = parse(file);
    extract_value("z", &mut v, &eq).to_string()
}

// fixing the Ripple Carry Adder
pub fn task_2(file: &str) -> String {
    let (_, equations) = parse::<Vec<_>>(file);
    let out_num = equations.iter().filter(|(k, _)| k.starts_with("z")).count() - 1;
    let last = format!("z{out_num:02}");
    equations
        .iter()
        .filter_map(|(out, equation)| {
            // all nodes that feed directly into the output that are not XOR (except the last one)
            if ((out.starts_with("z") && *equation.op() != Op::Xor && **out != last)
            // or *could* only feed outs, but feed intermediate instead
                || (*equation.op() == Op::Xor && !out.starts_with("z")))
                && !equation.contains_start_node()
            {
                return Some(out);
            }
            // find XORs that are not fed by x00/y00 and feed into something, that is not XOR
            // (faulty by design of RCA)
            if *equation.op() == Op::Xor
                && equation
                    .inputs()
                    .filter(|i| !i.ends_with("00"))
                    .any(is_start_node)
            {
                return (equations
                    .iter()
                    .all(|(_, e)| *e.op() != Op::Xor || (e.inputs().all(|i| i != *out))))
                .then_some(out);
            }
            // find ANDs that do not feed into ORs
            // (faulty by design of RCA)
            (*equation.op() == Op::And
                && equation.inputs().filter(|&i| i != "x00").any(is_start_node)
                && equations
                    .iter()
                    .all(|(_, e)| *e.op() != Op::Or || (e.inputs().all(|i| i != *out))))
            .then_some(out)
        })
        .sorted()
        .join(",")
}
