use common::read_path;

use std::collections::BTreeMap;

fn convert_to_intermediary_repr(input: String) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();

    let mut repr = vec![];

    for (idx, chunk) in chars.chunks(2).enumerate() {
        let (block_count, free_space_count) = match chunk.len() {
            2 => (
                chunk[0]
                    .to_string()
                    .parse::<usize>()
                    .expect("Failed to parse block count to usize."),
                chunk[1]
                    .to_string()
                    .parse::<usize>()
                    .expect("Failed to parse free space count to usize."),
            ),
            1 => (
                chunk[0]
                    .to_string()
                    .parse::<usize>()
                    .expect("Failed to parse block count to usize."),
                0,
            ),
            _ => panic!("Incompatible input."),
        };

        for _ in 0..block_count {
            repr.push(format!("{idx}"))
        }

        for _ in 0..free_space_count {
            repr.push(String::from("."))
        }
    }

    repr
}

fn rearrange(mut repr: Vec<String>) -> Vec<String> {
    let (mut left, mut right) = (0, repr.len() - 1);

    while left < right {
        if repr[left] != "." {
            left += 1;
        }
        if repr[right] == "." {
            right -= 1;
        }

        if repr[right] != "." && repr[left] == "." {
            repr.swap(left, right);
        }
    }
    repr
}

fn rearrange_whole_files(mut repr: Vec<String>) -> Vec<String> {
    let mut map: BTreeMap<u64, usize> = BTreeMap::new();

    for s in repr.iter() {
        if s != "." {
            let key = s.parse::<u64>().expect("Failed to parse value to u64");
            match map.get_mut(&key) {
                Some(count) => *count += 1,
                None => {
                    map.insert(key, 1);
                }
            }
        }
    }

    let (mut left, mut right) = (0, repr.len());
    let mut file_space = 0;

    while let Some((key, block_size)) = map.pop_last() {
        while left < right && repr[left] != format!("{key}") {
            if repr[left] == "." && repr[left + 1] != "." && block_size <= (file_space + 1) {
                let mut i = left;
                while i < right {
                    if repr[i] == format!("{key}") {
                        repr[i] = ".".to_string();
                    }
                    i += 1;
                }

                for j in (left - file_space)..(left - file_space + block_size) {
                    repr[j] = format!("{key}");
                }
                break;
            } else if repr[left] == "." {
                file_space += 1;
            } else {
                file_space = 0;
            }
            left += 1;
        }
        left = 0;
        file_space = 0;
        right -= block_size;
    }
    repr
}

fn find_checksum(repr: Vec<String>) -> u64 {
    let mut checksum: u64 = 0;
    for (idx, c) in repr.iter().enumerate() {
        if *c == "." {
            continue;
        }

        checksum += c.parse::<u64>().expect("Failed to parse to u64.") * idx as u64;
    }
    checksum
}

mod solutions {
    use std::fs;

    use crate::{convert_to_intermediary_repr, find_checksum, rearrange, rearrange_whole_files};

    pub fn part_1_solution(path: &str) -> u64 {
        let mut input = fs::read_to_string(path).expect("Failed to read input.");

        input.truncate(input.len() - 1);

        let repr = convert_to_intermediary_repr(input);

        let rearranged_repr = rearrange(repr);

        find_checksum(rearranged_repr)
    }

    pub fn part_2_solution(path: &str) -> u64 {
        let mut input = fs::read_to_string(path).expect("Failed to read input.");

        input.truncate(input.len() - 1);

        let repr = convert_to_intermediary_repr(input);

        let rearranged_repr = rearrange_whole_files(repr);

        find_checksum(rearranged_repr)
    }
}

fn main() {
    let path = read_path();

    let solution = solutions::part_1_solution(&path);

    println!("{solution}");

    let solution = solutions::part_2_solution(&path);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_intermediary_repr() {
        let input = String::from("12305");

        let actual = convert_to_intermediary_repr(input);

        let expected = vec!["0", ".", ".", "1", "1", "1", "2", "2", "2", "2", "2"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rearrange() {
        let repr = vec![
            '0', '0', '.', '.', '.', '1', '1', '1', '.', '.', '.', '2', '.', '.', '.', '3', '3',
            '3', '.', '4', '4', '.', '5', '5', '5', '5', '.', '6', '6', '6', '6', '.', '7', '7',
            '7', '.', '8', '8', '8', '8', '9', '9',
        ];

        let actual = rearrange(repr.iter().map(|x| x.to_string()).collect());

        let expected = vec![
            '0', '0', '9', '9', '8', '1', '1', '1', '8', '8', '8', '2', '7', '7', '7', '3', '3',
            '3', '6', '4', '4', '6', '5', '5', '5', '5', '6', '6', '.', '.', '.', '.', '.', '.',
            '.', '.', '.', '.', '.', '.', '.', '.',
        ];

        assert_eq!(
            actual,
            expected
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );
    }
}
