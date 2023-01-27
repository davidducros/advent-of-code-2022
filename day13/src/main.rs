use std::{
    collections::VecDeque,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

struct PacketPair {
    left: Packet,
    right: Packet,
}

impl Display for PacketPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.left)?;
        writeln!(f, "{}", self.right)
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(i) => {
                write!(f, "{}", i)?;
            }
            Packet::List(list) => {
                let mut output = String::new();

                for p in list {
                    if !output.is_empty() {
                        output.push(',');
                    }
                    output.push_str(p.to_string().as_str());
                }
                write!(f, "[{}]", output)?;
            }
        }

        Ok(())
    }
}

fn load_packet_list(chars: &mut impl Iterator<Item = char>) -> Packet {
    let mut result = Vec::new();

    let mut number = String::new();
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            number.push(c);
        } else if c == ',' {
            if !number.is_empty() {
                result.push(Packet::Integer(number.parse::<u32>().unwrap()));
                number.clear();
            }
        } else if c == '[' {
            result.push(load_packet_list(chars));
        } else if c == ']' {
            if !number.is_empty() {
                result.push(Packet::Integer(number.parse::<u32>().unwrap()));
            }
            break;
        }
    }

    Packet::List(result)
}

fn load_packet(line: &String) -> Packet {
    let mut iter = line.chars();

    // consume the first [
    iter.next();

    load_packet_list(&mut iter)
}

fn load_packet_str(line: &str) -> Packet {
    load_packet(&line.to_string())
}

fn load_packets(file: &str) -> Vec<PacketPair> {
    let file = File::open(file).unwrap();

    let mut lines: VecDeque<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let mut packets = Vec::new();
    while !lines.is_empty() {
        packets.push(PacketPair {
            left: load_packet(&lines.pop_front().unwrap()),
            right: load_packet(&lines.pop_front().unwrap()),
        });

        // there might be a blank line to consume
        if !lines.is_empty() {
            lines.pop_front();
        }
    }

    packets
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Integer(left), Packet::Integer(right)) => {
                return left.cmp(right);
            }
            (Packet::Integer(_), Packet::List(_)) => {
                return Packet::List(vec![self.clone()]).cmp(other);
            }
            (Packet::List(_), Packet::Integer(_)) => {
                return self.cmp(&Packet::List(vec![other.clone()]));
            }
            (Packet::List(left), Packet::List(right)) => {
                for item in left.iter().zip_longest(right.iter()) {
                    let result = match item {
                        itertools::EitherOrBoth::Both(left, right) => left.cmp(right),
                        itertools::EitherOrBoth::Left(_) => std::cmp::Ordering::Greater,
                        itertools::EitherOrBoth::Right(_) => std::cmp::Ordering::Less,
                    };

                    match result {
                        std::cmp::Ordering::Equal => (),
                        _ => {
                            return result;
                        }
                    }
                }

                std::cmp::Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate(file: &str) -> u32 {
    let packets = load_packets(file);

    let mut result = 0;
    for (i, p) in packets.iter().enumerate() {
        match p.left.cmp(&p.right) {
            std::cmp::Ordering::Less => result += i as u32 + 1,
            _ => (),
        }
    }

    result
}

fn calculate_part2(file: &str) -> u32 {
    let mut packets = Vec::new();
    for p in load_packets(file) {
        packets.push(p.left);
        packets.push(p.right);
    }

    packets.push(load_packet_str("[[2]]"));
    packets.push(load_packet_str("[[6]]"));

    packets.sort();

    let mut result = 1;
    for (i, p) in packets.iter().enumerate() {
        if *p == load_packet_str("[[2]]") {
            result *= i as u32 + 1;
        } else if *p == load_packet_str("[[6]]") {
            result *= i as u32 + 1;
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
    assert_eq!(13, calculate("input/example.txt"));
}

#[test]
fn test_example_part2() {
    assert_eq!(140, calculate_part2("input/example.txt"));
}

#[test]
fn cmp() {
    use std::cmp::Ordering::*;
    assert_eq!(
        Greater,
        load_packet_str("[9]").cmp(&load_packet_str("[[]]"))
    );
}
