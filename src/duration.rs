use crate::date::{HOUR_TO_MICROSEC, DAY_TO_MICROSEC};

pub struct Duration {
    value: i64
}

impl Duration {

    #[inline]
    pub fn from_micros(value: i64) -> Self {
        Duration { value }
    }

    #[inline]
    pub fn into_micros(&self) -> i64 {
        self.value
    }

    #[inline]
    pub fn into_millis(&self) -> i64 {
        self.value / 1000
    }

    #[inline]
    pub fn into_secs(&self) -> i64 {
        self.value / 1_000_000
    }

    #[inline]
    pub fn subsecs(&self) -> i64 {
        self.value % 1_000_000
    }

    #[inline]
    pub fn into_minutes(&self) -> i64 {
        self.value / 60_0000_000
    }

    #[inline]
    pub fn into_hours(&self) -> i64 {
        self.value / HOUR_TO_MICROSEC
    }

    #[inline]
    pub fn into_days(&self) -> i64 {
        self.value / DAY_TO_MICROSEC
    }

    #[inline]
    pub fn subday_micros(&self) -> i64 {
        self.value % DAY_TO_MICROSEC
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Duration::from_micros(self.value.abs())
    }
}