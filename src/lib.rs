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

    // impl PartialEq for Clock {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.hours == other.hours && self.minutes == other.minutes
    //     }
    // }
}
