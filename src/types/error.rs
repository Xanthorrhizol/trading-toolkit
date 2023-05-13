use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToolkitError {
    #[error("Empty data")]
    EmptyData,
    #[error("Data not enough")]
    DataNotEnough,
    #[error("Data invalid")]
    InvalidData,
}
