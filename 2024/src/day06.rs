use std::fs;

pub fn predict_positions(filepath: &str, debug: bool) -> i32 {
    let file_con = fs::read_to_string(filepath)
        .expect("Error reading file");

    let mut grid: Vec<Vec<char>> = file_con.lines()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect();

    let mut cursor_i: usize = 0;
    let mut cursor_j: usize = 0;
    let mut jump_i: isize = 0;
    let mut jump_j: isize = 0;
    let (max_i, max_j) = (grid.len() - 1, grid[0].len() - 1);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if matches!(grid[i][j], '>' | '<' | '^' | 'v') {
                (cursor_i, cursor_j) = (i, j);
                match grid[i][j] {
                    '>' => (jump_i, jump_j) = (0, 1),
                    '<' => (jump_i, jump_j) = (0, -1),
                    '^' => (jump_i, jump_j) = (-1, 0),
                    'v' => (jump_i, jump_j) = (1, 0),
                    _ => panic!("Unknown char: {:?}", grid[i][j])
                };
            }
        }
    }

    let mut in_grid = true;
    while in_grid {
        grid[cursor_i][cursor_j] = 'X';
        if (cursor_i == 0 && jump_i == -1)
            || (cursor_i == max_i && jump_i == 1)
            || (cursor_j == 0 && jump_j == -1)
            || (cursor_j == max_j && jump_j == 1) {
                in_grid = false;
            }
        else {
            if grid[cursor_i.saturating_add_signed(jump_i)][cursor_j.saturating_add_signed(jump_j)] == '#' {
                // Rotate 90 degrees right
                (jump_i, jump_j) = (jump_j, jump_i * -1);
            }
            cursor_i = cursor_i.saturating_add_signed(jump_i);
            cursor_j = cursor_j.saturating_add_signed(jump_j);
        }
    }

    let counter = grid.iter()
        .flatten()
        .filter(|x| **x == 'X')
        .count() as i32;

    if debug {
        println!("Day 6: Final result {counter}")
    }

    counter
}
