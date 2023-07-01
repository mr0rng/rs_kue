use crate::{KueError, TaskOptions};

pub trait KueTask<R> {
  fn execute(&mut self) -> Result<R, KueError>;
  fn options(&self) -> TaskOptions;
}