#[cfg(test)]
mod tests {
    use crate::indicator::{
        Channel, ElderRay, ForceIndex, MovingAverage, MovingAverageConvergenceDivergence,
        Stochastic,
    };
    use crate::types::{
        data::{BaseData, Candle},
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
    struct CandleData {
        open_price: f64,
        high_price: f64,
        low_price: f64,
        close_price: f64,
        tot_exec_volume: u64,
        epoch_time: Time,
    }
    impl CandleData {
        fn new(
            open_price: f64,
            high_price: f64,
            low_price: f64,
            close_price: f64,
            tot_exec_volume: u64,
            epoch_time: Time,
        ) -> Self {
            Self {
                open_price,
                high_price,
                low_price,
                close_price,
                tot_exec_volume,
                epoch_time,
            }
        }
    }
    impl BaseData for CandleData
    where
        Self: Candle,
    {
        fn value(&self) -> f64 {
            self.close_price()
        }
        fn weight(&self) -> u64 {
            self.tot_exec_volume()
        }
        fn epoch_time(&self) -> u128 {
            Candle::epoch_time(self)
        }
    }
    impl Candle for CandleData {
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
            CandleData::new(
                1200.0,
                1200.0,
                1000.0,
                1100.0,
                1000,
                now - Time::from_days(7),
            ),
            CandleData::new(
                1000.0,
                1200.0,
                950.0,
                1200.0,
                2000,
                now - Time::from_days(6),
            ),
            CandleData::new(
                1200.0,
                1300.0,
                1100.0,
                1150.0,
                2500,
                now - Time::from_days(5),
            ),
            CandleData::new(
                1150.0,
                1200.0,
                1000.0,
                1200.0,
                2000,
                now - Time::from_days(4),
            ),
            CandleData::new(
                1200.0,
                1200.0,
                1000.0,
                1000.0,
                2000,
                now - Time::from_days(3),
            ),
            CandleData::new(1000.0, 1100.0, 800.0, 900.0, 3000, now - Time::from_days(2)),
            CandleData::new(900.0, 1000.0, 800.0, 950.0, 1000, now - Time::from_days(1)),
        ];
        let envelope_band = Channel::envelope(&data, 0.1f64).inner();
        let bollinger_band = Channel::bollinger(&data, 2f64, true).unwrap().inner();
        assert!((envelope_band.upper - 1130.3140694754463).abs() < MAX_ERR);
        assert!((envelope_band.mid - 1027.5582449776784).abs() < MAX_ERR);
        assert!((envelope_band.lower - 924.8024204799107).abs() < MAX_ERR);
        assert!((bollinger_band.upper - 1269.8779312082795).abs() < MAX_ERR);
        assert!((bollinger_band.mid - 1027.5582449776784).abs() < MAX_ERR);
        assert!((bollinger_band.lower - 785.2385587470773).abs() < MAX_ERR);
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
        assert!((macd.fast() - (-216.43118581050044)).abs() < MAX_ERR);
        assert!((macd.slow() - (-257.9256624351475)).abs() < MAX_ERR);
        assert!((macd.macd_histogram() - 41.49447662464706).abs() < MAX_ERR);
    }

    #[test]
    fn test_force_index() {
        let now = Time::now().unwrap();
        let data = vec![
            CandleData::new(1000.0, 1100.0, 800.0, 900.0, 3000, now - Time::from_days(2)),
            CandleData::new(900.0, 1000.0, 800.0, 950.0, 1000, now - Time::from_days(1)),
        ];
        let force_index = ForceIndex::new(&data[0], &data[1]);
        assert_eq!(50000f64, force_index.inner());
    }

