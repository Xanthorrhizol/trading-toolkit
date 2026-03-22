use super::MovingAverage;
use crate::types::{data::BaseData, error::ToolkitError};

#[derive(Debug, Clone, Copy)]
pub struct MovingAverageConvergenceDivergence {
    signal: f64,
    ema_12: f64,
    ema_26: f64,
}

impl MovingAverageConvergenceDivergence {
    pub fn new<T>(data: &[T]) -> Result<Self, ToolkitError>
    where
        T: BaseData + Clone,
    {
        if data.len() < 34 {
            return Err(ToolkitError::DataNotEnough);
        }
        let mut data = data.to_vec();
        data.sort_by_key(|k| k.epoch_time());
        let n = data.len();

        // MACD 값을 BaseData로 wrapping하여 MovingAverage에 활용
        #[derive(Debug, Clone, Copy)]
        struct MacdPoint(f64, u128);
        impl BaseData for MacdPoint {
            fn value(&self) -> f64 {
                self.0
            }
            fn weight(&self) -> u64 {
                1
            }
            fn epoch_time(&self) -> u128 {
                self.1
            }
        }

        // EMA(26): 첫 26개 SMA로 seed → bar 26부터 rolling
        let mut ema26 = MovingAverage::simple(&data[0..26]);
        // EMA(12): 첫 12개 SMA로 seed → bar 25까지 rolling (ema26과 기준 bar 맞춤)
        let mut ema12 = MovingAverage::simple(&data[0..12]);
        for i in 12..26 {
            ema12 = MovingAverage::exponential_from(12, &ema12, &data[i]);
        }

        // bar 25 기준 첫 번째 MACD 값
        let mut macd_for_seed = vec![MacdPoint(
            ema12.inner() - ema26.inner(),
            data[25].epoch_time(),
        )];

        // bars 26..33: MACD seed용 9개 수집 (총 9개)
        for i in 26..34 {
            ema12 = MovingAverage::exponential_from(12, &ema12, &data[i]);
            ema26 = MovingAverage::exponential_from(26, &ema26, &data[i]);
            macd_for_seed.push(MacdPoint(
                ema12.inner() - ema26.inner(),
                data[i].epoch_time(),
            ));
        }

        // Signal: 첫 9개 MACD의 SMA로 seed → bar 34부터 rolling EMA(9)
        let mut signal = MovingAverage::simple(&macd_for_seed);
        for i in 34..n {
            ema12 = MovingAverage::exponential_from(12, &ema12, &data[i]);
            ema26 = MovingAverage::exponential_from(26, &ema26, &data[i]);
            let macd_point = MacdPoint(ema12.inner() - ema26.inner(), data[i].epoch_time());
            signal = MovingAverage::exponential_from(9, &signal, &macd_point);
        }

        Ok(Self {
            signal: signal.inner(),
            ema_12: ema12.inner(),
            ema_26: ema26.inner(),
        })
    }

    /// fast line / MACD line
    pub fn fast(&self) -> f64 {
        self.ema_12 - self.ema_26
    }

    /// slow line / Signal
    pub fn slow(&self) -> f64 {
        self.signal
    }

    /// MACD histogram
    pub fn macd_histogram(&self) -> f64 {
        self.fast() - self.slow()
    }
}
