use super::rand;
use super::rand::Rng;

pub fn compute(high: u32) -> u32 {
  return rand::thread_rng()
    .gen_range(1, high)
}
