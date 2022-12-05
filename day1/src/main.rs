use std::{io::{self, BufRead}, fs::File, collections::BTreeSet};


fn calculate(file: &str, count: usize) -> i32 {
    let file = File::open(file).unwrap();

    let mut acc = 0;

    let mut results: BTreeSet<i32> = BTreeSet::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            match line.as_str() {
                "" => {
                    results.insert(acc);
                    acc = 0;
                },
                _ => {
                    acc += line.parse::<i32>().unwrap();
                }
            }
        }
    }

    if acc != 0 {
        results.insert(acc);
    }

    results.iter().rev().take(count).cloned().sum()
}

fn main() {
    println!("result: {}", calculate("input/problem.txt", 3));
}

#[test]
fn example() {
    assert_eq!(45000, calculate("input/example.txt", 3));
}