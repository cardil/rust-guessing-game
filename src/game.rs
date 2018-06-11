use secret;
use std::cmp::Ordering;

pub trait GameEnded {
  fn game_has_ended(&self) -> bool;
}
pub trait CheckGuess {
  fn check_guess(&mut self, guess: u32);
}

pub struct Views {
  pub lower: fn(),
  pub higher: fn(),
  pub exact: fn(steps: u16)
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
  fn check_guess(&mut self, guess: u32) {
    self.steps = self.steps + 1;

    match guess.cmp(&self.secret) {
      Ordering::Less    => (self.views.lower)(),
      Ordering::Greater => (self.views.higher)(),
      Ordering::Equal   => {
        (self.views.exact)(self.steps);
        self.playing = false;
      }
    }
  }
}

impl Game {
  pub fn new(views: Views) -> Game {
    Game {
      secret: secret::copute(),
      steps: 0,
      playing: true,
      views
    }
  }
}
