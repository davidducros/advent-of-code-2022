use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use pathfinding::prelude::{astar, bfs};

#[derive(Debug)]
enum PointType {
    Start,
    End,
    Normal,
}

#[derive(Debug)]
struct Point {
    display: char,
    height: u32,
    point_type: PointType,
}

impl Point {
    fn new(value: char) -> Point {
        match value {
            'S' => Point {
                display: 'S',
                height: Point::height('a'),
                point_type: PointType::Start,
            },
            'E' => Point {
                display: 'E',
                height: Point::height('z'),
                point_type: PointType::End,
            },
            _ => Point {
                display: value,
                height: Point::height(value),
                point_type: PointType::Normal,
            },
        }
    }

    fn height(c: char) -> u32 {
        c as u32 - 'a' as u32
    }
}

struct Grid {
    points: Vec<Vec<Point>>,

    start: (usize, usize),
    end: (usize, usize),
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.points.iter() {
            for p in row {
                write!(f, "{}", p.display)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Grid {
    fn get_height(&self, p: (usize, usize)) -> Option<u32> {
        let (x, y) = p;

        let row = self.points.get(y)?;
        let point = row.get(x)?;

        Some(point.height)
    }

    fn distance(&self, p: (usize, usize)) -> usize {
        self.end.0.abs_diff(p.0) + self.end.1.abs_diff(p.1)
    }

    fn successors(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = (p.0, p.1);

        let start_height = self.get_height(p).unwrap();

        let mut maybe = vec![(x + 1, y), (x, y + 1)];

        if x > 0 {
            maybe.push((x - 1, y));
        }

        if y > 0 {
            maybe.push((x, y - 1));
        }

        let mut result = Vec::new();
        for m in maybe.iter() {
            if let Some(height) = self.get_height(*m) {
                if height <= start_height + 1 {
                    result.push(*m);
                }
            }
        }

        result
    }

    fn successors_cost(&self, p: (usize, usize)) -> Vec<((usize, usize), usize)> {
        self.successors(p).iter().map(|p| (*p, 1)).collect()
    }
}

fn load_grid(file: &str) -> Grid {
    let file = File::open(file).unwrap();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut points: Vec<Vec<Point>> = Vec::new();

    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut row = Vec::new();

        for c in line.chars() {
            let p = Point::new(c);

            match p {
                Point {
                    point_type: PointType::Start,
                    ..
                } => start = (row.len(), points.len()),
                Point {
                    point_type: PointType::End,
                    ..
                } => end = (row.len(), points.len()),
                Point {
                    point_type: PointType::Normal,
                    ..
                } => (),
            }

            row.push(p);
        }

        points.push(row);
    }

    Grid { points, start, end }
}

fn calculate(file: &str) -> usize {
    let grid = load_grid(file);

    let path = run_astar(&grid, grid.start);

    path.unwrap().len() - 1
}

fn calculate_part2(file: &str) -> usize {
    let grid = load_grid(file);

    let mut starts = Vec::new();
    for (y, row) in grid.points.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if p.height == 0 {
                starts.push((x, y));
            }
        }
    }

    let mut options = Vec::new();
    for start in starts {
        if let Some(result) = run_astar(&grid, start) {
            options.push(result.len() - 1);
        }
    }

    *options.iter().min().unwrap()
}

fn run_astar(grid: &Grid, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let result = astar(
        &start,
        |p| grid.successors_cost(*p),
        |p| grid.distance(*p),
        |p| *p == grid.end,
    )?;
    
    Some(result.0)
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
    println!("result part 2: {}", calculate_part2("input/problem.txt"));
}

#[test]
fn test_example() {
    assert_eq!(31, calculate("input/example.txt"));
}

#[test]
fn test_example_part2() {
    assert_eq!(29, calculate_part2("input/example.txt"));
}
