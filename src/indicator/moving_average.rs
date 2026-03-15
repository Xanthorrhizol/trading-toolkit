use crate::types::data::BaseData;

#[derive(Debug, Clone, Copy)]
pub enum MovingAverage {
    Simple(f64),
    Exponential(f64),
}

impl MovingAverage {
    /// Simple Moving Average
    pub fn simple<T>(data: &[T]) -> Self
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
    pub fn simple_from<T>(scope: usize, prev: &Self, oldest_data: &T, new_data: &T) -> Self
    where
        T: BaseData,
    {
        let prev = prev.inner();
        let numerator = prev * (scope as f64);
        Self::Simple((numerator - oldest_data.value() + new_data.value()) / (scope as f64))
    }

    /// Exponential Moving Average(EMA)
    pub fn exponential<T>(data: &[T]) -> Self
    where
        T: BaseData + Clone,
    {
        let mut data = data.to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let len = data.len() as f64;
        let k = 2f64 / (len as f64 + 1f64);

        let seed = data.iter().map(|d| d.value()).sum::<f64>() / len as f64;
        let mut result = seed;

        for curr in data.iter() {
            result = curr.value() * k + result * (1f64 - k);
        }
        Self::Exponential(result)
    }

    /// Exponential Moving Average(EMA)
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
