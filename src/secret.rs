extern crate rand;

use rand::Rng;

pub fn copute() -> u32 {
  return rand::thread_rng()
    .gen_range(1, 101)
}
