use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Grid<'a> {
    grid: [[u8; GRID_WIDTH]; GRID_HEIGHT],
    row_options: Vec<Vec<&'a [u8]>>,
    col_options: Vec<Vec<&'a [u8]>>,
}

const GRID_WIDTH: usize = 4;
const GRID_HEIGHT: usize = 4;
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

    //println!("{:?}", new_grid);

    // first, determine the variable which has the least options
    // if the number of options left is 0, this means the word can never be filled in,
    // so we return None, indicating this grid isn't solvable
    // if the number of options left is 1, we should check if the row or col is filled in fully:
    // if it's already full, we don't select is as the most constrained, otherwise we do

    let mut least_row_index = 0;
    for i in 0..GRID_HEIGHT {
        match grid.row_options[i].len() {
            0 => { return None; },
            1 => {
                if grid.grid[i].iter().any(|x| *x == 0) {
                    least_row_index = i;
                }
            },
            x if x > grid.row_options[least_row_index].len() => {
                least_row_index = i;
            },
            _ => (),
        }
    }

    let least_row = &grid.row_options[least_row_index];
    

    let mut least_col_index = 0;
    for i in 0..GRID_WIDTH {
        match grid.col_options[i].len() {
            0 => { return None; },
            1 => {
                if grid.grid.iter().map(|x| x[i]).any(|x| x == 0) {
                    least_col_index = i;
                }
            },
            x if x > grid.col_options[least_col_index].len() => {
                least_col_index = i;
            },
            _ => (),
        }
    }

    let least_col = &grid.col_options[least_col_index];

    // here, we check if the grid has no empty spaces left;
    // if this is true, we have succeeded in solving the puzzle,
    // the reason we check this here (and not at the start of the function)
    // is because if we did it at the start, the puzzle would always
    // return succesully, even if the last word didn't fit

    if grid.grid.iter().all(|w| w.iter().all(|c| *c != 0)) {
        return Some(new_grid);
    }
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
        for word in least_row {
            new_grid.grid[least_row_index] = (*word).try_into().expect("wrong length");
            if let Some(sol) = solve(&new_grid, dictionary) {
                return Some(sol);
            } else {
                continue;
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
