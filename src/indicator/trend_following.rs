use crate::types::{
    data::{Exec, Stock},
    error::ToolkitError,
};

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
        T: Exec + Clone,
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
        T: Stock + Clone,
    {
        if data.len() == 0 {
            return Err(ToolkitError::EmptyData);
        }
        let mut data = data.clone().to_owned();
        data.sort_by_key(|k| k.epoch_time());
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
            // calculate EMA
            let mut first = true;
            let mut result = 0f64;
            let mut i = 1f64;
            for curr in data.iter() {
                let k = 2f64 / (i + 1f64);
                if first {
                    result = curr.close_price();
                    first = false;
                }
                result = curr.close_price() * k + result * (1f64 - k);
                i += 1f64;
            }
            result
        } else {
            mean
        };
        let upper = mid + dev_mul * stdev;
        let lower = mid - dev_mul * stdev;

        Ok(Self::Bollinger(Band { upper, mid, lower }))
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel, MovingAverage};
    use crate::types::{
        data::{Exec, Stock},
        time::Time,
    };

    static MAX_ERR: f64 = 0.0000000001f64;

    #[derive(Clone)]
    struct ExecData {
        price: f64,
        volume: u64,
        epoch_time: Time,
    }
    impl ExecData {
        fn new(price: f64, volume: u64, epoch_time: Time) -> Self {
            Self {
                price,
                volume,
                epoch_time,
            }
        }
    }
    impl Exec for ExecData {
        fn price(&self) -> f64 {
            self.price
        }
        fn volume(&self) -> u64 {
            self.volume
        }
        fn epoch_time(&self) -> u128 {
            self.epoch_time.inner()
        }
    }

    #[derive(Clone)]
    struct StockData {
        open_price: f64,
        high_price: f64,
        low_price: f64,
        close_price: f64,
        tot_exec_amount: f64,
        tot_exec_volume: u64,
        epoch_time: Time,
    }
    impl StockData {
        fn new(
            open_price: f64,
            high_price: f64,
            low_price: f64,
            close_price: f64,
            tot_exec_amount: f64,
            tot_exec_volume: u64,
            epoch_time: Time,
        ) -> Self {
            Self {
                open_price,
                high_price,
                low_price,
                close_price,
                tot_exec_amount,
                tot_exec_volume,
                epoch_time,
            }
        }

        fn from(data: Vec<ExecData>) -> Self {
            let mut data = data.clone();
            data.sort_by_key(|k| k.epoch_time());
            let open_price = data[0].price();
            let close_price = data.last().unwrap().price();
            let epoch_time = data.last().unwrap().epoch_time();
            let mut high_price = data[0].price();
            let mut low_price = data[0].price();
            let mut tot_exec_amount = 0f64;
            let mut tot_exec_volume = 0u64;
            for elem in data.iter() {
                if high_price < elem.price() {
                    high_price = elem.price();
                }
                if low_price < elem.price() {
                    low_price = elem.price();
                }
                tot_exec_amount += elem.price() * (elem.volume() as f64);
                tot_exec_volume += elem.volume();
            }
            Self {
                open_price,
                high_price,
                low_price,
                close_price,
                tot_exec_amount,
                tot_exec_volume,
                epoch_time: Time::from(epoch_time),
            }
        }
    }
    impl Exec for StockData
    where
        Self: Stock,
    {
        fn price(&self) -> f64 {
            self.close_price()
        }
        fn volume(&self) -> u64 {
            self.tot_exec_volume()
        }
        fn epoch_time(&self) -> u128 {
            Stock::epoch_time(self)
        }
    }
    impl Stock for StockData {
        fn open_price(&self) -> f64 {
            self.open_price
        }
        fn high_price(&self) -> f64 {
            self.high_price
        }
        fn low_price(&self) -> f64 {
            self.low_price
        }
        fn close_price(&self) -> f64 {
            self.close_price
        }
        fn tot_exec_amount(&self) -> f64 {
            self.tot_exec_amount
        }
        fn tot_exec_volume(&self) -> u64 {
            self.tot_exec_volume
        }
        fn epoch_time(&self) -> u128 {
            self.epoch_time.inner()
        }
    }
    #[test]
    fn test_moving_average() {
        let now = Time::now().unwrap();
        let data = vec![
            ExecData::new(1100.0, 1, now - Time::from_days(7)),
            ExecData::new(1000.0, 2, now - Time::from_days(6)),
            ExecData::new(1200.0, 1, now - Time::from_days(5)),
            ExecData::new(1150.0, 3, now - Time::from_days(4)),
            ExecData::new(1200.0, 4, now - Time::from_days(3)),
            ExecData::new(1000.0, 1, now - Time::from_days(2)),
            ExecData::new(900.0, 1, now - Time::from_days(0)),
        ];
        assert!((MovingAverage::simple(&data).inner() - 1078.5714285714287).abs() < MAX_ERR);
        assert!((MovingAverage::exponential(&data).inner() - 1057.1428571428573).abs() < MAX_ERR);
    }

    #[test]
    fn test_channel() {
        let now = Time::now().unwrap();
        let data = vec![
            StockData::new(
                1200.0,
                1200.0,
                1000.0,
                1100.0,
                1050.0 * 1000.0,
                1000,
                now - Time::from_days(7),
            ),
            StockData::new(
                1000.0,
                1200.0,
                950.0,
                1200.0,
                1100.0 * 2000.0,
                2000,
                now - Time::from_days(6),
            ),
            StockData::new(
                1200.0,
                1300.0,
                1100.0,
                1150.0,
                1200.0 * 2500.0,
                2500,
                now - Time::from_days(5),
            ),
            StockData::new(
                1150.0,
                1200.0,
                1000.0,
                1200.0,
                1150.0 * 2000.0,
                2000,
                now - Time::from_days(4),
            ),
            StockData::new(
                1200.0,
                1200.0,
                1000.0,
                1000.0,
                1050.0 * 2000.0,
                2000,
                now - Time::from_days(3),
            ),
            StockData::new(
                1000.0,
                1100.0,
                800.0,
                900.0,
                900.0 * 3000.0,
                3000,
                now - Time::from_days(2),
            ),
            StockData::new(
                900.0,
                1000.0,
                800.0,
                950.0,
                900.0 * 1000.0,
                1000,
                now - Time::from_days(1),
            ),
        ];
        let envelope_band = Channel::envelope(&data, 0.1f64).inner();
        let bollinger_band = Channel::bollinger(&data, 2f64, true).unwrap().inner();
        assert!((envelope_band.upper - 1131.4285714285716).abs() < MAX_ERR);
        assert!((envelope_band.mid - 1028.5714285714287).abs() < MAX_ERR);
        assert!((envelope_band.lower - 925.7142857142858).abs() < MAX_ERR);
        assert!((bollinger_band.upper - 1546.503825949666).abs() < MAX_ERR);
        assert!((bollinger_band.mid - 1028.5714285714287).abs() < MAX_ERR);
        assert!((bollinger_band.lower - 510.6390311931914).abs() < MAX_ERR);
    }
}
