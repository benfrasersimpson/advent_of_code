mod tests {

    #[test]
    fn part_one() {
        let input = include_str!("test_input.txt");

        let answer = day_four::part_one(input);

        assert_eq!(answer, Some(4512));
    }

    #[test]
    fn part_two() {
        let input = include_str!("test_input.txt");

        let answer = day_four::part_two(input);

        assert_eq!(answer, Some(1924));
    }
}
