extern crate rand;

use std::cmp::Ordering;
use view::View;

mod secret;

pub trait GameEnded {
  fn game_has_ended(&self) -> bool;
}
pub trait CheckGuess {
  fn check_guess(&mut self, guess: u32, output: &View);
}

pub struct Views {
  pub lower: fn(&View),
  pub higher: fn(&View),
  pub exact: fn(&View, u16)
}

pub struct Game {
  secret: u32,
  steps: u16,
  playing: bool,
  views: Views
}

impl GameEnded for Game {
  fn game_has_ended(&self) -> bool {
    !self.playing
  }
}

impl CheckGuess for Game {
  fn check_guess(&mut self, guess: u32, output: &View) {
    self.steps = self.steps + 1;

    match guess.cmp(&self.secret) {
      Ordering::Less    => (self.views.lower)(output),
      Ordering::Greater => (self.views.higher)(output),
      Ordering::Equal   => {
        (self.views.exact)(output, self.steps);
        self.playing = false;
      }
    }
  }
}

impl Game {
  pub fn new(views: Views) -> Game {
    Game {
      secret: secret::compute(1001),
      steps: 0,
      playing: true,
      views
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use view::Output;

  #[test]
  fn constr() {
    let v = Views {
      lower: Output::print_to_small,
      higher: Output::print_to_big,
      exact: Output::print_win_in
    };
    let g = Game::new(v);
    assert_eq!(0, g.steps);
    assert!(g.secret <= 1000);
    assert!(g.secret >= 1);
    assert!(g.playing);
  }
}
