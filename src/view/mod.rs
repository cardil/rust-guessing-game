use std::io;
use std::num::ParseIntError;

pub fn read_guess() -> Result<u32, ParseIntError> {
  let mut guess = String::new();
  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
  guess
    .trim()
    .parse()
}

pub fn print_welcome_message() {
  println!("Guess the number between 1 and 100!")
}

pub fn print_please_input() {
  println!("Please input your guess.")
}

pub fn print_please_give_number() {
  println!("Please provide a number!")
}

pub fn print_to_small() {
  println!("Too small!")
}

pub fn print_to_big() {
  println!("Too big!")
}

pub fn print_win_in(steps: u16) {
  println!("You won in {} guesses!", steps)
}
