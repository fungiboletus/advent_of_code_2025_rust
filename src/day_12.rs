/**
 * I felt like the problem was way too hard to be solved in a day.
 *
 * So I waited. Eventually I got spoiled by Reddit and found that
 * the actual input data had some very nice property that made the problem
 * trivial to solve.
 *
 * I also saw a few versions of the algorithm that was very easy to implement.
 * So I implemented a good enough parser and did the quick and dirty solution.
 *
 * I don't think I will attempt a proper optimal bin-packing solution for this one
 * before a while, if ever.
 *
 * It was the last one, and perhaps the most unsatisfying one.
 *
 * But a good short advent of code season overall!
 */
use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn parse_size(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        nom::character::complete::usize,
        char('x'),
        nom::character::complete::usize,
    )
    .parse(input)
}

type InputLine = ((usize, usize), Vec<usize>);
fn parse_line(input: &str) -> IResult<&str, InputLine> {
    separated_pair(
        parse_size,
        (char(':'), space1),
        separated_list1(space1, nom::character::complete::usize),
    )
    .parse(input)
}

fn parse_input_data(input: &str) -> IResult<&str, Vec<InputLine>> {
    separated_list1(line_ending, parse_line).parse(input)
}

pub fn day_12_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(&data[96..]).expect("Failed to parse input data");

    data.par_iter()
        .filter(|((w, h), n)| {
            let needed = n.iter().sum::<usize>();
            let available = (w / 3) * (h / 3);
            needed <= available
        })
        .count() as i64
}

pub fn day_12_part_2(_data: &str) -> i64 {
    42
}
