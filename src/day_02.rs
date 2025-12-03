use std::collections::HashSet;

use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending, combinator::opt,
    multi::separated_list0, sequence::separated_pair,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

    data.iter().fold(0, |mut sum_of_invalids, (start, end)| {
        /*assert!(start <= end, "Invalid range: {}-{}", start, end);
        assert!(
            start != &0,
            "Start of range cannot be zero: {}-{}",
            start,
            end
        );*/
        let mut nb_digits_start = start.ilog10() + 1;
        let mut nb_digits_end = end.ilog10() + 1;
        // If we have an odd number of digits at start,
        // we start from one digit more as we need a clean half
        if nb_digits_start % 2 == 1 {
            nb_digits_start += 1;
        }
        // Similarly we stop earlier for the end
        if nb_digits_end % 2 == 1 {
            nb_digits_end -= 1;
        }

        for nb_digits in nb_digits_start..=nb_digits_end {
            let half_size = nb_digits / 2;
            let divisor = 10u64.pow(half_size);

            let left_half_start = 10u64.pow(half_size - 1);
            let left_half_end = divisor - 1;

            for left_half in left_half_start..=left_half_end {
                let symmetric_number = left_half * divisor + left_half;
                if symmetric_number < *start {
                    continue;
                }
                if symmetric_number > *end {
                    break;
                }
                sum_of_invalids += symmetric_number as i64;
            }
        }

        sum_of_invalids
    })
}

pub fn day_02_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    data.par_iter()
        .map(|(start, end)| {
            let mut unique_symmetric_numbers: HashSet<u64> = HashSet::new();
            let nb_digits_end = end.ilog10() + 1;
            let max_nb_digits = nb_digits_end;

            for nb_repeating_patterns in 1..=max_nb_digits / 2 {
                let divisor = 10u64.pow(nb_repeating_patterns);
                let pattern_start = 10u64.pow(nb_repeating_patterns - 1);
                let pattern_end = divisor - 1;
                for pattern in pattern_start..=pattern_end {
                    for nb_repeats in 2..=nb_digits_end / nb_repeating_patterns {
                        let mut symmetric_number = 0u64;
                        for _i in 0..nb_repeats {
                            symmetric_number = symmetric_number * divisor + pattern;
                        }

                        if symmetric_number < *start {
                            continue;
                        }
                        if symmetric_number > *end {
                            break;
                        }
                        unique_symmetric_numbers.insert(symmetric_number);
                    }
                }
            }
            unique_symmetric_numbers.iter().sum::<u64>()
        })
        .sum::<u64>() as i64
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
        assert_eq!(day_02_part_2(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_day_02_part_2_minimal1() {
        assert_eq!(day_02_part_2("11-22"), 33);
    }

    #[test]
    fn test_day_02_part_2_minimal2() {
        assert_eq!(day_02_part_2("95-115"), 210);
    }

    #[test]
    fn test_day_02_part_2_minimal3() {
        assert_eq!(day_02_part_2("998-1012"), 2009);
    }

    #[test]
    fn test_day_02_part_2_minimal4() {
        assert_eq!(day_02_part_2("1188511880-1188511890"), 1188511885);
    }

    #[test]
    fn test_day_02_part_2_minimal5() {
        assert_eq!(day_02_part_2("222220-222224"), 222222);
    }

    #[test]
    fn test_part1_optimisation_fail1() {
        assert_eq!(day_02_part_1("825-1162"), 2121);
    }
}
