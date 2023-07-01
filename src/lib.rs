pub mod reverse_string {
    use unicode_segmentation::UnicodeSegmentation;

    /// Reverses a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_exercise::reverse_string::reverse;
    /// let result = reverse("uüu");
    /// assert_eq!(result, "uüu");
    /// ```

    pub fn reverse(input: &str) -> String {
        input.graphemes(true).rev().collect()
    }

    #[allow(dead_code)]
    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&reverse(input), expected)
    }

    #[test]
    fn test_empty_string() {
        process_reverse_case("", "");
    }

    #[test]
    fn test_word_1() {
        process_reverse_case("qwer", "rewq");
    }
}
