use crate::{KueTask, KueError};

pub trait KueLogger<R, T: KueTask<R>> {
  fn empty_loop_iteration(&mut self) -> Result<(), KueError>;
  fn task_received(&mut self, task: &T) -> Result<(), KueError>;
  fn task_executed(&mut self, task: &T, result: &Result<R, KueError>) -> Result<(), KueError>;
}