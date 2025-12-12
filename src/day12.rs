use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    shapes: Vec<[bool; 9]>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    width: usize,
    height: usize,
    requirements: [u8; 6],
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    // Parse shapes and regions from input
    let mut lines = input.lines().into_iter().filter(|line| !line.is_empty());
    while let Some(line) = lines.next() {
        if line.ends_with(':') {
            // the next three lines define a 3x3 shape
            let mut shape = [false; 9];
            for (i, x) in (&mut lines)
                .take(3)
                .flat_map(|line| line.chars().map(|c| c == '#'))
                .enumerate()
            {
                shape[i] = x;
            }
            shapes.push(shape);
        } else {
            // the current line defines a region
            // 01234567...
            // WWxHH: r0 r1 r2 r3 ...
            let width: usize = line[0..2].parse().unwrap();
            let height: usize = line[3..5].parse().unwrap();
            let requirements = line[7..]
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u8>>();
            regions.push(Region {
                width,
                height,
                requirements: requirements.try_into().unwrap(),
            });
        }
    }

    Input { shapes, regions }
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    // ########## NOTE TO THE READER ##########
    //
    // Do you really want to read this solution? This contains a huge spoiler about todays puzzle.
    //
    // I recommend that you don't read this solution. This is probably not the
    // solution you are looking for, anyways. I promise reading this won't help
    // you solve the puzzle.
    //
    // Before you go on, take a deep breath and decide if you want the problem to be spoiled.
    //
    // Solution follows after the blank lines:
    //
    //
    //
    //
    //
    //
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    //
    // ##### SPOILER TO DAY 12 PART 1 IN A BIT #####
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    //
    // ##### ONE MORE NOTE #####
    //
    // If you go on you need to know that the solution you are about to
    // read is a basically cheating. If you want to solve the puzzle yourself,
    // maybe you shouldn't have it spoiled like this.
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    // S
    // P
    // O
    // I
    // L
    // E
    // R
    // S
    //
    //
    // Ok, here you are:
    input
        .regions
        .iter()
        .filter(|region| {
            let total_required = region
                .requirements
                .iter()
                .copied()
                .map(usize::from)
                .sum::<usize>();
            let total_required_area = total_required * 9;
            let total_available_area = region.width * region.height;
            total_required_area <= total_available_area
        })
        .count()
}
