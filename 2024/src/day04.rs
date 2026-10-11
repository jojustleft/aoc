use std::fs;

fn check_spot(search_grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut matches = 0;
    let max_i = search_grid.len() - 1;
    let max_j = search_grid[i].len() - 1;

    // All possible directions the word can occur
    let directions: Vec<(isize, isize)> = [-1, 0, 1].iter()
        .flat_map(|&a| [-1, 0, 1].iter().map(move |&b| (a, b)))
        .collect();

    for (jump_i, jump_j) in directions {
        // Automatic skips
        // No word to form
        if jump_i == 0 && jump_j == 0 { continue; }
        // Spot too close to out of bounds
        else if (i < 3 && jump_i == -1) || (i > max_i - 3 && jump_i == 1) { continue; }
        else if (j < 3 && jump_j == -1) || (j > max_j - 3 && jump_j == 1) { continue; }

        // Create word by jumping 3 times and compare with expected input
        let mut cursor_i = i;
        let mut cursor_j = j;
        let mut word_arr: Vec<char> = vec![search_grid[cursor_i][cursor_j]]; 
        for _ in 0..3 {
            // Boundary checks above ensure this won't be a problem
            cursor_i = cursor_i.saturating_add_signed(jump_i);
            cursor_j = cursor_j.saturating_add_signed(jump_j);
            word_arr.push(search_grid[cursor_i][cursor_j]);
        }
        let word: String = word_arr.iter().collect();
        if word == "XMAS" { matches += 1; }
    }

    matches
}

fn spot_is_x_pattern(search_grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let max_i = search_grid.len() - 1;
    let max_j = search_grid[i].len() - 1;

    if i == 0 || j == 0 || i == max_i || j == max_j { false}
    else {
        let tl = search_grid[i - 1][j - 1];
        let tr = search_grid[i - 1][j + 1];
        let bl = search_grid[i + 1][j - 1];
        let br = search_grid[i + 1][j + 1];

        let accepted = &['M', 'S'];

        accepted.contains(&tl)
            && accepted.contains(&tr)
            && accepted.contains(&bl)
            && accepted.contains(&br)
            && tl != br
            && tr != bl
    }
}

/*
Alternatively, we could do the following:
 - Create a matrix where every spot equals int(spot == 'X')
 - Create a matrix where every spot equals int(spot == 'M') and offset it by one (according to the direction of the word)
    - Missing entries are set to 0
 - Multiply each element of these two (Hadamard Product)
 - Repeat this process for the other two letters: 'A' and 'S' (mind that the offset must increment as well)
 - Count the number of 1s in the final result

This process must be repeated for each possible word direction, since the offset changes.
Since this approach is matrix based, it *may* be more efficient for very large inputs.
*/
pub fn word_search(filepath: &str, debug: bool) -> i32 {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");
    
    let search_grid: Vec<Vec<char>> = file_con
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;
    
    for i in 0..search_grid.len() {
        for j in 0..search_grid[i].len() {
            if search_grid[i][j] == 'X' {
                counter += check_spot(&search_grid, i, j);
            }
        }
    }

    if debug {
        println!("Day 4: Final result {counter}")
    }

    counter
}

pub fn word_search_p2(filepath: &str, debug: bool) -> i32 {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");
    
    let search_grid: Vec<Vec<char>> = file_con
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter = 0;
    
    for i in 0..search_grid.len() {
        for j in 0..search_grid[i].len() {
            if search_grid[i][j] == 'A' {
                if spot_is_x_pattern(&search_grid, i, j) == true { counter += 1 };
            }
        }
    }

    if debug {
        println!("Day 4 (part 2): Final result {counter}")
    }

    counter
}
