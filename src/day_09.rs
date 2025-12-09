/*
    Part 1 is pretty easy. The O(n²) obvious solution was slightly optimised with
    a sort on one axis first, that is my classic trick to speed up such problems.

    I thought about solving part 2 quick&dirty first, but the grid is relatively big.
    I think it's more about detecting whether we are crossing a closed path kind of algorithm,
    but I'm not sure I know the algorithm yet.

    I didn't want spoilers, and I noticed that many people were struggling with part 2 based on
    the stats page.

    I decided to reduce the problem size first, hoping that would help.

    It's pretty slow. It works, but it's pretty slow. About 100ms on my M1 macbook pro.
    I added rayon to parallelise the area computations, which helps a bit, but it's still not great.

    -- After solving the problem myself, I found the mipmap tip on the subreddit.
    This is exactly what I needed to speed up the flood-fill check.

    Now, this is fast enough. About 6ms on my M1 macbook pro.
*/

use std::collections::{HashMap, hash_map::Entry};

use ndarray::{Array2, s};
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::line_ending,
    multi::separated_list1, sequence::separated_pair,
};
use rayon::iter::{ParallelBridge, ParallelIterator};

fn parse_input_data(data: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(
        line_ending,
        separated_pair(
            nom::character::complete::u64,
            tag(","),
            nom::character::complete::u64,
        )
        .map(|(a, b)| (b, a)),
    )
    .parse(data)
}

pub fn day_09_part_1(data: &str) -> i64 {
    let (_, points) = parse_input_data(data).expect("Failed to parse input data");
    //println!("Parsed {:?} points", points);

    // Convert to i64 for easier computations
    let mut sorted_points = points
        .iter()
        .map(|(row, col)| (*row as i64, *col as i64))
        .collect::<Vec<_>>();
    // Sort on one axis to speed up slightly the computations
    sorted_points.sort_unstable_by_key(|(row, _col)| *row);

    let mut max_area = 0_i64;
    for (index_a, (row_a, col_a)) in sorted_points.iter().enumerate() {
        for (row_b, col_b) in sorted_points.iter().skip(index_a + 1) {
            let area = (row_b - row_a + 1).abs() * (col_b - col_a + 1).abs();
            if area > max_area {
                /*println!(
                    "New max area {} with points ({},{}) and ({},{})",
                    area, row_a, col_a, row_b, col_b
                );*/
                max_area = area;
            }
        }
    }
    max_area
}

