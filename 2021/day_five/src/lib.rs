#[cfg(test)]
mod tests {
    use crate::coords_to_line;

    #[test]
    fn test_coords_to_line() {
        let coords = "0,9 -> 5,9";

        let ((x1, y1), (x2, y2)) = coords_to_line(coords);

        assert_eq!(x1, 0);
        assert_eq!(y1, 9);
        assert_eq!(x2, 5);
        assert_eq!(y2, 9);
    }
}

fn coords_to_line(input: &str) -> ((u32, u32),(u32, u32))  {
    let (start, end) = input.split_once(" -> ").unwrap();

    let (x1, y1) = start.split_once(",").map(|(x1, y1)| {
        (
            x1.parse::<u32>().unwrap_or_default(),
            y1.parse::<u32>().unwrap_or_default(),
        )
    }).unwrap();

    let (x2, y2) = end.split_once(",").map(|(x2, y2)| {
        (
            x2.parse::<u32>().unwrap_or_default(),
            y2.parse::<u32>().unwrap_or_default(),
        )
    }).unwrap();

    ((x1,y1),(x2,y2))
}
