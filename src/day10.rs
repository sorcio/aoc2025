use std::{collections::VecDeque, fmt::Display, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{NumberExt, example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pattern(u16);

impl Pattern {
    fn all_zero() -> Self {
        Pattern(0)
    }

    fn from_machine_config_string(input: &str) -> (Self, u8) {
        // [.###.#] = Pattern(0b011101)
        let mut bits = 0;
        let mut bit_count = 0;
        let input = input.strip_prefix('[').unwrap();
        let input = input.strip_suffix(']').unwrap();
        for c in input.chars() {
            bits = bits << 1 | ((c == '#') as u16);
            bit_count += 1;
        }
        (Pattern(bits), bit_count)
    }

    fn from_button_wiring_string(input: &str, bit_count: u8) -> Self {
        // (0,2,3,4) = Pattern(0b011101)
        let mut bits = 0;
        let input = input.strip_prefix('(').unwrap();
        let input = input.strip_suffix(')').unwrap();
        for c in input.split(',') {
            bits |= 1 << (bit_count - c.parse::<u8>().unwrap() - 1);
        }
        Pattern(bits)
    }

    fn click(self, button: Pattern) -> Self {
        Self(self.0 ^ button.0)
    }

    fn as_index(self) -> usize {
        self.0.into()
    }

    fn to_string(self, bit_count: u8) -> String {
        let mut s = String::new();
        for i in (0..bit_count).rev() {
            s.push(if self.0 & (1 << i) != 0 { '#' } else { '.' });
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JoltageState([u16; 10]);

impl JoltageState {
    fn zero() -> Self {
        Self([0; 10])
    }

    fn from_expected_joltage(input: &str) -> Option<Self> {
        let input = input.strip_prefix('{')?;
        let input = input.strip_suffix('}').unwrap();
        let mut state = Self::zero();
        for (i, val) in input
            .split(',')
            .rev()
            .map(|s| s.parse::<u16>().unwrap())
            .enumerate()
        {
            state.0[i] = val;
        }
        Some(state)
    }
}

#[derive(Debug, Clone)]
struct Machine {
    bit_count: u8,
    expected_state: Pattern,
    buttons: Vec<Pattern>,
    expected_joltage: JoltageState,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let mut expected_joltage = JoltageState::zero();
        let mut buttons = vec![];
        let (expected_state, bit_count) =
            Pattern::from_machine_config_string(parts.next().unwrap());
        for part in parts {
            if let Some(j) = JoltageState::from_expected_joltage(part) {
                expected_joltage = j;
            } else {
                buttons.push(Pattern::from_button_wiring_string(part, bit_count));
            }
        }
        Ok(Machine {
            bit_count,
            expected_state,
            buttons,
            expected_joltage,
        })
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_min_clicks_to_switch_on(machine: &Machine) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = [false; 1024];
    queue.push_back((Pattern::all_zero(), 0));
    visited[Pattern::all_zero().as_index()] = true;

    println!(
        "+ Starting search; expected: {}",
        machine.expected_state.to_string(machine.bit_count)
    );
    while let Some((state, clicks)) = queue.pop_front() {
        println!(
            "State: {}; clicks: {clicks}",
            state.to_string(machine.bit_count)
        );
        if state == machine.expected_state {
            return clicks;
        }
        for &button in &machine.buttons {
            let next_state = state.click(button);
            let next_clicks = clicks + 1;
            if next_state == machine.expected_state {
                println!("Found solution! {next_clicks} clicks");
                return next_clicks;
            }
            if !visited[next_state.as_index()] {
                visited[next_state.as_index()] = true;
                queue.push_back((next_state, next_clicks));
            }
        }
    }

    panic!("No solution found");
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    input.iter().map(find_min_clicks_to_switch_on).sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Matrix<T> {
    rows: u8,
    cols: u8,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    fn new(rows: u8, cols: u8) -> Self
    where
        T: Default,
    {
        let data = vec![T::default(); rows as usize * cols as usize];
        Self { rows, cols, data }
    }

    fn get(&self, row: u8, col: u8) -> T {
        self.data[row as usize * self.cols as usize + col as usize]
    }

    fn set(&mut self, row: u8, col: u8, value: T) {
        self.data[row as usize * self.cols as usize + col as usize] = value;
    }

    fn swap_row(&mut self, row_a: u8, row_b: u8) {
        if row_a == row_b {
            return;
        }
        let row1 = row_a.min(row_b) as usize;
        let row2 = row_a.max(row_b) as usize;
        let row2_start = row2 * self.cols as usize;
        let (part1, part2) = self.data.split_at_mut(row2_start);
        let row1_start = row1 * self.cols as usize;
        let row1_end = (row1 + 1) * self.cols as usize;
        part1[row1_start..row1_end].swap_with_slice(&mut part2[..self.cols as usize]);
    }

    fn divide_row(&mut self, row: u8, divisor: T)
    where
        T: std::ops::DivAssign,
    {
        let row_start = row as usize * self.cols as usize;
        let row_end = (row as usize + 1) * self.cols as usize;
        for x in self.data[row_start..row_end].iter_mut() {
            *x /= divisor
        }
    }

    fn subtract_from_row(&mut self, row1: u8, row2: u8, by: T)
    where
        T: std::ops::SubAssign + std::ops::Mul<Output = T>,
    {
        let row1_start = row1 as usize * self.cols as usize;
        let row2_start = row2 as usize * self.cols as usize;
        for i in 0..self.cols as usize {
            let v2 = self.data[row2_start + i];
            self.data[row1_start + i] -= v2 * by;
        }
    }

    fn reduced_row_echelon_form(&mut self)
    where
        T: std::ops::DivAssign
            + std::ops::SubAssign
            + std::ops::Mul<Output = T>
            + std::cmp::PartialEq
            + Default
            + Display,
    {
        let mut lead = 0;
        for row in 0..self.rows {
            if lead >= self.cols {
                return;
            }
            let mut i = row;
            while self.get(i, lead) == T::default() {
                i += 1;
                if i == self.rows {
                    i = row;
                    lead += 1;
                    if lead == self.cols {
                        return;
                    }
                }
            }
            if i != self.rows {
                self.swap_row(i, row);
            }
            self.divide_row(row, self.get(row, lead));
            for j in 0..self.rows {
                if j != row {
                    self.subtract_from_row(j, row, self.get(j, lead));
                }
            }
            lead += 1;
        }
    }

    fn find_pivot_column(&self, row: u8) -> Option<u8>
    where
        T: NumberExt + PartialEq,
    {
        (0..self.cols).find(|&col| self.get(row, col) != T::zero())
    }
}

#[derive(Debug, Clone, Copy)]
struct Fraction {
    numerator: i32,
    denominator: u32,
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl Fraction {
    const fn new(numerator: i32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    const fn intify(self) -> Result<i32, Self> {
        if self.denominator == 1 {
            Ok(self.numerator)
        } else {
            Err(self)
        }
    }

    fn simplify(self) -> Self {
        let gcd = self
            .numerator
            .abs()
            .cast_unsigned()
            .greatest_common_divisor(self.denominator);
        Self::new(
            self.numerator / i32::try_from(gcd).unwrap(),
            self.denominator / gcd,
        )
    }
}

impl NumberExt for Fraction {
    fn greatest_common_divisor(self, _other: Self) -> Self {
        todo!()
    }
    fn least_common_multiple(self, _other: Self) -> Self {
        todo!()
    }
    fn parity(self) -> aoc_utils::Parity {
        todo!()
    }
    fn split_odd_even(self) -> (Self, Self) {
        todo!()
    }
    fn zero() -> Self {
        Self::new(0, 1)
    }
    fn one() -> Self {
        Self::new(1, 1)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let Fraction {
            numerator: a,
            denominator: b,
        } = self.simplify();
        let Fraction {
            numerator: c,
            denominator: d,
        } = other.simplify();
        a == c && b == d
    }
}

impl Eq for Fraction {}

impl std::ops::DivAssign<Fraction> for Fraction {
    fn div_assign(&mut self, other: Fraction) {
        self.numerator *= i32::try_from(other.denominator).unwrap();
        self.denominator *= other.numerator.abs().cast_unsigned();
        self.numerator *= other.numerator.signum();
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.numerator == 0 || other.numerator == 0 {
            return Self::new(0, 1);
        }
        let numerator = i64::from(self.numerator) * i64::from(other.numerator);
        let denominator = u64::from(self.denominator) * u64::from(other.denominator);
        let gcd = numerator
            .abs()
            .cast_unsigned()
            .greatest_common_divisor(denominator);
        Self::new(
            (numerator / i64::try_from(gcd).unwrap())
                .try_into()
                .unwrap(),
            (denominator / gcd).try_into().unwrap(),
        )
        .simplify()
    }
}

impl From<u16> for Fraction {
    fn from(value: u16) -> Self {
        Self::new(value.into(), 1)
    }
}

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Self::new(value, 1)
    }
}

impl std::ops::SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        let other = other.simplify();
        let lcm = i32::try_from(self.denominator.least_common_multiple(other.denominator)).unwrap();
        let mut num = (self.numerator.strict_mul(lcm)) / i32::try_from(self.denominator).unwrap();
        num -= (other.numerator.strict_mul(lcm)) / i32::try_from(other.denominator).unwrap();
        self.numerator = num;
        self.denominator = lcm.try_into().unwrap();
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

#[allow(unused)]
struct DisplayMatrix<'a, T>(&'a Matrix<T>);

impl<T: Display + Copy> Display for DisplayMatrix<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in 0..self.0.rows {
            for col in 0..self.0.cols {
                write!(f, "{:>4} ", self.0.get(row, col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct CartesianProductIterator {
    max: usize,
    current: Vec<usize>,
    done: bool,
}

impl CartesianProductIterator {
    fn new(max: usize, length: usize) -> Self {
        let current = vec![0; length];
        Self {
            max,
            current,
            done: false,
        }
    }
}

impl Iterator for CartesianProductIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.current.clone();
            for i in (0..self.current.len()).rev() {
                if self.current[i] < self.max {
                    self.current[i] += 1;
                    for j in i + 1..self.current.len() {
                        self.current[j] = 0;
                    }
                    return Some(result);
                }
            }
            self.done = true;
            Some(result)
        }
    }
}

fn find_min_clicks_to_setup_joltage(machine: &Machine) -> usize {
    println!(
        "Solving {}",
        machine.expected_state.to_string(machine.bit_count)
    );
    let rows = machine.bit_count;
    let cols = u8::try_from(machine.buttons.len()).unwrap() + 1;
    let mut x = Matrix::<Fraction>::new(rows, cols);
    for (col, button) in machine.buttons.iter().enumerate() {
        for row in (0..rows).rev() {
            if button.0 & (1 << row) != 0 {
                x.set(row, col.try_into().unwrap(), Fraction::one());
            }
        }
    }
    for row in 0..rows {
        x.set(
            row,
            cols - 1,
            machine.expected_joltage.0[row as usize].into(),
        );
    }
    x.reduced_row_echelon_form();

    let mut free_vars = (0..cols - 1).collect::<Vec<_>>();
    for row in 0..rows {
        if let Some(col) = x.find_pivot_column(row) {
            free_vars.retain(|&c| c != col);
        }
    }

    if free_vars.is_empty() {
        // unique solution!
        let mut solution = vec![0; cols as usize - 1];
        for row in 0..rows {
            if let Some(col) = x.find_pivot_column(row) {
                solution[col as usize] = x.get(row, cols - 1).simplify().intify().unwrap();
            }
        }
        let sum: i32 = solution.iter().sum();
        println!("Unique solution: {solution:?} {sum}");
        return sum.try_into().unwrap();
    }

    let mut best_solution = None;
    let mut best_sum = i32::MAX;

    let max_rhs = *machine.expected_joltage.0.iter().max().unwrap();
    println!("Free vars: {}, range = 0..={max_rhs}", free_vars.len());

    // let mut assignments = vec![0i16; free_vars.len()];
    // while assignments.iter().all(|a| (*a as u16) < max_rhs) {
    for assignments in CartesianProductIterator::new(max_rhs as _, free_vars.len()) {
        let mut solution = vec![0; cols as usize - 1];
        for (i, &var) in free_vars.iter().enumerate() {
            solution[var as usize] = (assignments[i] as i32).into();
        }
        // println!("Assignments: {assignments:?}");

        // compute dependent variables
        let valid = (0..rows).all(|row| {
            if let Some(col) = x.find_pivot_column(row) {
                let mut val = x.get(row, cols - 1);
                for j in 0..(cols - 1) {
                    if j != col {
                        val -= x.get(row, j) * solution[j as usize].into();
                    }
                }
                val = val.simplify();
                if val.numerator < 0 {
                    return false;
                }
                if let Ok(value) = val.simplify().intify() {
                    solution[col as usize] = value;
                } else {
                    // println!("Discarding fractional value: {}", val);
                    return false;
                }
            }
            true
        });

        // println!("Candidate solution: {solution:?}; valid = {valid:?}");

        // we have a candidate solution
        if valid {
            let sum = solution.iter().sum();
            // println!("Solution: {solution:?} Sum: {sum}");
            if sum < best_sum {
                best_solution = Some(solution);
                best_sum = sum;
            }
        }
    }

    if best_solution.is_none() {
        panic!("No solution found");
    }

    println!("Best Solution: {best_solution:?} Sum: {best_sum}");
    best_sum.try_into().unwrap()
}

#[cfg(test)]
mod tests_math {
    use super::{Fraction, Matrix};
    #[test]
    fn test_fraction_ops1() {
        let mut a = Fraction::new(3, 4);
        a -= Fraction::new(1, 2);
        assert_eq!(a.numerator, 1);
        assert_eq!(a.denominator, 4);
        a -= Fraction::new(1, 2);
        assert_eq!(a.numerator, -1);
        assert_eq!(a.denominator, 4);
        a /= Fraction::new(-4, 8);
        a = a.simplify();
        assert_eq!(a.numerator, 1);
        assert_eq!(a.denominator, 2);
    }
    #[test]
    fn test_fraction_ops2() {
        // 2/3 - 7/8
        let mut a = Fraction::new(2, 3);
        a -= Fraction::new(7, 8);
        assert_eq!(a.numerator, -5);
        assert_eq!(a.denominator, 24);
        a = a.simplify();
        assert_eq!(a.numerator, -5);
        assert_eq!(a.denominator, 24);
    }

    #[test]
    fn test_matrix_rref_fraction() {
        let data = [
            0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 42, // row
            0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 46, // row
            0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 58, // row
            0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 64, // row
            0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 61, // row
            0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 28, // row
            1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 67, // row
            1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 43, // row
            1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 58, // row
            0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 104, // row
        ]
        .map(Fraction::from)
        .into();
        let mut matrix = Matrix {
            rows: 10,
            cols: 13,
            data,
        };
        matrix.reduced_row_echelon_form();
        // println!("{}", DisplayMatrix(&matrix));
        assert_eq!(matrix.get(0, 0), Fraction::new(1, 1));
        assert_eq!(matrix.get(0, 1), Fraction::new(0, 1));
        assert_eq!(matrix.get(0, 11), Fraction::new(-3, 2));
    }

    #[test]
    fn test_matrix_rref_float() {
        let data = [
            0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 42, // row
            0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 46, // row
            0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 58, // row
            0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 64, // row
            0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 61, // row
            0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 28, // row
            1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 67, // row
            1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 43, // row
            1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 58, // row
            0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 104, // row
        ]
        .map(f64::from)
        .into();
        let mut matrix = Matrix {
            rows: 10,
            cols: 13,
            data,
        };
        matrix.reduced_row_echelon_form();
        // println!("{}", DisplayMatrix(&matrix));
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 1), 0.0);
        assert_eq!(matrix.get(0, 11), -1.5);
    }
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> usize {
    input.iter().map(find_min_clicks_to_setup_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_from_expected_state() {
        let (pattern, n) = Pattern::from_machine_config_string("[#.#.#]");
        assert_eq!(n, 5);
        assert_eq!(pattern.to_string(n), "#.#.#");
        assert_eq!(pattern.0, 0b10101);

        let (pattern, n) = Pattern::from_machine_config_string("[..#.#]");
        assert_eq!(n, 5);
        assert_eq!(pattern.to_string(n), "..#.#");
        assert_eq!(pattern.0, 0b00101);

        let (pattern, n) = Pattern::from_machine_config_string("[.....]");
        assert_eq!(n, 5);
        assert_eq!(pattern.to_string(n), ".....");
        assert_eq!(pattern.0, 0);
    }

    #[test]
    fn test_pattern_from_button() {
        let pattern = Pattern::from_button_wiring_string("(0,1,2)", 5);
        assert_eq!(pattern.to_string(5), "###..");
        assert_eq!(pattern.0, 0b11100);
        let pattern = Pattern::from_button_wiring_string("(0)", 5);
        assert_eq!(pattern.to_string(5), "#....");
        assert_eq!(pattern.0, 0b10000);
    }

    #[test]
    fn test_expected_joltage() {
        let joltage = JoltageState::from_expected_joltage("{1,2,3}").unwrap();
        assert_eq!(joltage.0, [3, 2, 1, 0, 0, 0, 0, 0, 0, 0]);
    }
}

example_tests! {
    "
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ",
    part1 => 7,
    part2 => 33,
}

known_input_tests! {
    input: include_str!("../input/2025/day10.txt"),
    part1 => 500,
    // part2 => 19763, // too slow for debug mode :(
}
