use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{Annotate, AnnotateExt, SliceUtils, example_tests, known_input_tests};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn new(x: u32, y: u32) -> Self {
        Pos { x, y }
    }
    fn unpack(self) -> [u32; 2] {
        [self.x, self.y]
    }
}

fn area(pos1: Pos, pos2: Pos) -> u64 {
    (1 + pos1.x.abs_diff(pos2.x)) as u64 * (1 + pos1.y.abs_diff(pos2.y)) as u64
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            Pos { x, y }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Pos]) -> u64 {
    (0..input.len())
        .flat_map(|i| (i + 1..input.len()).map(move |j| area(input[i], input[j])))
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[Pos]) -> u64 {
    // While this is a general solution that is correct, also check out
    // part2_fast for a much faster solution that works well with the given
    // input data. I had fun implementing this one, and the idea of splitting
    // the grid into 3x3 sub-tiles is a trick that can be applied to other
    // puzzles (I did something similar for AOC 2023 day 10, apparently!)

    let mut areas = BinaryHeap::with_capacity(input.len() * (input.len() - 1) / 2);
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let area = area(input[i], input[j]);
            areas.push(area.annotate((i, j)));
        }
    }
    // We apply a weird transformation to the coordinates that will make things
    // easier later when we need to find intersections between rectangle sides
    // and the segments that make up the bounding polygon. We can imagine it as
    // subdividing each tile into 3x3 smaller tiles, and only use the external
    // edges for the bounding polygon.
    //
    // See day9-composite.svg for a visual representation of this transformation.
    //
    // E.g. a single tile is represented like this:
    //
    //     |
    //   # # #     y * 3
    // - # o # -   y * 3 + 1
    //   # # #     y * 3 + 2
    //     |
    //
    // The sides of the polygon can only live on the # subtiles, either
    // internally or externally. So a horizontal side will always have even y
    // coordinates, and a vertical side will always have even x coordinates. On
    // the other hand the sides of the rectangle can only live on the odd x or y
    // coordinates. This way, parallel sides will never overlap.
    //
    // The external/internal distinction requires a bit of work to make right.
    // E.g. imagine a corner like this in the original coordinates:
    //
    //   # . #
    //   . . .
    //   . . #
    //
    // It could be transformed to either
    //
    //     *-------------+
    //   . o . . . . . o |
    //   . . . . . . . . |
    //   . . . . . . . . |
    //   . . . . . . . . |
    //   . . . . . . . . |
    //   . . . . . . . . |
    //   . . . . . . . o *
    //   . . . . . . . .
    //
    //  or
    //
    //   . . . . . . . . .
    //   . o . . . . . o .
    //     *---------+ . .
    //   . . . . . . | . .
    //   . . . . . . | . .
    //   . . . . . . | . .
    //   . . . . . . | . .
    //   . . . . . . * o .
    //   . . . . . .   . .
    //
    // (We want the sub-tile centers o to be always inside the polygon, so the
    // other two possibilities are not valid. On the other hand, we necessarily
    // need to consider the next point in order to determine the next end of
    // each segment, and that's why it's left as a * in the diagram.)
    //
    // Without extra context we can't determine whether the polygon lives on the
    // internal or external part. That said, given the point specification,
    // there are exactly two possibilities for the entire polygon. We can take
    // one extreme of the polygon (e.g. one of the topmost points) and decide
    // that it has to live on the smallest y coordinate, and follow the loop
    // from there.
    let (topmost_index, topmost_y) = input
        .iter()
        .map(|p| p.y)
        .enumerate()
        .min_by_key(|&(_, y)| y)
        .unwrap();

    // Identify the next point that forms a horizontal segment together with the
    // already identified topmost point. It could be either the one before or
    // after. Note that 1 implies clockwise, -1 implies counter-clockwise.
    let direction = [-1, 1]
        .into_iter()
        .find(|&dx| {
            let next_index = topmost_index
                .checked_add_signed(dx)
                .unwrap_or(input.len() - 1)
                % input.len();
            topmost_y == input[next_index].y
        })
        .unwrap();

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Heading {
        Right,
        Down,
        Left,
        Up,
    }

    impl Heading {
        const fn clockwise(self) -> Self {
            match self {
                Heading::Right => Heading::Down,
                Heading::Down => Heading::Left,
                Heading::Left => Heading::Up,
                Heading::Up => Heading::Right,
            }
        }
        const fn opposite(self) -> Self {
            match self {
                Heading::Right => Heading::Left,
                Heading::Down => Heading::Up,
                Heading::Left => Heading::Right,
                Heading::Up => Heading::Down,
            }
        }
        const fn counter_clockwise(self) -> Self {
            self.clockwise().opposite()
        }
    }

    let mut transformed_points = Vec::<Pos>::new();
    // The first corner we consider is actually the next one after the topmost
    // point we found, so that we can make use of the information of the
    // current heading.
    let start_idx = topmost_index
        .checked_add_signed(direction)
        .unwrap_or(input.len() - 1)
        % input.len();
    let mut heading = if direction == 1 {
        Heading::Right
    } else {
        Heading::Left
    };
    for i in 0..input.len() {
        let idx = (start_idx + i) % input.len();
        let next_idx = idx.checked_add_signed(direction).unwrap_or(input.len() - 1) % input.len();

        let point = input[idx];
        let next_point = input[next_idx];

        // Eight possibilities depending on the direction of the two
        // segments that meet at the corner, and the winding direction:
        //
        // Positive winding (center is inside the corner):
        //
        //       ----+     +----     . . |     | . .
        //       . o |     | o .     . o |     | o .
        //       . . |     | . .     ----+     +----
        //
        //  CW    r-d       u-r       d-l       l-u
        //  CCW   u-l       l-d       r-u       d-r
        //
        // Negative winding (center is outside the corner):
        //
        //                           |             |
        //       . . .     . . .   --+ . .     . . +--
        //       . o .     . o .     . o .     . o .
        //     --+ . .     . . +--   . . .     . . .
        //       |             |
        //
        //  CW    u-l       l-d       r-u       d-r
        //  CCW   r-d       u-r       d-l       l-u
        //
        // But ultimately we only need to care about the position of the corner
        // in the 3x3 sub-tile, and there are only four possible positions.
        let next_heading = if point.x == next_point.x {
            // horizontal
            if point.y < next_point.y {
                Heading::Down
            } else {
                Heading::Up
            }
        } else {
            if point.x < next_point.x {
                Heading::Right
            } else {
                Heading::Left
            }
        };
        let positive_heading = if direction == 1 {
            heading.clockwise()
        } else {
            heading.counter_clockwise()
        };
        let is_positive = positive_heading == next_heading;
        // This thing is full of symmetries to exploit but I can think about
        // it more explicitly if I just write down the whole table with all
        // the cases.
        use Heading::*;
        let (dx, dy) = match (is_positive, heading, next_heading) {
            (true, Up, Left) | (true, Right, Down) => (2, 0),
            (true, Up, Right) | (true, Left, Down) => (0, 0),
            (true, Down, Left) | (true, Right, Up) => (2, 2),
            (true, Down, Right) | (true, Left, Up) => (0, 2),
            (false, Up, Left) | (false, Right, Down) => (0, 2),
            (false, Up, Right) | (false, Left, Down) => (2, 2),
            (false, Down, Left) | (false, Right, Up) => (0, 0),
            (false, Down, Right) | (false, Left, Up) => (2, 0),
            _ => {
                unreachable!("impossible: is_positive={is_positive} {heading:?}->{next_heading:?}")
            }
        };
        let transformed_point = Pos::new(point.x * 3 + dx, point.y * 3 + dy);
        transformed_points.push(transformed_point);
        heading = next_heading;
    }

    let mut horizontal_segments = Vec::with_capacity(input.len());
    let mut vertical_segments = Vec::with_capacity(input.len());

    for (&p1, &p2) in transformed_points.iter().zip(
        transformed_points
            .iter()
            .skip(1)
            .chain(std::iter::once(&transformed_points[0])),
    ) {
        if p1.x == p2.x {
            vertical_segments.push((p1, p2));
        } else {
            debug_assert!(p1.y == p2.y);
            horizontal_segments.push((p1, p2));
        }
    }
    horizontal_segments.sort_unstable_by_key(|(p1, p2)| Reverse(p1.x.abs_diff(p2.x)));
    vertical_segments.sort_unstable_by_key(|(p1, p2)| Reverse(p1.y.abs_diff(p2.y)));

    'areas: while let Some(Annotate {
        annotation: (i, j),
        value: area,
    }) = areas.pop()
    {
        let pos1 = input[i];
        let pos2 = input[j];
        let x1 = pos1.x.min(pos2.x) * 3 + 1;
        let x2 = pos1.x.max(pos2.x) * 3 + 1;
        let y1 = pos1.y.min(pos2.y) * 3 + 1;
        let y2 = pos1.y.max(pos2.y) * 3 + 1;
        for &segment in &horizontal_segments {
            let (Pos { x: ex1, y: ey }, Pos { x: ex2, .. }) = segment;
            if x1 >= ex1 && x1 <= ex2 && ey >= y1 && ey <= y2 {
                continue 'areas;
            }
            if x2 >= ex1 && x2 <= ex2 && ey >= y1 && ey <= y2 {
                continue 'areas;
            }
        }
        for &segment in &vertical_segments {
            let (Pos { x: ex, y: ey1 }, Pos { y: ey2, .. }) = segment;
            if ex >= x1 && ex <= x2 && ey1 >= y1 && ey1 <= y2 {
                continue 'areas;
            }
            if ex >= x1 && ex <= x2 && ey2 >= y1 && ey2 <= y2 {
                continue 'areas;
            }
        }
        if cfg!(feature = "draw-visuals") {
            draw_svg(input, &transformed_points, (pos1, pos2));
        }
        return area;
    }

    if cfg!(feature = "draw-visuals") {
        draw_svg(input, &transformed_points, (Pos::new(0, 0), Pos::new(0, 0)));
    }

    panic!("No solution found")
    // best_area
}

