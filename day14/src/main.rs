use itertools::Itertools;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug)]
enum Substance {
    Air,
    Rock,
    Sand,
}

enum SandResult {
    Success(Point),
    Abyss,
}

enum Floor {
    Abyss(u32),
    Floor(u32),
}

fn point_from_string(s: &str) -> Point {
    let v = s
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let x = v[0];
    let y = v[1];

    Point { x, y }
}

fn load_rock_line(result: &mut BTreeMap<Point, Substance>, line: &String) {
    let points = line
        .split(" -> ")
        .map(point_from_string)
        .collect::<Vec<Point>>();

    for (p1, p2) in points.iter().tuple_windows() {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);

        for x in min_x..=max_x {
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            for y in min_y..=max_y {
                result.insert(Point { x, y }, Substance::Rock);
            }
        }
    }
}

fn load_rocks(file: &str) -> BTreeMap<Point, Substance> {
    let file = File::open(file).unwrap();

    let mut result = BTreeMap::new();
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        load_rock_line(&mut result, &line);
    }

    result
}

fn add_sand(state: &mut BTreeMap<Point, Substance>, p: Point, floor: Floor) -> SandResult {
    let mut p = p;

    loop {
        let options = [
            Point { x: p.x, y: p.y + 1 },
            Point {
                x: p.x - 1,
                y: p.y + 1,
            },
            Point {
                x: p.x + 1,
                y: p.y + 1,
            },
        ];

        let mut moved = false;
        for o in options.iter() {
            match *state.get(o).unwrap_or(&Substance::Air) {
                Substance::Air => {
                    p = *o;
                    moved = true;
                    break;
                }
                Substance::Rock | Substance::Sand => (),
            }
        }

        if !moved {
            break;
        }

        match floor {
            Floor::Abyss(y) => {
                if p.y > y {
                    return SandResult::Abyss;
                }
            }
            Floor::Floor(y) => {
                if p.y + 1 == y {
                    break;
                }
            }
        };
    }

    state.insert(p, Substance::Sand);

    SandResult::Success(p)
}

fn highest_y(state: &BTreeMap<Point, Substance>) -> u32 {
    state.iter().map(|(k, _v)| k.y).max().unwrap()
}

fn calculate(file: &str) -> u32 {
    let mut state = load_rocks(file);

    let abyss = highest_y(&state);

    let mut result = 0;
    while let SandResult::Success(_) =
        add_sand(&mut state, Point { x: 500, y: 0 }, Floor::Abyss(abyss))
    {
        result += 1;
    }

    result
}

fn calculate_part2(file: &str) -> u32 {
    let mut state = load_rocks(file);

    let floor = highest_y(&state) + 2;

    let mut result = 0;
    while let SandResult::Success(p) =
        add_sand(&mut state, Point { x: 500, y: 0 }, Floor::Floor(floor))
    {
        result += 1;
        
        if p == (Point{ x: 500, y: 0 }) {
            break;
        }
    }

    result
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
    println!("result part 2: {}", calculate_part2("input/problem.txt"));
}

#[test]
fn test_example() {
    assert_eq!(24, calculate("input/example.txt"));
}

#[test]
fn test_example_part2() {
    assert_eq!(93, calculate_part2("input/example.txt"));
}
