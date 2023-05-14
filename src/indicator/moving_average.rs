use crate::types::data::BaseData;

#[derive(Debug)]
pub enum MovingAverage {
    Simple(f64),
    Exponential(f64),
}

impl MovingAverage {
    /// Simple Moving Average
    pub fn simple<T>(data: &Vec<T>) -> Self
    where
        T: BaseData,
    {
        let mut sum = 0f64;
        let mut count = 0f64;
        for elem in data.iter() {
            sum += elem.value();
            count += 1f64;
        }
        Self::Simple(sum / count)
    }

    /// Simple Moving Average from previous data
    pub fn simple_from<T>(scope: usize, prev: &Self, new_data: &T) -> Self
    where
        T: BaseData,
    {
        let prev = prev.inner();
        let numerator = prev * (scope as f64);
        Self::Simple((numerator - prev + new_data.value()) / (scope as f64))
    }

    /// Exponential Moving Average(EMA)
    // TODO: is it really right?
    pub fn exponential<T>(data: &Vec<T>) -> Self
    where
        T: BaseData + Clone,
    {
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let mut first = true;
        let mut result = 0f64;
        let mut i = 1f64;
        for curr in data.iter() {
            let k = 2f64 / (i + 1f64);
            if first {
                result = curr.value();
                first = false;
            }
            result = curr.value() * k + result * (1f64 - k);
            i += 1f64;
        }
        Self::Exponential(result)
    }

    /// Exponential Moving Average(EMA)
    // TODO: is it really right?
    pub fn exponential_from<T>(scope: usize, prev: &Self, new_data: &T) -> Self
    where
        T: BaseData + Clone,
    {
        let k = 2f64 / ((scope + 1) as f64);
        Self::Exponential(new_data.value() * k + prev.inner() * (1f64 - k))
    }

    pub fn inner(&self) -> f64 {
        match self {
            Self::Simple(f) | Self::Exponential(f) => f.clone(),
        }
    }
}
