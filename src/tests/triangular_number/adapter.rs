use crate::{KueAdapter, KueError};

use super::{TNResult, TNTask};

#[derive(Debug, PartialEq)]
pub struct TNAdapter {
  pub result: i32,
  pub current: i32,
  pub iterations: i32
}

impl TNAdapter {
  pub fn new(iterations: i32) -> Self { 
    Self { result: 0, current: 1, iterations } 
  }
}

impl KueAdapter<TNResult, TNTask> for TNAdapter {
  fn get_task(&mut self) -> Result<Option<TNTask>, KueError> {
    if self.iterations == 0 {
      return Ok(None);
    }

    let task = TNTask::new(self.result, self.current);
    self.iterations -= 1;
    self.current += 1;

    Ok(Some(task))
  }

  fn register_result(
    &mut self, 
    _: &TNTask, 
    result: &Result<TNResult, KueError>
  ) -> Result<(), KueError> {
    match result {
      Ok(r) => { 
        self.result = *r;
        Ok(())
      },
      Err(e) => Err(e.clone())
    }
  }
}