#[allow(dead_code)]
fn display_grid(grid: &Array2<bool>) {
    for row in grid.rows() {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn day_09_part_2(data: &str) -> i64 {
    let (_, points) = parse_input_data(data).expect("Failed to parse input data");

    /*let (min_row, max_row, min_col, max_col) = points.iter().fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |(min_r, max_r, min_c, max_c), (r, c)| {
            (
                min_r.min(*r as usize),
                max_r.max(*r as usize),
                min_c.min(*c as usize),
                max_c.max(*c as usize),
            )
        },
    );

    let nb_rows = (max_row - min_row + 1) as usize;
    let nb_cols = (max_col - min_col + 1) as usize;

    println!(
        "Grid from rows {}-{} and cols {}-{} (size {}x{})",
        min_row, max_row, min_col, max_col, nb_rows, nb_cols
    );*/

    let (mut rows, mut cols): (Vec<u64>, Vec<u64>) = points.iter().cloned().unzip();
    rows.sort_unstable();
    cols.sort_unstable();

    let mut rows_index_map: HashMap<u64, usize> = HashMap::with_capacity(rows.len());
    let mut cols_index_map: HashMap<u64, usize> = HashMap::with_capacity(cols.len());

    let mut index = 1;
    for row in rows.iter().cloned() {
        if let Entry::Vacant(e) = rows_index_map.entry(row) {
            e.insert(index);
            index += 2;
        }
    }
    let max_row_index = index - 2;

    index = 1;
    for col in cols.iter().cloned() {
        if let Entry::Vacant(e) = cols_index_map.entry(col) {
            e.insert(index);
            index += 2;
        }
    }
    let max_col_index = index - 2;

    let nb_rows = (max_row_index + 2).next_power_of_two();
    let nb_cols = (max_col_index + 2).next_power_of_two();

    let mut grid = Array2::<bool>::default((nb_rows, nb_cols));

    //let v = vec![(1, 2), (3, 4), (5, 6)];

    for (prev, next) in points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
    {
        //println!("{:?} -> {:?}", prev, next);
        let (prev_row, prev_col) = prev;
        let (next_row, next_col) = next;
        let prev_row_index = *rows_index_map.get(prev_row).unwrap();
        let prev_col_index = *cols_index_map.get(prev_col).unwrap();
        let next_row_index = *rows_index_map.get(next_row).unwrap();
        let next_col_index = *cols_index_map.get(next_col).unwrap();
        /*println!(
            "  Mapped to grid coords ({},{}) -> ({},{})",
            prev_row_index, prev_col_index, next_row_index, next_col_index
        );*/

        let mut view = grid.slice_mut(s![
            prev_row_index.min(next_row_index)..=prev_row_index.max(next_row_index),
            prev_col_index.min(next_col_index)..=prev_col_index.max(next_col_index)
        ]);
        view.fill(true);
    }

    //display_grid(&grid);

    let mut filled_grid = Array2::<bool>::from_elem((nb_rows, nb_cols), true);
    //display_grid(&filled_grid);

    // flood fill from (0,0)
    let mut stack = vec![(0_usize, 0_usize)];
    while let Some((row, col)) = stack.pop() {
        if !filled_grid[(row, col)] || grid[(row, col)] {
            continue;
        }
        filled_grid[(row, col)] = false;
        if row > 0 {
            stack.push((row - 1, col));
        }
        if row + 1 < nb_rows {
            stack.push((row + 1, col));
        }
        if col > 0 {
            stack.push((row, col - 1));
        }
        if col + 1 < nb_cols {
            stack.push((row, col + 1));
        }
    }
    //display_grid(&filled_grid);
    // power of 2 nb_rows and nb_cols
    //let nb_rows_pow2 = nb_rows.next_power_of_two();
    //let nb_cols_pow2 = nb_cols.next_power_of_two();

    // println!("size of filled grid: {:?}", filled_grid.dim());
    // println!(
    //     "size of filled grid (pow2): {}x{}",
    //     nb_rows_pow2, nb_cols_pow2
    // );

    // build a mipmap of blocks representing block_size*block_size areas that are fully filled
    let block_size = 8;
    let nb_block_rows = nb_rows.div_ceil(block_size);
    let nb_block_cols = nb_cols.div_ceil(block_size);
    let mut mipmap_grid = Array2::<bool>::from_elem((nb_block_rows, nb_block_cols), false);
    for row in 0..nb_block_rows {
        for col in 0..nb_block_cols {
            let row_start = row * block_size;
            let col_start = col * block_size;
            //if row_start >= nb_rows || col_start >= nb_cols {
            //    continue;
            //}
            let row_end = (row_start + block_size).min(nb_rows);
            let col_end = (col_start + block_size).min(nb_cols);
            let view = filled_grid.slice(s![row_start..row_end, col_start..col_end]);
            if view.iter().all(|cell| *cell) {
                mipmap_grid[(row, col)] = true;
            }
        }
    }
    //println!("Mipmap grid created.");
    //display_grid(&mipmap_grid);

    let mut sorted_points = points
        .iter()
        .map(|(row, col)| (*row as i64, *col as i64))
        .collect::<Vec<_>>();
    // Sort on one axis to speed up slightly the computations
    sorted_points.sort_unstable_by_key(|(row, _col)| *row);

    //let mut max_area = 0_i64;
    //for (index_a, (row_a, col_a)) in sorted_points.iter().enumerate() {
    sorted_points
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(index_a, (row_a, col_a))| {
            let mut max_area = 0_i64;
            'outer: for (row_b, col_b) in sorted_points.iter().skip(index_a + 1) {
                //for (row_b, col_b) in sorted_points.iter() {
                let height = (row_b - row_a).abs() + 1;
                let width = (col_b - col_a).abs() + 1;
                let area = height * width;
                //if area > max_area {
                if area > max_area {
                    /*println!(
                        "New max area {} with points ({},{}) and ({},{})",
                        area, row_a, col_a, row_b, col_b
                    );*/
                    // we have a potential area, let's check if it only contains true in the filled_grid
                    let row_a_index = *rows_index_map.get(&(*row_a as u64)).unwrap();
                    let col_a_index = *cols_index_map.get(&(*col_a as u64)).unwrap();
                    let row_b_index = *rows_index_map.get(&(*row_b as u64)).unwrap();
                    let col_b_index = *cols_index_map.get(&(*col_b as u64)).unwrap();

                    let row_start = row_a_index.min(row_b_index);
                    let row_end = row_a_index.max(row_b_index);
                    let col_start = col_a_index.min(col_b_index);
                    let col_end = col_a_index.max(col_b_index);

                    //let view = filled_grid.slice(s![row_start..=row_end, col_start..=col_end]);
                    //if view.iter().all(|cell| *cell) {

                    // check the mipmap first and then the filled_grid if needed
                    let mipmap_row_start = row_start / block_size;
                    let mipmap_row_end = row_end / block_size;
                    let mipmap_col_start = col_start / block_size;
                    let mipmap_col_end = col_end / block_size;

                    for mipmap_row in mipmap_row_start..=mipmap_row_end {
                        for mipmap_col in mipmap_col_start..=mipmap_col_end {
                            if !mipmap_grid[(mipmap_row, mipmap_col)] {
                                // not fully filled block
                                // let's check the filled grid for the right area (not the whole block)
                                let actual_row_start = row_start.max(mipmap_row * block_size);
                                let actual_row_end = ((mipmap_row + 1) * block_size - 1)
                                    .min(row_end)
                                    .min(nb_rows - 1);
                                let actual_col_start = col_start.max(mipmap_col * block_size);
                                let actual_col_end = ((mipmap_col + 1) * block_size - 1)
                                    .min(col_end)
                                    .min(nb_cols - 1);
                                // sometimes we go out of bounds
                                /*if actual_row_start >= nb_rows || actual_col_start >= nb_cols {
                                    continue;
                                }*/

                                let view = filled_grid.slice(s![
                                    actual_row_start..=actual_row_end,
                                    actual_col_start..=actual_col_end
                                ]);
                                if !view.iter().all(|cell| *cell) {
                                    //println!("  Rejected area {} due to grid coords ({},{}) and ({},{})", area, row_a_index, col_a_index, row_b_index, col_b_index);
                                    continue 'outer;
                                }
                            }
                        }
                    }

                    //if true {
                    /*println!("view for area {}, (dim: {:?})", area, view.dim());
                    for r in view.rows() {
                        for cell in r {
                            if *cell {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                        println!();
                    }
                    println!(
                        "  Confirmed area {} with grid coords ({},{}) and ({},{})",
                        area, row_a_index, col_a_index, row_b_index, col_b_index
                    );
                    println!(
                        "total coords: rows {}-{}, cols {}-{}",
                        row_a, row_b, col_a, col_b
                    );*/
                    //max_area = area;
                    max_area = area;
                    //}
                }
            }
            max_area
        })
        .max()
        .expect("At least one area")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_day_09_part_1() {
        assert_eq!(day_09_part_1(EXAMPLE), 50);
    }

    #[test]
    fn test_day_09_part_2() {
        assert_eq!(day_09_part_2(EXAMPLE), 24);
    }

    #[test]
    fn test_day_09_part_2_harder() {
        let harder_example = "1,0
3,0
3,6
16,6
16,0
18,0
18,9
13,9
13,7
6,7
6,9
1,9";
        assert_eq!(day_09_part_2(harder_example), 30);
    }
}
