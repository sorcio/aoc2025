use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, FromGridLike, example_tests, known_input_tests};

struct Banks {
    width: usize,
    data: Vec<u8>,
}

impl Banks {
    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.data.chunks(self.width)
    }
}

impl FromGridLike for Banks {
    type Cell = u8;
    fn from_cells(data: Vec<Self::Cell>, width: usize, _height: usize) -> Self {
        Banks { width, data }
    }
}

#[aoc_generator(day3)]
fn parse(input: &[u8]) -> Banks {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day3, part1)]
fn part1(input: &Banks) -> u64 {
    let mut total = 0;
    for row in input.rows() {
        let len = row.len();
        let (first, first_byte) = row[..len - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(_, value)| **value)
            .unwrap();
        let second_byte = row[first + 1..].iter().max().unwrap();
        let first_value = first_byte - b'0';
        let second_value = second_byte - b'0';
        total += (first_value * 10 + second_value) as u64;
    }
    total
}

#[aoc(day3, part2)]
fn part2(input: &Banks) -> u64 {
    let n = 12;
    let mut total = 0;
    for row in input.rows() {
        assert!(row.len() >= n);
        let mut numbers = row.to_vec();
        'a: while numbers.len() > n {
            for i in 0..numbers.len() - 1 {
                if numbers[i] < numbers[i + 1] {
                    numbers.remove(i);
                    continue 'a;
                }
            }
            break;
        }
        let row_value = numbers
            .into_iter()
            .take(n)
            .fold(0, |acc, x| acc * 10 + (x - b'0') as u64);
        total += row_value;
    }
    total
}

example_tests! {
    b"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    ",
    part1 => 357,
    part2 => 3121910778619,
}

known_input_tests! {
    input: include_bytes!("../input/2025/day3.txt"),
    part1 => 17074,
    part2 => 169512729575727,
}
