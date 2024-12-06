use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Grid<'a> {
    grid: [[u8; GRID_WIDTH]; GRID_HEIGHT],
    row_options: Vec<Vec<&'a [u8]>>,
    col_options: Vec<Vec<&'a [u8]>>,
}

#[derive(Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

const GRID_WIDTH: usize = 2;
const GRID_HEIGHT: usize = 2;
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
        grid: [[97, 0], [98, 0]],
        row_options: vec![dictionary[GRID_WIDTH].clone(); GRID_HEIGHT],
        col_options: vec![dictionary[GRID_WIDTH].clone(); GRID_WIDTH],
    };

    let start = Instant::now();

    update_bounds(&mut grid);
    println!("{:?}", grid);

    let elapsed = start.elapsed();

    println!("took {:?}", elapsed);

    let solution = solve(grid, &dictionary);
    if let Some(s) = solution {
        println!("{}", format_solution(&s));
    } else {
        println!("no solution found")
    }

    Ok(())
}

fn solve<'a>(grid: Grid<'a>, dictionary: &'a Vec<Vec<&'a [u8]>>) -> Option<Grid<'a>> {
    let mut new_grid = (grid).clone();

    //    for (index, row) in grid.iter().enumerate() {
    //        if row[0] == 0 {
    //            'word_loop: for word in dictionary[GRID_WIDTH].iter() {
    //                new_grid[index] = (*word).try_into().expect("wrong word length");
    //                if let Some(sol) = solve(new_grid, dictionary) {
    //                    for x in 0..GRID_WIDTH {
    //                        let mut word = [0; GRID_HEIGHT];
    //
    //                        for y in 0..GRID_HEIGHT {
    //                            word[y] = sol[y][x];
    //                        }
    //                        if !dictionary[GRID_HEIGHT].binary_search(&&word[..]).is_ok() {
    //                            continue 'word_loop;
    //                        }
    //                    }
    //                    return Some(sol);
    //                }
    //            }
    //        }
    //    }

    // if let Some((direction, index)) = update_bounds(&mut grid.clone(), &dictionary) {
    //     match direction {
    //         Direction::Horizontal => { return None; },
    //         Direction::Vertical => { return None; },
    //     }
    // } else {
    //     return Some(grid);
    // };

    Some(new_grid)
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
