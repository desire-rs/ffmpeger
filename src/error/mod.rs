use desire::{IntoResponse, Response, Result};
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
  #[error("Anyhow::Error")]
  AnyError(#[from] anyhow::Error),
  #[error("Desire::Error")]
  DesireError(#[from] desire::Error),
}

impl IntoResponse for Error {
  fn into_response(self) -> Result {
    let val = self.to_string();
    Response::with_status(500, val)
  }
}
