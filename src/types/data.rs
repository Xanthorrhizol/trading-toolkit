pub trait BaseData {
    fn value(&self) -> f64;
    fn weight(&self) -> u64;
    fn epoch_time(&self) -> u128;
}

pub trait Stock {
    fn open_price(&self) -> f64;
    fn high_price(&self) -> f64;
    fn low_price(&self) -> f64;
    fn close_price(&self) -> f64;
    fn tot_exec_amount(&self) -> f64;
    fn tot_exec_volume(&self) -> u64;
    fn epoch_time(&self) -> u128;
}

impl BaseData for dyn Stock {
    fn value(&self) -> f64 {
        self.close_price()
    }

    fn weight(&self) -> u64 {
        self.tot_exec_volume()
    }
    fn epoch_time(&self) -> u128 {
        self.epoch_time()
    }
}
