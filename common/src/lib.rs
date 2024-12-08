use std::env;

pub fn read_path() -> String {
    let args: Vec<String> = env::args().collect();

    args[1].clone()
}

pub fn build_grid(input: String) -> Vec<Vec<char>> {
    let mut grid = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}
