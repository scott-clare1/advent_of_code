use common::read_path;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

fn collect_points(grid: &mut Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    for x in 0..grid.len() {
        for y in 0..grid.len() {
            let point = Point {
                x: x as f32,
                y: y as f32,
            };
            let c = grid[x][y];

            if c != '.' {
                match map.get_mut(&c) {
                    Some(value) => value.push(point),
                    None => {
                        map.insert(c, vec![point]);
                    }
                }
            }
        }
    }
    map
}

fn find_gradient(a: Point, b: Point) -> f32 {
    let dy = b.y - a.y;
    let dx = b.x - a.x;

    dy as f32 / dx as f32
}

fn find_distance(a: Point, b: Point) -> f32 {
    let x_diff = a.x - b.x;
    let y_diff = a.y - b.y;

    (x_diff.powi(2) + y_diff.powi(2)).sqrt()
}

fn move_point(start: Point, distance: f32, grad: f32) -> Point {
    let angle = grad.atan();
    let x = start.x + distance * angle.cos();
    let y = start.y + distance * angle.sin();
    Point { x, y }
}

mod solutions {
    use std::fs;

    use std::collections::HashSet;

    use common::build_grid;

    use crate::{collect_points, find_distance, find_gradient, move_point};

    pub fn part_1_solution(path: &str) -> usize {
        let input = fs::read_to_string(path).expect("Failed to read input.");
        let mut grid = build_grid(input);

        let points = collect_points(&mut grid);

        let mut locations: HashSet<(i32, i32)> = HashSet::new();

        for v in points.values() {
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let (a, b) = (v[i], v[j]);

                    let grad = find_gradient(a, b);

                    let distance = find_distance(a, b);

                    let (left, right) = (
                        move_point(a, -distance, grad),
                        move_point(b, distance, grad),
                    );

                    if left.x.round() >= 0.0
                        && left.x.round() < grid.len() as f32
                        && left.y.round() >= 0.0
                        && left.y.round() < grid.len() as f32
                    {
                        locations.insert((left.x.round() as i32, left.y.round() as i32));
                    }
                    if right.x.round() >= 0.0
                        && right.x.round() < grid.len() as f32
                        && right.y.round() >= 0.0
                        && right.y.round() < grid.len() as f32
                    {
                        locations.insert((right.x.round() as i32, right.y.round() as i32));
                    }
                }
            }
        }
        locations.len()
    }

    pub fn part_2_solution(path: &str) -> usize {
        let input = fs::read_to_string(path).expect("Failed to read input.");
        let mut grid = build_grid(input);

        let points = collect_points(&mut grid);

        let mut locations: HashSet<(i32, i32)> = HashSet::new();

        for v in points.values() {
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let (a, b) = (v[i], v[j]);

                    let grad = find_gradient(a, b);

                    let distance = find_distance(a, b);

                    locations.insert((a.x as i32, a.y as i32));
                    locations.insert((b.x as i32, b.y as i32));

                    let (mut left, mut right) = (
                        move_point(a, -distance, grad),
                        move_point(b, distance, grad),
                    );

                    while left.x.round() >= 0.0
                        && left.x.round() < grid.len() as f32
                        && left.y.round() >= 0.0
                        && left.y.round() < grid.len() as f32
                    {
                        locations.insert((left.x.round() as i32, left.y.round() as i32));
                        left = move_point(left, -distance, grad);
                    }

                    while right.x.round() >= 0.0
                        && right.x.round() < grid.len() as f32
                        && right.y.round() >= 0.0
                        && right.y.round() < grid.len() as f32
                    {
                        locations.insert((right.x.round() as i32, right.y.round() as i32));
                        right = move_point(right, distance, grad);
                    }
                }
            }
        }
        locations.len()
    }
}

fn main() {
    let path = read_path();

    let solution = solutions::part_1_solution(&path);

    println!("{solution}");

    let path = read_path();

    let solution = solutions::part_2_solution(&path);

    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use std::f32::NEG_INFINITY;

    use super::*;

    #[test]
    fn test_find_gradient() {
        let a = Point { x: 5.0, y: 6.0 };
        let b = Point { x: 9.0, y: 9.0 };

        let gradient = find_gradient(a, b);

        assert_eq!(gradient, 0.75);

        let a = Point { x: 5.0, y: 5.0 };
        let b = Point { x: 5.0, y: 4.0 };

        let gradient = find_gradient(a, b);

        assert_eq!(gradient, NEG_INFINITY);

        let a = Point { x: 4.0, y: 5.0 };
        let b = Point { x: 5.0, y: 5.0 };

        let gradient = find_gradient(a, b);

        assert_eq!(gradient, 0.0);
    }
}
