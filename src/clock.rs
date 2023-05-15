/*
Instructions
Implement a clock that handles times without dates.
You should be able to add and subtract minutes to it.
Two clocks that represent the same time should be equal to each other.
You will also need to implement .to_string() for the Clock struct. We will be using this to display the Clock's state. You can either do it via implementing it directly or using the Display trait.
Did you implement .to_string() for the Clock struct?
If so, try implementing the Display trait for Clock instead.
Traits allow for a common way to implement functionality for various types.
For additional learning, consider how you might implement String::from for the Clock type. You don't have to actually implement this—it's redundant with Display, which is generally the better choice when the destination type is String—but it's useful to have a few type-conversion traits in your toolkit.
*/
// use std::fmt;
// #[derive(PartialEq,Debug)]
// pub struct Clock{
//     h :i32,
//     m :i32,
// }

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:0>2}:{:0>2}", self.h, self.m)
//     }
// }

// fn check_hour(hours: i32) -> i32{
//     if hours >= 0 && hours < 24 {
//         hours
//     }else if hours >= 24 {
//         hours % 24
//     }else if hours <0 && hours > -24 {
//         24 + hours
//     }else{
//         if hours % 24 == 0
//         {
//             0
//         } else{
//             24 + (hours % 24)
//         }
//     }
// }

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         //unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");

//         let mut h:i32;
//         let mut m = 0;
//         h = check_hour(hours);

//         if minutes >= 0 && minutes < 60 {
//             m = minutes;
//         }else if minutes >= 60{
//             m = minutes % 60;
//             h += minutes / 60;
//             h = check_hour(h);
//         }else if minutes < 0 && minutes >= -60 {
//             m = 60 + minutes;
//             h-=1;
//             h = check_hour(h);
//         }else if minutes < -60{
//             if  minutes % 60 == 0{
//                 m = 0;
//                 h += minutes / 60;
//             }else{
//                 m = 60 + ( minutes % 60);
//                 h += -1 + minutes / 60;
//             }
//             h = check_hour(h);
//         }

//         Clock{h, m}
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {

//         let mut h:i32 = self.h;
//         let mut m = 0;
//         let m2 = self.m + minutes;
//         if m2 >= 0 && m2 < 60 {
//             m = m2;
//         }else if m2 >= 60{
//             m = m2 % 60;
//             h += m2 / 60;
//             h = check_hour(h);
//         }else if m2 < 0 && m2 >= -60 {
//             m = 60 + m2;
//             h-=1;
//             h = check_hour(h);
//         }else if m2 < -60{
//             if  m2 % 60 == 0{
//                 m = 0;
//                 h += m2 / 60;
//             }else{
//                 m = 60 + ( m2 % 60);
//                 h += -1 + (m2 / 60);
//             }
//             h = check_hour(h);
//         }
//         Clock {h, m}
//     }
// }

use std::fmt;
const MIN_PER_DAY: i64 = 24 * 60;
const MIN_PER_HOUR: i64 = 60;
#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        let minutes =
            (((hours * MIN_PER_HOUR + minutes) % MIN_PER_DAY) + MIN_PER_DAY) % MIN_PER_DAY;
        //                                                 |             |
        //  convert all to minutes per day >>>>>>>>>>>>>>>>|             |
        //                                                               |
        //   handles negative values >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>|

        Clock { minutes: minutes }
    }

    pub fn add_minutes(self, minutes: i64) -> Clock {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MIN_PER_HOUR,
            self.minutes % MIN_PER_HOUR
        )
    }
}

#[cfg(test)]
mod clock_tests {
    use super::Clock;
    //
    // Clock Creation
    //
    #[test]
    fn test_on_the_hour() {
        assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    }

    #[test]
    fn test_past_the_hour() {
        assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    }
    #[test]
    fn test_midnight_is_zero_hours() {
        assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    }

    #[test]
    fn test_hour_rolls_over() {
        assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    }

    #[test]
    fn test_hour_rolls_over_continuously() {
        assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    }

    #[test]
    fn test_sixty_minutes_is_next_hour() {
        assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }
    #[test]
    fn test_minutes_roll_over() {
        assert_eq!(Clock::new(0, 160).to_string(), "02:40");
    }

