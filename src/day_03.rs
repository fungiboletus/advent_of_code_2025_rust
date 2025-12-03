/*
    easy day, first try on part 2
*/

use nom::{
    AsChar, IResult, Parser,
    character::complete::{line_ending, satisfy},
    combinator::map,
    multi::{many1, separated_list0},
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list0(
        line_ending,
        many1(map(satisfy(|c| c.is_dec_digit()), |c| {
            c.as_char() as u8 - b'0'
        })),
    )
    .parse(data)
}

pub fn day_03_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    data.iter()
        .map(|bank| {
            // find the highest digit and its position from left to right
            let (max_digit, max_pos) = bank[..bank.len() - 1].iter().enumerate().fold(
                (0u8, 0usize),
                |(current_max, current_pos), (idx, &digit)| {
                    if digit > current_max {
                        (digit, idx)
                    } else {
                        (current_max, current_pos)
                    }
                },
            );
            // find second highest digit after the position of the highest digit
            let second_max_digit =
                bank.iter()
                    .skip(max_pos + 1)
                    .fold(0u8, |current_max, &digit| {
                        if digit > current_max {
                            digit
                        } else {
                            current_max
                        }
                    });
            (max_digit * 10 + second_max_digit) as i64
        })
        .sum()
}

pub fn day_03_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    data.iter()
        .map(|bank| {
            let mut number: i64 = 0;
            let mut previous_digit_position = -1isize;
            let bank_len = bank.len();

            for digit_id in (1..=12).rev() {
                // find the highest digit after the previous digit position
                let (highest_digit, highest_digit_position) = bank
                    [(previous_digit_position + 1) as usize..bank_len - (digit_id - 1)]
                    .iter()
                    .enumerate()
                    .fold(
                        (0u8, 0usize),
                        |(current_max, current_pos), (idx, &digit)| {
                            if digit > current_max {
                                (digit, idx)
                            } else {
                                (current_max, current_pos)
                            }
                        },
                    );
                number = number * 10 + highest_digit as i64;
                previous_digit_position += (highest_digit_position + 1) as isize;
            }
            number
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_day_03_part_1() {
        assert_eq!(day_03_part_1(EXAMPLE), 357);
    }

    #[test]
    fn test_day_03_part_2() {
        assert_eq!(day_03_part_2(EXAMPLE), 3121910778619);
    }
}
