use std::ops::{Add, AddAssign, Sub, SubAssign};

static SEC_IN_MILLISEC: u64 = 1000u64;
static MINUTE_IN_MILLISEC: u64 = 60 * SEC_IN_MILLISEC;
static HOUR_IN_MILLISEC: u64 = 60 * MINUTE_IN_MILLISEC;
static DAY_IN_MILLISEC: u64 = 24 * HOUR_IN_MILLISEC;

#[derive(Eq, Clone)]
pub struct Time(u64);

impl From<u64> for Time {
    fn from(epoch: u64) -> Self {
        Self(epoch)
    }
}

impl Time {
    pub fn from_days(days: u64) -> Self {
        Time(days * DAY_IN_MILLISEC)
    }
    pub fn inner(&self) -> u64 {
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
