fn main() {
    let input = include_str!("input.txt").lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let (gamma_rate, epsilon_rate) = calculate_rates(&input);

    println!("The answer to part one is {}", gamma_rate * epsilon_rate);

    let (oxygen_rating, co2_rating) = calculate_life_support_ratings(&input);

    println!("Tthe answer to part two is {}", oxygen_rating * co2_rating);
}

fn calculate_rates(input: &Vec<String>) -> (u32, u32) {
    let halfway = (input.len() / 2) as u32;

    let vec_capacity = input[0].len();

    let my_counts: Vec<u32> = input.iter().fold(vec![0u32;vec_capacity],
                                   |mut counts, line| {
        line.chars().into_iter().enumerate().for_each(|(idx, value)| counts[idx]    += value.to_digit(10).unwrap());
        counts
        },);

    let gamma_rate = my_counts
        .into_iter()
        .map(|x| if x < halfway { "0" } else { "1" })
        .collect::<String>();

    let epsilon_rate: String = gamma_rate.chars().into_iter().map(|c| if c == '0' { "1" } else {"0"}).collect();


    let gamma_rate = u32::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = u32::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    (gamma_rate, epsilon_rate)
}

fn calculate_life_support_ratings(input: &Vec<String>) -> (u32, u32) {

    let mut oxygen_rating: String = "".to_string();
    let mut co2_rating: String = "".to_string();

    for i in 0..input[0].chars().count() {
        oxygen_rating.push(most_common_character(input, i, '1', oxygen_rating.as_str()));
    }

    for i in 0..input[0].chars().count() {
        co2_rating.push(least_common_character(input, i, '0', co2_rating.as_str()));
    }

    println!("Oxy rating: {}", oxygen_rating);

    println!("CO2 {}", co2_rating);

    let oxygen_rating = u32::from_str_radix(oxygen_rating.as_str(), 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_rating.as_str(), 2).unwrap();

    (oxygen_rating, co2_rating)

}

fn most_common_character(input: &Vec<String>, index: usize, equally_common_tiebreak: char, prefix: &str) -> char {
    println!("Index is {}", index);
    println!("Prefix is {}", prefix);

    let filter: Vec<&String> = input.into_iter().filter(|line| line.starts_with(prefix)).inspect(|x| println!("{}", x)).collect();

    if filter.len() == 1 {
        return filter[0].chars().nth(index).unwrap()
    }

    let (zero_count, one_count) = filter.iter().fold((0,0), |(mut zero, mut one), line|{
        if line.chars().nth(index).unwrap() == '0' {zero += 1} else { one += 1 }
        (zero, one)
    },);

    println!("Zero count is {}", zero_count);

    let to_return = if zero_count > one_count {
        '0'
    } else if zero_count < one_count {
        '1'
    } else {
        equally_common_tiebreak
    };

    println!("Ttoreturn is {}", to_return);

    to_return
}


fn least_common_character(input: &Vec<String>, index: usize, equally_common_tiebreak: char, prefix: &str) -> char {
    println!("Index is {}", index);
    println!("Prefix is {}", prefix);

    let filter: Vec<&String> = input.into_iter().filter(|line| line.starts_with(prefix)).inspect(|x| println!("{}", x)).collect();

    if filter.len() == 1 {
        return filter[0].chars().nth(index).unwrap()
    }

    let (zero_count, one_count) = filter.iter().fold((0,0), |(mut zero, mut one), line|{
        if line.chars().nth(index).unwrap() == '0' {zero += 1} else { one += 1 }
        (zero, one)
    },);

    println!("Zero count is {}", zero_count);

    let to_return = if zero_count > one_count {
        '1'
    } else if zero_count < one_count {
        '0'
    } else {
        equally_common_tiebreak
    };

    println!("Ttoreturn is {}", to_return);

    to_return
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt").lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let (gamma_rate, epsilon_rate) = calculate_rates(input);

    assert_eq!(gamma_rate, 22);

    assert_eq!(epsilon_rate, 9);
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt").lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let (oxygen, co2) = calculate_life_support_ratings(&input);

    println!("Oxygen {}, co2: {}", oxygen, co2);

    assert_eq!(oxygen, 23);
    assert_eq!(co2, 10);
}
