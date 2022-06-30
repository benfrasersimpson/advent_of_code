use day_five::{Line, Point};

fn main() {
    let input = include_str!("input.txt");

    let answer_one = day_five::part_one(input);

    println!("Part one answer is {}", answer_one);

    let answer_two = day_five::part_two(input);

    println!("Part two answer is {}", answer_two);
}
