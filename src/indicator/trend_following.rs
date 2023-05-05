use crate::types::data::Data;

/// 이동평균
#[derive(Debug)]
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
        T: Data + Clone,
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

#[cfg(test)]
mod tests {
    use super::MovingAverage;
    use crate::types::{data::Data, time::Time};

    #[derive(Clone)]
    struct MAData {
        price: f64,
        epoch_time: Time,
    }
    impl MAData {
        fn new(price: f64, epoch_time: Time) -> Self {
            Self { price, epoch_time }
        }
    }
    impl Data for MAData {
        fn price(&self) -> f64 {
            self.price
        }
        fn epoch_time(&self) -> u128 {
            self.epoch_time.inner()
        }
    }
    #[test]
    fn test_moving_average() {
        let now = Time::now().unwrap();
        let raw_data = vec![
            (1100.0, now - Time::from_days(7u128)),
            (1000.0, now - Time::from_days(6u128)),
            (1200.0, now - Time::from_days(5u128)),
            (1150.0, now - Time::from_days(4u128)),
            (1200.0, now - Time::from_days(3u128)),
            (1000.0, now - Time::from_days(2u128)),
            (900.0, now - Time::from_days(1u128)),
        ];
        let data = {
            let mut result = vec![];
            for elem in raw_data.iter() {
                result.push(MAData::new(elem.0, elem.1));
            }
            result
        };
        assert!((MovingAverage::simple(&data).inner() - 1078.5714285714287).abs() < 0.0000000001);
        assert!(
            (MovingAverage::exponential(&data).inner() - 1057.1428571428573).abs() < 0.0000000001
        );
    }
}
