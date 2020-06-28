use chrono::{Date, DateTime, TimeZone, Utc};
use lazy_static::lazy_static;

lazy_static! {
    static ref EPOCH_2000: DateTime<Utc> = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
}

pub const SECONDS_PER_DAY: u32 = 60 * 60 * 24; // 86400

/// Returns whether a given date combination is valid or not
///
/// A date is considered valid if it's an actual calendar, with a range of
/// (0,9999] for year.
///
///# Example
/// ```
/// use pkhexcore::util::dateutil::is_date_valid;
/// assert_eq!(true, is_date_valid(2000, 1, 1));
/// assert_eq!(false, is_date_valid(0, 1, 1));
/// ```
///
pub fn is_date_valid(year: u32, month: u32, day: u32) -> bool {
    if year < 1 || year > 9999 {
        return false;
    }

    match Utc.ymd_opt(year as i32, month, day).single() {
        None => false,
        Some(_) => true,
    }
}

/// Returns the difference in seconds since Epoch 2000/1/1
///
/// Calculates the difference in seconds since the Epoch date of Jan 1 2000.
/// This is the lowest valid date for in game events.
///
///# Example
/// ```
/// use pkhexcore::util::dateutil::get_seconds_since2000;
/// use pkhexcore::util::dateutil::SECONDS_PER_DAY;
/// use chrono::{TimeZone, Utc};
///
/// // (2000/1/2)  (2000,1,1)
/// // 946771200 - 946684800 = 86,400 a full day in secs
/// assert_eq!(SECONDS_PER_DAY as u64, get_seconds_since2000(Utc.ymd(2000, 1, 2)));
/// ```
///
pub fn get_seconds_since2000(date: Date<Utc>) -> u64 {
    date.signed_duration_since(EPOCH_2000.date()).num_seconds() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn recognizes_correct_dates() {
        assert_eq!(true, is_date_valid(2000, 1, 1));
        assert_eq!(true, is_date_valid(2001, 1, 31));
    }

    #[test]
    fn recognizes_valid_month_boundaries() {
        assert_eq!(true, is_date_valid(2016, 1, 31));
        assert_eq!(true, is_date_valid(2016, 2, 28));
        assert_eq!(true, is_date_valid(2016, 3, 31));
        assert_eq!(true, is_date_valid(2016, 4, 30));
        assert_eq!(true, is_date_valid(2016, 5, 31));
        assert_eq!(true, is_date_valid(2016, 6, 30));
        assert_eq!(true, is_date_valid(2016, 7, 31));
        assert_eq!(true, is_date_valid(2016, 8, 31));
        assert_eq!(true, is_date_valid(2016, 9, 30));
        assert_eq!(true, is_date_valid(2016, 10, 31));
        assert_eq!(true, is_date_valid(2016, 11, 30));
        assert_eq!(true, is_date_valid(2016, 12, 31));
    }

    #[test]
    fn recognizes_correct_leap_year_test() {
        assert_eq!(true, is_date_valid(2004, 2, 29));
        assert_eq!(false, is_date_valid(2005, 2, 29));
    }

    #[test]
    fn fails_with_zero_date() {
        assert_eq!(false, is_date_valid(0, 0, 0));
    }

    #[test]
    fn fails_with_large_day() {
        assert_eq!(false, is_date_valid(2000, 1, 32));
    }

    #[test]
    fn fails_with_large_month() {
        assert_eq!(false, is_date_valid(2000, 13, 1));
    }

    #[test]
    fn fails_with_large_year() {
        assert_eq!(false, is_date_valid(10000, 1, 1));
    }

    #[test]
    fn fails_with_zero_day() {
        assert_eq!(false, is_date_valid(2000, 1, 0));
    }

    #[test]
    fn fails_with_zero_month() {
        assert_eq!(false, is_date_valid(2000, 0, 1));
    }

    #[test]
    fn fails_with_zero_year() {
        assert_eq!(false, is_date_valid(0, 1, 1));
    }

    #[test]
    fn dates_since_epoch2000_test() {
        // (2000/1/2)  (2000,1,1)
        // 946771200 - 946684800 = 86,400 a full day in secs
        assert_eq!(
            SECONDS_PER_DAY as u64,
            get_seconds_since2000(Utc.ymd(2000, 1, 2))
        );

        // (2000/2/1)  (2000/1/1)
        // 949363200 - 946684800 = 31,622,400 a full month 31 days in Jan
        assert_eq!(2678400, get_seconds_since2000(Utc.ymd(2000, 2, 1)));

        // (2001/1/1)  (2000/1/1)
        // 978307200 - 946684800 = 31,622,400 a full (leap) year in secs
        assert_eq!(31622400, get_seconds_since2000(Utc.ymd(2001, 1, 1)));
    }
}
