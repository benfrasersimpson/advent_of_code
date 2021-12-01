fn main() {
    println!("Hello, world!");

    let input = Seat::from_string(include_str!("input.txt"));

    let updated = update_world_2(input);

    let occupied_seats = count_occupied(&updated);

    println!("Occupied: {}", occupied_seats);
}

fn update_world(mut world: Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    loop {
        let updated = Seat::update_neighbours(&world);

        if world == updated {
            break updated;
        } else {
            world = updated;
        }
    }
}

fn update_world_2(mut world: Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    loop {
        let updated = Seat::update_neighbours_2(&world);

        if world == updated {
            break updated;
        } else {
            world = updated;
        }
    }
}

fn seat_occupied(x: i8, y: i8, seat_plan: &Vec<Vec<Seat>>) -> u8 {
    let seat = seat_plan
        .get(y as usize)
        .and_then(|col| col.get(x as usize));

    match seat {
        Some(&Seat::Occupied) => 1,
        _ => 0,
    }
}

fn count_occupied(seat_plan: &Vec<Vec<Seat>>) -> usize {
    seat_plan
        .into_iter()
        .flatten()
        .filter(|&&seat| seat == Seat::Occupied)
        .count()
}

static NEIGHBOURS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(PartialEq, Debug, Clone, Copy)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl Seat {
    fn from_char(input: char) -> Seat {
        match input {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            _ => {
                println!("Unreachable code {}", input);
                unreachable!()
            }
        }
    }

    fn from_line(line: &str) -> Vec<Seat> {
        line.chars()
            .map(|c| Seat::from_char(c))
            .collect::<Vec<Seat>>()
    }

    fn from_string(lines: &str) -> Vec<Vec<Seat>> {
        lines
            .lines()
            .map(|line| Seat::from_line(line))
            .collect::<Vec<Vec<Seat>>>()
    }

    fn update_neighbours(seat_plan: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
        let mut result = seat_plan.clone();

        for y in 0..result.len() {
            for x in 0..result[0].len() {
                let seat = seat_plan[y][x];
                if seat == Seat::Empty
                    && Seat::occupied_neighbour_count(seat_plan, Coordinate { x, y }) == 0
                {
                    result[y][x] = Seat::Occupied;
                } else if seat == Seat::Occupied
                    && Seat::occupied_neighbour_count(seat_plan, Coordinate { x, y }) >= 4
                {
                    result[y][x] = Seat::Empty
                }
            }
        }

        result
    }

    fn update_neighbours_2(seat_plan: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
        let mut result = seat_plan.clone();

        for y in 0..result.len() {
            for x in 0..result[0].len() {
                let seat = seat_plan[y][x];
                if seat == Seat::Empty
                    && Seat::occupied_visible_neighbour_count(seat_plan, Coordinate { x, y }) == 0
                {
                    result[y][x] = Seat::Occupied;
                } else if seat == Seat::Occupied
                    && Seat::occupied_visible_neighbour_count(seat_plan, Coordinate { x, y }) >= 5
                {
                    result[y][x] = Seat::Empty
                }
            }
        }

        result
    }
    fn occupied_neighbour_count(seat_plan: &Vec<Vec<Seat>>, coordinate: Coordinate) -> u8 {
        let (x, y) = (coordinate.x as i8, coordinate.y as i8);

        NEIGHBOURS
            .iter()
            .map(|(nx, ny)| seat_occupied(x + nx, y + ny, seat_plan))
            .sum()
    }

    fn occupied_visible_neighbour_count(seat_plan: &Vec<Vec<Seat>>, coordinate: Coordinate) -> u8 {
        let (x, y) = (coordinate.x as i8, coordinate.y as i8);
        NEIGHBOURS
            .iter()
            .map(|(x_dir, y_dir)| {
                let mut next_x = x + x_dir;
                let mut next_y = y + y_dir;

                loop {
                    let seat = seat_plan
                        .get(next_y as usize)
                        .and_then(|col| col.get(next_x as usize));
                    match seat {
                        Some(Seat::Occupied) => break (1),
                        None => break (0),
                        _ => {
                            next_x = next_x + x_dir;
                            next_y = next_y + y_dir;
                            continue;
                        }
                    }
                }
            })
            .sum()
    }
}

#[test]
fn test_neightbour_count() {
    let test_input = include_str!("test2.txt");

    let test_input = test_input
        .lines()
        .map(|line| Seat::from_line(line))
        .collect::<Vec<Vec<Seat>>>();

    let count = Seat::occupied_neighbour_count(&test_input, Coordinate { x: 9, y: 0 });

    assert_eq!(count, 2);
}

#[test]
fn test_update_world_from_empty() {
    let initial_state = include_str!("test.txt")
        .lines()
        .map(|line| Seat::from_line(line))
        .collect::<Vec<Vec<Seat>>>();

    let after_one_round = include_str!("test1.txt")
        .lines()
        .map(|line| Seat::from_line(line))
        .collect::<Vec<Vec<Seat>>>();

    let actual = Seat::update_neighbours(&initial_state);

    assert_eq!(actual, after_one_round);
}

#[test]
fn test_update_world_from_non_empty() {
    let initial_state = Seat::from_string(include_str!("test1.txt"));

    let after_one_round = Seat::from_string(include_str!("test2.txt"));

    let actual = Seat::update_neighbours(&initial_state);

    assert_eq!(after_one_round, actual);
}

#[test]
fn test_run_until_stable() {
    let world = Seat::from_string(include_str!("test.txt"));

    let actual = update_world(world);

    let expected = Seat::from_string(include_str!("test_end_state.txt"));

    assert_eq!(expected, actual);
}

#[test]
fn test_occupied_count() {
    let world = Seat::from_string(include_str!("test_end_state.txt"));

    let actual = count_occupied(&world);

    let expected = 37;

    assert_eq!(expected, actual);
}

#[test]
fn test_occupied_visible_neighbour_count_all() {
    let all_occupied_input = Seat::from_string(include_str!("all_occupied_test.txt"));

    let actual =
        Seat::occupied_visible_neighbour_count(&all_occupied_input, Coordinate { x: 3, y: 4 });

    assert_eq!(actual, 8);
}

#[test]
fn test_occupied_visible_neighbour_count_one() {
    let one_occupied_input = Seat::from_string(include_str!("one_seat_occupied.txt"));

    let actual =
        Seat::occupied_visible_neighbour_count(&one_occupied_input, Coordinate { x: 1, y: 1 });

    assert_eq!(actual, 1);
}

#[test]
fn test_occupied_visible_neighbour_count_zero() {
    let none_occupied_input = Seat::from_string(include_str!("none_occupied.txt"));

    let actual =
        Seat::occupied_visible_neighbour_count(&none_occupied_input, Coordinate { x: 3, y: 3 });

    assert_eq!(actual, 0);
}

#[test]
fn test_run_until_stable_2() {
    let world = Seat::from_string(include_str!("round_2_start.txt"));

    let actual = update_world_2(world);

    let expected = Seat::from_string(include_str!("round_2_end.txt"));

    assert_eq!(expected, actual);
}
