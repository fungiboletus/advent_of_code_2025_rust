/*
    Day 10 part 1 looks like a breadth-first search problem, with some dynamic programming or memoization.
    I think there is a hack / trick to do it very quickly, and I feel like we have seen that kind of problem before,
    in earlier years, but I will implement the straightforward solution first. It might be faster for part 1.

    Part 2 looks harder.

    ---
    One quick optimisation for part 1 is to convert the Vec<bool> to a BitVec.

    It runs a bit below 1ms on my machine.

    I implemented the XOR operation after getting the hint from the subreddit.
    ---
    As expected, part 2 is harder and I think I have to implement a proper solution
    this time. I'm guessing on graphs, and I will check the solution instead of
    trying to guess it.

*/

use std::{cmp::Reverse, collections::BinaryHeap};

use bitvec::{field::BitField, order::Lsb0, vec::BitVec, view::BitView};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, space1},
    combinator::value,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn parse_light_diagram(data: &str) -> IResult<&str, BitVec> {
    delimited(
        tag("["),
        many1(alt((value(false, char('.')), value(true, char('#')))))
            .map(|v| v.into_iter().collect()),
        tag("]"),
    )
    .parse(data)
}

fn parse_button_wiring_schematic(data: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("("),
        separated_list1(tag(","), nom::character::complete::usize),
        tag(")"),
    )
    .parse(data)
}

fn parse_joltage_requirements(data: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::u64),
        tag("}"),
    )
    .parse(data)
}

#[derive(Debug)]
struct Machine {
    light_diagram: BitVec,
    button_wiring_schematics: Vec<Vec<usize>>,
    joltage_requirements: Vec<u64>,
}

fn parse_machine(data: &str) -> IResult<&str, Machine> {
    (
        parse_light_diagram,
        space1,
        separated_list1(space1, parse_button_wiring_schematic),
        space1,
        parse_joltage_requirements,
    )
        .map(
            |(light_diagram, _, button_wiring_schematics, _, joltage_requirements)| Machine {
                light_diagram,
                button_wiring_schematics,
                joltage_requirements,
            },
        )
        .parse(data)
}

fn parse_input_data(data: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, parse_machine).parse(data)
}

fn part_1_machine_computation(machine: &Machine) -> usize {
    /*println!("bitvec: {:?}", machine.light_diagram);
    // size of bitvec in memory (struct included)
    println!("size: {}", std::mem::size_of_val(&machine.light_diagram));
    // serialise to u16*/
    let u16_light_diagram = machine.light_diagram.load_be::<u16>();
    //println!("as u16: {:016b}", u16_light_diagram);
    //println!("size of u16: {}", std::mem::size_of_val(&u16_light_diagram));

    let mut binary_heap: BinaryHeap<Reverse<(usize, u16)>> = BinaryHeap::new();
    binary_heap.push(Reverse((0, u16_light_diagram)));

    //let mut visited: BitVec = BitVec::repeat(false, 1 << machine.light_diagram.len());
    //let mut visited: HashSet<u16> = HashSet::new();
    let mut visited: Vec<bool> = vec![false; 1 << machine.light_diagram.len()];

    let wirings_as_u16: Vec<u16> = machine
        .button_wiring_schematics
        .iter()
        .map(|wiring| {
            let mut wiring_u16: u16 = 0;
            let bitvec_view = wiring_u16.view_bits_mut::<Lsb0>();
            for &button_index in wiring {
                bitvec_view.set(button_index, true);
            }

            wiring_u16
        })
        .collect();

    while let Some(Reverse((steps, state))) = binary_heap.pop() {
        if state == 0 {
            return steps;
        }

        //if *visited.get(state as usize).unwrap() {
        //if visited.contains(&state) {
        if visited[state as usize] {
            continue;
        }
        //visited.set(state as usize, true);
        //visited.insert(state);
        visited[state as usize] = true;

        /*for wiring in &machine.button_wiring_schematics {
            let mut new_state = state;
            let new_state_bitvec = new_state.view_bits_mut::<Lsb0>();

            for &button_index in wiring {
                new_state_bitvec.set(button_index, !new_state_bitvec[button_index]);
            }
            binary_heap.push(Reverse((steps + 1, new_state)));
        }*/
        for wiring_u16 in &wirings_as_u16 {
            let new_state = state ^ wiring_u16;
            binary_heap.push(Reverse((steps + 1, new_state)));
        }
    }

    unreachable!("Should always be able to reach the zero state")
}

pub fn day_10_part_1(data: &str) -> i64 {
    let (_, machines) = parse_input_data(data).expect("Failed to parse input data");

    machines
        .par_iter()
        .map(part_1_machine_computation)
        .sum::<usize>() as i64
}

pub fn day_10_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_day_10_part_1() {
        assert_eq!(day_10_part_1(EXAMPLE), 7);
    }

    #[test]
    fn test_day_10_part_2() {
        assert_eq!(day_10_part_2(EXAMPLE), 42);
    }
}
