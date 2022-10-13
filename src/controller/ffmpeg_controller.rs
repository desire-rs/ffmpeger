use crate::schema::task_schema::Task;
use crate::service::ffmpeg_service;
use crate::types::Tasks;
use crate::ApiPageResult;
use crate::ApiResult;
use crate::PageData;
use crate::Resp;
use desire::Request;
use uuid::Uuid;
pub async fn m3u8(mut req: Request) -> ApiResult<Task> {
  let task = req.body::<Task>().await?;
  info!("task {:#?}", task);
  let json = task.clone();
  let _handle = tokio::task::spawn(async move {
    let res = ffmpeg_service::m3u8(task).await;
    println!("res {:#?}", res);
  });
  Ok(Resp::data(json))
}

pub async fn task(req: Request) -> ApiPageResult<Task> {
  let tasks = req.inner.extensions().get::<Tasks>();
  let list = &*tasks.unwrap().lock().unwrap();
  info!("tasks {:?}", tasks);
  let total = list.len() as u64;
  let result = PageData::new(list.to_vec(), total);
  Ok(Resp::data(result))
}

pub async fn remove(req: Request) -> ApiResult<String> {
  let id = req.param::<String>("id")?;
  if let Some(tasks) = req.extensions().get::<Tasks>() {
    let mut tasks = tasks.lock().unwrap();
    let list = tasks
      .drain_filter(|task| task.id == Uuid::parse_str(&id).unwrap())
      .collect::<Vec<Task>>();
  }
  Ok(Resp::data("OK".to_string()))
}

pub async fn clear(req: Request) -> ApiResult<String> {
  if let Some(tasks) = req.extensions().get::<Tasks>() {
    let mut tasks = tasks.lock().unwrap();
    tasks.clear()
  }
  Ok(Resp::data("OK".to_string()))
}
