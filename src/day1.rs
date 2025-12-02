use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{example_tests, known_input_tests};

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left(u16),
    Right(u16),
}

#[derive(Debug, Clone, Copy)]
struct Dial(u16);

impl Dial {
    const START: u16 = 50;
    const CAP: u16 = 100;

    const fn new() -> Self {
        Dial(Self::START)
    }

    const fn rotate(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Left(amount) => Dial((self.0 + Self::CAP - amount % Self::CAP) % Self::CAP),
            Rotation::Right(amount) => Dial((self.0 + amount % Self::CAP) % Self::CAP),
        }
    }

    const fn rotate_counting_zeroes(self, rotation: Rotation) -> (Self, u16) {
        match rotation {
            Rotation::Left(amount) => {
                let extra_turns = amount / Self::CAP;
                let net_amount = amount % Self::CAP;
                let new_dial = Dial((self.0 + Self::CAP - net_amount) % Self::CAP);
                let mut zeroes = extra_turns;
                if self.0 != 0 && net_amount >= self.0 {
                    zeroes += 1;
                }
                (new_dial, zeroes)
            }
            Rotation::Right(amount) => {
                let extra_turns = amount / Self::CAP;
                let net_amount = amount % Self::CAP;
                let new_dial = Dial((self.0 + net_amount) % Self::CAP);
                let mut zeroes = extra_turns;
                if self.0 != 0 && self.0 + net_amount >= Self::CAP {
                    zeroes += 1;
                }
                (new_dial, zeroes)
            }
        }
    }

    const fn value(&self) -> u16 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_counting_zeroes() {
        let dial = Dial(50);
        let (new_dial, zeroes) = dial.rotate_counting_zeroes(Rotation::Right(1000));
        assert_eq!(new_dial.value(), 50);
        assert_eq!(zeroes, 10);
    }

    #[test]
    fn test_rotate_counting_zeroes_left() {
        let dial = Dial(50);
        let (new_dial, zeroes) = dial.rotate_counting_zeroes(Rotation::Left(1000));
        assert_eq!(new_dial.value(), 50);
        assert_eq!(zeroes, 10);
    }

    #[test]
    fn test_rotate_counting_zeroes_2() {
        let dial = Dial(50);
        let (new_dial, zeroes) = dial.rotate_counting_zeroes(Rotation::Right(60));
        assert_eq!(new_dial.value(), 10);
        assert_eq!(zeroes, 1);

        let (new_dial, zeroes) = dial.rotate_counting_zeroes(Rotation::Left(60));
        assert_eq!(new_dial.value(), 90);
        assert_eq!(zeroes, 1);
    }

    #[test]
    fn test_rotate_counting_zeroes_3() {
        let dial = Dial(50);
        let (new_dial, zeroes) = dial.rotate_counting_zeroes(Rotation::Right(25));
        assert_eq!(new_dial.value(), 75);
        assert_eq!(zeroes, 0);
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .flat_map(|line| {
            let (dir, rest) = line.split_at(1);
            match dir {
                "L" => Some(Rotation::Left(rest.parse().unwrap())),
                "R" => Some(Rotation::Right(rest.parse().unwrap())),
                _ => None,
            }
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Rotation]) -> u32 {
    let mut dial = Dial::new();
    let mut zero_count = 0;
    for rotation in input {
        dial = dial.rotate(*rotation);
        if dial.value() == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

#[aoc(day1, part2)]
fn part2(input: &[Rotation]) -> u32 {
    let mut dial = Dial::new();
    let mut zero_count = 0;
    for rotation in input {
        let zeroes;
        (dial, zeroes) = dial.rotate_counting_zeroes(*rotation);
        zero_count += zeroes as u32;
    }
    zero_count
}

example_tests! {
    "
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82
    ",

    part1 => 3,
    part2 => 6,
}

known_input_tests! {
    input: include_str!("../input/2025/day1.txt"),
    part1 => 1064,
    part2 => 6122,
}
