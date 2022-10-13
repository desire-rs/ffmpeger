use crate::config::SLACK_WEBHOOK_URL;
use crate::schema::message_schema::Message;
use crate::AnyResult;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;

pub async fn send_message(payload: String) -> AnyResult<String> {
  let message: Message = Message::new(payload);
  let mut headers = HeaderMap::new();
  headers.append(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
  let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

  let res = client
    .post(SLACK_WEBHOOK_URL)
    .json(&message)
    .send()
    .await?
    .text()
    .await?;
  info!("res {:?}", res);
  Ok(res)
}
