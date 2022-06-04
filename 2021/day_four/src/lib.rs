use bitvec::vec::BitVec;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn has_won_winning_board() {
        let called: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        let expected = Board {
            numbers: vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        };
        assert!(expected.has_won(&called))
    }

    #[test]
    fn has_won_losing_board() {
        let called: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21];
        let expected = Board {
            numbers: vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        };
        assert!(!expected.has_won(&called))
    }

    #[test]
    fn new_board_from_str() {
        let input = "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        let expected = Board {
            numbers: vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        };

        assert_eq!(expected, Board::new(input))
    }
}

#[derive(PartialEq, Debug)]
struct Board {
    numbers: Vec<Vec<u8>>,
}

impl Board {
    fn has_won(&self, called: &[u8]) -> bool {
        self.numbers
            .iter()
            .any(|row| row.iter().all(|cell| called.contains(cell)))
    }

    fn new(input: &str) -> Board {
        let output = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<u8>().unwrap_or_default())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        Board { numbers: output }
    }
}
