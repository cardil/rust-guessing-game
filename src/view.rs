use std::io;
use std::num::ParseIntError;
use std::io::Stdin;

pub struct View {
  print: fn(String),
  read: fn() -> Stdin
}

pub trait Input {
  fn read_guess(&self) -> Result<u32, ParseIntError>;
}
pub trait Output {
  fn print_welcome_message(&self);
  fn print_please_input(&self);
  fn print_please_give_number(&self);
  fn print_to_small(&self);
  fn print_to_big(&self);
  fn print_win_in(&self, steps: u16);
}

impl Input for View {
  fn read_guess(&self) -> Result<u32, ParseIntError> {
    let mut guess = String::new();
    (self.read)()
      .read_line(&mut guess)
      .expect("Failed to read line");
    guess
      .trim()
      .parse()
  }
}

impl Output for View {
  fn print_welcome_message(&self) {
    (self.print)("Guess the number between 1 and 1000!".to_string())
  }

  fn print_please_input(&self) {
    (self.print)("Please input your guess.".to_string())
  }

  fn print_please_give_number(&self) {
    (self.print)("Please provide a number!".to_string())
  }

  fn print_to_small(&self) {
    (self.print)("Too small!".to_string())
  }

  fn print_to_big(&self) {
    (self.print)("Too big!".to_string())
  }

  fn print_win_in(&self, steps: u16) {
    (self.print)(format!("You won in {} guesses!", steps))
  }
}

impl View {
  pub fn new() -> View {
    View::of(
      |s| { println!("{}", s) },
      io::stdin
    )
  }

  fn of(print: fn(String), read: fn() -> Stdin) -> View {
    View {
      print, read
    }
  }
}

#[cfg(test)]
mod tests {
  use super::View;

  #[test]
  fn test_print_win_in() {
    let mut out = String::new();
//    let v = View::of(
//      |s| { out = out + s },
//      io::stdin
//    );
//    print_win_in(42)
  }
}
