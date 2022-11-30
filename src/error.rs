#[derive(Debug)]
pub enum Error {
    NotInFumNet,
    ReqwestError(reqwest::Error),
    PortalNotAvailable,
}

pub type Result<T> = std::result::Result<T, Error>;
