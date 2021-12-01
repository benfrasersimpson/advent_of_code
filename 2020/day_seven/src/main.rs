use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    let input = parse_input(input);

    let count = find(&input);

    println!("There are {}", count);
}

fn line_to_record(input: &str) -> (&str, Vec<(u32, &str)>) {
    let (node_name, elements) = input.split_once(" contain ").unwrap();

    let (node_name, _) = node_name.rsplit_once(' ').unwrap();

    if elements == "no other bags." {
        (node_name, vec![])
    } else {
        let elements = elements.strip_suffix('.').unwrap();
        let elements = elements.split(',');

        let elements: Vec<(u32, &str)> = elements
            .map(|ele| {
                let ele = ele.trim_start();
                let (count, end_node) = ele.split_once(' ').unwrap();
                let (end_node, _) = end_node.rsplit_once(' ').unwrap();
                (count.parse::<u32>().unwrap(), end_node)
            })
            .collect();
        (node_name, elements)
    }
}

fn parse_input(input: &str) -> HashMap<&str, Vec<(u32, &str)>> {
    input.lines().map(|line| line_to_record(line)).collect()
}

fn find(input: &HashMap<&str, Vec<(u32, &str)>>) -> usize {
    let mut memo: HashSet<&str> = HashSet::new();

    memo.insert("shiny gold");

    input.keys().filter(|k| go(k, &mut memo, input)).count() - 1
}

fn go<'a>(
    bag: &'a str,
    memo: &mut HashSet<&'a str>,
    input: &HashMap<&'a str, Vec<(u32, &'a str)>>,
) -> bool {
    if memo.contains(bag) {
        true
    } else {
        let elements = input.get(bag).unwrap();
        let found = elements
            .iter()
            .any(|&(_count, end_bag)| go(end_bag, memo, input));

        if found {
            memo.insert(bag);
        }
        found
    }
}

#[test]
fn test_line_to_record() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

    assert_eq!(
        line_to_record(input),
        ("light red", vec![(1, "bright white"), (2, "muted yellow")])
    )
}

#[test]
fn test_parse_input() {
    let input = include_str!("test.txt");

    let output = parse_input(input);

    assert_eq!(output.len(), 9);

    let bright_white = output.get("bright white").unwrap();

    assert_eq!(bright_white[0], (1, "shiny gold"));

    let faded_blue = output.get("faded blue").unwrap();

    assert!(faded_blue.is_empty());
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    let input = parse_input(input);

    let answer = find(&input);

    assert_eq!(answer, 4);
}
