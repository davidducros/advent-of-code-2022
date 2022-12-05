use std::{fs::File, io::{self, BufRead}};


#[derive(Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0
}

fn score_result(m: Move, r: Result) -> i32 {
   m as i32 + r as i32 
}

fn score(moves: (Move, Move)) -> i32 {
    let m = moves.1;

    match moves {
        (Move::Rock, Move::Rock) => return score_result(m, Result::Draw),
        (Move::Rock, Move::Paper) => return score_result(m, Result::Win),
        (Move::Rock, Move::Scissors) => return score_result(m, Result::Lose),
        (Move::Paper, Move::Rock) => return score_result(m, Result::Lose),
        (Move::Paper, Move::Paper) => return score_result(m, Result::Draw),
        (Move::Paper, Move::Scissors) => return score_result(m, Result::Win),
        (Move::Scissors, Move::Rock) => return score_result(m, Result::Win),
        (Move::Scissors, Move::Paper) => return score_result(m, Result::Lose),
        (Move::Scissors, Move::Scissors) => return score_result(m, Result::Draw),
    }
}

fn input(line: &str) -> (Move, Move) {
    match line {
        "A X" => return (Move::Rock, Move::Rock),
        "A Y" => return (Move::Rock, Move::Paper),
        "A Z" => return (Move::Rock, Move::Scissors),
        "B X" => return (Move::Paper, Move::Rock),
        "B Y" => return (Move::Paper, Move::Paper),
        "B Z" => return (Move::Paper, Move::Scissors),
        "C X" => return (Move::Scissors, Move::Rock),
        "C Y" => return (Move::Scissors, Move::Paper),
        "C Z" => return (Move::Scissors, Move::Scissors),
        _ => panic!("unexpected input")
    }
}

fn calculate(file: &str) -> i32 {
    let mut acc = 0;

    let file = File::open(file).unwrap();
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let m = input(line.as_str());
            let s = score(m);
            acc += s;
        }
    }

    acc
}

#[test]
fn test_example() {
    assert_eq!(15, calculate("input/example.txt"));
}

#[test]
fn test_example2() {
    assert_eq!(27 + 18, calculate("input/example2.txt"));
}

fn main() {
    println!("result: {}", calculate("input/problem.txt"));
}
