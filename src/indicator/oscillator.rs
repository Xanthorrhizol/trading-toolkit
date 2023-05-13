use super::trend_following::MovingAverage;
use crate::types::{
    data::{Exec, Stock},
    error::ToolkitError,
};

#[derive(Debug)]
pub struct MovingAverageConvergenceDivergence {
    ema_9: f64,
    ema_12: f64,
    ema_26: f64,
}

impl MovingAverageConvergenceDivergence {
    pub fn new<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: Exec + Clone,
    {
        if data.len() < 26 {
            return Err(ToolkitError::DataNotEnough);
        }
        let data = data.split_at(data.len() - 26).1;
        let ema_26 = MovingAverage::exponential(&data[0..26].to_vec());
        let ema_12 = MovingAverage::exponential(&data[26 - 12..26].to_vec());
        let ema_9 = MovingAverage::exponential(&data[26 - 9..26].to_vec());
        Ok(Self {
            ema_9: ema_9.inner(),
            ema_12: ema_12.inner(),
            ema_26: ema_26.inner(),
        })
    }

    /// fast line / MACD line
    pub fn fast(&self) -> f64 {
        self.ema_12 - self.ema_26
    }

    /// slow line / Signal
    pub fn slow(&self) -> f64 {
        self.ema_9
    }

    /// MACD histogram
    pub fn macd_histogram(&self) -> f64 {
        self.fast() - self.slow()
    }
}

#[derive(Debug)]
pub struct ForceIndex {
    inner: f64,
    epoch_time: u128,
}

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
