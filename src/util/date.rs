use chrono::{DateTime, TimeZone, Utc, Date};


// workaround for Rust not support lazy constants
fn epoch_2000() -> DateTime<Utc> {
    Utc.ymd(2000, 1, 1).and_hms(0, 0, 0)
}

/// Returns whether a given date combination is valid or not
///
/// A date is considered valid if its an actual calendar, with a range of
/// (0,9999] for year. This caveat is in place to maintain compatibility with
/// the original C# version of PKHeX.Core
///# Example
/// ```
/// use pkhexcore::util::date::is_date_valid;
/// assert_eq!(true, is_date_valid(2000, 1, 1));
/// assert_eq!(false, is_date_valid(0, 1, 1));
/// ```
///
pub fn is_date_valid(year: i32, month: u32, day: u32) -> bool {
    // this is to keep compatibility with the C# PKHeX library, as in .NET the
    // allowed year range is (0,9999]
    if year < 1 || year > 9999 {
        return false;
    }
    Utc.ymd_opt(year, month, day).single() != None
}

/// Returns the difference in seconds since Epoch 2000/1/1
///
/// Calculates the difference in seconds since the Epoch date of Jan 1 2000.
/// This is the lowest valid date for in game events.
///# Example
/// ```
/// use pkhexcore::util::date::get_seconds_since2000;
/// use chrono::{TimeZone, Utc};
/// use std::time::Duration;
///
/// // (2000/1/2) 946771200 - (2000,1,1) 946684800 = 86,400 a full day in
/// // secs
/// assert_eq!(
///     get_seconds_since2000(Utc.ymd(2000, 1, 2)),
///     Duration::from_secs(24 * 3600).as_secs() as i64,
/// );
/// ```
///
pub fn get_seconds_since2000(date: Date<Utc>) -> i64 {
    date.signed_duration_since(epoch_2000().date()).num_seconds()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    struct TestCase {
        year: i32,
        month: u32,
        day: u32,
        expected: bool,
    }

    #[test]
    fn test_valid_dates() {
        let cases = [
            TestCase { year: 2000, month: 1, day: 1, expected: true },
            TestCase { year: 2001, month: 1, day: 31, expected: true },
            TestCase { year: 2016, month: 1, day: 31, expected: true },
            TestCase { year: 2016, month: 2, day: 28, expected: true },
            TestCase { year: 2016, month: 3, day: 31, expected: true },
            TestCase { year: 2016, month: 4, day: 30, expected: true },
            TestCase { year: 2016, month: 5, day: 31, expected: true },
            TestCase { year: 2016, month: 6, day: 30, expected: true },
            TestCase { year: 2016, month: 7, day: 31, expected: true },
            TestCase { year: 2016, month: 8, day: 31, expected: true },
            TestCase { year: 2016, month: 9, day: 30, expected: true },
            TestCase { year: 2016, month: 10, day: 31, expected: true },
            TestCase { year: 2016, month: 11, day: 30, expected: true },
            TestCase { year: 2016, month: 12, day: 31, expected: true },
            TestCase { year: 0, month: 0, day: 0, expected: false },
            TestCase { year: 2000, month: 1, day: 32, expected: false },
            TestCase { year: 2000, month: 13, day: 1, expected: false },
            TestCase { year: 10000, month: 1, day: 1, expected: false },
            TestCase { year: 2000, month: 1, day: 0, expected: false },
            TestCase { year: 2000, month: 0, day: 1, expected: false },
            TestCase { year: 0, month: 1, day: 1, expected: false },
        ];
        for case in cases.iter() {
            assert_eq!(
                case.expected,
                is_date_valid(case.year, case.month, case.day),
                "expected {}/{}/{} to be {}",
                case.year,
                case.month,
                case.day,
                if case.expected { "valid" } else { "invalid" },
            );
        }
    }

    #[test]
    fn test_recognizes_correct_leap_year() {
        let cases = [
            TestCase { year: 2004, month: 2, day: 29, expected: true },
            TestCase { year: 2005, month: 2, day: 29, expected: false },
        ];
        for case in cases.iter() {
            assert_eq!(case.expected, is_date_valid(case.year, case.month, case.day));
        }
    }

    #[test]
    fn test_dates_since_epoch2000() {

        // (2000/1/2) 946771200 - (2000,1,1) 946684800 = 86,400 a full day in
        // secs
        assert_eq!(
            get_seconds_since2000(Utc.ymd(2000, 1, 2)),
            Duration::from_secs(24 * 3600).as_secs() as i64,
        );

        // (2000/2/1) 949363200 - (2000/1/1) 946684800 = 31,622,400 a full month
        // 31 days in jan
        assert_eq!(
            get_seconds_since2000(Utc.ymd(2000, 2, 1)),
            Duration::from_secs(31 * 24 * 3600).as_secs() as i64,
        );

        // (2001/1/1) 978307200 - (2000/1/1) 946684800 = 31,622,400 a full
        // (leap) year in secs
        assert_eq!(
            get_seconds_since2000(Utc.ymd(2001, 1, 1)),
            Duration::from_secs(366 * 24 * 3600).as_secs() as i64,
        );
    }
}
