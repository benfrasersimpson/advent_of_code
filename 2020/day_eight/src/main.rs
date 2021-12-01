use std::{iter::Skip, process::exit};

#[derive(Debug, Copy, Clone)]
enum Operation {
    NOP(i32),
    JMP(i32),
    ACC(i32),
}

impl std::ops::Neg for Operation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Operation::NOP(value) => Operation::JMP(value),
            Operation::JMP(value) => Operation::NOP(value),
            Operation::ACC(value) => Operation::ACC(value),
        }
    }
}

fn parse_string_representation(repr: &str) -> Vec<Operation> {
    repr.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(instruction, value)| match instruction {
            "nop" => Operation::NOP(value.parse::<i32>().unwrap()),
            "jmp" => Operation::JMP(value.parse::<i32>().unwrap()),
            "acc" => Operation::ACC(value.parse::<i32>().unwrap()),
            _ => panic!(),
        })
        .collect()
}

fn run(operations: Vec<Operation>) -> (i32, bool) {
    let mut program_counter: i32 = 0;
    let mut accumulator: i32 = 0;

    let mut access_checker = vec![false; operations.len()];

    loop {
        if operations.len() == std::convert::TryInto::try_into(program_counter).unwrap() {
            return (accumulator, true);
        }

        let already_executed_instruction = access_checker.get(program_counter as usize).unwrap();

        if *already_executed_instruction {
            return (accumulator, false);
        }

        let operation = operations.get(program_counter as usize).unwrap();

        println!("Acc {}, Cnt {}", accumulator, program_counter);
        println!("operation {:?}", operation);

        access_checker[program_counter as usize] = true;

        //println!("Neg? {} val {}", is_negative, value);

        match operation {
            Operation::NOP(_) => program_counter += 1,
            Operation::JMP(value) => {
                program_counter += *value;
            }
            Operation::ACC(value) => {
                program_counter += 1;
                accumulator += *value;
            }
        }
    }
}

fn main() {
    let operations = parse_string_representation(include_str!("input.txt"));

    for i in 0..operations.len() {
        let mut this_run = operations.clone();
        this_run[i] = -this_run[i];

        println!("Ops: {:?}", this_run);

        let (answer, success) = run(this_run);

        if success {
            println!("Acc is {}", answer);
            break;
        }
    }
}
