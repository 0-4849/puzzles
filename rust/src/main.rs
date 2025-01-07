#![allow(unused_imports)]
use itertools::interleave;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Grid<'a> {
    grid: [[u8; GRID_WIDTH]; GRID_HEIGHT],
    row_options: Vec<Vec<&'a [u8]>>,
    col_options: Vec<Vec<&'a [u8]>>,
}

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 5;
const MIN_LOOK: usize = 20;
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

    // randomly shuffle lists
    for lists in dictionary.iter_mut() {
        lists.shuffle(&mut thread_rng());
    }
    println!("{:?}", dictionary);

    // shuffle dictionary in a deterministic way to increase performance
    // for list in dictionary.iter_mut() {
    //     if list.len() <= MIN_LOOK {
    //         continue;
    //     };

    //     let mut new_list = list.clone();
    //     for (j, word) in list.into_iter().enumerate().skip(1) {
    //         let num_of_groups = new_list.len() / MIN_LOOK;
    //         let offset = j / num_of_groups;
    //         let group_pos = MIN_LOOK * (j % num_of_groups);

    //         new_list[group_pos + offset] = word;
    //     }
    //     *list = new_list;
    // }
    // println!("shuffled");

    let mut grid = Grid {
        grid: [[0; GRID_WIDTH]; GRID_HEIGHT],
        row_options: vec![dictionary[GRID_WIDTH].clone(); GRID_HEIGHT],
        col_options: vec![dictionary[GRID_HEIGHT].clone(); GRID_WIDTH],
    };

    update_bounds(&mut grid);

    let start = Instant::now();
    let solution = solve(&grid, &dictionary);
    let elapsed = start.elapsed();

    println!("took {:?}", elapsed);

    if let Some(s) = solution {
        println!("{}", format_solution(&s));
    } else {
        println!("no solution found")
    }

    Ok(())
}

fn solve<'a>(grid: &Grid<'a>, dictionary: &'a Vec<Vec<&'a [u8]>>) -> Option<Grid<'a>> {
    let mut new_grid = grid.clone();

    update_bounds(&mut new_grid);

    if new_grid.grid.iter().all(|w| w.iter().all(|c| *c != 0))
        && !new_grid.row_options.iter().any(|x| x.is_empty())
        && !new_grid.col_options.iter().any(|x| x.is_empty())
    {
        return Some(new_grid);
    }

    //println!("{:?}", new_grid);

    // first, determine the variable which has the least options
    // if the number of options left is 0, this means the word can never be filled in,
    // so we return None, indicating this grid isn't solvable
    // if the number of options left is 1, we should check if the row or col is filled in fully:
    // if it's already full, we don't select is as the most constrained, otherwise we do

    let mut least_row_index = 0;
    for i in 0..GRID_HEIGHT {
        match grid.row_options[i].len() {
            0 => {
                return None;
            }
            1 => {
                if grid.grid[i].iter().any(|x| *x == 0) {
                    least_row_index = i;
                }
            }
            x if x > grid.row_options[least_row_index].len() => {
                least_row_index = i;
            }
            _ => (),
        }
    }

    let least_row = &grid.row_options[least_row_index];

    let mut least_col_index = 0;
    for i in 0..GRID_WIDTH {
        match grid.col_options[i].len() {
            0 => {
                return None;
            }
            1 => {
                if grid.grid.iter().map(|x| x[i]).any(|x| x == 0) {
                    least_col_index = i;
                }
            }
            x if x > grid.col_options[least_col_index].len() => {
                least_col_index = i;
            }
            _ => (),
        }
    }

    let least_col = &grid.col_options[least_col_index];

    // here, we check if the grid has no empty spaces left;
    // if this is true, we have succeeded in solving the puzzle,
    // the reason we check this here (and not at the start of the function)
    // is because if we did it at the start, the puzzle would always
    // return succesully, even if the last word didn't fit

    // then, try all of the words which are still possible
    // (depending on whether it's a row or column we have different procedures)
    if least_col.len() < least_row.len() {
        for word in least_col {
            for y in 0..GRID_HEIGHT {
                new_grid.grid[y][least_col_index] = word[y];
            }
            let solution = solve(&new_grid, dictionary);
            if solution.is_none() {
                continue;
            } else {
                return solution;
            }
        }
    } else {
        // let mut indices = (0..least_row.len()).collect::<Vec<_>>();
        // // let candidates = least_row[0..MIN_LOOK - 1];
        // let mut candidates_options = vec![0.0; std::cmp::min(MIN_LOOK, least_row.len())];
        // // let mut best_candidate_index = 0;
        // // let mut most_options: f64 = 0.0;

        // //TODO: remove unnecessary clones
        // let mut temp_grid = new_grid.clone();

        // for (candidate_index, candidate_options) in candidates_options.iter_mut().enumerate() {
        //     temp_grid.row_options = grid.row_options.clone();
        //     temp_grid.col_options = grid.col_options.clone();

        //     temp_grid.grid[least_row_index] = (*least_row[candidate_index])
        //         .try_into()
        //         .expect("wrong length");
        //     update_bounds(&mut temp_grid);

        //     let options_product: f64 = temp_grid
        //         .row_options
        //         .iter()
        //         .map(|x| x.len())
        //         .product::<usize>() as f64
        //         * temp_grid
        //             .col_options
        //             .iter()
        //             .map(|x| x.len())
        //             .product::<usize>() as f64;

        //     *candidate_options = options_product;
        // }

        // indices[0..candidates_options.len()].sort_by(|&j, &i| {
        //     if let Some(x) = candidates_options.get(i) {
        //         *x
        //     } else {
        //         i as f64
        //     }
        //     .partial_cmp(&if let Some(x) = candidates_options.get(j) {
        //         *x
        //     } else {
        //         j as f64
        //     })
        //     .unwrap()
        // });

        for word in least_row {
            new_grid.grid[least_row_index] = (*word).try_into().expect("wrong length");
            let solution = solve(&new_grid, dictionary);
            if solution.is_none() {
                continue;
            } else {
                return solution;
            }
        }
    }

    None
}

// update the list of possible words for all the rows and columns
// based on what letters are already filled in in the grid
fn update_bounds<'a>(grid: &mut Grid<'a>) {
    for row_index in 0..GRID_HEIGHT {
        grid.row_options[row_index].retain(|&word| {
            for i in 0..GRID_WIDTH {
                if grid.grid[row_index][i] != 0 && grid.grid[row_index][i] != word[i] {
                    return false;
                }
            }
            true
        });
    }

    for col_index in 0..GRID_WIDTH {
        grid.col_options[col_index].retain(|&word| {
            for i in 0..GRID_HEIGHT {
                if grid.grid[i][col_index] != 0 && grid.grid[i][col_index] != word[i] {
                    return false;
                }
            }
            true
        });
    }
}

fn format_solution(solution: &Grid) -> String {
    solution
        .grid
        .iter()
        .map(|s| String::from_utf8(s.to_vec()).expect("incorrect utf8 encoding"))
        .collect::<Vec<String>>()
        .join("\n")
}

// fn get_dictionary(file_name: String) -> std::io::Result<Vec<Vec<&[u8]>>> {
//     Ok(vec![vec![&[0]]])
// }
