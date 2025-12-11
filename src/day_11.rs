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

    //let mut queue: Vec<(u16, Vec<u16>)> = vec![(svr, vec![])];
    let mut queue: Vec<(u16, bool, bool)> = vec![(svr, false, false)];

    let mut nb_paths = 0;

    /*fn u16_to_name(id: u16) -> String {
        let a = (id / 676) as u8 + b'a';
        let b = ((id % 676) / 26) as u8 + b'a';
        let c = (id % 26) as u8 + b'a';
        String::from_utf8(vec![a, b, c]).unwrap()
    }*/

    while let Some((current, visited_fft, visited_dac)) = queue.pop() {
        /*let new_history = {
            let mut h = history.clone();
            h.push(current);
            h
        };*/
        let visited_fft = visited_fft || (current == fft);
        let visited_dac = visited_dac || (current == dac);
        for neighbor in graph.neighbors(NodeIndex::new(current as usize)) {
            let neighbor_index = neighbor.index() as u16;
            if neighbor_index == out {
                //println!("{:?}", new_history);
                /*for id in &new_history {
                    print!("{} ", u16_to_name(*id));
                }
                println!();*/
                //if new_history.contains(&fft) && new_history.contains(&dac) {
                if visited_dac && visited_fft {
                    /*println!("OK {:?}", new_history);
                    for id in &new_history {
                        print!("{} ", u16_to_name(*id));
                    }
                    println!();*/
                    nb_paths += 1;
                }
            } else {
                queue.push((neighbor_index, visited_fft, visited_dac));
            }
        }
    }

    nb_paths
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
