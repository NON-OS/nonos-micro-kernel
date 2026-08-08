// NONOS Operating System (AGPL-3.0-or-later)
//! Calendar arithmetic, checked against known instants and for round-trip.
//!
//! The CMOS clock hands over a date and this converts to seconds; a PL031
//! hands over seconds and this converts to a date. Both directions ship, so
//! both are checked, and a disagreement between them would put the wall clock
//! wrong on exactly one architecture.

use crate::civil::days::{days_in_month, is_leap_year};
use crate::civil::time::{from_unix, to_unix, CivilTime};

fn at(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> CivilTime {
    CivilTime { year, month, day, hour, minute, second }
}

#[test]
fn epoch_is_the_first_of_january_1970() {
    assert_eq!(from_unix(0), at(1970, 1, 1, 0, 0, 0));
    assert_eq!(to_unix(&at(1970, 1, 1, 0, 0, 0)), 0);
}

/// A leap day the Gregorian century rule keeps: 2000 is divisible by 400.
#[test]
fn leap_day_2000() {
    assert_eq!(from_unix(951_782_400), at(2000, 2, 29, 0, 0, 0));
}

/// A century that is not a leap year, where a naive divisible-by-four rule
/// would be a day out for the rest of the century.
#[test]
fn nineteen_hundred_was_not_a_leap_year() {
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert_eq!(days_in_month(1900, 2), 28);
    assert_eq!(days_in_month(2000, 2), 29);
}

/// The instant the 32-bit second count a PL031 reports rolls over. Kept as a
/// known value so a change to the conversion cannot quietly move it.
#[test]
fn thirty_two_bit_rollover_lands_in_2106() {
    assert_eq!(from_unix(u32::MAX as u64), at(2106, 2, 7, 6, 28, 15));
}

#[test]
fn round_trips_across_four_decades() {
    // Every 100_000 seconds is a little over a day, so this walks 1970 to
    // about 2010 hitting every month and both sides of every leap day.
    let mut seconds = 0u64;
    while seconds < 1_300_000_000 {
        assert_eq!(to_unix(&from_unix(seconds)), seconds, "round trip at {seconds}");
        seconds += 100_000;
    }
}

#[test]
fn seconds_minutes_and_hours_carry() {
    assert_eq!(from_unix(59), at(1970, 1, 1, 0, 0, 59));
    assert_eq!(from_unix(60), at(1970, 1, 1, 0, 1, 0));
    assert_eq!(from_unix(3599), at(1970, 1, 1, 0, 59, 59));
    assert_eq!(from_unix(3600), at(1970, 1, 1, 1, 0, 0));
    assert_eq!(from_unix(86_399), at(1970, 1, 1, 23, 59, 59));
    assert_eq!(from_unix(86_400), at(1970, 1, 2, 0, 0, 0));
}
