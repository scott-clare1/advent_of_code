use common::read_path;
use regex::Regex;

fn find_multiplications(input: &String) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    re.find_iter(input.as_str()).map(|c| c.as_str()).collect()
}

fn find_conditional_multiplications(input: &String) -> Vec<&str> {
    let do_search = regex::escape("do()");
    let dont_search = regex::escape("don't()");
    let multiplication_search = r"mul\(\d{1,3},\d{1,3}\)";

    let pattern = format!("{}|{}|{}", multiplication_search, do_search, dont_search);

    let re = Regex::new(&pattern).unwrap();

    re.find_iter(input.as_str()).map(|c| c.as_str()).collect()
}

fn collect_multiplications(conditional_multiplications: Vec<&str>) -> Vec<&str> {
    let mut multiply: bool = true;

    let mut output = vec![];

    for token in conditional_multiplications {
        match token {
            "do()" => multiply = true,
            "don't()" => multiply = false,
            _ => {
                if multiply == true {
                    output.push(token)
                }
            }
        }
    }

    output
}

fn find_terms(multiplication: &str) -> (i32, i32) {
    let terms: Vec<&str> = multiplication.split(',').collect();

    let first_term = terms[0];
    let second_term = terms[1];

    let first_term = first_term
        .strip_prefix("mul(")
        .expect("Failed to remove 'mul(' prefix")
        .parse::<i32>()
        .expect("Failed to convert to i32.");
    let second_term = second_term
        .strip_suffix(")")
        .expect("Failed to remove ')' suffix")
        .parse::<i32>()
        .expect("Failed to convert to i32.");

    (first_term, second_term)
}

fn get_total(multiplications: Vec<&str>) -> i32 {
    let mut res = 0;

    for multiplication in multiplications.iter() {
        let (first_term, second_term) = find_terms(multiplication);
        res += first_term * second_term;
    }
    res
}

mod solutions {
    use std::fs;

    use crate::{
        collect_multiplications, find_conditional_multiplications, find_multiplications, get_total,
    };

    pub fn part_1_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Failed to read input file.");
        let multiplications = find_multiplications(&input);

        get_total(multiplications)
    }

    pub fn part_2_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Failed to read input file.");
        let tokens = find_conditional_multiplications(&input);
        let multiplications = collect_multiplications(tokens);

        get_total(multiplications)
    }
}

fn main() {
    let path = read_path();

    let solution = solutions::part_1_solution(&path);

    println!("{solution}");

    let solution = solutions::part_2_solution(&path);

    println!("{solution}");
}
