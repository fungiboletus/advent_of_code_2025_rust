/*
    Ugly code for part 1, but it works.

    Part 2 is the fun one. Required to do part 1 again in a cleaner way first.
*/

use ndarray::Array2;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, value},
    multi::{many1, separated_list1},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    BeamStart,
    Splitter,
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        value(Cell::Empty, tag(".")),
        value(Cell::BeamStart, tag("S")),
        value(Cell::Splitter, tag("^")),
    ))
    .parse(input)
}

fn parse_input_data(data: &str) -> IResult<&str, Array2<Cell>> {
    map(separated_list1(line_ending, many1(parse_cell)), |grid| {
        let nb_rows = grid.len();
        let nb_cols = grid.first().map_or(0, |row| row.len());
        Array2::from_shape_fn((nb_rows, nb_cols), |(row, col)| grid[row][col])
    })
    .parse(data)
}

pub fn day_07_part_1(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let (nb_rows, nb_cols) = grid.dim();
    assert!(nb_rows > 1, "We need at least two rows");
    assert!(nb_cols > 1, "We need at least two columns");
    let max_col_index = nb_cols - 1;

    let mut nb_splits = 0;
    let mut previous_row = vec![false; nb_cols];
    for (col_index, cell) in grid.row(0).iter().enumerate() {
        if cell == &Cell::BeamStart {
            previous_row[col_index] = true;
        }
    }
    for row in grid.rows() {
        let mut current_row = vec![false; nb_cols];
        for col_index in 0..=max_col_index {
            current_row[col_index] = previous_row[col_index]
                || (col_index > 0
                    && (row[col_index - 1] == Cell::Splitter && previous_row[col_index - 1]))
                || (col_index < max_col_index
                    && (row[col_index + 1] == Cell::Splitter && previous_row[col_index + 1]));
            if row[col_index] == Cell::Splitter && current_row[col_index] {
                nb_splits += 1;
                current_row[col_index] = false;
            }
        }

        // representation of the current row
        /*let mut _repr = String::with_capacity(nb_cols);
        for col_index in 0..nb_cols {
            if current_row[col_index] {
                _repr.push('B');
            } else {
                _repr.push('.');
            }
        }
        println!("Current row beams: {}", _repr);*/
        previous_row = current_row
    }

    nb_splits
}

pub fn day_07_part_2(data: &str) -> i64 {
    let (_, grid) = parse_input_data(data).expect("Failed to parse input data");
    let (nb_rows, nb_cols) = grid.dim();
    assert!(nb_rows > 1, "We need at least two rows");
    assert!(nb_cols > 1, "We need at least two columns");
    let max_col_index = nb_cols - 1;

    //let mut nb_splits = 0;
    let mut previous_row = vec![false; nb_cols];
    let mut nb_timelines_previous_row = vec![0_u64; nb_cols];
    for row in grid.rows() {
        let mut current_row = vec![false; nb_cols];
        let mut nb_timelines_current_row = vec![0_u64; nb_cols];
        for col_index in 0..=max_col_index {
            match (row[col_index], previous_row[col_index]) {
                (Cell::BeamStart, false) => {
                    current_row[col_index] = true;
                    nb_timelines_current_row[col_index] = 1;
                }
                (Cell::Empty, true) => {
                    current_row[col_index] = true;
                    nb_timelines_current_row[col_index] += nb_timelines_previous_row[col_index];
                }
                (Cell::Splitter, true) => {
                    //nb_splits += 1;
                    // split the timelines to left and right
                    if col_index > 0 {
                        current_row[col_index - 1] = true;
                        nb_timelines_current_row[col_index - 1] +=
                            nb_timelines_previous_row[col_index];
                    }
                    if col_index < max_col_index {
                        current_row[col_index + 1] = true;
                        nb_timelines_current_row[col_index + 1] +=
                            nb_timelines_previous_row[col_index];
                    }
                }
                _ => {}
            }
        }

        // show the line
        /*let mut _repr_grid = String::with_capacity(nb_cols);
        for col_index in 0..nb_cols {
            match row[col_index] {
                Cell::Empty => _repr_grid.push_str(" ."),
                Cell::BeamStart => _repr_grid.push_str(" S"),
                Cell::Splitter => _repr_grid.push_str(" ^"),
            }
        }
        println!("{}", _repr_grid);
        // representation of the current row
        let mut _repr = String::with_capacity(nb_cols);
        for col_index in 0..nb_cols {
            if current_row[col_index] {
                _repr.push_str(" B");
            } else {
                _repr.push_str("  ");
            }
        }
        println!("{}", _repr);
        // representation of the number of timelines per cell
        let mut _repr_timelines = String::with_capacity(nb_cols * 2);
        for col_index in 0..nb_cols {
            if nb_timelines_current_row[col_index] > 0 {
                _repr_timelines.push_str(&format!("{:2}", nb_timelines_current_row[col_index]));
            } else {
                _repr_timelines.push_str("  ");
            }
        }
        println!("{}", _repr_timelines);*/
        previous_row = current_row;
        nb_timelines_previous_row = nb_timelines_current_row;
    }

    //println!("Number of splits: {}", nb_splits);
    nb_timelines_previous_row.iter().sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_day_07_part_1() {
        assert_eq!(day_07_part_1(EXAMPLE), 21);
    }

    #[test]
    fn test_day_07_part_2() {
        assert_eq!(day_07_part_2(EXAMPLE), 40);
    }
}
