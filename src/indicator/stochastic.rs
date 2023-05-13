use super::MovingAverage;
use crate::types::{
    data::{BaseData, Stock},
    error::ToolkitError,
};

#[derive(Debug, Clone)]
pub enum Stochastic {
    Fast(f64, u128),
    Slow(f64, u128),
}

impl Stochastic {
    pub fn fast<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: Stock + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let last_close_price = data.last().unwrap().close_price(); // it's safe since the vector's length > 0
        let mut max_high_price = 0f64;
        let mut min_low_price = std::f64::MAX;
        let last_epoch_time = data.last().unwrap().epoch_time(); // it's safe since the vector's length > 0
        for elem in data.iter() {
            max_high_price = max_high_price.max(elem.high_price());
            min_low_price = min_low_price.min(elem.low_price());
        }
        Ok(Self::Fast(
            (last_close_price - min_low_price) / (max_high_price - min_low_price) * 100f64,
            last_epoch_time,
        ))
    }

    pub fn slow<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: Stock + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let mut sum_numerator = 0f64;
        let mut sum_denominator = 0f64;
        let last_epoch_time = data.last().unwrap().epoch_time(); // it's safe since the vector's length > 0
        for elem in data.iter() {
            sum_numerator += elem.close_price() - elem.low_price();
            sum_denominator += elem.high_price() - elem.low_price();
        }
        Ok(Self::Slow(
            (sum_numerator / sum_denominator) * 100f64,
            last_epoch_time,
        ))
    }

    pub fn inner(&self) -> f64 {
        match self {
            Self::Fast(f, _epoch_time) | Self::Slow(f, _epoch_time) => f.to_owned(),
        }
    }

    pub fn into_slow(data: &Vec<Self>) -> Result<Self, ToolkitError> {
        if data
            .iter()
            .filter(|elem| match elem {
                Self::Slow(_slow, _epoch_time) => true,
                _ => false,
            })
            .count()
            > 0
        {
            return Err(ToolkitError::InvalidData);
        }
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let last_epoch_time = data.last().unwrap().epoch_time(); // it's safe since the vector's length > 0
        Ok(Self::Slow(
            MovingAverage::simple(&data).inner(),
            last_epoch_time,
        ))
    }
}

impl BaseData for Stochastic {
    fn value(&self) -> f64 {
        match self {
            Self::Fast(f, _epoch_time) | Self::Slow(f, _epoch_time) => f.to_owned(),
        }
    }

    fn weight(&self) -> u64 {
        1
    }

    fn epoch_time(&self) -> u128 {
        match self {
            Self::Fast(_f, epoch_time) | Self::Slow(_f, epoch_time) => epoch_time.to_owned(),
        }
    }
}
