use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::{AsciiUtils, example_tests, known_input_tests};

// #[derive(Clone, Copy, PartialEq, Eq, Hash)]
// struct Label([u8; 3]);

// impl Label {
//     const YOU: Label = Label([b'y', b'o', b'u']);
//     const OUT: Label = Label([b'o', b'u', b't']);
//     const SVR: Label = Label([b's', b'v', b'r']);
//     const DAC: Label = Label([b'd', b'a', b'c']);
//     const FFT: Label = Label([b'f', b'f', b't']);

//     const fn new(input: &[u8]) -> Self {
//         let mut label = [0; 3];
//         label.copy_from_slice(input);
//         Self(label)
//     }

//     fn as_index(self) -> usize {
//         debug_assert!(self.0.iter().all(|&c| c.is_ascii_lowercase()));
//         (self.0[0] - b'a') as usize
//             + ((self.0[1] - b'a') as usize) * 26
//             + ((self.0[2] - b'a') as usize) * 26 * 26
//     }
// }

// impl std::fmt::Debug for Label {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Label(u16);

impl Label {
    const YOU: Label = Label(0);
    const OUT: Label = Label(1);
    const SVR: Label = Label(2);
    const DAC: Label = Label(3);
    const FFT: Label = Label(4);

    fn as_index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    label: Label,
    children: Box<[Label]>,
}

#[aoc_generator(day11)]
fn parse(input: &[u8]) -> Vec<Node> {
    let mut labels = HashMap::new();
    // add notable labels
    labels.insert([b'y', b'o', b'u'], Label::YOU);
    labels.insert([b'o', b'u', b't'], Label::OUT);
    labels.insert([b's', b'v', b'r'], Label::SVR);
    labels.insert([b'f', b'f', b't'], Label::FFT);
    labels.insert([b'd', b'a', b'c'], Label::DAC);
    let mut new_label = |label: &[u8]| {
        let key = [label[0], label[1], label[2]];
        let suggested_label = labels.len();
        *labels
            .entry(key)
            .or_insert(Label(suggested_label.try_into().unwrap()))
    };
    input
        .ascii_lines()
        .map(|line| {
            let label = new_label(&line[..3]);
            let children = line[5..]
                .chunks(4)
                .map(|chunk| new_label(&chunk[..3]))
                .collect();
            Node { label, children }
        })
        .collect()
}

fn count_paths_between(edges: &HashMap<Label, Box<[Label]>>, start: Label, end: Label) -> u64 {
    const MAX_LABELS: usize = 600;

    fn recurse(
        edges: &HashMap<Label, Box<[Label]>>,
        start: Label,
        end: Label,
        counts: &mut [u64; MAX_LABELS],
    ) -> u64 {
        debug_assert_eq!(counts[start.as_index()], u64::MAX);
        let mut count = 0;
        if let Some(children) = edges.get(&start) {
            for child in children {
                if *child == end {
                    count += 1;
                } else if counts[child.as_index()] != u64::MAX {
                    count += counts[child.as_index()];
                } else {
                    count += recurse(edges, *child, end, counts);
                }
            }
        }
        *(counts.get_mut(start.as_index()).unwrap()) = count;
        count
    }

    #[allow(clippy::large_stack_arrays)]
    let mut counts = [u64::MAX; MAX_LABELS];
    recurse(edges, start, end, &mut counts) as _
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
