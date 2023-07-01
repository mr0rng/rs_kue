//! Kue 
//! 
//! Kue is a job queue backed by pg/memory/redis.
//!
//! ```edition2018
//! use kue::{KueAdapter, KueTask, KueError, TaskOptions};
//!
//! type TNResult = i32;
//! 
//! #[derive(Debug, PartialEq)]
//! pub struct TNAdapter {
//!   pub result: i32,
//!   pub current: i32,
//!   pub iterations: i32
//! }
//! 
//! #[derive(Debug, PartialEq)]
//! pub struct TNTask {
//!   pub result: i32, 
//!   pub current: i32
//! }
//! 
//! impl TNAdapter {
//!   pub fn new(iterations: i32) -> Self { 
//!     Self { result: 0, current: 1, iterations } 
//!   }
//! }
//! 
//! impl KueAdapter<TNResult, TNTask> for TNAdapter {
//!   fn get_task(&mut self) -> Result<Option<TNTask>, KueError> {
//!     if self.iterations == 0 {
//!       return Ok(None);
//!     }
//! 
//!     let task = TNTask::new(self.result, self.current);
//!     self.iterations -= 1;
//!     self.current += 1;
//! 
//!     Ok(Some(task))
//!   }
//! 
//!   fn register_result(
//!     &mut self, 
//!     _: &TNTask, 
//!     result: &Result<TNResult, KueError>
//!   ) -> Result<(), KueError> {
//!     match result {
//!       Ok(r) => { 
//!         self.result = *r;
//!         Ok(())
//!       },
//!       Err(e) => Err(e.clone())
//!     }
//!   }
//! }
//! 
//! impl TNTask {
//!   pub fn new(result: i32, current: i32) -> Self { 
//!     Self { result, current } 
//!   }
//! }
//! 
//! impl KueTask<TNResult> for TNTask {
//!   fn execute(&mut self) -> Result<TNResult, KueError> {
//!     Ok(self.result + self.current)
//!   }
//! 
//!  fn options(&self) -> TaskOptions {
//!    TaskOptions::new(0, 0)
//!  }
//! }
//! ```

#[cfg(test)]mod tests;
mod adapter;
mod engine;
mod error;
mod loggers;
mod options;
mod task_options;
mod task;

pub use adapter::*;
pub use engine::*;
pub use error::*;
pub use loggers::*;
pub use options::*;
pub use task_options::*;
pub use task::*;