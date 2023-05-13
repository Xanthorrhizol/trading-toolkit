use crate::types::data::{Exec, Stock};

#[derive(Debug)]
pub struct ForceIndex {
    inner: f64,
    epoch_time: u128,
}

// make Force Index usable for MovingAverage
impl Exec for ForceIndex {
    fn price(&self) -> f64 {
        self.inner
    }

    fn volume(&self) -> u64 {
        1
    }

    fn epoch_time(&self) -> u128 {
        self.epoch_time
    }
}

impl ForceIndex {
    pub fn new<T>(prev: &T, curr: &T) -> Self
    where
        T: Stock,
    {
        Self {
            inner: (curr.close_price() - prev.close_price()) * (curr.tot_exec_volume() as f64),
            epoch_time: curr.epoch_time(),
        }
    }

    pub fn inner(&self) -> f64 {
        self.inner
    }
}
