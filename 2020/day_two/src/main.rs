use std::error::Error;

use std::convert::TryFrom;

use regex::internal::Input;

fn main() {
    let input = include_str!("input.txt");

    let valid_password_count = input
        .lines()
        .filter_map(|line| SledPasswordEntry::from(line).ok())
        .filter(|entry| entry.is_valid())
        .count();

    println!("There are {} valid passwords", valid_password_count);

    let valid_password_count_2 = input
        .lines()
        .filter_map(|line| TobogganPasswordEntr::from(line).ok())
        .filter(|entry| entry.is_valid())
        .count();

    println!(
        "There are {} valid toboggan     passwords",
        valid_password_count_2
    );
}

#[derive(Debug)]
struct SledPasswordEntry {
    range: std::ops::Range<u64>,
    character: char,
    candidate: String,
}

struct TobogganPasswordEntr {
    index_1: usize,
    index_2: usize,
    character: char,
    candidate: String,
}

impl SledPasswordEntry {
    fn from(item: &str) -> Result<Self, Box<dyn Error>> {
        use regex::Regex;

        let re = Regex::new(r"(\d+)-(\d+) ([a-z]{1}): ([a-z]+)").unwrap();

        let cap = re
            .captures(&item)
            .ok_or("Error processing password entry")?;

        let start = cap[1].parse::<u64>()?;
        let end = cap[2].parse::<u64>()?;
        let end = end + 1;
        let range = std::ops::Range { start, end };

        let character = cap[3].parse::<char>()?;
        let candidate = cap[4].to_string();

        Ok(SledPasswordEntry {
            range,
            character,
            candidate,
        })
    }

    fn is_valid(&self) -> bool {
        let count = self.candidate.matches(self.character).count();
        let count = u64::try_from(count).unwrap();

        self.range.contains(&count)
    }
}

impl std::cmp::PartialEq for SledPasswordEntry {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
            && self.character == other.character
            && self.candidate == other.candidate
    }
}

impl TobogganPasswordEntr {
    fn from(item: &str) -> Result<Self, Box<dyn Error>> {
        use regex::Regex;

        let re = Regex::new(r"(\d+)-(\d+) ([a-z]{1}): ([a-z]+)").unwrap();

        let cap = re
            .captures(&item)
            .ok_or("Error processing password entry")?;

        let index_1 = cap[1].parse::<usize>()?;
        let index_1 = index_1 - 1;
        let index_2 = cap[2].parse::<usize>()?;
        let index_2 = index_2 - 1;

        let character = cap[3].parse::<char>()?;
        let candidate = cap[4].to_string();

        Ok(TobogganPasswordEntr {
            index_1,
            index_2,
            character,
            candidate,
        })
    }

    fn is_valid(&self) -> bool {
        (self.candidate.chars().nth(self.index_1).unwrap() == self.character)
            ^ (self.candidate.chars().nth(self.index_2).unwrap() == self.character)
    }
}

impl std::cmp::PartialEq for TobogganPasswordEntr {
    fn eq(&self, other: &Self) -> bool {
        self.index_1 == other.index_1
            && self.index_2 == other.index_2
            && self.character == other.character
            && self.candidate == other.candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SledPasswordEntry;

    #[test]
    fn parse_string_to_password_entry() {
        let input = "1-3 a: abcde";

        let processed = SledPasswordEntry::from(input);

        assert!(processed.is_ok());

        assert_eq!(
            SledPasswordEntry {
                range: (1..4),
                character: 'a',
                candidate: String::from("abcde")
            },
            processed.unwrap()
        );
    }

    #[test]
    fn password_entry_parse_fails_on_invalid_input() {
        let input = "1--3 a: abcde";

        let processed = SledPasswordEntry::from(input);

        assert!(processed.is_err());
    }

    #[test]
    fn password_entry_parse_fails_on_invalid_range() {
        let input = "1-z a: abcde";

        let processed = SledPasswordEntry::from(input);

        assert!(processed.is_err());
    }

    #[test]
    fn check_toboggan_password_entr_is_valid() {
        let parsed = TobogganPasswordEntr::from("1-3 a: abcde");
        assert!(parsed.is_ok());
        assert!(parsed.unwrap().is_valid());
    }

    #[test]
    fn check_toboggan_password_entr_is_not_valid_1() {
        let parsed = TobogganPasswordEntr::from("1-3 b: cdefg");
        assert!(parsed.is_ok());
        assert!(!parsed.unwrap().is_valid());
    }

    #[test]
    fn check_toboggan_password_entr_is_not_valid_2() {
        let parsed = TobogganPasswordEntr::from("2-9 c: ccccccccc");
        assert!(parsed.is_ok());
        assert!(!parsed.unwrap().is_valid());
    }
}
