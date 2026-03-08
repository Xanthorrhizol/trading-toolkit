use super::MovingAverage;
use crate::types::{data::BaseData, error::ToolkitError};

#[derive(Debug, Clone, Copy)]
pub struct MovingAverageConvergenceDivergence {
    signal: f64,
    ema_12: f64,
    ema_26: f64,
}

impl MovingAverageConvergenceDivergence {
    pub fn new<T>(data: &Vec<T>) -> Result<Self, ToolkitError>
    where
        T: BaseData + Clone,
    {
        if data.len() < 34 {
            return Err(ToolkitError::DataNotEnough);
        }
        let mut data = data.clone().to_vec();
        data.sort_by_key(|k| k.epoch_time());
        data = data.split_at(data.len() - 34).1.to_vec();
        let n = data.len();
        let ema_26 = MovingAverage::exponential(&data[n - 26..n].to_vec());
        let ema_12 = MovingAverage::exponential(&data[n - 12..n].to_vec());
        let macd_series: Vec<_> = (0..9)
            .map(|i| {
                let end = n - (8 - i); // 9번째 마지막이 현재 bar
                let e26 = MovingAverage::exponential(&data[end - 26..end].to_vec()).inner();
                let e12 = MovingAverage::exponential(&data[end - 12..end].to_vec()).inner();
                e12 - e26
            })
            .collect();

        // MACD 값들의 EMA_9 → Signal Line
        // BaseData로 wrapping해서 기존 exponential 활용
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
        let macd_data: Vec<MacdPoint> = macd_series
            .iter()
            .enumerate()
            .map(|(i, v)| MacdPoint(*v, i as u128))
            .collect();

        let signal = MovingAverage::exponential(&macd_data).inner();
        Ok(Self {
            signal: signal,
            ema_12: ema_12.inner(),
            ema_26: ema_26.inner(),
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
