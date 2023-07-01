use crate::{KueTask, KueError, TaskOptions};

use super::TNResult;

#[derive(Debug, PartialEq)]
pub struct TNTask {
  pub result: i32, 
  pub current: i32
}

impl TNTask {
  pub fn new(result: i32, current: i32) -> Self { 
    Self { result, current } 
  }
}

impl KueTask<TNResult> for TNTask {
  fn execute(&mut self) -> Result<TNResult, KueError> {
    Ok(self.result + self.current)
  }

  fn options(&self) -> TaskOptions {
    TaskOptions::new(0, 0)
  }
}