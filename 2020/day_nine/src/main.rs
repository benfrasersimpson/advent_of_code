fn main() {
    println!("Hello, world!");

    let input = include_str!("input.txt");

    let input = strings_to_ints(input).unwrap();

    let missing_num = find_missing_number(25, &input);

    match missing_num {
        Some(x) => println!("missing num: {}", x),
        None => println!("No result found"),
    }

    let answer = missing_num.unwrap();

    let contiguous_range = find_contiguous_numbers(&input, answer);

    let encryption_weakness = find_encryption_weakness(contiguous_range);

    println!("Enc weakness: {}", encryption_weakness);
}

fn strings_to_ints(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    input.lines().map(|line| line.parse::<u64>()).collect()
}

fn find_missing_number(preamable_length: usize, input: &[u64]) -> Option<u64> {
    for i in 0..(input.len() - preamable_length - 1) {
        if !pair_exists(
            &input[i..i + preamable_length + 1],
            input[i + preamable_length + 1],
        ) {
            println!("nums: {:?}", &input[i..i + preamable_length]);
            println!("target: {}", input[i + preamable_length + 1]);
            return Some(input[i + preamable_length + 1]);
        }
    }

    return None;
}

fn pair_exists(nums: &[u64], target: u64) -> bool {
    let x = nums
        .iter()
        .enumerate()
        .flat_map(|(i, a)| nums[i + 1..].iter().map(move |b| a + b))
        .collect::<Vec<u64>>();

    x.contains(&target)
}

fn find_contiguous_numbers(nums: &[u64], target: u64) -> &[u64] {
    let mut lead = 0;
    let mut lag = 0;
    let mut sum = 0;

    while sum != target {
        if sum < target {
            sum += nums[lead];
            lead += 1;
        } else {
            sum -= nums[lag];
            lag += 1;
        }
    }

    &nums[lag..lead]
}

fn find_encryption_weakness(contiguous_range: &[u64]) -> u64 {
    let min = contiguous_range.iter().min().unwrap();
    let max = contiguous_range.iter().max().unwrap();

    min + max
}

#[test]
fn test_strings_to_int_happy_case() {
    let input = "1\n2\n3\n";

    assert_eq!(strings_to_ints(input), Ok(vec![1, 2, 3]));
}

#[test]
fn test_strings_to_int_unhappy_case() {
    let input = "1\na\n3\n";

    assert!(strings_to_ints(input).is_err());
}

#[test]
fn test_find_missing_number() {
    let input = include_str!("test.txt");
    let input = strings_to_ints(input).unwrap();

    let answer = find_missing_number(5, &input);

    assert_eq!(answer, Some(127));
}

#[test]
fn test_find_enc_weakness() {
    let input = include_str!("test.txt");
    let input = strings_to_ints(input).unwrap();

    let answer = find_missing_number(5, &input).unwrap();

    let contiguous_range = find_contiguous_numbers(&input, answer);

    println!("Cont Range {:?}", contiguous_range);
    let enc_weaknesses = find_encryption_weakness(contiguous_range);

    assert_eq!(62, enc_weaknesses);
}