    #[test]
    fn test_elder_ray() {
        let now = Time::now().unwrap();
        let data = vec![
            CandleData::new(
                1180.0,
                1210.0,
                1155.0,
                1200.0,
                3200,
                now - Time::from_days(30),
            ),
            CandleData::new(
                1200.0,
                1235.0,
                1185.0,
                1220.0,
                2800,
                now - Time::from_days(29),
            ),
            CandleData::new(
                1215.0,
                1250.0,
                1200.0,
                1240.0,
                4100,
                now - Time::from_days(28),
            ),
            CandleData::new(
                1240.0,
                1260.0,
                1215.0,
                1225.0,
                3500,
                now - Time::from_days(27),
            ),
            CandleData::new(
                1225.0,
                1245.0,
                1190.0,
                1195.0,
                2900,
                now - Time::from_days(26),
            ),
            CandleData::new(
                1195.0,
                1210.0,
                1160.0,
                1170.0,
                3800,
                now - Time::from_days(25),
            ),
            CandleData::new(
                1170.0,
                1185.0,
                1130.0,
                1145.0,
                4200,
                now - Time::from_days(24),
            ),
            CandleData::new(
                1145.0,
                1165.0,
                1100.0,
                1115.0,
                5100,
                now - Time::from_days(23),
            ),
            CandleData::new(
                1115.0,
                1130.0,
                1070.0,
                1080.0,
                4700,
                now - Time::from_days(22),
            ),
            CandleData::new(
                1080.0,
                1095.0,
                1040.0,
                1055.0,
                5500,
                now - Time::from_days(21),
            ),
            CandleData::new(
                1055.0,
                1075.0,
                1020.0,
                1035.0,
                6000,
                now - Time::from_days(20),
            ),
            CandleData::new(
                1035.0,
                1060.0,
                995.0,
                1050.0,
                5300,
                now - Time::from_days(19),
            ),
            CandleData::new(
                1050.0,
                1090.0,
                1030.0,
                1075.0,
                4600,
                now - Time::from_days(18),
            ),
            CandleData::new(
                1075.0,
                1100.0,
                1055.0,
                1090.0,
                3900,
                now - Time::from_days(17),
            ),
            CandleData::new(
                1090.0,
                1115.0,
                1065.0,
                1070.0,
                3400,
                now - Time::from_days(16),
            ),
            CandleData::new(
                1070.0,
                1085.0,
                1030.0,
                1045.0,
                3700,
                now - Time::from_days(15),
            ),
            CandleData::new(
                1045.0,
                1070.0,
                1020.0,
                1060.0,
                3200,
                now - Time::from_days(14),
            ),
            CandleData::new(
                1060.0,
                1080.0,
                1035.0,
                1040.0,
                2900,
                now - Time::from_days(13),
            ),
            CandleData::new(
                1040.0,
                1065.0,
                1010.0,
                1055.0,
                3100,
                now - Time::from_days(12),
            ),
            CandleData::new(
                1055.0,
                1075.0,
                1025.0,
                1030.0,
                3600,
                now - Time::from_days(11),
            ),
            CandleData::new(
                1030.0,
                1055.0,
                1005.0,
                1045.0,
                3300,
                now - Time::from_days(10),
            ),
            CandleData::new(
                1045.0,
                1070.0,
                1020.0,
                1065.0,
                3800,
                now - Time::from_days(9),
            ),
            CandleData::new(
                1065.0,
                1095.0,
                1045.0,
                1085.0,
                4100,
                now - Time::from_days(8),
            ),
            CandleData::new(
                1085.0,
                1110.0,
                1060.0,
                1095.0,
                3700,
                now - Time::from_days(7),
            ),
            CandleData::new(
                1095.0,
                1120.0,
                1075.0,
                1100.0,
                3400,
                now - Time::from_days(6),
            ),
            CandleData::new(
                1100.0,
                1130.0,
                1080.0,
                1115.0,
                3100,
                now - Time::from_days(5),
            ),
            CandleData::new(
                1115.0,
                1140.0,
                1085.0,
                1105.0,
                2800,
                now - Time::from_days(4),
            ),
            CandleData::new(
                1105.0,
                1125.0,
                1070.0,
                1090.0,
                3200,
                now - Time::from_days(3),
            ),
            CandleData::new(
                1090.0,
                1115.0,
                1055.0,
                1075.0,
                3500,
                now - Time::from_days(2),
            ),
            CandleData::new(
                1075.0,
                1095.0,
                1040.0,
                1060.0,
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
            CandleData::new(
                1200.0,
                1200.0,
                1000.0,
                1100.0,
                1000,
                now - Time::from_days(7),
            ),
            CandleData::new(
                1000.0,
                1200.0,
                950.0,
                1200.0,
                2000,
                now - Time::from_days(6),
            ),
            CandleData::new(
                1200.0,
                1300.0,
                1100.0,
                1150.0,
                2500,
                now - Time::from_days(5),
            ),
            CandleData::new(
                1150.0,
                1200.0,
                1000.0,
                1200.0,
                2000,
                now - Time::from_days(4),
            ),
            CandleData::new(
                1200.0,
                1200.0,
                1000.0,
                1000.0,
                2000,
                now - Time::from_days(3),
            ),
            CandleData::new(1000.0, 1100.0, 800.0, 900.0, 3000, now - Time::from_days(2)),
            CandleData::new(900.0, 1000.0, 800.0, 950.0, 1000, now - Time::from_days(1)),
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
        assert!((slow_stochastics.first().unwrap().inner() - 58.333333333333336).abs() < MAX_ERR);
        assert!((slow_stochastics.last().unwrap().inner() - 44.444444444444436).abs() < MAX_ERR);
        assert!(
            (Stochastic::into_slow(&fast_stochastics).unwrap().inner() - 41.07142857142857).abs()
                < MAX_ERR,
        );
    }
}
