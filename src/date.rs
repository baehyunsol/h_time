use crate::misc::{count_year, count_year_rev, yday_to_mday, is_leap, NORMAL_YEAR_DAYS_REV, LEAP_YEAR_DAYS_REV, self};
use crate::duration::Duration;

/// `month` and `m_day` starts with 1, but `y_day` starts with 0
/// if you want to change the values of it's fields, use methods. DO NOT change them directly.
#[derive(Copy, Clone, Debug)]
pub struct Date {
    absolute_value: i64,
    pub year: i32,
    pub month: i32,
    pub m_day: i32,
    pub y_day: i32,
    pub w_day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,

    /// microseconds
    pub sub_second: i32
}

impl Date {

    /// microseconds since 1970/01/01
    pub fn from_i64(n: i64) -> Date {
        let mut days = n / DAY_TO_MICROSEC;
        let mut sub_day = n % DAY_TO_MICROSEC;
        let w_day = if n < 0 {

            if sub_day < 0 {
                sub_day += DAY_TO_MICROSEC;
                days -= 1;
            }

            (7 + days % 7) as i32 + 3
        }

        else {
            (days as i32 + 3) % 7
        };

        let (year, y_day) = count_year(days as i32);
        let (month, m_day) = yday_to_mday(y_day, is_leap(year));

        let sub_second = (sub_day % 1_000_000) as i32;
        sub_day /= 1_000_000;

        let second = (sub_day % 60) as i32;
        sub_day /= 60;

        let minute = (sub_day % 60) as i32;
        sub_day /= 60;

        let hour = sub_day as i32;

        Date {
            absolute_value: n,
            sub_second, second, minute, hour, w_day, m_day, month, y_day: y_day + 1, year
        }
    }

    /// microseconds since 1970/01/01
    pub fn to_i64(&self) -> i64 {
        self.absolute_value
    }

    pub fn now() -> Date {
        Date::from_i64(misc::now())
    }

    pub fn add_hours(&self, n: i64) -> Date {
        Date::from_i64(self.absolute_value + n * HOUR_TO_MICROSEC)
    }

    pub fn add_days(&self, n: i64) -> Date {
        Date::from_i64(self.absolute_value + n * DAY_TO_MICROSEC)
    }

    pub fn add_weeks(&self, n: i64) -> Date {
        Date::from_i64(self.absolute_value + n * 7 * DAY_TO_MICROSEC)
    }

    pub fn add_hms(&self, h: i32, m: i32, s: i32) -> Date {
        Date::from_i64(self.absolute_value + (h * 3600 + m * 60 + s) as i64 * 1_000_000)
    }

    pub fn set_hms(&self, h: i32, m: i32, s: i32) -> Date {
        assert!(0 <= h && h < 24 && 0 <= m && m < 60 && 0 <= s && s < 60);

        Date::from_i64(
            (
                count_year_rev(self.year) + if is_leap(self.year) {
                    LEAP_YEAR_DAYS_REV[self.month as usize][self.m_day as usize]
                } else {
                    NORMAL_YEAR_DAYS_REV[self.month as usize][self.m_day as usize]
                }
            ) as i64 * DAY_TO_MICROSEC + (
                h * 3600 + m * 60 + s
            ) as i64 * 1_000_000
        )
    }

    pub fn reset_hms(&self) -> Date {
        Date::from_ymd(self.year, self.month, self.m_day)
    }

    pub fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        assert!(month > 0 && month < 13 && day > 0 && day < 32);

        Date::from_i64(
            (
                count_year_rev(year) + if is_leap(year) {
                    LEAP_YEAR_DAYS_REV[month as usize][day as usize]
                } else {
                    NORMAL_YEAR_DAYS_REV[month as usize][day as usize]
                }
            ) as i64 * DAY_TO_MICROSEC
        )
    }

    pub fn duration_since(&self, other: &Date) -> Duration {
        Duration::from_micros(self.absolute_value - other.absolute_value)
    }

}

pub const HOUR_TO_MICROSEC: i64 = 1000 * 1000 * 60 * 60;
pub const DAY_TO_MICROSEC: i64 = 1000 * 1000 * 60 * 60 * 24;

use std::cmp::Ordering;
use std::default::Default;

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.absolute_value.cmp(&other.absolute_value))
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.absolute_value == other.absolute_value
    }
}

impl Default for Date {
    fn default() -> Self {
        Date::from_i64(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Date;

    #[test]
    fn ymd_conversion() {

        for y in 1800..2200 {

            for m in 1..13 {

                for d in 1..29 {
                    let date = Date::from_ymd(y, m, d);
                    assert_eq!(y, date.year);
                    assert_eq!(m, date.month);
                    assert_eq!(d, date.m_day);
                }

            }

        }

    }

    #[test]
    fn from_ymd_test() {
        let dates = vec![
            (1970, 1, 1, 0, 0, 0),
            (1800, 1, 1, 0, 0, 0),
            (1800, 12, 25, 0, 0, 0),
            (1800, 12, 25, 2, 4, 5),
            (1900, 1, 1, 16, 30, 23),
            (2020, 3, 2, 14, 0, 0),
            (2021, 9, 9, 8, 30, 0),
            (99999, 12, 31, 23, 59, 59)
        ];

        for (y, mo, d, h, mi, s) in dates.into_iter() {
            let date_1 = Date::from_ymd(y, mo, d);
            let date_2 = date_1.set_hms(h, mi, s);

            assert_eq!(date_2.year, y);
            assert_eq!(date_2.month, mo);
            assert_eq!(date_2.m_day, d);
            assert_eq!(date_2.hour, h);
            assert_eq!(date_2.minute, mi);
            assert_eq!(date_2.second, s);
            assert_eq!(date_1.add_hms(h, mi, s), date_2);
            assert_eq!(date_2.reset_hms(), date_1);
        }

    }

    #[test]
    fn comparison_test() {
        assert!(Date::from_ymd(1999, 1, 20) < Date::from_ymd(2020, 3, 2));
    }

    #[test]
    fn weekday_test() {
        let today = Date::from_ymd(2023, 2, 5);
        assert_eq!(today.w_day, 6);

        for w in -20000..20000 {
            let another_week = today.add_weeks(w);

            assert_eq!(another_week.w_day, 6);
        }

        assert_eq!(Date::from_ymd(2020, 3, 2).w_day, 0);
        assert_eq!(Date::from_ymd(2022, 3, 31).w_day, 3);
        assert_eq!(Date::from_ymd(2020, 2, 29).w_day, 5);
    }

}