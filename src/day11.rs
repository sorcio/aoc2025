use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, example_tests, known_input_tests};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Label([u8; 3]);

impl Label {
    const YOU: Label = Label([b'y', b'o', b'u']);
    const OUT: Label = Label([b'o', b'u', b't']);
    const SVR: Label = Label([b's', b'v', b'r']);
    const DAC: Label = Label([b'd', b'a', b'c']);
    const FFT: Label = Label([b'f', b'f', b't']);

    const fn new(input: &[u8]) -> Self {
        let mut label = [0; 3];
        label.copy_from_slice(input);
        Self(label)
    }
}

impl std::fmt::Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    label: Label,
    children: Box<[Label]>,
}

#[aoc_generator(day11)]
fn parse(input: &[u8]) -> Vec<Node> {
    input
        .ascii_lines()
        .map(|line| {
            let label = Label::new(&line[..3]);
            let children = line[5..]
                .chunks(4)
                .map(|chunk| Label::new(&chunk[..3]))
                .collect();
            Node { label, children }
        })
        .collect()
}

fn topo_sort_visit(node: Label, topo: &mut Vec<Label>, edges: &HashMap<Label, Box<[Label]>>) {
    if topo.contains(&node) {
        return;
    }
    if let Some(children) = edges.get(&node) {
        for child in children {
            topo_sort_visit(*child, topo, edges);
        }
    }
    topo.push(node);
}

fn count_paths_between(edges: &HashMap<Label, Box<[Label]>>, start: Label, end: Label) -> u64 {
    // TODO: maybe don't do this each time
    let mut predecessors = HashMap::new();
    for node in edges.keys() {
        for child in &edges[node] {
            predecessors.entry(*child).or_insert(Vec::new()).push(*node);
        }
    }

    let mut topo = vec![];
    topo_sort_visit(start, &mut topo, edges);

    let mut counts = HashMap::new();
    counts.insert(end, 1);
    for node in &topo {
        if node == &start {
            break;
        }
        if let Some(&count) = counts.get(node)
            && count > 0
        {
            for predecessor in predecessors.get(node).unwrap_or(&Vec::new()) {
                *counts.entry(*predecessor).or_insert(0) += count;
            }
        }
    }
    *counts.get(&start).unwrap_or(&0)
}

#[aoc(day11, part1)]
fn part1(input: &[Node]) -> u64 {
    let mut edges = std::collections::HashMap::new();
    for node in input {
        edges.insert(node.label, node.children.clone());
    }
    count_paths_between(&edges, Label::YOU, Label::OUT)
}

#[aoc(day11, part2)]
fn part2(input: &[Node]) -> u64 {
    let mut edges = std::collections::HashMap::new();
    for node in input {
        edges.insert(node.label, node.children.clone());
    }
    let svr_to_dac = count_paths_between(&edges, Label::SVR, Label::DAC);
    let dac_to_fft = count_paths_between(&edges, Label::DAC, Label::FFT);
    let svr_to_fft = count_paths_between(&edges, Label::SVR, Label::FFT);
    let fft_to_dac = count_paths_between(&edges, Label::FFT, Label::DAC);
    let dac_to_out = count_paths_between(&edges, Label::DAC, Label::OUT);
    let fft_to_out = count_paths_between(&edges, Label::FFT, Label::OUT);
    svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out
}

example_tests! {
    b"
    aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out
    ",
    parser: super::parse,
    part1 => 5,

    parser: super::parse,
    b"
    svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out
    ",
    part2 => 2,
}

known_input_tests! {
    input: include_bytes!("../input/2025/day11.txt"),
    part1 => 497,
    part2 => 358564784931864,
}
