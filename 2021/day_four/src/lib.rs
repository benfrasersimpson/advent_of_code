#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn has_won_winning_board_row() {
        let called: Vec<u32> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
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
        let called: Vec<u32> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 16, 6, 8];
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
        let called: Vec<u32> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21];
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

    #[test]
    fn uncalled_numbers() {
        let called: Vec<u32> = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        let board = Board {
            numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            board_size: 5,
        };

        let expected_uncalled = vec![10, 16, 15, 19, 18, 8, 26, 20, 22, 13, 6, 12, 3];

        assert_eq!(board.uncalled_numbers(&called), expected_uncalled)
    }
}

#[derive(PartialEq, Debug)]
struct Board {
    numbers: Vec<u32>,
    board_size: usize,
}

impl Board {
    fn has_won(&self, called: &[u32]) -> bool {
        let rows = self
            .numbers
            .chunks(self.board_size)
            .any(|row| row.iter().all(|cell| called.contains(cell)));

        let columns = (0..self.board_size).any(|offset| {
            self.numbers
                .iter()
                .skip(offset)
                .step_by(self.board_size)
                .all(|x| called.contains(x))
        });

        rows || columns
    }

    fn new(input: &str) -> Board {
        let output = input
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap_or_default())
            .collect::<Vec<u32>>();

        Board {
            numbers: output,
            board_size: 5,
        }
    }

    fn uncalled_numbers(&self, called: &[u32]) -> Vec<u32> {
        let mut uncalled = self.numbers.clone();
        uncalled.retain(|x| !called.contains(x));
        uncalled
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (called, boards) = input.split_once("\n\n").unwrap();

    let called = called
        .split(',')
        .map(|x| x.parse::<u32>().unwrap_or_default())
        .collect::<Vec<u32>>();

    let boards = boards
        .split("\n\n")
        .map(|board| Board::new(board))
        .collect::<Vec<Board>>();

    (1..called.len()).find_map(|i| {
        boards
            .iter()
            .find(|board| board.has_won(&called[0..=i]))
            .map(|board| {
                board
                    .uncalled_numbers(&called[0..=i])
                    .into_iter()
                    .sum::<u32>()
                    * called[i]
            })
    })
}
