pub struct KueOptions {
  pub is_infinite_loop: bool,
  pub stop_on_error: bool
}

impl KueOptions {
  pub fn new(is_infinite_loop: bool, stop_on_error: bool) -> Self { 
    Self { is_infinite_loop, stop_on_error } 
  }
}