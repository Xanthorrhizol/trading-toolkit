use super::MovingAverage;
use crate::types::{
    data::{BaseData, Stock},
    error::ToolkitError,
};

#[derive(Debug)]
pub enum Channel {
    Envelope(Band),
    Bollinger(Band),
}

#[derive(Debug, Clone)]
pub struct Band {
    pub upper: f64,
    pub mid: f64,
    pub lower: f64,
}

impl Channel {
    pub fn inner(&self) -> Band {
        match self {
            Channel::Envelope(band) | Channel::Bollinger(band) => band.clone(),
        }
    }
    pub fn envelope<T>(data: &Vec<T>, coefficient: f64) -> Self
    where
        T: BaseData + Clone,
    {
        let ema = MovingAverage::exponential(&data).inner();
        Self::Envelope(Band {
            upper: ema * (1f64 + coefficient),
            mid: ema,
            lower: ema * (1f64 - coefficient),
        })
    }
    pub fn envelope_from(ema: MovingAverage, coefficient: f64) -> Self {
        let ema = ema.inner();
        Self::Envelope(Band {
            upper: ema * (1f64 + coefficient),
            mid: ema,
            lower: ema * (1f64 - coefficient),
        })
    }

    pub fn bollinger<T>(
        data: &Vec<T>,
        dev_mul: f64,
        exponential: bool,
    ) -> Result<Self, ToolkitError>
    where
        T: Stock + BaseData + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut data = data.clone().to_owned();
        data.sort_by_key(|k| Stock::epoch_time(k));
        let mut sum = 0f64;
        let mut variation = 0f64;

        for elem in data.iter() {
            sum += (elem.open_price() + elem.high_price() + elem.low_price()) / 3f64;
        }
        let mean = sum / (data.len() as f64);
        for elem in data.iter() {
            variation +=
                (mean - (elem.open_price() + elem.high_price() + elem.low_price()) / 3f64).powi(2);
        }
        let stdev = variation.sqrt();

        let mid = if exponential {
            MovingAverage::exponential(&data).inner()
        } else {
            mean
        };
        let upper = mid + dev_mul * stdev;
        let lower = mid - dev_mul * stdev;

        Ok(Self::Bollinger(Band { upper, mid, lower }))
    }
}
