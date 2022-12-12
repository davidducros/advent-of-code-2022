use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead},
};

enum Mode {
    Single,
    Multiple
}

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse(lines: &mut impl Iterator<Item = String>) -> Stacks {
        let mut stacks: Vec<Vec<char>> = vec![];

        for line in lines {
            if line.contains("[") {
                for (i, c) in line.chars().enumerate() {
                    if i % 4 == 1 && c != ' ' {
                        let n = i / 4; // which stack is this

                        if stacks.len() < n + 1 {
                            stacks.resize(n + 1, vec![]);
                        }

                        stacks.get_mut(n).unwrap().push(c);
                    }
                }
            } else {
                break;
            }
        }

        stacks.iter_mut().for_each(|v| v.reverse());

        Stacks { stacks: stacks }
    }

    fn move_crates(&mut self, from: usize, to: usize, count: u32) {
        for _ in 0..count {
            if let Some(v) = self.stacks.get_mut(from - 1).unwrap().pop() {
                self.stacks.get_mut(to - 1).unwrap().push(v);
            }
        }
    }

    fn move_multiple_crates(&mut self, from: usize, to: usize, count: u32) {
        let mut tmp: Vec<char> = vec![];

        {
            let from = self.stacks.get_mut(from - 1).unwrap();

            for _ in 0..count {
                tmp.push(from.pop().unwrap());
            }
        }

        let to = self.stacks.get_mut(to - 1).unwrap();

        for c in tmp.iter().rev() {
            to.push(*c);
        }
    }


    fn result(&self) -> String {
        let mut s = String::new();
        for stack in &self.stacks {
            match stack.last() {
                Some(value) => s.push(*value),
                None => (),
            }
        }

        s
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            writeln!(
                f,
                "{}: {}",
                i + 1,
                stack
                    .iter()
                    .map(|c| String::from(*c))
                    .collect::<Vec<String>>()
                    .join(" ")
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Move {
    count: u32,
    from: usize,
    to: usize,
}

impl Move {
    fn new(count: u32, from: usize, to: usize) -> Move {
        Move {
            count: count,
            from: from,
            to: to,
        }
    }

    fn parse(lines: &mut impl Iterator<Item = String>) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        for line in lines {
            if line.contains("move") {
                let tokens: Vec<&str> = line.split(" ").collect();

                let count = tokens.get(1).unwrap().parse::<u32>().unwrap();
                let from = tokens.get(3).unwrap().parse::<usize>().unwrap();
                let to = tokens.get(5).unwrap().parse::<usize>().unwrap();

                moves.push(Move::new(count, from, to));
            }
        }
        moves
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "move {} from {} to {}", self.count, self.from, self.to)
    }
}

fn calculate(file: &str, mode: Mode) -> String {
    let file = File::open(file).unwrap();

    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());

    let mut stacks = Stacks::parse(&mut lines);

    println!("{}", stacks);

    let moves = Move::parse(&mut lines);

    println!("moves: {:?}", moves);

    for m in moves {
        match mode {
            Mode::Single => stacks.move_crates(m.from, m.to, m.count),
            Mode::Multiple => stacks.move_multiple_crates(m.from, m.to, m.count)
        }
    }

    println!("{}", stacks);

    stacks.result()
}

fn main() {
    println!("result: {}", calculate("input/problem.txt", Mode::Single));
    println!("result: {}", calculate("input/problem.txt", Mode::Multiple));
}

#[test]
fn test_example() {
    assert_eq!("CMZ", calculate("input/example.txt", Mode::Single));
}

#[test]
fn test_example_part2() {
    assert_eq!("MCD", calculate("input/example.txt", Mode::Multiple));
}
