/*
    Part 1: mostly playing with rust and ndarray
*/
use ndarray::{Array2, s};
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
    PaperRoll,
}

// implement to u8
impl From<Cell> for u8 {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Empty => 0,
            Cell::PaperRoll => 1,
        }
    }
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        value(Cell::Empty, tag(".")),
        value(Cell::PaperRoll, tag("@")),
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

fn pad(input: &Array2<Cell>, pad_width: usize, pad_value: Cell) -> Array2<Cell> {
    // create the empty new
    let mut new_array = Array2::from_elem(
        (
            input.nrows() + pad_width + pad_width,
            input.ncols() + pad_width + pad_width,
        ),
        pad_value,
    );

    // copy the input into the center of the new array
    let mut view = new_array.slice_mut(s![
        pad_width..pad_width + input.nrows(),
        pad_width..pad_width + input.ncols()
    ]);
    view.assign(input);
    new_array
}

pub fn day_04_part_1(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");

    // Padding to avoid dealing with boundaries
    let padded = pad(&data, 1, Cell::Empty);

    padded
        .windows((3, 3))
        .into_iter()
        .filter(|window| {
            let center = window[(1, 1)];
            if center != Cell::PaperRoll {
                return false;
            }
            let sum = window.iter().fold(0, |acc, cell| acc + u8::from(*cell));
            sum < 5
        })
        .count() as i64
}

pub fn day_04_part_2(data: &str) -> i64 {
    let (_, data) = parse_input_data(data).expect("Failed to parse input data");
    let padded = pad(&data, 1, Cell::Empty);

    let mut grid = padded;
    let mut count = 0_i64;

    let mut queue: Vec<(usize, usize)> = Vec::with_capacity(grid.len());
    // Check everywhere at least once
    for row in 1..(grid.nrows() - 1) {
        for col in 1..(grid.ncols() - 1) {
            if grid[(row, col)] == Cell::PaperRoll {
                queue.push((row, col));
            }
        }
    }

    while let Some((row, col)) = queue.pop() {
        if grid[(row, col)] != Cell::PaperRoll {
            continue;
        }
        let sum: u8 = u8::from(grid[(row - 1, col - 1)])
            + u8::from(grid[(row - 1, col)])
            + u8::from(grid[(row - 1, col + 1)])
            + u8::from(grid[(row, col - 1)])
            + u8::from(grid[(row, col + 1)])
            + u8::from(grid[(row + 1, col - 1)])
            + u8::from(grid[(row + 1, col)])
            + u8::from(grid[(row + 1, col + 1)]);
        if sum < 4 {
            grid[(row, col)] = Cell::Empty;
            count += 1;
            // add neighbors to the queue
            if grid[(row - 1, col - 1)] == Cell::PaperRoll {
                queue.push((row - 1, col - 1));
            }
            if grid[(row - 1, col)] == Cell::PaperRoll {
                queue.push((row - 1, col));
            }
            if grid[(row - 1, col + 1)] == Cell::PaperRoll {
                queue.push((row - 1, col + 1));
            }
            if grid[(row, col - 1)] == Cell::PaperRoll {
                queue.push((row, col - 1));
            }
            if grid[(row, col + 1)] == Cell::PaperRoll {
                queue.push((row, col + 1));
            }
            if grid[(row + 1, col - 1)] == Cell::PaperRoll {
                queue.push((row + 1, col - 1));
            }
            if grid[(row + 1, col)] == Cell::PaperRoll {
                queue.push((row + 1, col));
            }
            if grid[(row + 1, col + 1)] == Cell::PaperRoll {
                queue.push((row + 1, col + 1));
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_day_04_part_1() {
        assert_eq!(day_04_part_1(EXAMPLE), 13);
    }

    #[test]
    fn test_day_04_part_2() {
        assert_eq!(day_04_part_2(EXAMPLE), 43);
    }
}
