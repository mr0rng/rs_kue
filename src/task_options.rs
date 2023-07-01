pub struct TaskOptions {
  pub attempt: i32,
  pub max_attempts: i32,
}

impl TaskOptions {
  pub fn new(attempt: i32, max_attempts: i32) -> Self { 
    Self { attempt, max_attempts } 
  }
}