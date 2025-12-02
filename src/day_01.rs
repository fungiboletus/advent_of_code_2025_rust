use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, value},
    multi::separated_list0,
};

fn parse_input_data(data: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(
        line_ending,
        map(
            (
                alt((value(-1, tag("L")), value(1, tag("R")))),
                nom::character::complete::i64,
            ),
            |(direction, distance)| direction * distance,
        ),
    )
    .parse(data)
}

pub fn day_01_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let mut position = 50;
    let mut count_at_zero = 0;
    for movement in data.iter() {
        position = (position + movement).rem_euclid(100);
        if position == 0 {
            count_at_zero += 1;
        }
    }

    count_at_zero
}

pub fn day_01_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    let mut position = 50;
    let mut count_at_zero = 0;

    for movement in data.iter() {
        if *movement < 0 {
            let started_at_zero = position == 0;
            position += movement;
            let crossed_zero = position <= 0 && !started_at_zero;
            if crossed_zero {
                count_at_zero += 1;
            }
        } else {
            position += movement;
        }
        let nb_cycles = position.abs().div_euclid(100);
        count_at_zero += nb_cycles;
        position = position.rem_euclid(100);
    }

    count_at_zero
}

// Quite an extensive set of tests as I tried to implement part 2
// while being sick. I tried a lot of edge cases suggested by redditors,
// without luck.
//
// It was much easier the next day.
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_day_01_part_1() {
        assert_eq!(day_01_part_1(EXAMPLE), 3);
    }

    #[test]
    fn test_day_01_part_2() {
        assert_eq!(day_01_part_2(EXAMPLE), 6);
    }

    #[test]
    fn test_day_01_part_2_minimal() {
        assert_eq!(day_01_part_2("L150"), 2);
        assert_eq!(day_01_part_2("R150"), 2);
    }

    #[test]
    fn test_day_01_part_2_minimal2() {
        assert_eq!(day_01_part_2("L51"), 1);
        assert_eq!(day_01_part_2("L1"), 0);
        assert_eq!(day_01_part_2("L50"), 1);
        assert_eq!(day_01_part_2("R50"), 1);
        assert_eq!(day_01_part_2("L50\nL100"), 2);
        assert_eq!(day_01_part_2("L50\nR100"), 2);
    }

    #[test]
    fn test_day_01_part_2_minimal3() {
        assert_eq!(day_01_part_2("R75"), 1);
    }

    #[test]
    fn test_day_01_part_2_minimal4() {
        assert_eq!(day_01_part_2("R49"), 0);
    }

    #[test]
    fn test_day_01_part_2_many() {
        assert_eq!(day_01_part_2("R1000"), 10);
        assert_eq!(day_01_part_2("L50\nR1000"), 11);
    }

    #[test]
    fn test_day_01_part_2_many2() {
        assert_eq!(day_01_part_2("L1000"), 10);
        assert_eq!(day_01_part_2("R50\nL1000"), 11);
        assert_eq!(day_01_part_2("L150"), 2);
        assert_eq!(day_01_part_2("L250"), 3);
        assert_eq!(day_01_part_2("L650"), 7);
    }

    #[test]
    fn test_1() {
        // 50 -> 99 -> 01 without crossing zero
        let count = day_01_part_2("R49\nL98");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_2() {
        // 50 -> 99 -> 00 ending up at zero
        let count = day_01_part_2("R49\nR1");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_3() {
        // 50 -> 99 -> 00 -> 01 stopping at zero once
        let count = day_01_part_2("R49\nR1\nR1");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_4() {
        // 50 -> 01 -> 00 -> 99 stopping at zero once
        let count = day_01_part_2("R49\nR1\nL1");
        assert_eq!(count, 1);
    }

    #[test]
    fn test_5() {
        // 50 -> 00 -> and a full rotation ending up at 00 again
        let count = day_01_part_2("L50\nL100");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_6() {
        // 50 -> 00 -> and a full rotation ending up at 00 again
        let count = day_01_part_2("R50\nR100");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_7() {
        // 50 -> 00 -> and 4 full rotations ending up at 00 again
        let count = day_01_part_2("L50\nL400");
        assert_eq!(count, 5);
    }

    #[test]
    fn test_8() {
        // 50 -> 00 -> and 4 full rotations ending up at 00 again
        let count = day_01_part_2("L50\nR400");
        assert_eq!(count, 5);
    }

    #[test]
    fn test_9() {
        // 50 and 10 full rotations ending up at 50 again
        let count = day_01_part_2("R1000");
        assert_eq!(count, 10);
    }

    #[test]
    fn test_again() {
        assert_eq!(day_01_part_2("L50\nR101"), 2);
        assert_eq!(day_01_part_2("L50\nL1"), 1);
    }

    #[test]
    fn test_again2() {
        assert_eq!(day_01_part_2("L50\nR400"), 5);
    }

    #[test]
    fn test_again3() {
        assert_eq!(day_01_part_2("L50\nR50"), 1);
        assert_eq!(day_01_part_2("L50\nL50"), 1);
        assert_eq!(day_01_part_2("R50\nR50\nL50\nL50\nR75\nL50"), 4);
        assert_eq!(day_01_part_2("L75\nR50"), 2);
    }

    #[test]
    fn test_again4() {
        assert_eq!(
            day_01_part_2("R50\nR50\nL50\nL50\nR75\nL50\nL25\nL75\nR50"),
            6
        );
    }

    #[test]
    fn test_actual_fail() {
        assert_eq!(day_01_part_2("R5\nR925"), 9);
    }

    #[test]
    fn test_actual_fail2() {
        assert_eq!(day_01_part_2("R30\nL687"), 7);
    }
}
