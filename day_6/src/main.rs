use common::{build_grid, read_path};

fn traverse_down(
    grid: &Vec<Vec<char>>,
    mut visited: Vec<(usize, usize)>,
    mut location: (usize, usize),
) -> (Vec<(usize, usize)>, Option<(usize, usize)>) {
    let mut terminate: Option<(usize, usize)> = Some(location);

    while grid[location.0][location.1] != '#' {
        visited.push(location);

        if location.0 == grid.len() - 1 {
            terminate = None;
            break;
        } else {
            terminate = Some(location)
        }

        location.0 += 1
    }

    (visited, terminate)
}

fn traverse_up(
    grid: &Vec<Vec<char>>,
    mut visited: Vec<(usize, usize)>,
    mut location: (usize, usize),
) -> (Vec<(usize, usize)>, Option<(usize, usize)>) {
    let mut terminate: Option<(usize, usize)> = Some(location);

    while grid[location.0][location.1] != '#' {
        visited.push(location);

        if location.0 == 0 {
            terminate = None;
            break;
        } else {
            terminate = Some(location)
        }

        location.0 -= 1
    }

    (visited, terminate)
}

fn traverse_right(
    grid: &Vec<Vec<char>>,
    mut visited: Vec<(usize, usize)>,
    mut location: (usize, usize),
) -> (Vec<(usize, usize)>, Option<(usize, usize)>) {
    let mut terminate: Option<(usize, usize)> = Some(location);

    while grid[location.0][location.1] != '#' {
        visited.push(location);

        if location.1 == grid.len() - 1 {
            terminate = None;
            break;
        } else {
            terminate = Some(location)
        }

        location.1 += 1
    }

    (visited, terminate)
}

fn traverse_left(
    grid: &Vec<Vec<char>>,
    mut visited: Vec<(usize, usize)>,
    mut location: (usize, usize),
) -> (Vec<(usize, usize)>, Option<(usize, usize)>) {
    let mut terminate: Option<(usize, usize)> = Some(location);

    while grid[location.0][location.1] != '#' {
        visited.push(location);

        if location.1 == 0 {
            terminate = None;
            break;
        } else {
            terminate = Some(location)
        }

        location.1 -= 1
    }

    (visited, terminate)
}

fn find_start_location(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == '^' {
                return (i, j);
            }
        }
    }
    panic!("Failed to find start.")
}

fn walk(grid: &Vec<Vec<char>>, mut location: Option<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut visited = vec![];

    loop {
        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_up(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_right(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_down(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_left(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;
    }
    visited
}

fn is_loop(
    grid: &Vec<Vec<char>>,
    mut location: Option<(usize, usize)>,
    unlooped_path: &Vec<(usize, usize)>,
) -> bool {
    let mut visited = vec![];

    let mut is_looped = false;

    loop {
        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_up(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_right(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_down(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        let l = match location {
            Some(location) => location,
            None => {
                break;
            }
        };

        let response = traverse_left(&grid, visited.clone(), l);

        visited = response.0;
        location = response.1;

        if visited.len() > unlooped_path.len() * 2 {
            is_looped = true;
            break;
        }
    }
    is_looped
}

mod solutions {
    use std::collections::HashSet;
    use std::fs;

    use crate::{build_grid, find_start_location, is_loop, walk};

    pub fn part_1_solution(path: &str) -> usize {
        let input = fs::read_to_string(path).expect("Failed to read input.");

        let grid = build_grid(input);

        let location = Some(find_start_location(&grid));

        let path = walk(&grid, location);

        let set: HashSet<(usize, usize)> = path.into_iter().collect();
        set.len()
    }

    pub fn part_2_solution(path: &str) -> usize {
        let input = fs::read_to_string(path).expect("Failed to read input.");

        let grid = build_grid(input);

        let location = Some(find_start_location(&grid));

        let visited = walk(&grid, location);

        let mut obstacles = 0;

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if visited.contains(&(i, j)) && grid[i][j] != '^' {
                    let mut copied_grid = grid.clone();
                    copied_grid[i][j] = '#';
                    if is_loop(&copied_grid, location, &visited) {
                        obstacles += 1;
                    }
                }
            }
        }
        obstacles
    }
}

fn main() {
    let path = read_path();

    let solution = solutions::part_1_solution(&path);

    println!("{solution}");

    let solution = solutions::part_2_solution(&path);

    println!("{solution}");
}
