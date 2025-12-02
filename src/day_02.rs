/*
    Comments.
*/

use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending, combinator::opt,
    multi::separated_list0, sequence::separated_pair,
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list0(
        (tag(","), opt(line_ending)),
        separated_pair(
            nom::character::complete::u64,
            tag("-"),
            nom::character::complete::u64,
        ),
    )
    .parse(data)
}

pub fn day_02_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let mut sum_of_invalids = 0;

    for (start, end) in data.iter() {
        /*assert!(start <= end, "Invalid range: {}-{}", start, end);
        assert!(
            start != &0,
            "Start of range cannot be zero: {}-{}",
            start,
            end
        );*/

        for number in *start..=*end {
            let nb_digits = number.ilog10() + 1;
            // Odd number of digits cannot have symmetric halves
            if nb_digits % 2 == 0 {
                let half_size = nb_digits / 2;
                let divisor = 10u64.pow(half_size);
                let left_half = number / divisor;
                let right_half = number % divisor;
                if left_half == right_half {
                    sum_of_invalids += number as i64;
                }
            }
        }
    }

    sum_of_invalids
}

pub fn day_02_part_2(data: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_day_02_part_1() {
        assert_eq!(day_02_part_1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_day_02_part_2() {
        assert_eq!(day_02_part_2(EXAMPLE), 42);
    }
}
