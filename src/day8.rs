use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
    // time::Instant,
};

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{Annotate, AnnotateExt, example_tests, known_input_tests};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: u32,
    y: u32,
    z: u32,
}

impl Pos {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Pos { x, y, z }
    }

    fn pack(self) -> PackedPos {
        PackedPos::new(self.x, self.y, self.z)
    }

    fn squared_distance(self, other: Pos) -> u64 {
        let x1: i64 = self.x.into();
        let y1: i64 = self.y.into();
        let z1: i64 = self.z.into();
        let x2: i64 = other.x.into();
        let y2: i64 = other.y.into();
        let z2: i64 = other.z.into();
        ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).cast_unsigned()
    }
}

impl FromStr for Pos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let x = parts[0].parse().map_err(|_| ())?;
        let y = parts[1].parse().map_err(|_| ())?;
        let z = parts[2].parse().map_err(|_| ())?;
        Ok(Pos::new(x, y, z))
    }
}

impl core::fmt::Debug for Pos {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}

impl core::fmt::Display for Pos {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = format!("{},{},{}", self.x, self.y, self.z);
        s.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PackedPos(u64);

impl PackedPos {
    fn new(x: u32, y: u32, z: u32) -> Self {
        debug_assert!(x < (0x1FFFF), "{x}");
        debug_assert!(y < (0x1FFFF), "{y}");
        debug_assert!(z < (0x1FFFF), "{z}");
        Self(((x as u64) << 34) | ((y as u64) << 17) | (z as u64))
    }

    fn squared_distance(self, other: PackedPos) -> u64 {
        let x1: i64 = (self.0 >> 34) as i64;
        let y1: i64 = ((self.0 >> 17) & 0x1FFFF) as i64;
        let z1: i64 = (self.0 & 0x1FFFF) as i64;
        let x2: i64 = (other.0 >> 34) as i64;
        let y2: i64 = ((other.0 >> 17) & 0x1FFFF) as i64;
        let z2: i64 = (other.0 & 0x1FFFF) as i64;
        ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)).cast_unsigned()
    }

    fn x(self) -> u32 {
        (self.0 >> 34) as u32
    }
}

impl FromStr for PackedPos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pos::from_str(s).map(Pos::pack)
    }
}

#[aoc_generator(day8, part1)]
fn parse(input: &str) -> Vec<Pos> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn find_n_closest_links(nodes: &[Pos], n: usize) -> Vec<Annotate<u64, (Pos, Pos)>> {
    let mut top_n = BinaryHeap::with_capacity(n);
    for (i, &a) in nodes.iter().enumerate() {
        for &b in &nodes[i + 1..] {
            if a != b {
                let distance = a.squared_distance(b);
                if top_n.len() < n {
                    top_n.push(distance.annotate((a, b)));
                } else if let Some(&Annotate { value: max, .. }) = top_n.peek() {
                    if distance < max {
                        top_n.pop();
                        top_n.push(distance.annotate((a, b)));
                    }
                }
            }
        }
    }
    top_n.into_sorted_vec()
}

#[aoc(day8, part1)]
fn part1(input: &[Pos]) -> usize {
    let n: usize = if input.len() < 100 {
        // example
        10
    } else {
        // real input data
        1000
    };
    let top_n = find_n_closest_links(input, n);

    if cfg!(debug_assertions) {
        for Annotate {
            annotation: (a, b),
            value: distance,
        } in &top_n
        {
            println!("{a:11}   {b:11}  d = {distance}");
        }
    }

    let edges = top_n
        .iter()
        .map(
            |Annotate {
                 annotation: (a, b), ..
             }| (*a, *b),
        )
        .collect::<Vec<_>>();

    let nodes = edges
        .iter()
        .flat_map(|&(a, b)| [a, b])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // find connected components
    let mut labels = vec![0; nodes.len()];
    let mut components: Vec<Vec<Pos>> = vec![];

    for (i, _node) in nodes.iter().copied().enumerate() {
        if labels[i] != 0 {
            continue;
        }
        components.push(vec![nodes[i]]);
        let component_id = components.len();
        let component = &mut components[component_id - 1];
        labels[i] = component_id;
        let mut queue = vec![i];
        while let Some(j) = queue.pop() {
            if labels[j] == 0 || labels[j] == component_id {
                if labels[j] == 0 {
                    labels[j] = component_id;
                    component.push(nodes[j]);
                }
                for (k, neighbor) in nodes.iter().copied().enumerate() {
                    if labels[k] == 0
                        && (edges.contains(&(nodes[j], neighbor))
                            || edges.contains(&(neighbor, nodes[j])))
                    {
                        queue.push(k);
                    }
                }
            }
        }
    }

    // println!("Number of components: {}", components.len());
    // println!("Components:");
    // for (id, component) in components.iter().enumerate() {
    //     println!("Component {}: {:?}", id + 1, component);
    // }
    components.sort_unstable_by_key(|component| Reverse(component.len()));
    components
        .iter()
        .take(3)
        .fold(1, |acc, component| acc * component.len())
}

#[aoc_generator(day8, part2)]
fn parse_part2(input: &str) -> Vec<PackedPos> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day8, part2)]
fn part2_big(nodes: &[PackedPos]) -> u64 {
    const SIZE: usize = 1000;
    const EDGES: usize = SIZE * (SIZE - 1) / 2;
    part2::<SIZE, EDGES>(nodes)
}

