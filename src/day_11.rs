/*
    Part 1 is about exploring a directed graph.
    I use petgraph and tried to make it fast.

    Part 2 was a bit harder to me because I didn't implement memoization correctly at first.
    I had to ask AI agents (yes I know) to help me do it correctly. A bit weird, because
    I have done memoization correctly in the past, I guess I was rusty.

    I thought about checking intermediate paths and multiplying them though.
*/

use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{line_ending, satisfy, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};
use petgraph::graph::{DiGraph, NodeIndex};

fn parse_identifier(input: &str) -> IResult<&str, u16> {
    map(
        (
            satisfy(|c: char| c.is_ascii_lowercase()),
            satisfy(|c: char| c.is_ascii_lowercase()),
            satisfy(|c: char| c.is_ascii_lowercase()),
        ),
        |(a, b, c)| {
            (a as u16 - b'a' as u16) * 676
                + (b as u16 - b'a' as u16) * 26
                + (c as u16 - b'a' as u16)
        },
    )
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (u16, Vec<u16>)> {
    separated_pair(
        parse_identifier,
        tag(": "),
        separated_list1(space1, parse_identifier),
    )
    .parse(input)
}

fn parse_input_data(input: &str) -> IResult<&str, Vec<(u16, Vec<u16>)>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn nb_possible_paths(graph: &DiGraph<(), (), u16>, start: u16, end: u16) -> i64 {
    let mut nb_paths = 0;
    let mut queue: Vec<u16> = vec![start];

    while let Some(current) = queue.pop() {
        for neighbor in graph.neighbors(NodeIndex::new(current as usize)) {
            let neighbor_index = neighbor.index() as u16;
            if neighbor_index == end {
                nb_paths += 1;
            } else {
                queue.push(neighbor_index);
            }
        }
    }
    nb_paths
}

fn nb_possible_paths_v2(graph: &DiGraph<(), (), u16>, start: u16, end: u16) -> i64 {
    // For a DAG, we can use memoization - no need to track visited nodes
    // since we can never revisit a node anyway
    fn count_paths(
        graph: &DiGraph<(), (), u16>,
        current: u16,
        end: u16,
        memo: &mut HashMap<u16, i64>,
    ) -> i64 {
        if current == end {
            return 1;
        }

        if let Some(&cached) = memo.get(&current) {
            return cached;
        }

        let count: i64 = graph
            .neighbors(NodeIndex::new(current as usize))
            .map(|neighbor| count_paths(graph, neighbor.index() as u16, end, memo))
            .sum();

        memo.insert(current, count);
        count
    }

    let mut memo = HashMap::new();
    count_paths(graph, start, end, &mut memo)
}

pub fn day_11_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let max_index = 17576; // 26^3

    let mut graph = DiGraph::<(), (), u16>::with_capacity(max_index, max_index * 2);

    for (device_from, edges) in data.iter() {
        graph.extend_with_edges(edges.iter().map(|device_to| (*device_from, *device_to)));
    }

    let (_, you) = parse_identifier("you").expect("Can't parse you");
    let (_, out) = parse_identifier("out").expect("Can't parse out");

    nb_possible_paths(&graph, you, out)
}

pub fn day_11_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let max_index = 17576; // 26^3

    let mut graph = DiGraph::<(), (), u16>::with_capacity(max_index, max_index * 2);

    for (device_from, edges) in data.iter() {
        graph.extend_with_edges(edges.iter().map(|device_to| (*device_from, *device_to)));
    }

    let (_, svr) = parse_identifier("svr").expect("Can't parse svr");
    let (_, out) = parse_identifier("out").expect("Can't parse out");
    let (_, fft) = parse_identifier("fft").expect("Can't parse fft");
    let (_, dac) = parse_identifier("dac").expect("Can't parse dac");

    let svr_to_fft = nb_possible_paths_v2(&graph, svr, fft);
    let fft_to_dac = nb_possible_paths_v2(&graph, fft, dac);
    let dac_to_out = nb_possible_paths_v2(&graph, dac, out);
    let svr_to_dac = nb_possible_paths_v2(&graph, svr, dac);
    let dac_to_fft = nb_possible_paths_v2(&graph, dac, fft);
    let fft_to_out = nb_possible_paths_v2(&graph, fft, out);

    svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(
            day_11_part_1(
                "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
            ),
            5
        );
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(
            day_11_part_2(
                "svr: aaa bbb
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
hhh: out"
            ),
            2
        );
    }
}
