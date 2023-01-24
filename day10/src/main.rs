use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};


struct Cpu {
    cycle: u32,
    x: i32,

    instruction: Option<Instruction>
}

impl Cpu {
    fn load(&mut self, i: InstructionType) {
        self.instruction = match i {
            InstructionType::NOOP => Some(Instruction{ cycles: 1, result: self.x }),
            InstructionType::ADDX(v) => Some(Instruction { cycles: 2, result: self.x + v })
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        match &mut self.instruction {
            Some(i) => {
                i.cycles -= 1;

                if i.cycles == 0 {
                    self.x = i.result;
                    self.instruction = None;
                }
            }
            None => panic!("this shouldn't happen")
        }
    }
}

enum InstructionType {
    NOOP,
    ADDX(i32)
}

struct Instruction {
    cycles: u32,
    result: i32
}

fn load_instructions(file: &str) -> VecDeque<InstructionType> {
    let mut result: VecDeque<InstructionType> = VecDeque::new();

    let file = File::open(file).unwrap();
    for line in BufReader::new(file).lines().map(|l| l.unwrap()) {
        let tokens: Vec<&str> = line.split(" ").collect();

        let instruction = match tokens.get(0).unwrap() {
            &"noop" => InstructionType::NOOP,
            &"addx" => InstructionType::ADDX(tokens.get(1).unwrap().parse::<i32>().unwrap()),
            _ => panic!("unexpected instruction")
        };

        result.push_back(instruction);
    }

    result
}

fn calculate(file: &str, special_cycles: &[u32]) -> i32 {
    let mut instructions = load_instructions(file);

    let mut cpu = Cpu{ cycle: 1, x: 1, instruction: None };
    let mut result: i32 = 0;

    while !instructions.is_empty() {
        if special_cycles.contains(&cpu.cycle) {
            result += cpu.x * cpu.cycle as i32;
        }

        if cpu.instruction.is_none() {
            cpu.load(instructions.pop_front().unwrap())
        }

        cpu.tick();

    }

    result
}

fn main() {
    println!("result: {}", calculate("input/problem.txt", &[20, 60, 100, 140, 180, 220]));
}

#[test]
fn test_example() {
    assert_eq!(13140, calculate("input/example.txt", &[20, 60, 100, 140, 180, 220]));
}