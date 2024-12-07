use common::read_path;
use std::env;

fn get_rows(input: String) -> Vec<Vec<i32>> {
    let mut rows = vec![];

    for row in input.lines() {
        let chars = row.split(' ');
        let mut input_row = vec![];
        for char in chars {
            input_row.push(char.parse::<i32>().expect("Failed to parse char to i32."))
        }
        rows.push(input_row);
    }
    rows
}

fn is_row_safe(row: &Vec<i32>) -> bool {
    let res = match row.windows(2).all(|value| value[0] <= value[1]) {
        false => match row.windows(2).all(|value| value[0] >= value[1]) {
            true => "descending",
            false => return false,
        },
        true => "ascending",
    };

    if res == "ascending" {
        match row
            .windows(2)
            .all(|value| value[0] + 1 <= value[1] && value[1] <= value[0] + 3)
        {
            true => return true,
            false => return false,
        }
    } else {
        match row
            .windows(2)
            .all(|value| value[0] - 1 >= value[1] && value[1] >= value[0] - 3)
        {
            true => return true,
            false => return false,
        }
    }
}

fn single_bad_level(row: &Vec<i32>) -> bool {
    for i in 0..row.len() {
        let leftover: Vec<i32> = row
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &x)| x)
            .collect();
        if is_row_safe(&leftover) {
            return true;
        }
    }

    false
}

mod solutions {
    use std::fs;

    use crate::{get_rows, is_row_safe, single_bad_level};

    pub fn part_1_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Cannot read txt file.");

        let rows = get_rows(input);

        let mut res: i32 = 0;

        for row in rows {
            match is_row_safe(&row) {
                true => res += 1,
                false => {}
            }
        }

        res
    }

    pub fn part_2_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Cannot read txt file.");

        let rows = get_rows(input);

        let mut res: i32 = 0;

        for row in rows {
            match is_row_safe(&row) {
                true => res += 1,
                false => match single_bad_level(&row) {
                    true => res += 1,
                    false => {}
                },
            }
        }

        res
    }
}

fn main() {
    let path = read_path();

    let result = solutions::part_1_solution(&path);
    println!("{}", result);

    let result = solutions::part_2_solution(&path);
    println!("{}", result);
}
