fn main() {
    let input = lines_to_i32_vec(include_str!("input.txt"));

    println!(
        "There are {} greater than 0 with a window of 2",
        window_sum_larger_than_0(&input, 2)
    );

    println!(
        "There are {} greater than 0 with a window of 3",
        window_sum_larger_than_previous(&input, 3)
    );
}

fn window_sum_larger_than_0(input: &Vec<i32>, windows_size: usize) -> usize {
    input
        .windows(windows_size)
        .map(|x| x[1] - x[0])
        .filter(|&x| x > 0)
        .count()
}

fn window_sum_larger_than_previous(input: &Vec<i32>, windows_size: usize) -> usize {
    input
        .windows(windows_size)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count()
}

fn lines_to_i32_vec(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[test]
fn test_window_2() {
    let input = lines_to_i32_vec(include_str!("test.txt"));

    let result = window_sum_larger_than_0(&input, 2);

    assert_eq!(result, 7);
}

#[test]
fn test_window_3() {
    let input = lines_to_i32_vec(include_str!("test.txt"));

    let result = window_sum_larger_than_previous(&input, 3);

    assert_eq!(result, 5);
}
