/*
    Part 1 is a bit harder to do it fast.
    I decided to wait for the end of the day to do it properly instead of rushing
    a O(n^2) solution.

    I first try with a variation of my favourite algorithm for spatial problems:
    sweep and prune. because it's fast and simple. I don't expect to be remotely
    close to the worst case.

    It appears during testing that for the actual input data, the O(n²) solution
    is actually slightly faster. I kept the slower sleep and prune because I find it cool
    and it's not significantly slower, but yeah. That's a bit disappointing.

    My initial plan was to use petgraph, but I got a spoiler by mistake and
    learned that DSU would be the more appropriate data structure for this problem.

    So I decided to not ignore the spoiler, and go with a DSU crate. disjoint looked
    promising.

    Part 2 was a bit more challenging to do fast. I did use a shortcut
    by reusing code from the part 1 and hard-coding some shortcut threshold
    with a failover to the full O(n²) solution if needed.

    disjoin set unions seem like a nice fit.

    I think this could be further optimised, but this sounds good enough for now.
*/

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
};

use disjoint::DisjointSet;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending, combinator::map,
    multi::separated_list1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
}

impl JunctionBox {
    fn square_distance(&self, other: &JunctionBox) -> u64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

fn parse_junction_box(input: &str) -> IResult<&str, JunctionBox> {
    map(
        (
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        |(x, _, y, _, z)| JunctionBox { x, y, z },
    )
    .parse(input)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<JunctionBox>> {
    separated_list1(line_ending, parse_junction_box).parse(data)
}

#[derive(Eq, PartialEq)]
struct ConnectingPair {
    a: JunctionBox,
    index_a: usize,
    b: JunctionBox,
    index_b: usize,
    square_distance: u64,
}

impl Ord for ConnectingPair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.square_distance.cmp(&other.square_distance)
    }
}

impl PartialOrd for ConnectingPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_closest_junction_boxes(
    junction_boxes: &[JunctionBox],
    nb_max_connections: usize,
) -> BinaryHeap<ConnectingPair> {
    let mut shortest_connections: BinaryHeap<ConnectingPair> =
        BinaryHeap::with_capacity(nb_max_connections);

    // sort the junction boxes by x coordinate
    let mut boxes_indexed: Vec<(JunctionBox, usize)> = junction_boxes
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, jb)| (jb, i))
        .collect();
    //let mut sorted_boxes = junction_boxes.to_owned();
    boxes_indexed.sort_unstable_by_key(|(jb, _)| jb.x);

    //let mut active_boxes: Vec<JunctionBox> = Vec::with_capacity(nb_max_connections);
    let mut active_boxes: VecDeque<(JunctionBox, usize)> =
        VecDeque::with_capacity(nb_max_connections);

    for (current_box, current_index) in boxes_indexed.iter() {
        if shortest_connections.len() == nb_max_connections {
            let max_distance = shortest_connections
                .peek()
                .expect("Heap is not empty")
                .square_distance;
            //active_boxes.retain(|jb| (current_box.x - jb.x) as u64 <= max_distance);
            while let Some((front_box, _)) = active_boxes.front() {
                if (current_box.x - front_box.x) as u64 > max_distance {
                    active_boxes.pop_front();
                } else {
                    break;
                }
            }
        }

        for (active_box, active_index) in active_boxes.iter() {
            let square_distance = current_box.square_distance(active_box);
            if shortest_connections.len() < nb_max_connections {
                shortest_connections.push(ConnectingPair {
                    a: *current_box,
                    index_a: *current_index,
                    b: *active_box,
                    index_b: *active_index,
                    square_distance,
                });
            } else if let Some(max_connection) = shortest_connections.peek()
                && square_distance < max_connection.square_distance
            {
                shortest_connections.pop();
                shortest_connections.push(ConnectingPair {
                    a: *current_box,
                    index_a: *current_index,
                    b: *active_box,
                    index_b: *active_index,
                    square_distance,
                });
            }
        }

        active_boxes.push_back((*current_box, *current_index));
    }

    shortest_connections
}

pub fn day_08_part_1(data: &str) -> i64 {
    let (_, junction_boxes) = parse_input_data(data).expect("Failed to parse input data");
    //println!("Parsed junction boxes: {:?}", junction_boxes);

    // Quick hack to switch between example data and actual data
    let nb_max_connections = if junction_boxes.len() >= 32 { 1000 } else { 10 };

    let shortest_connections = find_closest_junction_boxes(&junction_boxes, nb_max_connections);

    let mut disjoint_set = DisjointSet::with_len(junction_boxes.len());
    //println!("Shortest connections:");
    for connection in shortest_connections.iter() {
        /*println!(
            "{:?} <-> {:?} : {}",
            connection.a, connection.b, connection.square_distance
        );*/
        disjoint_set.join(connection.index_a, connection.index_b);
    }

    let mut sets = disjoint_set.sets();
    let len = sets.len();
    assert!(
        len >= 3,
        "We need at least 3 sets of connected junction boxes"
    );
    let k = len - 3;
    // find the 3 largest sets
    sets.select_nth_unstable_by(k, |a, b| a.len().cmp(&b.len()));
    let sets = &sets[k..];

    //println!("sets: {:?}", sets);

    (sets[0].len() * sets[1].len() * sets[2].len()) as i64
}

fn compute_all_connection_pairs(junction_boxes: &[JunctionBox]) -> Vec<ConnectingPair> {
    let mut connection_pairs: Vec<ConnectingPair> =
        Vec::with_capacity(junction_boxes.len() * (junction_boxes.len() - 1) / 2);

    for (index_a, box_a) in junction_boxes.iter().enumerate() {
        for (index_b, box_b) in junction_boxes.iter().enumerate().skip(index_a + 1) {
            let square_distance = box_a.square_distance(box_b);
            connection_pairs.push(ConnectingPair {
                a: *box_a,
                index_a,
                b: *box_b,
                index_b,
                square_distance,
            });
        }
    }
    connection_pairs.sort_unstable_by_key(|conn| conn.square_distance);

    connection_pairs
}

pub fn day_08_part_2(data: &str) -> i64 {
    let (_, junction_boxes) = parse_input_data(data).expect("Failed to parse input data");
    let nb_junction_boxes = junction_boxes.len();

    let mut disjoint_set = DisjointSet::with_len(nb_junction_boxes);
    let mut current_count_of_sets = nb_junction_boxes;

    // try with low values and then the max (yes it's cheating a bit)
    for nb_connections in [
        0,
        //nb_junction_boxes * 2,
        nb_junction_boxes * 5,
        nb_junction_boxes * 10,
        nb_junction_boxes * 15,
        usize::MAX,
    ]
    .windows(2)
    {
        let previous_nb_connections = nb_connections[0];
        let nb_connections = nb_connections[1];
        let shortest_connections = if nb_connections < usize::MAX {
            find_closest_junction_boxes(&junction_boxes, nb_connections).into_sorted_vec()
        } else {
            compute_all_connection_pairs(&junction_boxes)
        };

        for connection in shortest_connections.iter().skip(previous_nb_connections) {
            if disjoint_set.join(connection.index_a, connection.index_b) {
                current_count_of_sets -= 1;
            }
            if current_count_of_sets == 1 {
                return (connection.a.x * connection.b.x) as i64;
            }
        }
    }

    unreachable!("Should have found the last connection");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
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
425,690,689";

    #[test]
    fn test_day_08_part_1() {
        assert_eq!(day_08_part_1(EXAMPLE), 40);
    }

    #[test]
    fn test_day_08_part_2() {
        assert_eq!(day_08_part_2(EXAMPLE), 25272);
    }
}
