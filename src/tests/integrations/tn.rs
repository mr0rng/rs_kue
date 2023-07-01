use crate::{KueEngine, KueOptions, SillentLogger};

use super::super::triangular_number::TNAdapter;

#[test]
pub fn triangular_numbers() {
  assert_eq!(get_by_number(0), 0);
  assert_eq!(get_by_number(1), 1);
  assert_eq!(get_by_number(2), 3);
  assert_eq!(get_by_number(3), 6);
  assert_eq!(get_by_number(4), 10);
  assert_eq!(get_by_number(5), 15);
}

fn get_by_number (iterations: i32) -> i32 {
  let mut engine = KueEngine::new(
    TNAdapter::new(iterations), 
    KueOptions::new(false, true), 
    SillentLogger::new()
  );

  engine.kue_loop().unwrap();

  engine.adapter.result
}