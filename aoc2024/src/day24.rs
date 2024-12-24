use std::collections::HashMap;
use std::error::Error;

use itertools::Itertools;

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution, Box<dyn Error>> {
    let (signals, operations) = parse_input(input);
    // print(operations.clone());
    Ok(Solution::default()
        .part1(part1(signals, &operations))
        .part2(part2(&operations)))
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    OR,
    AND,
    XOR,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Operation<'a> {
    a: &'a str,
    b: &'a str,
    op: Op,
    res: &'a str,
}

fn parse_input(input: &str) -> (HashMap<&str, bool>, Vec<Operation>) {
    let (init, operations) = input
        .split_once("\n\n")
        .expect("There should be 2 sections");
    let mut signals = HashMap::new();
    for line in init.lines() {
        signals.insert(&line[..3], line.as_bytes()[5] == b'1');
    }
    let operations = operations.lines().map(|line| {
        let (operation, res) = line.split_once(" -> ").expect("There should be 2 sections");
        let op = match operation.as_bytes()[4] {
            b'O' => Op::OR,
            b'A' => Op::AND,
            b'X' => Op::XOR,
            _ => unreachable!(),
        };
        Operation {
            a: &operation[..3],
            b: &operation[operation.len() - 3..],
            op,
            res,
        }
    });
    (signals, operations.collect())
}

#[must_use]
fn part1<'a>(mut signals: HashMap<&'a str, bool>, operations: &[Operation<'a>]) -> u64 {
    let mut output = 0;
    let mut changed = true;
    let mut processed = vec![false; operations.len()];
    while changed {
        changed = false;
        for (i, operation) in operations.iter().enumerate() {
            if processed[i] {
                continue;
            }
            let Some(&a) = signals.get(&operation.a) else {
                continue;
            };
            let Some(&b) = signals.get(&operation.b) else {
                continue;
            };
            let result = match operation.op {
                Op::OR => a || b,
                Op::AND => a && b,
                Op::XOR => a != b,
            };
            signals.insert(operation.res, result);
            changed = true;
            processed[i] = true;
            if operation.res.as_bytes()[0] == b'z' && result {
                let shift = operation.res[1..]
                    .parse::<u8>()
                    .expect("All signals starting with z should end in a valid number");
                output |= 1 << shift;
            }
        }
    }
    output
}

#[allow(dead_code)]
fn print(mut operations: Vec<Operation>) {
    let first = |x: &str| x.as_bytes()[0];
    let is_input = |x: &str| [b'x', b'y'].contains(&first(x));
    operations.sort_unstable_by_key(|op| {
        if &op.a[1..] == "00" || &op.b[1..] == "00" {
            0
        } else if is_input(op.a) && is_input(op.b) {
            1 + u32::from(op.op == Op::XOR)
        } else {
            3
        }
    });
    for op in operations.iter_mut().filter(|op| op.b < op.a) {
        std::mem::swap(&mut op.a, &mut op.b);
    }
    let mut result = String::from("flowchart TB\n");
    for (i, op) in operations.iter().enumerate() {
        result.push_str(&format!("{}{{{}}} --> {}[{:?}]\n", op.a, op.a, i, op.op));
        result.push_str(&format!("{}{{{}}} --> {}\n", op.b, op.b, i));
        result.push_str(&format!("{} --> {}{{{}}}\n", i, op.res, op.res));
    }
    println!(include_str!("day24-mermaid-template.html"), result);
}

#[must_use]
fn part2(operations: &[Operation]) -> String {
    let first = |x: &str| x.as_bytes()[0];
    let is_input = |x: &str| [b'x', b'y'].contains(&first(x));
    let contains_input = |op, input| {
        operations
            .iter()
            .filter(|operation| operation.op == op)
            .any(|op| [op.a, op.b].contains(input))
    };
    operations
        .iter()
        .filter(|operation| {
            if first(operation.res) == b'z' && operation.op != Op::XOR && operation.res != "z45" {
                return true;
            }
            if first(operation.res) != b'z'
                && operation.op == Op::XOR
                && !is_input(operation.a)
                && !is_input(operation.b)
            {
                return true;
            }
            if is_input(operation.a)
                && is_input(operation.b)
                && &operation.a[1..] != "00"
                && &operation.b[1..] != "00"
            {
                return (operation.op == Op::XOR && !contains_input(Op::XOR, &operation.res))
                    || (operation.op == Op::AND && !contains_input(Op::OR, &operation.res));
            }
            false
        })
        .map(|operation| operation.res)
        .sorted()
        .join(",")
}
