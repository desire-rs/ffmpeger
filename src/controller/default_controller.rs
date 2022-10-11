use crate::types::Resp;
use crate::ApiResult;
use desire::IntoResponse;
use desire::Request;
pub async fn liveness(_req: Request) -> impl IntoResponse {
  "liveness"
}

pub async fn hello(_req: Request) -> ApiResult<String> {
  Ok(Resp::data("Hello!".to_string()))
}
