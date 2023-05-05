use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::{SystemTime, UNIX_EPOCH};

static SEC_IN_MILLISEC: u128 = 1000u128;
static MINUTE_IN_MILLISEC: u128 = 60 * SEC_IN_MILLISEC;
static HOUR_IN_MILLISEC: u128 = 60 * MINUTE_IN_MILLISEC;
static DAY_IN_MILLISEC: u128 = 24 * HOUR_IN_MILLISEC;

/// Time
/// it's basically a milliseconds value
/// it can mean certain time or duration of time
///
/// example)
/// ```
/// use trading_toolkit::types::time::Time;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let now = Time::now().unwrap();
/// assert_eq!(
///     now.inner(),
///     SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
/// );
///
/// let days = 2;
/// let days_in_time = Time::from_days(days);
/// assert_eq!(days_in_time.inner(), (days as u128) * 24 * 60 * 60 * 1000 as u128);
/// ```
#[derive(Eq, Clone, Ord, PartialOrd)]
pub struct Time(u128);

impl From<u128> for Time {
    fn from(epoch: u128) -> Self {
        Self(epoch)
    }
}

impl Time {
    /// return passed milliseconds as Time
    /// from UNIX_EPOCH(1970-01-01T00:00:00.000Z)
    pub fn now() -> Result<Self, std::time::SystemTimeError> {
        Ok(Self(
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        ))
    }

    /// return duration of days
    pub fn from_days(days: usize) -> Self {
        Self(days as u128 * DAY_IN_MILLISEC)
    }

    /// return duration of hours
    pub fn from_hours(hours: usize) -> Self {
        Self(hours as u128 * HOUR_IN_MILLISEC)
    }

    /// return duration of minutes
    pub fn from_minutes(minutes: usize) -> Self {
        Self(minutes as u128 * MINUTE_IN_MILLISEC)
    }

    /// return duration of seconds
    pub fn from_seconds(seconds: usize) -> Self {
        Self(seconds as u128 * SEC_IN_MILLISEC)
    }

    /// return inner milliseconds value
    pub fn inner(&self) -> u128 {
        self.0
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Copy for Time {}
