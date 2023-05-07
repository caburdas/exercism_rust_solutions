/*
Your task is to determine the date and time one gigasecond after a certain date.
A gigasecond is one thousand million seconds. That is a one with nine zeros after it.
If you were born on January 24th, 2015 at 22:00 (10:00:00pm), then you would be a gigasecond old on October 2nd, 2046 at 23:46:40 (11:46:40pm).
If you're unsure what operations you can perform on PrimitiveDateTime take a look at the time crate which is listed as a dependency in the Cargo.toml file for this exercise.
 */
#[warn(dead_code)]
mod gigasecond {
    use time::{Duration, PrimitiveDateTime as DateTime};
    // Returns a DateTime one billion seconds after start.
    pub fn after(start: DateTime) -> DateTime {
        start + Duration::new(1000_000_000, 0)
    }
}

#[cfg(test)]
mod gigasecond_test {

    use time::PrimitiveDateTime as DateTime;

    use crate::gigasecond::*;
    /// Create a datetime from the given numeric point in time.

    ///

    /// Panics if any field is invalid.

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

        assert_eq!(gigasecond::after(start_date), dt(2043, 1, 1, 1, 46, 40));
    }

    #[test]

    fn test_another_date() {
        let start_date = dt(1977, 6, 13, 0, 0, 0);

        assert_eq!(gigasecond::after(start_date), dt(2009, 2, 19, 1, 46, 40));
    }

    #[test]

    fn test_third_date() {
        let start_date = dt(1959, 7, 19, 0, 0, 0);

        assert_eq!(gigasecond::after(start_date), dt(1991, 3, 27, 1, 46, 40));
    }

    #[test]

    fn test_datetime() {
        let start_date = dt(2015, 1, 24, 22, 0, 0);

        assert_eq!(gigasecond::after(start_date), dt(2046, 10, 2, 23, 46, 40));
    }

    #[test]

    fn test_another_datetime() {
        let start_date = dt(2015, 1, 24, 23, 59, 59);

        assert_eq!(gigasecond::after(start_date), dt(2046, 10, 3, 1, 46, 39));
    }
}
