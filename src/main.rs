extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number between 1 and 100!");

  let secret = rand::thread_rng()
    .gen_range(1, 101);

  let mut steps: u32 = 0;
  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess
      .trim()
      .parse() {
        Ok(num) => num,
        Err(_)  => {
          println!("Please type a number!");
          continue;
        }
      };

    steps = steps + 1;

    match guess.cmp(&secret) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win in {} guesses!", steps);
        break;
      }
    }
  }
}
