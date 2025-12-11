/*
    Part 1 is about exploring a directed graph.
    I use petgraph and tried to make it fast.
*/

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

pub fn day_11_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let max_index = 17576; // 26^3

    let mut graph = DiGraph::<(), (), u16>::with_capacity(max_index, max_index * 2);

    for (device_from, edges) in data.iter() {
        graph.extend_with_edges(edges.iter().map(|device_to| (*device_from, *device_to)));
    }

    let (_, you) = parse_identifier("you").expect("Can't parse you");
    let (_, out) = parse_identifier("out").expect("Can't parse out");

    let mut queue: Vec<u16> = vec![you];

    let mut nb_paths = 0;

    while let Some(current) = queue.pop() {
        for neighbor in graph.neighbors(NodeIndex::new(current as usize)) {
            let neighbor_index = neighbor.index() as u16;
            if neighbor_index == out {
                nb_paths += 1;
            } else {
                queue.push(neighbor_index);
            }
        }
    }

    nb_paths
}

pub fn day_11_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    #[test]
    fn test_day_11_part_1() {
        assert_eq!(day_11_part_1(EXAMPLE), 5);
    }

    #[test]
    fn test_day_11_part_2() {
        assert_eq!(day_11_part_2(EXAMPLE), 42);
    }
}
