use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    count: u32,
}

fn move_leader(p: &mut Point, d: &Direction) {
    *p = match d {
        Direction::Up => Point { x: p.x, y: p.y + 1 },
        Direction::Down => Point { x: p.x, y: p.y - 1 },
        Direction::Left => Point { x: p.x - 1, y: p.y },
        Direction::Right => Point { x: p.x + 1, y: p.y },
    };
}

fn move_follower(follower: &mut Point, leader: &Point) {
    let x_diff: i32 = leader.x as i32 - follower.x as i32;
    let y_diff: i32 = leader.y as i32 - follower.y as i32;

    let replacement = match (x_diff, y_diff) {
        (0, 2) => Point::new(follower.x, follower.y + 1),
        (0, -2) => Point::new(follower.x, follower.y - 1),
        (2, 0) => Point::new(follower.x + 1, follower.y),
        (-2, 0) => Point::new(follower.x - 1, follower.y),
        (1, 2) => Point::new(follower.x + 1, follower.y + 1),
        (2, 1) => Point::new(follower.x + 1, follower.y + 1),
        (2, -1) => Point::new(follower.x + 1, follower.y - 1),
        (1, -2) => Point::new(follower.x + 1, follower.y - 1),
        (-1, -2) => Point::new(follower.x - 1, follower.y - 1),
        (-2, -1) => Point::new(follower.x - 1, follower.y - 1),
        (-2, 1) => Point::new(follower.x - 1, follower.y + 1),
        (-1, 2) => Point::new(follower.x - 1, follower.y + 1),
        (2, 2) => Point::new(follower.x + 1, follower.y + 1),
        (-2, 2) => Point::new(follower.x - 1, follower.y + 1),
        (2, -2) => Point::new(follower.x + 1, follower.y - 1),
        (-2, -2) => Point::new(follower.x - 1, follower.y - 1),
        _ => follower.clone(),
    };

    *follower = replacement;
}

fn load_instructions(file: &str) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = vec![];

    let file = File::open(file).unwrap();
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let tokens: Vec<&str> = line.split(" ").collect();

        let direction = match tokens.get(0).unwrap() {
            &"U" => Direction::Up,
            &"D" => Direction::Down,
            &"L" => Direction::Left,
            &"R" => Direction::Right,
            _ => panic!("unsupported instruction"),
        };

        let count = tokens.get(1).unwrap().parse::<u32>().unwrap();

        result.push(Instruction {
            direction: direction,
            count: count,
        })
    }

    result
}

fn calculate(file: &str) -> u32 {
    let instructions = load_instructions(file);

    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut tail_history = vec![head.clone()];

    for i in instructions {
        for _ in 0..i.count {
            move_leader(&mut head, &i.direction);
            move_follower(&mut tail, &head);

            tail_history.push(tail.clone());
        }
    }

    tail_history.iter().unique().count() as u32
}

fn print_points(points: &Vec<Point>, size: usize) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; size]; size];

    let mut max_x = size as i32 - 1;
    let mut max_y = size as i32 - 1;

    for (i, point) in points.iter().enumerate() {
        if point.x > max_x {
            for row in grid.iter_mut() {
                row.resize(point.x as usize + 1, '.');
                max_x = point.x;
            }
        }

        if point.y > max_y {
            grid.resize(point.y as usize + 1, vec!['.'; max_x as usize + 1]);
            max_y = point.y;
        }

        let c = grid
            .get_mut(point.y as usize)
            .unwrap()
            .get_mut(point.x as usize)
            .unwrap();

        if *c == '.' {
            *c = char::from_digit(i as u32, 10).unwrap();
        }
    }

    for row in grid.iter().rev() {
        for c in row {
            print!("{} ", c);
        }

        print!("\n");
    }

    print!("\n");
}

fn calculate_part2(file: &str, print: bool) -> u32 {
    let instructions = load_instructions(file);

    let mut points = vec![Point::new(0, 0); 10];

    let mut results = vec![points.last().unwrap().clone()];

    for i in instructions {
        for _ in 0..i.count {
            move_leader(points.first_mut().unwrap(), &i.direction);

            for i in 1..points.len() {
                let leader = points.get(i - 1).unwrap().clone();
                let follower = points.get_mut(i).unwrap();
                move_follower(follower, &leader);
            }

            if print {
                print_points(&points, 6);
            }

            results.push(points.last().unwrap().clone());
        }
    }

    results.iter().unique().count() as u32
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
    println!(
        "result part2: {}",
        calculate_part2("input/problem.txt", false)
    );
}

#[test]
fn test_example() {
    assert_eq!(13, calculate("input/example.txt"));
}

#[test]
fn test_example_part2() {
    assert_eq!(1, calculate_part2("input/example.txt", true));
}
