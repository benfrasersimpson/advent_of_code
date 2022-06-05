use day_four;

fn main() {
    let input = include_str!("../input.txt");

    let answer = day_four::part_one(input);

    match answer {
        None => {
            println!("Error calculating part one")
        }
        Some(i) => {
            println!("Part one is {}", i)
        }
    }
}
