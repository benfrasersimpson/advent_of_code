use std::collections::HashSet;

fn main() {
    let seats: HashSet<(u32, u32)> = include_str!("input.txt")
        .lines()
        .map(|line| seat_position(line))
        .collect();

    let max = seats
        .clone()
        .into_iter()
        .map(|(row, col)| seat_to_id(row, col))
        .max()
        .unwrap();

    println!("Max: {}", max);

    let all_seats = (0..=127u32)
        .flat_map(|x| (0..=7u32).map(move |y| (x, y)))
        .collect::<HashSet<(u32, u32)>>();

    let x = all_seats
        .into_iter()
        .filter(|seat| !seats.contains(seat))
        .find(|&(row, column)| {
            seats.contains(&(row, column + 1)) && seats.contains(&(row, column - 1))
        })
        .map(|(r, c)| r * 8 + c)
        .unwrap();

    println!("{}", x);
}

fn seat_to_id(row: u32, col: u32) -> u32 {
    (row * 8) + col
}

fn seat_position(boarding_card: &str) -> (u32, u32) {
    let mut row_high = 127u32;
    let mut row_low = 0u32;
    let mut col_high = 7u32;
    let mut col_low = 0u32;

    boarding_card.chars().for_each(|c| match c {
        'F' => {
            row_high = (row_high + row_low) / 2;
            println!("F: Row high {}, Row low {}", row_high, row_low)
        }
        'B' => {
            row_low = ((row_high + row_low) / 2) + 1;
            println!("B: Row high {}, Row low {}", row_high, row_low)
        }
        'R' => {
            col_low = ((col_high + col_low) / 2) + 1;
            println!("R: Col high {}, Col low {}", col_high, col_low)
        }
        'L' => {
            col_high = (col_high + col_low) / 2;
            println!("L: Col high {}, Col low {}", col_high, col_low)
        }

        _ => panic!(),
    });

    (row_high, col_high)
}

mod tests {
    use super::*;

    #[test]
    fn test_get_boarding_pass() {
        vec![
            ("FBFBBFFRLR", 357),
            ("BFFFBBFRRR", 567),
            ("FFFBBBFRRR", 119),
            ("BBFFBBFRLL", 820),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            let (row, column) = seat_position(input);
            assert_eq!(seat_to_id(row, column), expected);
        });
    }
}
