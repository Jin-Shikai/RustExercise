#![allow(unused)]

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

pub mod gigasecond {
    use time::PrimitiveDateTime as DateTime;

    // Returns a DateTime one billion seconds after start.
    pub fn after(start: DateTime) -> DateTime {
        use time::ext::NumericalDuration;
        const GIGA_SECOND: i64 = 1_000_000_000;
        start + GIGA_SECOND.seconds()
    }

    /// Create a datetime from the given numeric point in time.
    ///
    /// Panics if any field is invalid.
    ///
    fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
        use time::{Date, Time};

        DateTime::new(
            Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
            Time::from_hms(hour, minute, second).unwrap(),
        )
    }

    #[test]
    fn test_date() {
        let start_date = dt(2011, 4, 25, 0, 0, 0);

        assert_eq!(after(start_date), dt(2043, 1, 1, 1, 46, 40));
    }
}
