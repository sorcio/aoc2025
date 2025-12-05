use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

#[derive(Debug, Clone)]
struct Input {
    intervals: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let mut intervals = Vec::new();
    let mut ids = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (start, end) = line.split_once('-').unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        intervals.push(start..=end);
    }

    while let Some(line) = lines.next() {
        ids.push(line.parse().unwrap());
    }

    Input { intervals, ids }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> usize {
    input
        .ids
        .iter()
        .filter(|&&id| {
            input
                .intervals
                .iter()
                .any(|interval| interval.contains(&id))
        })
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u64 {
    let mut intervals = input.intervals.clone();
    intervals.sort_unstable_by_key(|interval| *interval.start());
    sum_intervals_overlapping(&intervals)
}

fn sum_intervals_overlapping(intervals: &[RangeInclusive<u64>]) -> u64 {
    debug_assert!(intervals.is_sorted_by_key(|i| *i.start()));
    let mut result = 0;
    let mut last_end = 0;
    for interval in intervals {
        let non_overlapping_part = if *interval.start() < last_end + 1 {
            (*interval.end() + 1).saturating_sub(last_end)
        } else {
            (*interval.end() + 1) - *interval.start()
        };
        result += non_overlapping_part;
        last_end = last_end.max(*interval.end() + 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_intervals_non_overlapping() {
        let intervals = vec![1..=3, 4..=7, 9..=11];
        assert_eq!(sum_intervals_overlapping(&intervals), 3 + 4 + 3);
    }

    #[test]
    fn test_sum_intervals_overlapping() {
        let intervals = vec![1..=10, 5..=15];
        assert_eq!(sum_intervals_overlapping(&intervals), 10 + 5);
        let intervals = vec![1..=10, 5..=15, 21..=30];
        assert_eq!(sum_intervals_overlapping(&intervals), 10 + 5 + 10);
    }
}

example_tests! {
    "
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    ",
    part1 => 3,
    part2 => 14,
}

known_input_tests! {
    input: include_str!("../input/2025/day5.txt"),
    part1 => 638,
    part2 => 352946349407338,
}
