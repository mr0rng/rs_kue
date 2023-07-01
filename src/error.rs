#[derive(Clone, Debug, PartialEq)]
pub enum KueError {
  Adapter(String),
  Task(String)
}