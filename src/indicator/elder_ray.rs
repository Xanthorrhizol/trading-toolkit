use super::MovingAverage;
use crate::types::{
    data::{BaseData, Stock},
    error::ToolkitError,
};

#[derive(Debug, Clone, Copy)]
pub struct ElderRay {
    /// seller's force
    ask_force: f64,
    /// buyer's force
    bid_force: f64,
}

impl ElderRay {
    pub fn new<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: Stock + BaseData + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut sorted = data.clone();
        sorted.sort_by_key(|k| Stock::epoch_time(k));

        let ema = MovingAverage::exponential(data).inner();
        let last = sorted.last().ok_or(ToolkitError::EmptyData)?;

        Ok(Self {
            ask_force: last.low_price() - ema,
            bid_force: last.high_price() - ema,
        })
    }

    pub fn ask_force(&self) -> f64 {
        self.ask_force
    }

    pub fn bid_force(&self) -> f64 {
        self.bid_force
    }
}
