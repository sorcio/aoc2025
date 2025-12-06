use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, FromAscii, example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn initial(self) -> u64 {
        match self {
            Operation::Add => 0,
            Operation::Multiply => 1,
        }
    }

    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Multiply),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

#[aoc_generator(day6, part1)]
fn parse_part1(input: &str) -> Input {
    let mut numbers = Vec::new();
    let mut operations = Vec::new();
    for line in input.lines() {
        debug_assert!(operations.is_empty());
        if line.starts_with('*') || line.starts_with('+') {
            operations.extend(
                line.split_whitespace()
                    .map(|s| -> Operation { s.chars().next().unwrap().try_into().unwrap() }),
            );
        } else {
            let row = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            numbers.push(row);
        }
    }
    Input {
        numbers,
        operations,
    }
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u64 {
    let cols = input.numbers[0].len();
    (0..cols)
        .map(|col| {
            let op = input.operations[col];
            input
                .numbers
                .iter()
                .fold(op.initial(), |acc, row| op.apply(acc, row[col]))
        })
        .sum()
}

#[aoc_generator(day6, part2)]
fn parse_part2(input: &str) -> Input {
    let input = input.as_bytes();
    let all_lines = input.ascii_lines().collect::<Vec<_>>();
    let (&op_line, lines) = all_lines.split_last().unwrap();
    let mut starts = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    for (i, char) in op_line.iter().enumerate() {
        if let Ok(op) = (*char as char).try_into() {
            starts.push(i);
            operations.push(op);
        }
    }
    let max_line_len = all_lines.iter().map(|line| line.len()).max().unwrap();
    let ends: Vec<usize> = starts
        .iter()
        .skip(1)
        .chain(&[max_line_len + 1])
        .copied()
        // spaces can be ignored
        .map(|end| end - 1)
        .collect();
    assert_eq!(starts.len(), ends.len());
    assert_eq!(starts.len(), operations.len());
    let mut numbers = Vec::new();
    for (&start, &end) in starts.iter().zip(ends.iter()) {
        // Supposed to be right-to-left, top-to-bottom, but since all ops
        // are commutative, we can just read the numbers in any order.
        let mut group = vec![0; end - start];
        for (i, col) in (start..=end).enumerate() {
            for line in lines {
                if let Ok(digit) =
                    <u64 as FromAscii>::from_ascii(&[*line.get(col).unwrap_or(&b' ')])
                {
                    group[i] = group[i] * 10 + digit;
                }
            }
        }
        numbers.push(group);
    }

    Input {
        numbers,
        operations,
    }
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> u64 {
    input
        .numbers
        .iter()
        .zip(&input.operations)
        .map(|(group, op)| group.iter().fold(op.initial(), |acc, &n| op.apply(acc, n)))
        .sum()
}

example_tests! {
    "
    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +
    ",

    parser: super::parse_part1,
    part1 => 4277556,

    parser: super::parse_part2,
    part2 => 3263827,
}

known_input_tests! {
    input: include_str!("../input/2025/day6.txt"),

    parser: super::parse_part1,
    part1 => 4719804927602,

    parser: super::parse_part2,
    part2 => 9608327000261,
}
