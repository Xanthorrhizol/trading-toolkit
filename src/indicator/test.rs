#[cfg(test)]
mod tests {
    use crate::indicator::{
        oscillator::{ForceIndex, MovingAverageConvergenceDivergence},
        trend_following::{Channel, MovingAverage},
    };
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

    #[test]
    fn test_moving_average_convergence_divergence() {
        let now = Time::now().unwrap();
        let data = vec![
            ExecData::new(2000.0, 1, now - Time::from_days(30)),
            ExecData::new(1900.0, 1, now - Time::from_days(29)),
            ExecData::new(1950.0, 1, now - Time::from_days(28)),
            ExecData::new(1850.0, 1, now - Time::from_days(27)),
            ExecData::new(1750.0, 1, now - Time::from_days(26)),
            ExecData::new(1700.0, 1, now - Time::from_days(25)),
            ExecData::new(1600.0, 1, now - Time::from_days(24)),
            ExecData::new(1800.0, 1, now - Time::from_days(23)),
            ExecData::new(1750.0, 1, now - Time::from_days(22)),
            ExecData::new(1500.0, 1, now - Time::from_days(21)),
            ExecData::new(1300.0, 1, now - Time::from_days(20)),
            ExecData::new(1250.0, 1, now - Time::from_days(19)),
            ExecData::new(1300.0, 1, now - Time::from_days(18)),
            ExecData::new(1350.0, 1, now - Time::from_days(17)),
            ExecData::new(1200.0, 1, now - Time::from_days(16)),
            ExecData::new(1300.0, 1, now - Time::from_days(15)),
            ExecData::new(1100.0, 1, now - Time::from_days(14)),
            ExecData::new(950.0, 1, now - Time::from_days(13)),
            ExecData::new(900.0, 1, now - Time::from_days(12)),
            ExecData::new(1000.0, 1, now - Time::from_days(11)),
            ExecData::new(1150.0, 1, now - Time::from_days(10)),
            ExecData::new(1100.0, 1, now - Time::from_days(9)),
            ExecData::new(1000.0, 1, now - Time::from_days(8)),
            ExecData::new(1100.0, 1, now - Time::from_days(7)),
            ExecData::new(1000.0, 2, now - Time::from_days(6)),
            ExecData::new(1200.0, 1, now - Time::from_days(5)),
            ExecData::new(1150.0, 3, now - Time::from_days(4)),
            ExecData::new(1200.0, 4, now - Time::from_days(3)),
            ExecData::new(1000.0, 1, now - Time::from_days(2)),
            ExecData::new(900.0, 1, now - Time::from_days(0)),
        ];
        let macd = MovingAverageConvergenceDivergence::new(&data).unwrap();
        assert!((macd.fast() - (-64.24501424501454)).abs() < MAX_ERR);
        assert!((macd.slow() - 1062.2222222222226).abs() < MAX_ERR);
        assert!((macd.macd_histogram() - (-1126.4672364672372)).abs() < MAX_ERR);
    }

    #[test]
    fn test_force_index() {
        let now = Time::now().unwrap();
        let data = vec![
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
        let force_index = ForceIndex::new(&data[0], &data[1]);
        assert_eq!(50000f64, force_index.inner());
    }
}
