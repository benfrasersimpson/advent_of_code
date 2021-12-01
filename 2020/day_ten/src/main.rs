fn main() {
    println!("Hello, world!");

    let input = include_str!("input.txt");

    let (one, three) = part_one(input);

    println!("part one: {}", one * three);
}

fn part_one(input: &str) -> (u64, u64) {
    let mut input = strings_to_ints(input).unwrap();

    input.sort();

    count_1_and_3_jumps(&input)
}

fn count_1_and_3_jumps(input: &[u64]) -> (u64, u64) {
    let (mut one, mut three) = (1, 1);

    for i in 0..(input.len() - 1) {
        let diff = input[i + 1] - input[i];
        if diff == 1 {
            one += 1;
        } else if diff == 3 {
            three += 1;
        }
    }

    (one, three)
}

fn strings_to_ints(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    input.lines().map(|line| line.parse::<u64>()).collect()
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(part_one(input), (7, 5));
}

#[test]
fn test_part_one_again() {
    let input = include_str!("test2.txt");
    assert_eq!(part_one(input), (22, 10));
}
