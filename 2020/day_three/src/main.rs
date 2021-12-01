use std::ops::Index;

fn main() {
    let input = include_str!("input.txt");
    let tm = TobaggonMap::from(input);

    let runs: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let count = runs
        .into_iter()
        .map(|(across, down)| tm.tree_count(down, across))
        .reduce(|a, b| a * b)
        .unwrap();

    println!("There are {} trees", count)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MapPoint {
    TREE,
    CLEAR,
}

struct Row {
    width: usize,
    characters: Vec<MapPoint>,
}

impl Row {
    fn from(line: &str) -> Row {
        let characters: Vec<MapPoint> = line
            .chars()
            .map(|character| match character {
                '.' => MapPoint::CLEAR,
                '#' => MapPoint::TREE,
                _ => MapPoint::CLEAR,
            })
            .collect();

        Row {
            width: characters.len(),
            characters,
        }
    }
}

struct TobaggonMap {
    data: Vec<Row>,
}

impl Index<usize> for Row {
    type Output = MapPoint;

    fn index(&self, index: usize) -> &Self::Output {
        let index = index % self.width;

        &self.characters[index]
    }
}

impl Index<(usize, usize)> for TobaggonMap {
    type Output = MapPoint;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl TobaggonMap {
    fn from(lines: &str) -> TobaggonMap {
        let data: Vec<Row> = lines.lines().map(|line| Row::from(line)).collect();

        TobaggonMap { data }
    }

    fn tree_count(&self, down: usize, across: usize) -> usize {
        let x_iter = (0usize..).step_by(across);
        let y_iter = (0..self.data.len()).step_by(down);

        x_iter
            .zip(y_iter)
            .map(|(x, y)| self[(y, x)])
            .filter(|&item| item == MapPoint::TREE)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_is_parsed() {
        let row = Row::from("..##.......");

        assert!(row[0] == MapPoint::CLEAR);
        assert!(row[2] == MapPoint::TREE);
        assert!(row[14] == MapPoint::TREE);
        assert!(row[15] == MapPoint::CLEAR);
    }

    #[test]
    fn parse_map() {
        let input = include_str!("test.txt");

        let parsed = TobaggonMap::from(input);

        assert!(parsed[(0, 0)] == MapPoint::CLEAR);
        assert!(parsed[(1, 4)] == MapPoint::TREE);
    }

    #[test]
    fn count_map_trees() {
        let input = include_str!("test.txt");

        let parsed = TobaggonMap::from(input);

        assert_eq!(parsed.tree_count(1, 3), 7);

        assert_eq!(parsed.tree_count(1, 1), 2);

        assert_eq!(parsed.tree_count(1, 5), 3);

        assert_eq!(parsed.tree_count(1, 7), 4);

        assert_eq!(parsed.tree_count(2, 1), 2);
    }
}
