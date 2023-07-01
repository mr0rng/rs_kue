use crate::{KueTask, KueError};

pub trait KueAdapter<R, T: KueTask<R>> {
  fn get_task(&mut self) -> Result<Option<T>, KueError>;
  fn register_result(&mut self, task: &T, result: &Result<R, KueError>) -> Result<(), KueError>;
}