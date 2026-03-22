use super::MovingAverage;
use crate::types::{
    data::{BaseData, Candle},
    error::ToolkitError,
};

#[derive(Debug, Clone, Copy)]
pub enum Channel {
    Envelope(Band),
    Bollinger(Band),
}

#[derive(Debug, Clone, Copy)]
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
    pub fn envelope<T>(data: &[T], coefficient: f64) -> Self
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

    pub fn bollinger<T>(data: &[T], dev_mul: f64, exponential: bool) -> Result<Self, ToolkitError>
    where
        T: Candle + BaseData + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut data = data.to_owned();
        data.sort_by_key(|k| Candle::epoch_time(k));
        let mut sum = 0f64;
        for elem in data.iter() {
            sum += elem.close_price();
        }
        let mean = sum / (data.len() as f64);

        let mid = if exponential {
            MovingAverage::exponential(&data).inner()
        } else {
            mean
        };

        let mut variation = 0f64;
        for elem in data.iter() {
            variation += (mid - elem.close_price()).powi(2);
        }
        let stdev = (variation / data.len() as f64).sqrt();
        let upper = mid + dev_mul * stdev;
        let lower = mid - dev_mul * stdev;

        Ok(Self::Bollinger(Band { upper, mid, lower }))
    }
}
