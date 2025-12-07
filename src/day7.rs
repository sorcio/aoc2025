use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, FromGridLike, example_tests, grid_cell_enum, known_input_tests};

grid_cell_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Cell {
        Empty => b'.',
        Splitter => b'^',
        Start => b'S',
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: Position,
}

impl Grid {
    fn is_splitter(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x] == Cell::Splitter
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BeamTracker<'g> {
    grid: &'g Grid,
    y: usize,
    beams: Box<[bool]>,
}

impl<'g> BeamTracker<'g> {
    fn start(grid: &'g Grid) -> Self {
        let mut beams = vec![false; grid.width].into_boxed_slice();
        beams[grid.start.x] = true;
        let y = grid.start.y;
        Self { grid, y, beams }
    }

    fn step(&mut self) -> Option<usize> {
        let mut splitters_hit = 0;
        // because of the input structure, we know that splitter rows are
        // interleaved with empty rows, so we can skip two rows at a time
        let y = self.y + 2;
        if y >= self.grid.height {
            return None;
        }
        self.y += 2;
        let mut beams = vec![false; self.grid.width];
        for x in 0..self.grid.width {
            if self.beams[x] {
                if self.grid.is_splitter(x, self.y) {
                    debug_assert!(x > 0 && x < self.grid.width - 1);
                    beams[x - 1] = true;
                    beams[x + 1] = true;
                    splitters_hit += 1;
                } else {
                    beams[x] = true;
                }
            }
        }
        self.beams = beams.into_boxed_slice();
        debug_assert!(splitters_hit > 0);
        Some(splitters_hit)
    }
}

impl FromGridLike for Grid {
    type Cell = Cell;

    fn from_cells(cells: Vec<Self::Cell>, width: usize, height: usize) -> Self {
        let start_index = cells
            .iter()
            .position(|&cell| cell == Cell::Start)
            .expect("there should be a starting cell");
        let start = Position {
            x: start_index % width,
            y: start_index / width,
        };
        Self {
            cells,
            width,
            height,
            start,
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &[u8]) -> Grid {
    input.grid_like().unwrap().into_grid()
}

#[aoc(day7, part1)]
fn part1(input: &Grid) -> usize {
    let mut beam = BeamTracker::start(input);
    let mut total = 0;
    while let Some(splitters_hit) = beam.step() {
        total += splitters_hit;
    }
    total
}
#[aoc(day7, part2)]
fn part2(input: &Grid) -> usize {
    let mut beams1 = vec![0; input.width];
    let mut beams2 = vec![0; input.width];
    let mut beams = &mut beams1;
    let mut new_beams = &mut beams2;
    beams[input.start.x] = 1;
    for y in input.start.y..(input.height - 2) {
        new_beams.fill(0);
        let ny = y + 2;
        for x in 0..input.width {
            let count = beams[x];
            if count > 0 {
                if input.is_splitter(x, ny) {
                    debug_assert!(x > 0 && x < input.width - 1);
                    new_beams[x - 1] += count;
                    new_beams[x + 1] += count;
                } else {
                    new_beams[x] += count;
                }
            }
        }
        (beams, new_beams) = (new_beams, beams);
    }
    beams.iter().sum()
}

example_tests! {
    b"
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
    ",
    part1 => 21,
    part2 => 40,
}

known_input_tests! {
    input: include_bytes!("../input/2025/day7.txt"),
    part1 => 1585,
    part2 => 16716444407407,
}
