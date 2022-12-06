use std::{
    fs::File,
    io::{self, BufRead},
};

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Range {
        Range {
            start: start,
            end: end,
        }
    }

    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.end >= self.start && other.start <= self.start)
            || (other.start <= self.end && other.end >= self.start)
    }
}

fn parse(line: &str) -> Vec<Range> {
    let mut results: Vec<Range> = vec![];

    for elf in line.split(",") {
        let range: Vec<&str> = elf.split("-").collect();

        let start = range.get(0).unwrap().parse::<u32>().unwrap();
        let end = range.get(1).unwrap().parse::<u32>().unwrap();

        results.push(Range::new(start, end));
    }

    results
}

fn calculate(file: &str) -> u32 {
    let mut acc = 0;

    let file = File::open(file).unwrap();
    for line in io::BufReader::new(file).lines() {
        let ranges = parse(line.unwrap().as_str());

        let r1 = ranges.get(0).unwrap();
        let r2 = ranges.get(1).unwrap();

        if r1.contains(r2) || r2.contains(r1) {
            acc += 1;
        }
    }

    acc
}

fn calculate_part2(file: &str) -> u32 {
    let mut acc = 0;

    let file = File::open(file).unwrap();
    for line in io::BufReader::new(file).lines() {
        let ranges = parse(line.unwrap().as_str());

        let r1 = ranges.get(0).unwrap();
        let r2 = ranges.get(1).unwrap();

        if r1.overlaps(r2) {
            acc += 1;
        }
    }

    acc
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
    println!("result part 2: {}", calculate_part2("input/problem.txt"));
}

#[test]
fn test_example() {
    assert_eq!(2, calculate("input/example.txt"));
}

#[test]
fn test_example_part2() {
    assert_eq!(4, calculate_part2("input/example.txt"));
}
