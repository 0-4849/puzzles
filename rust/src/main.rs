use std::fs::File;
use std::io::prelude::*;

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

    let grid = [[0; GRID_WIDTH]; GRID_HEIGHT];

    println!(
        "{:?}",
        solve(grid, &dictionary)
            .map(|x| x.into_iter().map(|s| s.to_owned()))
            .unwrap()
    );

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
                            word[y] = new_grid[y][x];
                        }
                        if !dictionary[GRID_HEIGHT].binary_search(&&word[..]).is_ok() {
                            println!("{:?}", word);
                            continue 'word_loop;
                        }
                    }
                    return Some(sol);
                } else {
                    continue;
                };
            }
        }
    }

    Some(grid)
}

// fn get_dictionary(file_name: String) -> std::io::Result<Vec<Vec<&[u8]>>> {
//     Ok(vec![vec![&[0]]])
// }
