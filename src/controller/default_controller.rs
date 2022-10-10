use crate::types::ApiResult;
use desire::IntoResponse;
use desire::Request;
pub async fn root(_req: Request) -> impl IntoResponse {
  "Hello World"
}

pub async fn hello(_req: Request) -> ApiResult<String> {
  Ok("Hello!".to_string())
}
