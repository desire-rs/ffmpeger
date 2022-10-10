use crate::types::ApiResult;
use desire::IntoResponse;
use desire::Request;
pub async fn liveness(_req: Request) -> impl IntoResponse {
  "liveness"
}

pub async fn hello(_req: Request) -> ApiResult<String> {
  Ok("Hello!".to_string())
}
