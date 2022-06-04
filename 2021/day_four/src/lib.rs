#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn has_won_winning_board_row() {
        let called: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        let expected = Board {
            numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            board_size: 5,
        };
        assert!(expected.has_won(&called))
    }

    #[test]
    fn has_won_winning_board_column() {
        let called: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 16, 6, 8];
        let expected = Board {
            numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            board_size: 5,
        };
        assert!(expected.has_won(&called))
    }

    #[test]
    fn has_won_losing_board() {
        let called: Vec<u8> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21];
        let expected = Board {
            numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            board_size: 5,
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
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            board_size: 5,
        };

        assert_eq!(expected, Board::new(input))
    }
}

#[derive(PartialEq, Debug)]
struct Board {
    numbers: Vec<u8>,
    board_size: usize,
}

impl Board {
    fn has_won(&self, called: &[u8]) -> bool {
        let rows = self.numbers
            .chunks(self.board_size)
            .any(|row| row.iter().all(|cell| called.contains(cell)));

        let columns = (0..self.board_size).any(|offset| {
            self.numbers.iter().skip(offset).step_by(self.board_size).all(|x| called.contains(x))
        });


        rows || columns
    }

    fn new(input: &str) -> Board {
        let output = input
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap_or_default())
            .collect::<Vec<u8>>();

        Board {
            numbers: output,
            board_size: 5,
        }
    }
}
