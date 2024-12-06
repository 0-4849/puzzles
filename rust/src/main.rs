use std::cmp::min_by;
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
        grid: [[97, 98],[0,0]],
        row_options: vec![dictionary[GRID_WIDTH].clone(); GRID_HEIGHT],
        col_options: vec![dictionary[GRID_WIDTH].clone(); GRID_WIDTH],
    };

    println!("{:?}", grid);
    update_bounds(&mut grid, &dictionary);
    println!("{:#?}", grid);


    let start = Instant::now();
    let solution = solve(grid, &dictionary);
    let elapsed = start.elapsed();

    let solution_string = solution.map(|s| format_solution(&s));

    println!("took {:?}", elapsed);

    let partial = [[98, 101, 100], [0, 0, 0], [0, 0, 0]];


    if let Some(s) = solution_string {
        println!("{}", s);
    } else {
        println!("no solution found")
    }

    Ok(())
}

fn solve<'a>(
    mut grid: Grid<'a>,
    dictionary: &'a Vec<Vec<&'a [u8]>>,
) -> Option<Grid<'a>> {
    //let mut new_grid = grid;

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

    Some(grid)
}

// return the direction (row = hor, col = vert)
// along with the row/col number and (TODO) constraints:
// the bounds wherein the possible words lie
fn update_bounds<'a>(
    grid: &mut Grid<'a>,
    dictionary: &Vec<Vec<&'a [u8]>>,
) {
    // store the number of possible words which
    // can be filled in for each of the columns
    //let mut options_rows = vec![Vec::new(); GRID_HEIGHT];
    //let mut options_cols = vec![Vec::new(); GRID_HEIGHT];

    for (row_index, row) in grid.grid.iter().enumerate() {
        grid.row_options[row_index].retain(|&word| {

        });
        // 'word_loop: for word_index in 0..grid.row_options[row_index].len()-1 {
        //     for i in 0..GRID_WIDTH-1 {
        //         if row[i] != 0 && row[i] != grid.row_options[row_index][row_index][i] {
        //             grid.row_options[row_index].remove(word_index);
        //         }
        //     }
        // }
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
