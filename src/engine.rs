use std::marker::PhantomData;

use crate::{KueTask, KueAdapter, KueError, KueOptions, KueLogger};

pub struct KueEngine<
  R, 
  T: KueTask<R>, 
  A: KueAdapter<R, T>,
  L: KueLogger<R, T>
> {
  r: PhantomData<R>,
  t: PhantomData<T>,
  pub adapter: A,
  pub options: KueOptions,
  pub logger: L
}

impl<R, T: KueTask<R>, A: KueAdapter<R, T>, L: KueLogger<R, T>> KueEngine<R, T, A, L> {
  pub fn new(adapter: A, options: KueOptions, logger: L) -> Self { 
    Self { r: PhantomData, t: PhantomData, adapter, options, logger } 
  }

  pub fn kue_loop (&mut self) -> Result<(), KueError> {
    loop {
      let mut task = match self.adapter.get_task()? {
        Some(t) => {
          self.logger.task_received(&t)?;
          t
        },
        None => {
          self.logger.empty_loop_iteration()?;
          if self.options.is_infinite_loop {
            continue;
          }

          return Ok(());
        }
      };

      let result = task.execute();
      self.logger.task_executed(&task, &result)?;
      self.adapter.register_result(&task, &result)?;

      match result {
        Ok(_) => { },
        Err(e) => if self.options.stop_on_error { return Err(e) }
      };
    }
  }
}