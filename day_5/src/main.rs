use std::collections::HashMap;

fn build_update_map(page_orders: Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for page_order in page_orders.iter() {
        match map.get_mut(&page_order.0) {
            Some(value) => value.push(page_order.1),
            None => {
                map.insert(page_order.0, vec![page_order.1]);
            }
        }
    }

    map
}

fn get_page_orders(page_orders: &str) -> Vec<(i32, i32)> {
    let mut res = vec![];
    for line in page_orders.lines() {
        let mut order = vec![];
        for page_order in line.split('|').collect::<Vec<&str>>().iter() {
            order.push(page_order.parse::<i32>().expect("Failed to parse to i32."))
        }
        res.push((order[0], order[1]))
    }
    res
}

fn get_updates(updates: &str) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for line in updates.lines() {
        let mut update = vec![];
        for page_update in line.split(',').collect::<Vec<&str>>().iter() {
            update.push(page_update.parse::<i32>().expect("Failed to parse to i32."))
        }
        res.push(update)
    }
    res
}

fn is_valid(
    update: &Vec<i32>,
    allowed_values: Option<&Vec<i32>>,
    idx: usize,
) -> (Option<bool>, bool) {
    let mut failed = false;
    if allowed_values.is_some() {
        let is_valid = update[idx + 1..]
            .iter()
            .all(|&item| allowed_values.unwrap().contains(&item));
        return (Some(is_valid), failed);
    } else {
        if idx < update.len() - 1 {
            failed = true;
        }
        return (None, failed);
    };
}

fn reorder(mut update: Vec<i32>, map: HashMap<i32, Vec<i32>>) -> Vec<i32> {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let value: i32 = update[j];
            let allowed_values = map.get(&value);

            let values = match allowed_values {
                Some(value) => value,
                None => break,
            };

            if values.contains(&update[i]) {
                let temp = update[i];
                update[i] = value;
                update[j] = temp;
            }
        }
    }
    update
}

fn split_into_page_orders_and_updates<'a>(input: &'a String) -> Vec<&'a str> {
    input.split("\n\n").collect::<Vec<&str>>()
}

mod solutions {
    use std::fs;

    use crate::{
        build_update_map, get_page_orders, get_updates, is_valid, reorder,
        split_into_page_orders_and_updates,
    };

    pub fn part_1_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Failed to read to string.");

        let strings = split_into_page_orders_and_updates(&input);

        let page_orders = get_page_orders(strings[0]);

        let updates = get_updates(strings[1]);

        let map = build_update_map(page_orders);

        let mut res = 0;

        for update in updates.iter() {
            let mut failed: bool = false;
            for (idx, value) in update.iter().enumerate() {
                let allowed_values = map.get(value);

                let is_valid = is_valid(update, allowed_values, idx);

                failed = is_valid.1;

                match is_valid.0 {
                    Some(value) => {
                        if !value {
                            failed = true;
                            break;
                        }
                    }
                    None => break,
                };
            }
            if !failed {
                let middle = update.len() / 2;

                res += update[middle]
            }
        }
        res
    }

    pub fn part_2_solution(path: &str) -> i32 {
        let input = fs::read_to_string(path).expect("Failed to read to string.");

        let strings = split_into_page_orders_and_updates(&input);

        let page_orders = get_page_orders(strings[0]);

        let updates = get_updates(strings[1]);

        let map = build_update_map(page_orders);

        let mut res = 0;

        for update in updates.iter() {
            let mut failed: bool = false;
            for (idx, value) in update.iter().enumerate() {
                let allowed_values = map.get(value);

                let is_valid = is_valid(update, allowed_values, idx);

                failed = is_valid.1;

                match is_valid.0 {
                    Some(value) => {
                        if !value {
                            failed = true;
                            break;
                        }
                    }
                    None => break,
                };
            }
            if failed {
                let reordered = reorder(update.to_vec(), map.clone());
                println!("{:?}", reordered);

                let middle = reordered.len() / 2;

                res += reordered[middle];
            }
        }
        res
    }
}

fn main() {
    let solution = solutions::part_1_solution("src/mock.txt");

    println!("{solution}");

    let solution = solutions::part_2_solution("src/input.txt");

    println!("{solution}");
}
