use crate::types::data::Exec;

#[derive(Debug)]
pub enum MovingAverage {
    Simple(f64),
    Exponential(f64),
}

impl MovingAverage {
    /// Simple Moving Average
    pub fn simple<T>(data: &Vec<T>) -> Self
    where
        T: Exec,
    {
        let mut sum = 0f64;
        let mut count = 0f64;
        for elem in data.iter() {
            sum += elem.price();
            count += 1f64;
        }
        Self::Simple(sum / count)
    }

    /// Exponential Moving Average(EMA)
    pub fn exponential<T>(data: &Vec<T>) -> Self
    where
        T: Exec + Clone,
    {
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let mut first = true;
        let mut result = 0f64;
        let mut i = 1f64;
        for curr in data.iter() {
            let k = 2f64 / (i + 1f64);
            if first {
                result = curr.price();
                first = false;
            }
            result = curr.price() * k + result * (1f64 - k);
            i += 1f64;
        }
        Self::Exponential(result)
    }

    pub fn inner(&self) -> f64 {
        match self {
            Self::Simple(f) | Self::Exponential(f) => f.clone(),
        }
    }
}
