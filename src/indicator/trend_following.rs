use super::data::Data;

/// 이동평균
pub enum MovingAverage {
    Simple(f64),
    Exponential(f64),
}

impl MovingAverage {
    pub fn simple<T>(data: &Vec<T>) -> Self
    where
        T: Data,
    {
        let mut sum = 0f64;
        let mut count = 0f64;
        for elem in data.iter() {
            sum += elem.price();
            count += 1f64;
        }
        Self::Simple(sum / count)
    }

    pub fn exponential<T>(data: &Vec<T>) -> Self
    where
        T: Data + Ord + Clone,
    {
        let mut data = data.clone().to_vec();
        let k = 2f64 / (data.len() as f64 + 1f64);
        data.sort();
        let mut first = true;
        let mut result = 0f64;
        for curr in data.iter() {
            if first {
                result = curr.price();
                first = false;
            }
            result = curr.price() * k + result * (1f64 - k);
        }
        Self::Exponential(1f64)
    }
}
