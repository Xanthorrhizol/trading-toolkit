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
        assert!((MovingAverage::exponential(&data).inner() - 1049.703107561384).abs() < MAX_ERR);
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
        assert!((envelope_band.upper - 1130.3140694754463).abs() < MAX_ERR);
        assert!((envelope_band.mid - 1027.5582449776784).abs() < MAX_ERR);
        assert!((envelope_band.lower - 924.8024204799107).abs() < MAX_ERR);
        assert!((bollinger_band.upper - 1253.4352207039913).abs() < MAX_ERR);
        assert!((bollinger_band.mid - 1027.5582449776784).abs() < MAX_ERR);
        assert!((bollinger_band.lower - 801.6812692513656).abs() < MAX_ERR);
    }

    #[test]
    fn test_moving_average_convergence_divergence() {
        let now = Time::now().unwrap();
        let data = vec![
            RawBaseData::new(2000.0, 1, now - Time::from_days(33)),
            RawBaseData::new(2200.0, 1, now - Time::from_days(32)),
            RawBaseData::new(2000.0, 1, now - Time::from_days(31)),
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
            RawBaseData::new(1000.0, 1, now - Time::from_days(1)),
            RawBaseData::new(900.0, 1, now - Time::from_days(0)),
        ];
        let macd = MovingAverageConvergenceDivergence::new(&data).unwrap();
        assert!((macd.fast() - (-89.40857762786618)).abs() < MAX_ERR);
        assert!((macd.slow() - (-130.0014980518957)).abs() < MAX_ERR);
        assert!((macd.macd_histogram() - 40.59292042402953).abs() < MAX_ERR);
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
            StockData::new(
                1180.0,
                1210.0,
                1155.0,
                1200.0,
                3808000.0,
                3200,
                now - Time::from_days(30),
            ),
            StockData::new(
                1200.0,
                1235.0,
                1185.0,
                1220.0,
                3388000.0,
                2800,
                now - Time::from_days(29),
            ),
            StockData::new(
                1215.0,
                1250.0,
                1200.0,
                1240.0,
                5032750.0,
                4100,
                now - Time::from_days(28),
            ),
            StockData::new(
                1240.0,
                1260.0,
                1215.0,
                1225.0,
                4313750.0,
                3500,
                now - Time::from_days(27),
            ),
            StockData::new(
                1225.0,
                1245.0,
                1190.0,
                1195.0,
                3509000.0,
                2900,
                now - Time::from_days(26),
            ),
            StockData::new(
                1195.0,
                1210.0,
                1160.0,
                1170.0,
                4493500.0,
                3800,
                now - Time::from_days(25),
            ),
            StockData::new(
                1170.0,
                1185.0,
                1130.0,
                1145.0,
                4861500.0,
                4200,
                now - Time::from_days(24),
            ),
            StockData::new(
                1145.0,
                1165.0,
                1100.0,
                1115.0,
                5763000.0,
                5100,
                now - Time::from_days(23),
            ),
            StockData::new(
                1115.0,
                1130.0,
                1070.0,
                1080.0,
                5158250.0,
                4700,
                now - Time::from_days(22),
            ),
            StockData::new(
                1080.0,
                1095.0,
                1040.0,
                1055.0,
                5871250.0,
                5500,
                now - Time::from_days(21),
            ),
            StockData::new(
                1055.0,
                1075.0,
                1020.0,
                1035.0,
                6270000.0,
                6000,
                now - Time::from_days(20),
            ),
            StockData::new(
                1035.0,
                1060.0,
                995.0,
                1050.0,
                5525250.0,
                5300,
                now - Time::from_days(19),
            ),
            StockData::new(
                1050.0,
                1090.0,
                1030.0,
                1075.0,
                4887500.0,
                4600,
                now - Time::from_days(18),
            ),
            StockData::new(
                1075.0,
                1100.0,
                1055.0,
                1090.0,
                4221750.0,
                3900,
                now - Time::from_days(17),
            ),
            StockData::new(
                1090.0,
                1115.0,
                1065.0,
                1070.0,
                3672000.0,
                3400,
                now - Time::from_days(16),
            ),
            StockData::new(
                1070.0,
                1085.0,
                1030.0,
                1045.0,
                3912750.0,
                3700,
                now - Time::from_days(15),
            ),
            StockData::new(
                1045.0,
                1070.0,
                1020.0,
                1060.0,
                3368000.0,
                3200,
                now - Time::from_days(14),
            ),
            StockData::new(
                1060.0,
                1080.0,
                1035.0,
                1040.0,
                3045000.0,
                2900,
                now - Time::from_days(13),
            ),
            StockData::new(
                1040.0,
                1065.0,
                1010.0,
                1055.0,
                3247250.0,
                3100,
                now - Time::from_days(12),
            ),
            StockData::new(
                1055.0,
                1075.0,
                1025.0,
                1030.0,
                3753000.0,
                3600,
                now - Time::from_days(11),
            ),
            StockData::new(
                1030.0,
                1055.0,
                1005.0,
                1045.0,
                3423750.0,
                3300,
                now - Time::from_days(10),
            ),
            StockData::new(
                1045.0,
                1070.0,
                1020.0,
                1065.0,
                4009000.0,
                3800,
                now - Time::from_days(9),
            ),
            StockData::new(
                1065.0,
                1095.0,
                1045.0,
                1085.0,
                4407500.0,
                4100,
                now - Time::from_days(8),
            ),
            StockData::new(
                1085.0,
                1110.0,
                1060.0,
                1095.0,
                4033000.0,
                3700,
                now - Time::from_days(7),
            ),
            StockData::new(
                1095.0,
                1120.0,
                1075.0,
                1100.0,
                3731500.0,
                3400,
                now - Time::from_days(6),
            ),
            StockData::new(
                1100.0,
                1130.0,
                1080.0,
                1115.0,
                3433250.0,
                3100,
                now - Time::from_days(5),
            ),
            StockData::new(
                1115.0,
                1140.0,
                1085.0,
                1105.0,
                3108000.0,
                2800,
                now - Time::from_days(4),
            ),
            StockData::new(
                1105.0,
                1125.0,
                1070.0,
                1090.0,
                3512000.0,
                3200,
                now - Time::from_days(3),
            ),
            StockData::new(
                1090.0,
                1115.0,
                1055.0,
                1075.0,
                3788750.0,
                3500,
                now - Time::from_days(2),
            ),
            StockData::new(
                1075.0,
                1095.0,
                1040.0,
                1060.0,
                4056500.0,
                3800,
                now - Time::from_days(1),
            ),
        ];

        let elder_ray = ElderRay::new(&data).unwrap();
        assert!((elder_ray.bid_force() - (7.263579191057943)).abs() < MAX_ERR);
        assert!((elder_ray.ask_force() - (-47.73642080894206)).abs() < MAX_ERR);
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
        let slow_stochastics = vec![
            Stochastic::slow(&data[0..3].to_vec()).unwrap(),
            Stochastic::slow(&data[3..6].to_vec()).unwrap(),
        ];

        assert!((fast_stochastics.first().unwrap().inner() - 57.14285714285714).abs() < MAX_ERR);
        assert!((fast_stochastics.last().unwrap().inner() - 25f64).abs() < MAX_ERR);
        assert!((slow_stochastics.first().unwrap().inner() - 61.53846153846154).abs() < MAX_ERR);
        assert!((slow_stochastics.last().unwrap().inner() - 42.857142857142854).abs() < MAX_ERR);
        assert!(
            (Stochastic::into_slow(&fast_stochastics).unwrap().inner() - 41.07142857142857).abs()
                < MAX_ERR,
        );
    }
}
