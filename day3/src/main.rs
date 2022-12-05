use std::{fs::File, io::{self, BufRead}, collections::HashSet};


fn duplicate(s: &String) -> char {
    let mut items: HashSet<char> = HashSet::new();

    let n = s.len() / 2;

    for c in s.chars().take(n) {
        items.insert(c);
    }

    for c in s.chars().rev().take(n) {
        if items.contains(&c) {
            return c
        }
    }

    panic!("invalid input");
}

fn score(c: char) -> u32 {
    let v = c as u32;

    let a = 'a' as u32;
    let z = 'z' as u32;
    let A = 'A' as u32;

    if v >= a && v <= z {
        return v - a + 1; 
    }

    return v - A + 27;
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

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
}

#[test]
fn test_example() {
    assert_eq!(157, calculate("input/example.txt"))
}