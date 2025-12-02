use std::ops::RangeInclusive;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{MaxDigits, NumberDigitsExt, NumberExt, Parity, example_tests, known_input_tests};

fn parse_interval(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s.split_once('-').unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    start..=end
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim_ascii_end()
        .split(',')
        .map(|s| parse_interval(s))
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[RangeInclusive<u64>]) -> u64 {
    let mut total = 0;
    let mut buf = MaxDigits::<u64>::array();
    for range in input.iter().cloned() {
        for n in range {
            let len = n.digits_in(&mut buf).unwrap();
            let decimal = &buf[..len];
            if len.parity() == Parity::Even {
                let (half1, half2) = decimal.split_at(len / 2);
                if half1 == half2 {
                    total += n;
                }
            }
        }
    }
    total
}

#[aoc(day2, part2)]
fn part2(input: &[RangeInclusive<u64>]) -> u64 {
    let mut total = 0;
    let mut buf = MaxDigits::<u64>::array();
    for range in input.iter().cloned() {
        for n in range {
            let len = n.digits_in(&mut buf).unwrap();
            let decimal = &buf[..len];
            for sublen in 1..=(len / 2) {
                if len % sublen != 0 {
                    continue;
                }
                let first = &decimal[..sublen];
                if (1..(len / sublen)).all(|i| &decimal[sublen * i..sublen * (i + 1)] == first) {
                    total += n;
                    break;
                }
            }
        }
    }
    total
}

example_tests! {
    "
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    ",
    part1 => 1227775554,
    part2 => 4174379265,
}

known_input_tests! {
    input: include_str!("../input/2025/day2.txt"),
    part1 => 44487518055,
    part2 => 53481866137,
}
