use common::{build_grid, read_path};

fn count_terms(mut chars: Vec<char>) -> usize {
    let mut res = 0;

    for chunk in chars.windows(4) {
        if chunk == &['X', 'M', 'A', 'S'] {
            res += 1;
        }
    }

    chars.reverse();

    for chunk in chars.windows(4) {
        if chunk == &['X', 'M', 'A', 'S'] {
            res += 1;
        }
    }
    res
}

fn traverse_horizontal(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in 0..grid.len() {
        res += count_terms(grid[i].clone());
    }
    res
}

fn traverse_vertical(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;

    for i in 0..grid.len() {
        let mut row = vec![];
        for j in 0..grid.len() {
            row.push(grid[j][i]);
        }
        res += count_terms(row);
    }

    res
}

fn traverse_diagonal(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in 0..grid.len() - 1 {
        let mut chars = vec![];
        let mut row = i.clone();
        let mut col = 0;
        loop {
            chars.push(grid[row][col]);
            if row == 0 {
                break;
            }
            row -= 1;
            col += 1;
        }
        res += count_terms(chars)
    }
    res
}

fn traverse_diagonal_reverse(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in (0..grid.len()).rev() {
        let mut chars = vec![];
        let mut row = i.clone();
        let mut col = grid.len() - 1;
        loop {
            chars.push(grid[row][col]);
            if row == grid.len() - 1 {
                break;
            }
            row += 1;
            col -= 1
        }
        res += count_terms(chars)
    }
    res
}

fn traverse_diagonal_inverse(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in 0..grid.len() - 1 {
        let mut chars = vec![];
        let mut row = i.clone();
        let mut col = grid.len() - 1;
        loop {
            chars.push(grid[row][col]);
            if row == 0 {
                break;
            }
            row -= 1;
            col -= 1;
        }
        res += count_terms(chars)
    }
    res
}

fn traverse_diagonal_inverse_reverse(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in (0..grid.len()).rev() {
        let mut chars = vec![];
        let mut row = i.clone();
        let mut col = 0;
        loop {
            chars.push(grid[row][col]);
            if row == grid.len() - 1 {
                break;
            }
            row += 1;
            col += 1;
        }
        res += count_terms(chars)
    }
    res
}

fn count_valid(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    res += traverse_vertical(grid.clone());
    res += traverse_horizontal(grid.clone());
    res += traverse_diagonal(grid.clone());
    res += traverse_diagonal_inverse(grid.clone());
    res += traverse_diagonal_reverse(grid.clone());
    res += traverse_diagonal_inverse_reverse(grid.clone());

    res
}

fn is_valid_mas(grid: &Vec<Vec<char>>, location: (usize, usize)) -> bool {
    let corners = vec![
        grid[location.0 - 1][location.1 - 1], // upper left
        grid[location.0 - 1][location.1 + 1], // upper right
        grid[location.0 + 1][location.1 - 1], // lower left
        grid[location.0 + 1][location.1 + 1], // lower right
    ];

    match corners[0] {
        'M' => match corners[3] {
            'S' => {}
            _ => return false,
        },
        'S' => match corners[3] {
            'M' => {}
            _ => return false,
        },
        _ => return false,
    }

    match corners[1] {
        'M' => match corners[2] {
            'S' => {}
            _ => return false,
        },
        'S' => match corners[2] {
            'M' => {}
            _ => return false,
        },
        _ => return false,
    }

    true
}

fn find_mas(grid: Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] == 'A' {
                if is_valid_mas(&grid, (i, j)) == true {
                    res += 1;
                }
            }
        }
    }
    res
}

mod solutions {
    use std::fs;

    use crate::{build_grid, count_valid, find_mas};

    pub fn part_1_solution(path: &str) -> usize {
        let input = fs::read_to_string(path).expect("Failed to read input file.");

        let grid = build_grid(input);

        count_valid(grid)
    }

    pub fn part_2_solution(path: &str) -> usize {
        let input = fs::read_to_string(path).expect("Failed to read input file.");

        let grid = build_grid(input);

        find_mas(grid)
    }
}

fn main() {
    let path = read_path();

    let solution = solutions::part_1_solution(&path);

    println!("{solution}");

    let solution = solutions::part_2_solution(&path);

    println!("{solution}");
}
