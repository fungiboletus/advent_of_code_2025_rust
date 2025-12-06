/*
    Another relatively easy day.

    Trying to convert using maths, because string conversion
    are slow. But the whitespaces are somewhat important,
    so we will parse the grid as text, then rotate it,
    and then parse it again.

*/

use ndarray::{Array2, Axis};
use nom::{
    AsChar, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, satisfy, space0, space1},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Add, tag("+")),
        value(Operation::Multiply, tag("*")),
    ))
    .parse(input)
}

fn parse_operations(data: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(space1, parse_operation).parse(data)
}

fn parse_number_grid(data: &str) -> IResult<&str, Array2<u64>> {
    map(
        separated_list1(
            (space0, line_ending, space0),
            separated_list1(space1, nom::character::complete::u64),
        ),
        |grid| {
            let nb_rows = grid.len();
            let nb_cols = grid.first().map_or(0, |row| row.len());
            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| grid[row][col])
        },
    )
    .parse(data)
}

fn parse_input_data_part1(data: &str) -> IResult<&str, (Array2<u64>, Vec<Operation>)> {
    separated_pair(parse_number_grid, line_ending, parse_operations).parse(data)
}

pub fn day_06_part_1(data: &str) -> i64 {
    let (_, (number_grid, operations)) =
        parse_input_data_part1(data).expect("Failed to parse input data");
    //println!("Number grid:\n{:?}", number_grid);
    //println!("Operations: {:?}", operations);

    operations
        .iter()
        .enumerate()
        .map(|(index, operation)| {
            let column = number_grid.column(index);
            match operation {
                Operation::Add => column.sum(),
                Operation::Multiply => column.product(),
            }
        })
        .sum::<u64>() as i64
}

fn parse_grid_as_chars(data: &str) -> IResult<&str, Array2<char>> {
    map(
        separated_list1(
            line_ending,
            many1(satisfy(|c: char| c.is_dec_digit() || c.is_space())),
        ),
        |grid| {
            let nb_rows = grid.len();
            let nb_cols = grid.first().map_or(0, |row| row.len());
            Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| grid[row][col])
        },
    )
    .parse(data)
}

fn parse_input_data_part2(data: &str) -> IResult<&str, (Array2<char>, Vec<Operation>)> {
    separated_pair(parse_grid_as_chars, line_ending, parse_operations).parse(data)
}

fn parse_rotated_input_data(data: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(
        (line_ending, space0, line_ending),
        separated_list1(
            line_ending,
            delimited(space0, nom::character::complete::u64, space0),
        ),
    )
    .parse(data)
}

pub fn day_06_part_2(data: &str) -> i64 {
    let (_, (grid_chars, operations)) =
        parse_input_data_part2(data).expect("Failed to parse input data");
    //println!("Char grid:\n{:?}", grid_chars);
    //println!("Operations: {:?}", operations);

    // rotate the grid chars 90 degrees anticlockwise
    let mut rotated = grid_chars.t().to_owned();
    rotated.invert_axis(Axis(0));
    //println!("Rotated char grid:\n{:?}", rotated);

    // convert the grid char to a string
    let string = rotated.rows().into_iter().fold(
        String::with_capacity(grid_chars.len() + grid_chars.nrows()),
        |mut acc, row| {
            for c in row.iter() {
                acc.push(*c);
            }
            acc.push('\n');
            acc
        },
    );
    //println!("Rotated string:\n{}", string);

    let (_, rotated_number_grid) =
        parse_rotated_input_data(&string).expect("Failed to parse rotated number grid");
    //println!("Rotated number grid:\n{:?}", rotated_number_grid);

    let nb_columns = rotated_number_grid.len();

    operations
        .iter()
        .enumerate()
        .map(|(index, operation)| {
            let column = rotated_number_grid
                .get(nb_columns - 1 - index)
                .expect("Column index out of bounds");
            match operation {
                Operation::Add => column.iter().sum::<u64>(),
                Operation::Multiply => column.iter().product::<u64>(),
            }
        })
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    // Notice the x20 extra spaces at the end
    // because the autoformatter kept removing trailing spaces from the EXAMPLE string
    const EXAMPLE: &str = "123 328  51 64\x20
 45 64  387 23\x20
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_day_06_part_1() {
        assert_eq!(day_06_part_1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_day_06_part_2() {
        assert_eq!(day_06_part_2(EXAMPLE), 3263827);
    }
}
