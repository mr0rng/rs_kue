use crate::{KueLogger, KueTask};

pub struct SillentLogger {
}

impl SillentLogger {
  pub fn new() -> Self { Self { } }
}

impl<R, T: KueTask<R>> KueLogger<R, T> for SillentLogger {
  fn empty_loop_iteration(&mut self) -> Result<(), crate::KueError> {
    Ok(())
  }

  fn task_received(&mut self, _: &T) -> Result<(), crate::KueError> {
    Ok(())
  }

  fn task_executed(&mut self, _: &T, _: &Result<R, crate::KueError>) -> Result<(), crate::KueError> {
    Ok(())
  }
}