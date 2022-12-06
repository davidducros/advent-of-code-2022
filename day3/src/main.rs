use std::{
    collections::{HashSet, HashMap},
    fs::File,
    io::{self, BufRead}
};

use itertools::Itertools;

fn duplicate(s: &String) -> char {
    let mut items: HashSet<char> = HashSet::new();

    let n = s.len() / 2;

    for c in s.chars().take(n) {
        items.insert(c);
    }

    for c in s.chars().rev().take(n) {
        if items.contains(&c) {
            return c;
        }
    }

    panic!("invalid input");
}

fn score(c: char) -> u32 {
    let v = c as u32;

    let a = 'a' as u32;
    let z = 'z' as u32;
    let big_a = 'A' as u32;

    if v >= a && v <= z {
        return v - a + 1;
    }

    return v - big_a + 27;
}

fn calculate(file: &str) -> u32 {
    let mut acc = 0;

    let file = File::open(file).unwrap();
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            acc += score(duplicate(&line));
        }
    }

    acc
}

fn common(group: &Vec<String>) -> char {

    let mut items: HashMap<char, u32> = HashMap::new();

    for bag in group {
        for c in bag.chars().unique() {
            *items.entry(c).or_insert_with(|| 0) += 1;
        }
    }

    for entry in items {
        if entry.1 == group.len() as u32 {
            return entry.0;
        }
    }


    panic!("no common items");
}

fn calculate_part2(file: &str, group_size: usize) -> u32 {
    let mut acc = 0;

    let file = File::open(file).unwrap();

    let mut group: Vec<String> = vec![];

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            group.push(line);

            if group.len() == group_size {
                acc += score(common(&group));
                group.clear();
            }
        }
    }

    acc
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
    println!("result part 2: {}", calculate_part2("input/problem.txt", 3));
}

#[test]
fn test_example() {
    assert_eq!(157, calculate("input/example.txt"));
    assert_eq!(70, calculate_part2("input/example.txt", 3));
}
