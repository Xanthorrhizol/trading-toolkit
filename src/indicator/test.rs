#[cfg(test)]
mod tests {
    use crate::indicator::{
        Channel, ElderRay, ForceIndex, MovingAverage, MovingAverageConvergenceDivergence,
        Stochastic,
    };
    use crate::types::{
        data::{BaseData, Stock},
        time::Time,
    };

    static MAX_ERR: f64 = 0.0000000001f64;

    #[derive(Clone)]
    struct RawBaseData {
        price: f64,
        volume: u64,
        epoch_time: Time,
    }
    impl RawBaseData {
        fn new(price: f64, volume: u64, epoch_time: Time) -> Self {
            Self {
                price,
                volume,
                epoch_time,
            }
        }
    }
    impl BaseData for RawBaseData {
        fn value(&self) -> f64 {
            self.price
        }
        fn weight(&self) -> u64 {
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
    }
    impl BaseData for StockData
    where
        Self: Stock,
    {
        fn value(&self) -> f64 {
            self.close_price()
        }
        fn weight(&self) -> u64 {
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
            RawBaseData::new(1100.0, 1, now - Time::from_days(7)),
            RawBaseData::new(1000.0, 2, now - Time::from_days(6)),
            RawBaseData::new(1200.0, 1, now - Time::from_days(5)),
            RawBaseData::new(1150.0, 3, now - Time::from_days(4)),
            RawBaseData::new(1200.0, 4, now - Time::from_days(3)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(2)),
            RawBaseData::new(900.0, 1, now - Time::from_days(1)),
        ];
        let sma = MovingAverage::simple(&data);
        assert!((sma.inner() - 1078.5714285714287).abs() < MAX_ERR);
        assert!((MovingAverage::exponential(&data).inner() - 1057.1428571428573).abs() < MAX_ERR);
        assert!(
            (MovingAverage::simple_from(7, &sma, &data[0], &RawBaseData::new(900.0, 1, now))
                .inner()
                - 1050.0000000000002)
                .abs()
                < MAX_ERR
        );
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
            RawBaseData::new(2000.0, 1, now - Time::from_days(30)),
            RawBaseData::new(1900.0, 1, now - Time::from_days(29)),
            RawBaseData::new(1950.0, 1, now - Time::from_days(28)),
            RawBaseData::new(1850.0, 1, now - Time::from_days(27)),
            RawBaseData::new(1750.0, 1, now - Time::from_days(26)),
            RawBaseData::new(1700.0, 1, now - Time::from_days(25)),
            RawBaseData::new(1600.0, 1, now - Time::from_days(24)),
            RawBaseData::new(1800.0, 1, now - Time::from_days(23)),
            RawBaseData::new(1750.0, 1, now - Time::from_days(22)),
            RawBaseData::new(1500.0, 1, now - Time::from_days(21)),
            RawBaseData::new(1300.0, 1, now - Time::from_days(20)),
            RawBaseData::new(1250.0, 1, now - Time::from_days(19)),
            RawBaseData::new(1300.0, 1, now - Time::from_days(18)),
            RawBaseData::new(1350.0, 1, now - Time::from_days(17)),
            RawBaseData::new(1200.0, 1, now - Time::from_days(16)),
            RawBaseData::new(1300.0, 1, now - Time::from_days(15)),
            RawBaseData::new(1100.0, 1, now - Time::from_days(14)),
            RawBaseData::new(950.0, 1, now - Time::from_days(13)),
            RawBaseData::new(900.0, 1, now - Time::from_days(12)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(11)),
            RawBaseData::new(1150.0, 1, now - Time::from_days(10)),
            RawBaseData::new(1100.0, 1, now - Time::from_days(9)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(8)),
            RawBaseData::new(1100.0, 1, now - Time::from_days(7)),
            RawBaseData::new(1000.0, 2, now - Time::from_days(6)),
            RawBaseData::new(1200.0, 1, now - Time::from_days(5)),
            RawBaseData::new(1150.0, 3, now - Time::from_days(4)),
            RawBaseData::new(1200.0, 4, now - Time::from_days(3)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(2)),
            RawBaseData::new(900.0, 1, now - Time::from_days(0)),
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

    #[test]
    fn test_elder_ray() {
        let now = Time::now().unwrap();
        let data = vec![
            RawBaseData::new(2000.0, 1, now - Time::from_days(30)),
            RawBaseData::new(1900.0, 1, now - Time::from_days(29)),
            RawBaseData::new(1950.0, 1, now - Time::from_days(28)),
            RawBaseData::new(1850.0, 1, now - Time::from_days(27)),
            RawBaseData::new(1750.0, 1, now - Time::from_days(26)),
            RawBaseData::new(1700.0, 1, now - Time::from_days(25)),
            RawBaseData::new(1600.0, 1, now - Time::from_days(24)),
            RawBaseData::new(1800.0, 1, now - Time::from_days(23)),
            RawBaseData::new(1750.0, 1, now - Time::from_days(22)),
            RawBaseData::new(1500.0, 1, now - Time::from_days(21)),
            RawBaseData::new(1300.0, 1, now - Time::from_days(20)),
            RawBaseData::new(1250.0, 1, now - Time::from_days(19)),
            RawBaseData::new(1300.0, 1, now - Time::from_days(18)),
            RawBaseData::new(1350.0, 1, now - Time::from_days(17)),
            RawBaseData::new(1200.0, 1, now - Time::from_days(16)),
            RawBaseData::new(1300.0, 1, now - Time::from_days(15)),
            RawBaseData::new(1100.0, 1, now - Time::from_days(14)),
            RawBaseData::new(950.0, 1, now - Time::from_days(13)),
            RawBaseData::new(900.0, 1, now - Time::from_days(12)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(11)),
            RawBaseData::new(1150.0, 1, now - Time::from_days(10)),
            RawBaseData::new(1100.0, 1, now - Time::from_days(9)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(8)),
            RawBaseData::new(1100.0, 1, now - Time::from_days(7)),
            RawBaseData::new(1000.0, 2, now - Time::from_days(6)),
            RawBaseData::new(1200.0, 1, now - Time::from_days(5)),
            RawBaseData::new(1150.0, 3, now - Time::from_days(4)),
            RawBaseData::new(1200.0, 4, now - Time::from_days(3)),
            RawBaseData::new(1000.0, 1, now - Time::from_days(2)),
            RawBaseData::new(900.0, 1, now - Time::from_days(0)),
        ];
        let elder_ray = ElderRay::new(&data).unwrap();
        assert!((elder_ray.ask_force() - (-273.6559139784947)).abs() < MAX_ERR);
        assert!((elder_ray.bid_force() - 826.3440860215053).abs() < MAX_ERR);
    }

    #[test]
    fn test_stochastic() {
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
        let fast_stochastics = vec![
            Stochastic::fast(&data[0..3].to_vec()).unwrap(),
            Stochastic::fast(&data[3..6].to_vec()).unwrap(),
        ];
        assert!((fast_stochastics.first().unwrap().inner() - 57.14285714285714).abs() < MAX_ERR);
        assert!((fast_stochastics.last().unwrap().inner() - 25f64).abs() < MAX_ERR);
        let slow_stochastics = vec![
            Stochastic::fast(&data[0..3].to_vec()).unwrap(),
            Stochastic::fast(&data[3..6].to_vec()).unwrap(),
        ];
        assert!((slow_stochastics.first().unwrap().inner() - 57.14285714285714).abs() < MAX_ERR);
        assert!((slow_stochastics.last().unwrap().inner() - 25f64).abs() < MAX_ERR);
        assert!(
            (Stochastic::into_slow(&fast_stochastics).unwrap().inner() - 41.07142857142857).abs()
                < MAX_ERR,
        );
    }
}
