use crate::error::Error;
pub type AnyResult<T> = anyhow::Result<T, anyhow::Error>;
pub type ApiResult<T> = std::result::Result<T, Error>;
