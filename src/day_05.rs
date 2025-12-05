/*
    Part 1 is very easy, let's do it super stupidly in O(n*m) and see what parts 2 is about,
    before optimising both.

    Part 2 is finally not too bad.

    Then optimised part 1 with merging ranges and binary search.
*/

use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending,
    multi::separated_list1, sequence::separated_pair,
};

fn parse_range(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        nom::character::complete::u64,
        tag("-"),
        nom::character::complete::u64,
    )
    .parse(input)
}

type Ranges = Vec<(u64, u64)>;
fn parse_ranges(data: &str) -> IResult<&str, Ranges> {
    separated_list1(line_ending, parse_range).parse(data)
}

type Ingredients = Vec<u64>;
fn parse_ingredients(data: &str) -> IResult<&str, Ingredients> {
    separated_list1(line_ending, nom::character::complete::u64).parse(data)
}

fn parse_input_data(data: &str) -> IResult<&str, (Ranges, Ingredients)> {
    separated_pair(parse_ranges, (line_ending, line_ending), parse_ingredients).parse(data)
}

pub fn day_05_part_1(data: &str) -> i64 {
    let (_, (ranges, ingredients)) = parse_input_data(data).expect("Failed to parse input data");
    assert!(!ranges.is_empty(), "No ranges provided");
    //println!("Ranges: {:?}", ranges);
    //println!("Ingredients: {:?}", ingredients);
    // println!("nb ranges: {}", ranges.len());

    // optimisations done after part 2
    // reduce the range space
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|(start, _end)| *start);
    let mut reduced_ranges = Vec::new();
    let mut current_start = sorted_ranges[0].0;
    let mut current_end = sorted_ranges[0].1;
    for (start, end) in sorted_ranges.iter().skip(1) {
        if *start > current_end + 1 {
            // no overlap
            reduced_ranges.push((current_start, current_end));
            current_start = *start;
            current_end = *end;
        } else if *end > current_end {
            // extend the current range, only if higher end
            current_end = *end;
        }
    }
    // add the last range
    reduced_ranges.push((current_start, current_end));
    let sorted_ranges = reduced_ranges;
    // println!("nb ranges: {}", sorted_ranges.len());

    ingredients
        .iter()
        .map(|&ingredient| {
            let range_index =
                match sorted_ranges.partition_point(|&(start, _end)| start <= ingredient) {
                    0 => return false, // before first range
                    idx => idx - 1,
                };
            let (range_start, range_end) = sorted_ranges[range_index];
            ingredient >= range_start && ingredient <= range_end
        })
        .filter(|&is_valid| is_valid)
        .count() as i64
}

pub fn day_05_part_2(data: &str) -> i64 {
    let (_, (ranges, _)) = parse_input_data(data).expect("Failed to parse input data");
    assert!(!ranges.is_empty(), "No ranges provided");

    // sort the ranges by start, ascending
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|(start, _end)| *start);

    let mut space_covered = 0_i64;
    let mut current_start = sorted_ranges[0].0;
    let mut current_end = sorted_ranges[0].1;

    for (start, end) in sorted_ranges.iter().skip(1) {
        if *start > current_end + 1 {
            // no overlap
            space_covered += (current_end - current_start + 1) as i64;
            current_start = *start;
            current_end = *end;
        } else if *end > current_end {
            // extend the current range, only if higher end
            current_end = *end;
        }
    }
    // add the last range
    space_covered += (current_end - current_start + 1) as i64;

    space_covered
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_day_05_part_1() {
        assert_eq!(day_05_part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_05_part_2() {
        assert_eq!(day_05_part_2(EXAMPLE), 14);
    }
}
