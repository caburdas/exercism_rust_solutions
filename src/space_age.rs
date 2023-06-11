/*
Given an age in seconds, calculate how old someone would be on:
    Mercury: orbital period 0.2408467 Earth years
    Venus: orbital period 0.61519726 Earth years
    Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
    Mars: orbital period 1.8808158 Earth years
    Jupiter: orbital period 11.862615 Earth years
    Saturn: orbital period 29.447498 Earth years
    Uranus: orbital period 84.016846 Earth years
    Neptune: orbital period 164.79132 Earth years

So if you were told someone were 1,000,000,000 seconds old, you should be able to say that they're 31.69 Earth-years old.
If you're wondering why Pluto didn't make the cut, go watch this youtube video.
Some Rust topics you may want to read about while solving this problem:
    Traits, both the From trait and implementing your own traits
    Default method implementations for traits
    Macros, the use of a macro could reduce boilerplate and increase readability for this exercise. For instance, a macro can implement a trait for multiple types at once, though it is fine to implement years_during in the Planet trait itself. A macro could define both the structs and their implementations. Info to get started with macros can be found at:
        The Macros chapter in The Rust Programming Language
        an older version of the Macros chapter with helpful detail
        Rust By Example
 */

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}
pub trait Planet {
    const PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 / (Self::PERIOD * 31557600_f64)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const PERIOD: f64 = 0.2408467;
}
impl Planet for Venus {
    const PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    const PERIOD: f64 = 1_f64;
}
impl Planet for Mars {
    const PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const PERIOD: f64 = 11.862615;
}
impl Planet for Saturn {
    const PERIOD: f64 = 29.447498;
}
impl Planet for Uranus {
    const PERIOD: f64 = 84.016846;
}
impl Planet for Neptune {
    const PERIOD: f64 = 164.79132;
}

#[cfg(test)]
mod space_age_test {
    use crate::space_age::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();
        let delta: f64 = 0.01;
        if diff > delta {
            panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
        }
    }

    #[test]
    fn earth_age() {
        let duration = Duration::from(1_000_000_000);
        assert_in_delta(31.69, Earth::years_during(&duration));
    }

    #[test]
    fn mercury_age() {
        let duration = Duration::from(2_134_835_688);
        assert_in_delta(280.88, Mercury::years_during(&duration));
    }

    #[test]
    fn venus_age() {
        let duration = Duration::from(189_839_836);
        assert_in_delta(9.78, Venus::years_during(&duration));
    }

    #[test]
    fn mars_age() {
        let duration = Duration::from(2_129_871_239);
        assert_in_delta(35.88, Mars::years_during(&duration));
    }

    #[test]
    fn jupiter_age() {
        let duration = Duration::from(901_876_382);
        assert_in_delta(2.41, Jupiter::years_during(&duration));
    }

    #[test]
    fn saturn_age() {
        let duration = Duration::from(2_000_000_000);
        assert_in_delta(2.15, Saturn::years_during(&duration));
    }

    #[test]
    fn uranus_age() {
        let duration = Duration::from(1_210_123_456);
        assert_in_delta(0.46, Uranus::years_during(&duration));
    }

    #[test]
    fn neptune_age() {
        let duration = Duration::from(1_821_023_456);
        assert_in_delta(0.35, Neptune::years_during(&duration));
    }
}
