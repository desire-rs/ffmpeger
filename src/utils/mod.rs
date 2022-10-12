pub fn now_option() -> Option<chrono::DateTime<chrono::Utc>> {
  Some(chrono::Utc::now())
}

pub fn gen_uuid() -> uuid::Uuid {
  uuid::Uuid::new_v4()
}
