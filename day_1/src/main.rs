use common::read_path;
use std::collections::HashMap;

fn get_arrays(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut first_array = vec![];
    let mut second_array = vec![];

    for line in input.lines() {
        let values: Vec<&str> = line.split("   ").collect();

        let (first, last) = (values[0], values[1]);

        first_array.push(first.parse::<i32>().unwrap());
        second_array.push(last.parse::<i32>().unwrap());
    }

    (first_array, second_array)
}

fn calculate_total_distance_between_arrays(
    arrays: std::iter::Zip<std::slice::Iter<i32>, std::slice::Iter<i32>>,
) -> i32 {
    let mut res: i32 = 0;
    for (left, right) in arrays {
        let diff = match right > left {
            true => right - left,
            false => left - right,
        };

        res += diff;
    }
    res
}

fn calculate_total_similarity_between_arrays(arrays: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut res: i32 = 0;

    let mut cache: HashMap<i32, i32> = HashMap::new();

    for value in arrays.0.iter() {
        if cache.get(value).is_some() {
            res += cache.get(value).unwrap() * value;
        } else {
            let count: i32 = arrays.1.iter().filter(|&n| n == value).count() as i32;
            cache.insert(*value, count);
            res += count * value;
        }
    }
    res
}

mod solutions {
    use std::fs;

    pub fn part_1_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Cannot read txt file.");
        let (mut first_array, mut second_array) = super::get_arrays(&input);

        first_array.sort();
        second_array.sort();

        let res = super::calculate_total_distance_between_arrays(
            first_array.iter().zip(second_array.iter()),
        );
        res
    }

    pub fn part_2_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Cannot read txt file.");
        let arrays = super::get_arrays(&input);

        let res = super::calculate_total_similarity_between_arrays(arrays);
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
