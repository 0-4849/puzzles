use std::cmp::min_by;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

const GRID_WIDTH: usize = 3;
const GRID_HEIGHT: usize = 3;
const MAX_WORD_LENGTH: usize = 25;

fn main() -> std::io::Result<()> {
    let mut word_file = File::open("Woorden.txt")?;
    let mut word_buf = String::new();

    word_file.read_to_string(&mut word_buf)?;
    let word_list: Vec<&[u8]> = word_buf.split("\n").map(|s| s.as_bytes()).collect();

    let mut dictionary = vec![Vec::new(); MAX_WORD_LENGTH];

    for word in word_list.iter() {
        let length = word.len();
        if length < MAX_WORD_LENGTH {
            dictionary[length].push(*word);
        }
    }

    for lists in dictionary.iter_mut() {
        lists.sort();
    }

    let grid = [[0; GRID_WIDTH]; GRID_HEIGHT];

    let start = Instant::now();
    let solution = solve(grid, &dictionary);
    let elapsed = start.elapsed();

    let solution_string = solution.map(|s| format_solution(&s));

    println!("took {:?}", elapsed);

    let partial = [[98, 101, 100], [0, 0, 0], [0, 0, 0]];

    println!(
        "{:?}",
        determine_most_constrained_variable(&partial, &dictionary)
    );

    if let Some(s) = solution_string {
        println!("{}", s);
    } else {
        println!("no solution found")
    }

    Ok(())
}

fn solve(
    grid: [[u8; GRID_WIDTH]; GRID_HEIGHT],
    dictionary: &Vec<Vec<&[u8]>>,
) -> Option<[[u8; GRID_WIDTH]; GRID_HEIGHT]> {
    let mut new_grid = grid;

    for (index, row) in grid.iter().enumerate() {
        if row[0] == 0 {
            'word_loop: for word in dictionary[GRID_WIDTH].iter() {
                new_grid[index] = (*word).try_into().expect("wrong word length");
                if let Some(sol) = solve(new_grid, dictionary) {
                    for x in 0..GRID_WIDTH {
                        let mut word = [0; GRID_HEIGHT];

                        for y in 0..GRID_HEIGHT {
                            word[y] = sol[y][x];
                        }
                        if !dictionary[GRID_HEIGHT].binary_search(&&word[..]).is_ok() {
                            continue 'word_loop;
                        }
                    }
                    return Some(sol);
                }
            }
        }
    }

    Some(grid)
}

fn determine_most_constrained_variable(
    grid: &[[u8; GRID_WIDTH]; GRID_HEIGHT],
    dictionary: &Vec<Vec<&[u8]>>,
) -> Option<(Direction, usize)> {
    // store the number of possible words which
    // can be filled in for each of the columns
    let mut number_of_options_rows = vec![0; grid.len()];
    let mut number_of_options_cols = vec![0; grid[0].len()];

    for (number_of_options, row) in number_of_options_rows.iter_mut().zip(grid) {
        // we will binary search the (sorted) dictionary for
        // the lower bound, above which are words which can fill in row
        // the upper bound, below which are words which can fill in row
        // we expect the search to fail, but for it to return
        // the point where it has failed
        // we can then take the difference of the lower and upper bound
        // and this gives us the number of options which can fill in row
        let lower_bound = row;
        let mut upper_bound = row.clone();

        for c in &mut upper_bound {
            if *c == 0 {
                *c = u8::MAX;
            }
        }

        let lower_bound_pos = match dictionary[GRID_WIDTH].binary_search(&&lower_bound[..]) {
            Ok(x) => x,
            Err(x) => x,
        };

        let upper_bound_pos = match dictionary[GRID_WIDTH].binary_search(&&upper_bound[..]) {
            Ok(x) => x,
            Err(x) => x,
        };

        *number_of_options = upper_bound_pos - lower_bound_pos;
    }

    for (col_index, number_of_options) in number_of_options_cols.iter_mut().enumerate() {
        let mut lower_bound = [0; GRID_HEIGHT];
        for row_index in 0..grid.len() {
            lower_bound[row_index] = grid[row_index][col_index];
        }
        let mut upper_bound = lower_bound.clone();

        for c in &mut upper_bound {
            if *c == 0 {
                *c = u8::MAX;
            }
        }

        let lower_bound_pos = match dictionary[GRID_WIDTH].binary_search(&&lower_bound[..]) {
            Ok(x) => x,
            Err(x) => x,
        };

        let upper_bound_pos = match dictionary[GRID_WIDTH].binary_search(&&upper_bound[..]) {
            Ok(x) => x,
            Err(x) => x,
        };

        *number_of_options = upper_bound_pos - lower_bound_pos;
    }

    let (min_row_index, min_row) = number_of_options_rows
        .iter()
        .enumerate()
        .min_by(|(_, m), (_, n)| m.cmp(n))
        .unwrap();

    let (min_col_index, min_col) = number_of_options_cols
        .iter()
        .enumerate()
        .min_by(|(_, m), (_, n)| m.cmp(n))
        .unwrap();

    println!("{:?}, {:?}", number_of_options_cols, number_of_options_rows);

    if *min_row == 0 && *min_col == 0 {
        None
    } else if min_row > min_col {
        Some((Direction::Horizontal, min_row_index))
    } else {
        Some((Direction::Vertical, min_col_index))
    }
}

fn format_solution(&solution: &[[u8; GRID_WIDTH]; GRID_HEIGHT]) -> String {
    solution
        .iter()
        .map(|s| String::from_utf8(s.to_vec()).expect("incorrect utf8 encoding"))
        .collect::<Vec<String>>()
        .join("\n")
}

// fn get_dictionary(file_name: String) -> std::io::Result<Vec<Vec<&[u8]>>> {
//     Ok(vec![vec![&[0]]])
// }
