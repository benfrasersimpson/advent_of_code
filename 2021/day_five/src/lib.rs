use crate::ParsingError::{CoordinateSplit, PointParse, PointSplit};
use std::ops::RangeInclusive;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParsingError {
    #[error("problem parsing value")]
    PointParse(#[source] std::num::ParseIntError),
    #[error("problem splitting point values")]
    PointSplit,
    #[error("problem splitting coordinate values")]
    CoordinateSplit,
}

#[cfg(test)]
mod tests {
    use crate::{Line, part_one, Point};

    #[test]
    fn test_coords_to_line() {
        let coords = "0,9 -> 5,9";

        let result = Line::from(coords);

        assert_eq!(
            result,
            Ok(Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            })
        );
    }

    #[test]
    fn test_is_straight_line() {
        assert!((Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 }
        })
        .is_straight_line());

        assert!(!(Line {
            start: Point { x: 0, y: 10 },
            end: Point { x: 5, y: 8 }
        })
        .is_straight_line());
    }

    #[test]
    fn test_get_line_coords() {
        let line = Line::from("1,1 -> 1,3").unwrap();

        assert_eq!(
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 }
            ],
            line.get_line_coords()
        );
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("test_input.txt");

        let actual = part_one(input);

        assert_eq!(5, actual);
    }
}

#[derive(PartialEq, Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn from(input: &str) -> Result<Line, ParsingError> {
        let (start, end) = input.split_once(" -> ").ok_or(CoordinateSplit)?;

        let start = Point::from(start)?;
        let end = Point::from(end)?;

        Ok(Line { start, end })
    }

    pub fn is_straight_line(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn gen_range(start: u32, end: u32) -> RangeInclusive<u32> {
        if start > end {
            end..=start
        } else {
            start..=end
        }
    }

    pub fn get_line_coords(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            Line::gen_range(self.start.y, self.end.y)
                .map(|y| Point { x: self.start.x, y })
                .collect::<Vec<Point>>()
        } else if self.start.y == self.end.y {
            Line::gen_range(self.start.x, self.end.x)
                .map(|x| Point { x, y: self.start.y })
                .collect::<Vec<Point>>()
        } else {
            vec![]
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    fn from(input: &str) -> Result<Point, ParsingError> {
        let (x, y) = input.split_once(",").ok_or(PointSplit)?;
        let x = x.parse::<u32>().map_err(|e| PointParse(e))?;
        let y = y.parse::<u32>().map_err(|e| PointParse(e))?;

        Ok(Self { x, y })
    }
}

pub fn part_one(input: &str) -> usize {
    let lines = input
        .lines()
        .filter_map(|line| Line::from(line).ok())
        .filter(Line::is_straight_line)
        .map(|line| line.get_line_coords())
        .flatten()
        .collect::<Vec<Point>>();

    let mut state: Vec<[u8; 1000]> = vec![[0u8;1000];1000];

    lines.iter().for_each(|point| {
        state[point.y as usize][point.x as usize] += 1;
    });

    state
        .into_iter()
        .flatten()
        .filter(|&cell| cell > 1)
        .collect::<Vec<u8>>()
        .len()
}
