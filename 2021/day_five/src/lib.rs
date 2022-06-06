use crate::ParsingError::{CoordinateSplit, PointParse, PointSplit};
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
    use crate::{Line, Point};

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
}

#[derive(PartialEq, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from(input: &str) -> Result<Line, ParsingError> {
        let (start, end) = input.split_once(" -> ").ok_or(CoordinateSplit)?;

        let start = Point::from(start)?;
        let end = Point::from(end)?;

        Ok(Line { start, end })
    }

    fn is_straight_line(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn get_line_coords(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            (self.start.y..=self.end.y)
                .map(|y| Point { x: self.start.x, y })
                .collect::<Vec<Point>>()
        } else if self.start.y == self.end.y {
            (self.start.x..=self.end.x)
                .map(|x| Point { x, y: self.start.y })
                .collect::<Vec<Point>>()
        } else {
            vec![]
        }
    }
}

#[derive(PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn from(input: &str) -> Result<Point, ParsingError> {
        let (x, y) = input.split_once(",").ok_or(PointSplit)?;
        let x = x.parse::<u32>().map_err(|e| PointParse(e))?;
        let y = y.parse::<u32>().map_err(|e| PointParse(e))?;

        Ok(Self { x, y })
    }
}