    #[test]
    fn test_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
    }

    #[test]
    fn test_hours_and_minutes_roll_over() {
        assert_eq!(Clock::new(25, 160).to_string(), "03:40");
    }

    #[test]
    fn test_hours_and_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    }

    #[test]
    fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
        assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
    }
    #[test]
    fn test_negative_hour() {
        assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    }
    #[test]
    fn test_negative_hour_roll_over() {
        assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
    }

    #[test]
    fn test_negative_hour_roll_over_continuously() {
        assert_eq!(Clock::new(-91, 00).to_string(), "05:00");
    }

    #[test]
    fn test_negative_minutes() {
        assert_eq!(Clock::new(1, -40).to_string(), "00:20");
    }

    #[test]
    fn test_negative_minutes_roll_over() {
        assert_eq!(Clock::new(1, -160).to_string(), "22:20");
    }

    #[test]
    fn test_negative_minutes_roll_over_continuously() {
        assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
    }

    #[test]
    fn test_negative_sixty_minutes_is_prev_hour() {
        assert_eq!(Clock::new(2, -60).to_string(), "01:00");
    }

    #[test]
    fn test_negative_one_twenty_minutes_is_two_prev_hours() {
        assert_eq!(Clock::new(1, -120).to_string(), "23:00");
    }

    #[test]
    fn test_negative_hour_and_minutes_both_roll_over() {
        assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
    }

    #[test]
    fn test_negative_hour_and_minutes_both_roll_over_continuously() {
        assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
    }

    #[test]
    fn test_zero_hour_and_negative_minutes() {
        assert_eq!(Clock::new(0, -22).to_string(), "23:38");
    }

    //
    // Clock Math
    //
    #[test]
    fn test_add_minutes() {
        let clock = Clock::new(10, 0).add_minutes(3);
        assert_eq!(clock.to_string(), "10:03");
    }

    #[test]
    fn test_add_no_minutes() {
        let clock = Clock::new(6, 41).add_minutes(0);
        assert_eq!(clock.to_string(), "06:41");
    }

    #[test]
    fn test_add_to_next_hour() {
        let clock = Clock::new(0, 45).add_minutes(40);
        assert_eq!(clock.to_string(), "01:25");
    }

    #[test]
    fn test_add_more_than_one_hour() {
        let clock = Clock::new(10, 0).add_minutes(61);
        assert_eq!(clock.to_string(), "11:01");
    }

    #[test]
    fn test_add_more_than_two_hours_with_carry() {
        let clock = Clock::new(0, 45).add_minutes(160);
        assert_eq!(clock.to_string(), "03:25");
    }

    #[test]
    fn test_add_across_midnight() {
        let clock = Clock::new(23, 59).add_minutes(2);
        assert_eq!(clock.to_string(), "00:01");
    }

    #[test]
    fn test_add_more_than_one_day() {
        let clock = Clock::new(5, 32).add_minutes(1500);
        assert_eq!(clock.to_string(), "06:32");
    }

    #[test]
    fn test_add_more_than_two_days() {
        let clock = Clock::new(1, 1).add_minutes(3500);
        assert_eq!(clock.to_string(), "11:21");
    }

    #[test]
    fn test_subtract_minutes() {
        let clock = Clock::new(10, 3).add_minutes(-3);
        assert_eq!(clock.to_string(), "10:00");
    }

    #[test]
    fn test_subtract_to_previous_hour() {
        let clock = Clock::new(10, 3).add_minutes(-30);
        assert_eq!(clock.to_string(), "09:33");
    }

    #[test]
    fn test_subtract_more_than_an_hour() {
        let clock = Clock::new(10, 3).add_minutes(-70);
        assert_eq!(clock.to_string(), "08:53");
    }

    #[test]
    fn test_subtract_across_midnight() {
        let clock = Clock::new(0, 3).add_minutes(-4);
        assert_eq!(clock.to_string(), "23:59");
    }

    #[test]
    fn test_subtract_more_than_two_hours() {
        let clock = Clock::new(0, 0).add_minutes(-160);
        assert_eq!(clock.to_string(), "21:20");
    }

    #[test]
    fn test_subtract_more_than_two_hours_with_borrow() {
        let clock = Clock::new(6, 15).add_minutes(-160);
        assert_eq!(clock.to_string(), "03:35");
    }

    #[test]
    fn test_subtract_more_than_one_day() {
        let clock = Clock::new(5, 32).add_minutes(-1500);
        assert_eq!(clock.to_string(), "04:32");
    }

    #[test]
    fn test_subtract_more_than_two_days() {
        let clock = Clock::new(2, 20).add_minutes(-3000);
        assert_eq!(clock.to_string(), "00:20");
    }
    //
    // Test Equality
    //
    #[test]
    fn test_compare_clocks_for_equality() {
        assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
    }

    #[test]
    fn test_compare_clocks_a_minute_apart() {
        assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
    }

    #[test]
    fn test_compare_clocks_an_hour_apart() {
        assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
    }

    #[test]
    fn test_compare_clocks_with_hour_overflow() {
        assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
    }

    #[test]
    fn test_compare_clocks_with_hour_overflow_by_several_days() {
        assert_eq!(Clock::new(99, 11), Clock::new(3, 11));
    }

    #[test]
    fn test_compare_clocks_with_negative_hour() {
        assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
    }

    #[test]
    fn test_compare_clocks_with_negative_hour_that_wraps() {
        assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
    }

    #[test]
    fn test_compare_clocks_with_negative_hour_that_wraps_multiple_times() {
        assert_eq!(Clock::new(-83, 49), Clock::new(13, 49));
    }

    #[test]
    fn test_compare_clocks_with_minutes_overflow() {
        assert_eq!(Clock::new(0, 1441), Clock::new(0, 1));
    }

    #[test]
    fn test_compare_clocks_with_minutes_overflow_by_several_days() {
        assert_eq!(Clock::new(2, 4322), Clock::new(2, 2));
    }

    #[test]
    fn test_compare_clocks_with_negative_minute() {
        assert_eq!(Clock::new(3, -20), Clock::new(2, 40));
    }

    #[test]
    fn test_compare_clocks_with_negative_minute_that_wraps() {
        assert_eq!(Clock::new(5, -1490), Clock::new(4, 10));
    }

    #[test]
    fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
        assert_eq!(Clock::new(6, -4305), Clock::new(6, 15));
    }

    #[test]
    fn test_compare_clocks_with_negative_hours_and_minutes() {
        assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
    }

    #[test]
    fn test_compare_clocks_with_negative_hours_and_minutes_that_wrap() {
        assert_eq!(Clock::new(-54, -11_513), Clock::new(18, 7));
    }

    #[test]
    fn test_compare_full_clock_and_zeroed_clock() {
        assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
    }
}
