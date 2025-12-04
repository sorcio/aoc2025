use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, FromGridLike, example_tests, grid_cell_enum, known_input_tests};

grid_cell_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Cell {
        Occupied => b'@',
        Empty => b'.',
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn position(x: usize, y: usize) -> Position {
    Position { x, y }
}

impl Position {
    fn add(self, dx: isize, dy: isize) -> Option<Position> {
        let x = self.x.checked_add_signed(dx)?;
        let y = self.y.checked_add_signed(dy)?;
        Some(position(x, y))
    }
    fn neighbors(self) -> impl Iterator<Item = Position> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .filter_map(move |(dx, dy)| self.add(dx, dy))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Map {
    fn cell(&self, pos: Position) -> Option<Cell> {
        if pos.x < self.width && pos.y < self.height {
            self.cells.get(pos.y * self.width + pos.x).copied()
        } else {
            None
        }
    }

    fn neighbors(&self, pos: Position) -> impl Iterator<Item = (Position, Cell)> {
        pos.neighbors()
            .filter_map(move |neighbor| Some((neighbor, self.cell(neighbor)?)))
    }

    fn cells(&self) -> impl Iterator<Item = (Position, Cell)> {
        (0..=self.height).flat_map(move |y| {
            (0..=self.width).filter_map(move |x| Some((position(x, y), self.cell(position(x, y))?)))
        })
    }

    fn remove(&mut self, positions: impl Iterator<Item = Position>) -> Result<(), ()> {
        for pos in positions {
            if let Some(index) = pos
                .y
                .checked_mul(self.width)
                .and_then(|i| i.checked_add(pos.x))
            {
                assert!(self.cells[index] == Cell::Occupied);
                self.cells[index] = Cell::Empty;
            } else {
                return Err(());
            }
        }
        Ok(())
    }
}

impl FromGridLike for Map {
    type Cell = Cell;

    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        Map {
            width,
            height,
            cells,
        }
    }
}

#[aoc_generator(day4)]
fn parse(input: &[u8]) -> Map {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day4, part1)]
fn part1(input: &Map) -> usize {
    input
        .cells()
        .filter(|&(pos, cell)| {
            cell == Cell::Occupied
                && input
                    .neighbors(pos)
                    .filter(|&(_, cell)| cell == Cell::Occupied)
                    .count()
                    < 4
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Map) -> usize {
    let mut removed_count = 0;
    let mut map = input.clone();
    loop {
        let removable = map
            .cells()
            .filter(|&(pos, cell)| {
                cell == Cell::Occupied
                    && map
                        .neighbors(pos)
                        .filter(|&(_, cell)| cell == Cell::Occupied)
                        .count()
                        < 4
            })
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();
        if removable.is_empty() {
            break;
        }
        removed_count += removable.len();
        map.remove(removable.into_iter()).unwrap();
    }
    removed_count
}

example_tests! {
    b"
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
    ",
    part1 => 13,
    part2 => 43,
}

known_input_tests! {
    input: include_bytes!("../input/2025/day4.txt"),
    part1 => 1416,
    part2 => 9086,
}
