pub trait Data {
    fn price(&self) -> f64;
    fn epoch_time(&self) -> u64;
}
