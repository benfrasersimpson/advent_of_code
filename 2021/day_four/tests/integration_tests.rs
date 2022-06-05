mod tests {

    #[test]
    fn end_to_end() {
        let input = include_str!("test_input.txt");

        let answer = day_four::part_one(input);

        assert_eq!(answer, Some(4512));
    }
}