fn draw_svg(input: &[Pos], transformed: &[Pos], rect: (Pos, Pos)) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("day9.svg");
    let mut file = std::fs::File::create(&path).unwrap();
    write_svg(input, transformed, rect, &mut file).unwrap();
}

fn write_svg(
    input: &[Pos],
    transformed: &[Pos],
    rect: (Pos, Pos),
    mut w: impl std::io::Write,
) -> std::io::Result<()> {
    let min_x = input.iter().map(|p| p.x).min().unwrap() as f64;
    let min_y = input.iter().map(|p| p.y).min().unwrap() as f64;
    let max_x = input.iter().map(|p| p.x).max().unwrap() as f64;
    let max_y = input.iter().map(|p| p.y).max().unwrap() as f64;
    let width = 4.0 + max_x - min_x;
    let height = 4.0 + max_y - min_y;
    let figure_height = 1000.0 * height / width;
    write!(
        w,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1000\" height=\"{figure_height}\" viewBox=\"0 0 {width} {height}\">\n"
    )?;
    write!(w, "<polygon points=\"")?;
    for pos in transformed {
        let x = (pos.x - 1) as f64 / 3.0;
        let y = (pos.y - 1) as f64 / 3.0;
        write!(w, "{},{} ", x, y)?;
    }
    write!(
        w,
        "\" fill=\"blue\" stroke=\"green\" stroke-width=\"0.333333333\" />\n"
    )?;
    for pos in input {
        let x = pos.x;
        let y = pos.y;
        write!(
            w,
            "<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"#ffa0a070\"/>\n",
            x = x as f64 - 0.5,
            y = y as f64 - 0.5,
            w = 1.0,
            h = 1.0,
        )?;
        write!(
            w,
            "<rect x=\"{x}\" y=\"{y}\" width=\"{w}\" height=\"{h}\" fill=\"purple\"/>\n",
            x = x as f64 - 0.16666666,
            y = y as f64 - 0.16666666,
            w = 0.333333,
            h = 0.333333,
        )?;
    }
    {
        // draw rectangle
        let rx = rect.0.x.min(rect.1.x) as f64;
        let ry = rect.0.y.min(rect.1.y) as f64;
        let rw = rect.1.x.abs_diff(rect.0.x) as f64;
        let rh = rect.1.y.abs_diff(rect.0.y) as f64;
        write!(
            w,
            "<rect x=\"{rx}\" y=\"{ry}\" width=\"{rw}\" height=\"{rh}\" fill=\"#ff00007f\"/>\n",
        )?;
    }
    write!(w, "</svg>")
}

