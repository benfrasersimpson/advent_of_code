use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let input = include_str!("input.txt");

    let input = input
        .split("\n\n")
        .map(|entry| parse_entry(entry))
        .filter(|passport| validate_passport_fields(passport))
        .count();

    println!("There are {} valid entries", input);
}

fn count_valid_entries(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|entry| parse_entry(entry))
        .filter(|x| entry_has_required_keys(&x))
        .count()
}

fn parse_entry(input: &str) -> HashMap<&str, &str> {
    input
        .split_ascii_whitespace()
        .filter_map(|key_value| key_value.split_once(':'))
        .collect::<HashMap<_, _>>()
}

fn entry_has_required_keys(passport: &HashMap<&str, &str>) -> bool {
    static REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    println!("Start");
    passport.keys().for_each(|f| print!("{}, ", f));
    println!("Ends");

    println!("{:?}", REQUIRED_KEYS);
    let is_valid = REQUIRED_KEYS.iter().all(|k| {
        println!("{}", k);
        passport.contains_key(k)
    });
    println!("Valid: {}", is_valid);
    is_valid
}

fn validate_passport_fields(passport: &HashMap<&str, &str>) -> bool {
    entry_has_required_keys(passport)
        && passport.iter().all(|(&k, v)| match k {
            "byr" => valid_birthyear(v),
            "iyr" => valid_iyr(v),
            "eyr" => valid_eyr(v),
            "hgt" => validate_height(v),
            "hcl" => validate_hair_colour(v),
            "ecl" => valid_ecl(v),
            "pid" => valid_pid(v),
            "cid" => true,
            _ => false,
        })
}

fn valid_birthyear(byr: &str) -> bool {
    let valid = (1920..=2002).contains(&byr.parse::<u32>().unwrap_or_default());
    println!("VAlid Birthyear: {}", valid);
    valid
}

fn valid_iyr(iyr: &str) -> bool {
    let valid = (2010..=2020).contains(&iyr.parse::<u32>().unwrap_or_default());
    println!("VAlid iyr: {}", valid);
    valid
}

fn valid_eyr(eyr: &str) -> bool {
    let valid = (2020..=2030).contains(&eyr.parse::<u32>().unwrap_or_default());
    println!("valid eyr {}", valid);
    valid
}

fn valid_ecl(ecl: &str) -> bool {
    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl);
    println!("Valid ecl: {}", valid);
    valid
}

fn valid_pid(pid: &str) -> bool {
    let valid = pid.len() == 9 && pid.chars().all(|x| x.is_ascii_digit());
    println!("valid pid: {}", valid);
    valid
}

fn validate_height(height: &str) -> bool {
    let height_value = &height[..height.len() - 2].parse().unwrap_or_default();

    let is_valid = match &height[height.len() - 2..] {
        "cm" => (150..=193).contains(height_value),
        "in" => (59..=76).contains(height_value),
        _ => false,
    };

    println!("Valid height {}", is_valid);

    is_valid
}

fn validate_hair_colour(hair_colour: &str) -> bool {
    println!("Hair: {}", hair_colour);
    let is_valid = hair_colour.len() == 7
        && hair_colour.starts_with('#')
        && hair_colour.chars().skip(1).all(|x| x.is_ascii_hexdigit());

    println!("Valid hair: {}", is_valid);

    is_valid
}

mod tests {
    use super::*;
    #[test]
    fn test_entry_is_valid() {
        let input = include_str!("test.txt");

        assert_eq!(count_valid_entries(input), 2)
    }

    #[test]
    fn test_entry_has_required_keys() {
        let passport = parse_entry(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        );

        assert!(entry_has_required_keys(&passport));
    }

    #[test]
    fn test_valid_passports() {
        let count = include_str!("valid_test_cases.txt")
            .split("\n\n")
            .map(|entry| parse_entry(entry))
            .filter(|passport| validate_passport_fields(passport))
            .count();

        assert_eq!(count, 4)
    }

    #[test]
    fn test_invalid_passports() {
        let count = include_str!("invalid_test_cases.txt")
            .split("\n\n")
            .map(|entry| parse_entry(entry))
            .filter(|passport| validate_passport_fields(passport))
            .count();

        assert_eq!(count, 0);
    }
}
