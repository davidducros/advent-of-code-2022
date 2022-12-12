use std::{fs::{self}, collections::VecDeque};

use itertools::Itertools;



fn calculate_str(s: &str, window: usize) -> u32 {
    let mut v: VecDeque<char> = VecDeque::new();

    for (i, c) in s.chars().enumerate() {
        v.push_back(c);

        while v.len() > window {
            v.pop_front();
        }

        if v.len() == window && v.iter().all_unique() {
            return i as u32 + 1;
        }
    }

    0
}

fn calculate_file(file: &str, window: usize) -> u32 {
    let contents = fs::read_to_string(file).unwrap();

    calculate_str(contents.as_str(), window)
}

fn main() {
    println!("result: {}", calculate_file("input/problem.txt", 4));
    println!("result part 2: {}", calculate_file("input/problem.txt", 14));
}

#[test]
fn test_example() {
    assert_eq!(5, calculate_str("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, calculate_str("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, calculate_str("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, calculate_str("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
}

#[test]
fn test_example_part2() {
    assert_eq!(23, calculate_str("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, calculate_str("nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, calculate_str("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, calculate_str("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
}