#[aoc(day9, part2, faster)]
fn part2_fast(tiles: &[Pos]) -> u64 {
    // ported from https://github.com/enigma/aoc/blob/0cac403a10e199d84ff80ca1d0be1b4cbe392bfd/python/2025/9.py#L16
    //
    // Note that this seems to be buggy but give a coincidentally correct answer
    // for the example input (can be verified by hand, the rectangle 2,5,9,7 is
    // outside the bounding polygon, although it happens to also have a 8x3
    // area) so it might not work in general, but only for the given input data.
    // The commented out debug print statements are a left over in case one day
    // I want to debug this.
    let mut sorted_edges = Vec::with_capacity(tiles.len() + 1);
    for i in 0..tiles.len() {
        let [x1, y1] = tiles[i].unpack();
        let [x2, y2] = tiles[(i + 1) % tiles.len()].unpack();
        sorted_edges.push((x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)));
    }
    sorted_edges.sort_unstable_by_key(|&(x1, y1, x2, y2)| Reverse((x2 - x1 + 1) + (y2 - y1 + 1)));

    let mut best = 0;
    // let mut rectangles_tested = 0;
    for (p1, p2) in tiles.pairs() {
        let [x1, y1] = p1.unpack();
        let [x2, y2] = p2.unpack();
        // rectangles_tested += 1;
        let (bx1, by1, bx2, by2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));
        let area = (bx2 - bx1 + 1) as u64 * (by2 - by1 + 1) as u64;
        if area <= best {
            continue;
        }
        // println!(
        //     "\n\nRectangle {bx1} {by1} {bx2} {by2} area = {area} width = {} height = {}",
        //     bx2 - bx1 + 1,
        //     by2 - by1 + 1
        // );
        if !sorted_edges
            .iter()
            // .inspect(|&(ex1, ey1, ex2, ey2)| {
            //     println!("Checking edge: ({ex1}, {ey1}, {ex2}, {ey2})");
            // })
            .find(|&&(ex1, ey1, ex2, ey2)| bx1 < ex2 && bx2 > ex1 && by1 < ey2 && by2 > ey1)
            // .inspect(|&(ex1, ey1, ex2, ey2)| {
            //     println!("Edge passed: ({ex1}, {ey1}, {ex2}, {ey2})");
            // })
            .is_some()
        {
            // println!(
            //     "\n\nRectangle {bx1} {by1} {bx2} {by2} area = {area} width = {} height = {}",
            //     bx2 - bx1 + 1,
            //     by2 - by1 + 1
            // );
            // println!("New best rectangle found!");
            best = area;
        }
    }
    // println!("Rectangles tested: {rectangles_tested}");
    best
}

example_tests! {
    "
    7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3
    ",
    part1 => 50,
    part2 => 24,
    part2_fast => 24,
}

known_input_tests! {
    input: include_str!("../input/2025/day9.txt"),
    part1 => 4735268538,
    part2 => 1537458069,
    part2_fast => 1537458069,
}
