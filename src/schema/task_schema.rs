use crate::utils::now_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  pub id: Uuid,
  pub title: String,
  pub url: String,
  pub storage_path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub task_type: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub poster: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub published_at: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub referer: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_agent: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cookies: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub origin: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub headers: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub source: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub designation: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none", default = "now_option")]
  pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskReport {
  pub id: Uuid,
  pub title: String,
  pub url: String,
  pub storage_path: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub file_size: Option<f64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_time: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start_at: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub stop_at: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error_at: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub completed_at: Option<DateTime<Utc>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub download_result: Option<String>,
  pub created_at: DateTime<Utc>,
}

impl TaskReport {
  pub fn new(id: Uuid, title: String, url: String, storage_path: String) -> Self {
    TaskReport {
      id,
      title,
      url,
      storage_path,
      status: None,
      file_size: None,
      use_time: None,
      start_at: None,
      stop_at: None,
      error_at: None,
      completed_at: None,
      download_result: None,
      created_at: Utc::now(),
    }
  }
}
