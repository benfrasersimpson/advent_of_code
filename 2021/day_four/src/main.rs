use day_four;

fn main() {
    let input = include_str!("../input.txt");

    let answer = day_four::part_two(input);

    match answer {
        None => {
            println!("Error calculating part two")
        }
        Some(i) => {
            println!("Part one is {}", i)
        }
    }
}
