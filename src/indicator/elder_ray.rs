use super::MovingAverage;
use crate::types::{data::Exec, error::ToolkitError};

#[derive(Debug)]
pub struct ElderRay {
    /// seller's force
    ask_force: f64,
    /// buyer's force
    bid_force: f64,
}

impl ElderRay {
    pub fn new<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: Exec + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut high_price = 0f64;
        let mut low_price = std::f64::MAX;
        let ema = MovingAverage::exponential(data).inner();
        for elem in data.iter() {
            high_price = high_price.max(elem.price());
            low_price = low_price.min(elem.price());
        }

        Ok(Self {
            ask_force: low_price - ema,
            bid_force: high_price - ema,
        })
    }

    pub fn ask_force(&self) -> f64 {
        self.ask_force
    }

    pub fn bid_force(&self) -> f64 {
        self.bid_force
    }
}
