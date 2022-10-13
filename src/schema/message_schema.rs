use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  pub text: String,
}
impl Message {
  pub fn new(text: String) -> Self {
    Message { text }
  }
}
