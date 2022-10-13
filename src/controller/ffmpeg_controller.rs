use crate::config::{TASK_HASH, TASK_LIMIT};
use crate::error::option_error;
use crate::schema::task_schema::Task;
use crate::service::ffmpeg_service;
use crate::service::slack_service;
use crate::ApiPageResult;
use crate::ApiResult;
use crate::PageData;
use crate::Resp;
use desire::Request;
use redis::AsyncCommands;
use std::collections::BTreeMap;
pub async fn m3u8(mut req: Request) -> ApiResult<Task> {
  let task = req.body::<Task>().await?;
  info!("task {:#?}", task);
  let client = req
    .extensions()
    .get::<redis::Client>()
    .ok_or(option_error("redis"))?;
  let mut redis = client.get_async_connection().await?;
  let len: i64 = redis.hlen(TASK_HASH).await?;
  if len > TASK_LIMIT {
    return Ok(Resp::data(task));
  }
  redis
    .hset(
      TASK_HASH,
      &task.id.to_string(),
      serde_json::to_string(&task)?,
    )
    .await?;
  let json = task.clone();
  let _handle = tokio::task::spawn(async move {
    let res = ffmpeg_service::m3u8(task).await;
    println!("res {:#?}", res);
  });
  Ok(Resp::data(json))
}

pub async fn task(req: Request) -> ApiPageResult<Task> {
  let client = req
    .extensions()
    .get::<redis::Client>()
    .ok_or(option_error("redis"))?;
  let mut redis = client.get_async_connection().await?;
  let result: BTreeMap<String, String> = redis.hgetall(TASK_HASH).await?;
  let mut list = Vec::new();
  for (_key, value) in result {
    let item = serde_json::from_str(&value)?;
    list.push(item)
  }
  let total = list.len();
  let result = PageData::new(list, total);
  Ok(Resp::data(result))
}

pub async fn remove(req: Request) -> ApiResult<Task> {
  let id = req.param::<String>("id")?;
  let client = req
    .extensions()
    .get::<redis::Client>()
    .ok_or(option_error("redis"))?;
  let mut redis = client.get_async_connection().await?;
  let task: String = redis.hget(TASK_HASH, &id).await?;
  let task = serde_json::from_str(&task)?;
  redis.hdel(TASK_HASH, &id).await?;
  Ok(Resp::data(task))
}

pub async fn clear(req: Request) -> ApiResult<String> {
  let client = req
    .extensions()
    .get::<redis::Client>()
    .ok_or(option_error("redis"))?;
  let mut redis = client.get_async_connection().await?;
  slack_service::send_message("ffmpeger clear tasks").await?;
  redis.del(TASK_HASH).await?;
  Ok(Resp::data("OK".to_string()))
}