#[cfg(test)]
fn part2_small(nodes: &[PackedPos]) -> u64 {
    const SIZE: usize = 20;
    const EDGES: usize = SIZE * (SIZE - 1) / 2;
    part2::<SIZE, EDGES>(nodes)
}

fn part2<const SIZE: usize, const EDGES: usize>(nodes: &[PackedPos]) -> u64 {
    // let start = Instant::now();
    let mut pairs = BinaryHeap::with_capacity(nodes.len() * (nodes.len() - 1) / 2);

    // let mut pairs = aoc_utils::BinaryHeap::<EDGES, _>::new();
    let mut forest = [Node { parent: 0, size: 1 }; SIZE];

    for (idx_a, &a) in nodes.iter().enumerate() {
        forest[idx_a].parent = idx_a;
        for (j, &b) in nodes[idx_a + 1..].iter().enumerate() {
            let distance = a.squared_distance(b);
            let idx_b = idx_a + j + 1;
            pairs.push(Reverse(distance.annotate((idx_a as u16, idx_b as u16))));
            // let packed_idx = ((idx_a as u32) << 10) + idx_b as u32;
            // pairs.push(distance, packed_idx);
        }
    }
    println!("Pairs: {}", pairs.len());
    // let heap_size = pairs.capacity() * std::mem::size_of_val(&pairs.peek());
    // println!("Heap size: {heap_size}");
    // let after_push = Instant::now();
    // let push_duration = after_push.duration_since(start);
    // println!("Push duration: {:?}", push_duration);

    let mut result = 0;
    // while let Some(packed_idx) = pairs.pop() {
    //     let i = (packed_idx >> 10) as usize;
    //     let j = (packed_idx & 0x3ff) as usize;
    while let Some(Reverse(Annotate {
        annotation: (a, b), ..
    })) = pairs.pop()
    {
        let i = a as usize;
        let j = b as usize;
        let u = find_set(i, &mut forest);
        let v = find_set(j, &mut forest);
        if u != v {
            union_sets(u, v, &mut forest);
            if forest[v].size == nodes.len() {
                result = nodes[i].x() as u64 * nodes[j].x() as u64;
                break;
            }
        }
    }
    return result;
}

#[derive(Debug, Clone, Copy)]
struct Node {
    parent: usize,
    size: usize,
}

fn find_set(x: usize, forest: &mut [Node]) -> usize {
    if forest[x].parent != x {
        forest[x].parent = find_set(forest[x].parent, forest);
    }
    forest[x].parent
}

fn union_sets(x: usize, y: usize, forest: &mut [Node]) {
    let mut rx = find_set(x, forest);
    let mut ry = find_set(y, forest);
    if rx == ry {
        return;
    }
    if forest[rx].size < forest[ry].size {
        std::mem::swap(&mut rx, &mut ry);
    }
    forest[ry].parent = rx;
    forest[rx].size += forest[ry].size;
}

#[aoc(day8, part2, aa)]
fn part2_aa_big(pos: &[PackedPos]) -> i64 {
    const SIZE: usize = 1000;
    const EDGES: usize = SIZE * (SIZE - 1) / 2;
    part2_aa::<SIZE, EDGES>(pos)
}

#[cfg(test)]
fn part2_aa_small(pos: &[PackedPos]) -> i64 {
    const SIZE: usize = 20;
    const EDGES: usize = SIZE * (SIZE - 1) / 2;
    part2_aa::<SIZE, EDGES>(pos)
}

fn part2_aa<const SIZE: usize, const EDGES: usize>(pos: &[PackedPos]) -> i64 {
    // solution originally authored by AA

    let mut edges = [(0u16, 0u16); EDGES];
    let mut dists = [0; EDGES];
    let mut forest = [Node { parent: 0, size: 1 }; SIZE];

    // Initialize forest and build edges
    let mut edge_idx = 0;
    for i in 0..pos.len() {
        forest[i].parent = i;
        for j in (i + 1)..pos.len() {
            edges[edge_idx] = (i as u16, j as u16);
            dists[edge_idx] = pos[i].squared_distance(pos[j]);
            edge_idx += 1;
        }
    }

    let mut indices = vec![0; EDGES];
    for i in 0..EDGES {
        indices[i] = i as u16;
    }
    indices.sort_unstable_by_key(|&i| dists[i as usize]);

    let mut result = 0;

    for n in 0..edge_idx {
        let idx = indices[n] as usize;
        let (i, j) = (edges[idx].0 as usize, edges[idx].1 as usize);

        let u = find_set(i, &mut forest);
        let v = find_set(j, &mut forest);
        if u != v {
            union_sets(u, v, &mut forest);
            if forest[v].size == SIZE {
                result = (pos[i].x() as i64 * pos[j].x() as i64) as i64;
                break;
            }
        }
    }

    result
}

example_tests! {
    "
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    ",
    parser: super::parse,
    part1 => 40,

    parser: super::parse_part2,
    part2_small => 25272,

    parser: super::parse_part2,
    part2_aa_small => 25272,
}

known_input_tests! {
    input: include_str!("../input/2025/day8.txt"),
    parser: super::parse,
    part1 => 244188,
    parser: super::parse_part2,
    part2_big => 8361881885,

    // disabled because it overflows the stack
    // parser: super::parse_part2,
    // part2_aa_big => 8361881885,
}
