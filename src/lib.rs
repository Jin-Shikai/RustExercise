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

pub mod clock {
    #[derive(Debug, PartialEq, Eq)]
    pub struct Clock {
        hours: i32,
        minutes: i32,
    }

    impl Clock {
        pub fn new(hours: i32, minutes: i32) -> Self {
            let mut my_hour = hours % 24;
            if my_hour < 0 {
                my_hour = 24 + my_hour;
            }
            let carry = minutes / 60;
            my_hour = (my_hour + carry) % 24;
            let mut my_minutes = minutes % 60;
            if my_minutes < 0 {
                my_hour = my_hour - 1;
                if my_hour < 0 {
                    my_hour = 24 + my_hour;
                }
                my_minutes = 60 + my_minutes;
            }
            if my_hour < 0 {
                my_hour = 24 + my_hour;
            }
            Self {
                hours: my_hour % 24,
                minutes: my_minutes % 60,
            }
        }

        pub fn add_minutes(&self, minutes: i32) -> Self {
            let minutes = self.minutes + minutes;
            let carry = minutes / 60;
            let mut my_minutes = minutes % 60;
            let mut my_hour = (self.hours + carry) % 24;
            if my_minutes < 0 {
                my_hour = my_hour - 1;
                if my_hour < 0 {
                    my_hour = 24 + my_hour;
                }
                my_minutes = 60 + my_minutes;
            }
            if my_hour < 0 {
                my_hour = 24 + my_hour;
            }
            Self {
                hours: my_hour,
                minutes: my_minutes,
            }
        }

        pub fn to_string(&self) -> String {
            format!("{:0>2}:{:0>2}", self.hours, self.minutes)
        }

        #[cfg(test)]
        fn test_on_the_hour() {
            assert_eq!(Clock::new(8, 0).to_string(), "08:00");
        }

        #[cfg(test)]
        fn test_past_the_hour() {
            assert_eq!(Clock::new(11, 9).to_string(), "11:09");
        }
    }
}

pub mod anagram {
    use std::collections::HashSet;

    pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
        let mut res: HashSet<&'a str> = HashSet::new();
        let low_case_word = word.to_lowercase();
        let mut sorted_word: Vec<char> = low_case_word.chars().collect();
        sorted_word.sort_unstable();
        for s in possible_anagrams {
            let low_case_s = s.to_lowercase();
            if low_case_s == low_case_word {
                continue;
            }
            let mut sorted_s: Vec<char> = low_case_s.chars().collect();
            sorted_s.sort_unstable();
            if sorted_s == sorted_word {
                res.insert(s);
            }
        }
        res
    }

    fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
        let result = anagrams_for(word, inputs);

        let expected: HashSet<&str> = expected.iter().cloned().collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_anagrams() {
        let word = "allergy";

        let inputs = [
            "gallery",
            "ballerina",
            "regally",
            "clergy",
            "largely",
            "leading",
        ];

        let outputs = vec!["gallery", "regally", "largely"];

        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn test_case_insensitive_anagrams() {
        let word = "Orchestra";

        let inputs = ["cashregister", "Carthorse", "radishes"];

        let outputs = vec!["Carthorse"];

        process_anagram_case(word, &inputs, &outputs);
    }
}
