use super::MovingAverage;
use crate::types::{data::BaseData, error::ToolkitError};

#[derive(Debug, Clone, Copy)]
pub struct MovingAverageConvergenceDivergence {
    ema_9: f64,
    ema_12: f64,
    ema_26: f64,
}

impl MovingAverageConvergenceDivergence {
    pub fn new<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: BaseData + Clone,
    {
        if data.len() < 26 {
            return Err(ToolkitError::DataNotEnough);
        }
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        data = data.split_at(data.len() - 26).1.to_vec();
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
