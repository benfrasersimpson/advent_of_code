use std::collections::HashSet;

fn main() {
    let part_one = part_one(include_str!("input.txt"));

    println!("Part One: There are {}", part_one);

    let part_two = part_two(include_str!("input.txt"));

    println!("Part Two: There are {}", part_two);
}

fn group_to_set(input: &str) -> HashSet<char> {
    let input = input.replace("\n", "");
    input.chars().collect::<HashSet<char>>()
}

fn group_to_unique_answers(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|a, b| &a & &b)
        .unwrap()
        .len()
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group_to_set(group).len())
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| group_to_unique_answers(group))
        .sum()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(include_str!("test.txt")), 11);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(include_str!("test.txt")), 6)
}
