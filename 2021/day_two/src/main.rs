enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Command {
    fn from_str(input: &str) -> Command {
        let (direction, amount) = input.split_once(' ').unwrap();
        let amount: u32 = amount.parse::<u32>().unwrap();

        match direction {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => panic!("Unknown direction {}", direction),
        }
    }
}

fn main() {
    let commands = input_to_commands(include_str!("input.txt"));

    println!("The answer to part one is {}", part_one(&commands));

    println!("The answer to part two is {}", part_two(&commands));

}

fn input_to_commands(input: &str) -> Vec<Command> {
    input.lines().map(Command::from_str).collect()
}

fn part_one(commands: &[Command]) -> u32 {
    let (horizontal, depth) = commands.iter().fold(
        (0, 0),
        |(mut horizontal, mut depth), command| {
            match command {
                Command::Forward(value) => horizontal += value,
                Command::Down(value) => depth += value,
                Command::Up(value) => depth -= value,
            }
            (horizontal, depth)
        },
    );

    horizontal * depth

}

fn part_two(commands: &[Command]) -> u32 {
    let (horizontal, depth, _) = commands.iter().fold(
        (0, 0, 0),
        |(mut horizontal, mut depth, mut aim), command| {
            match command {
                Command::Forward(value) => { horizontal += value; depth += aim * value } ,
                Command::Down(value) => aim += value,
                Command::Up(value) => aim -= value,
            }
            (horizontal, depth, aim)
        },
    );

    horizontal * depth

}

#[test]
fn test_part_one() {
    let input = input_to_commands(include_str!("test.txt"));

    assert_eq!(part_one(&input), 150);
}

#[test]
fn test_part_two() {
    let input = input_to_commands(include_str!("test.txt"));

    assert_eq!(part_two(&input), 900);
}
