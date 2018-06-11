extern crate rand;

use game::Views;
use game::CheckGuess;
use game::GameEnded;
use game::Game;

mod secret;
mod view;
mod game;

fn main() {
  view::print_welcome_message();

  let mut game = Game::new(Views {
    lower: view::print_to_small,
    higher: view::print_to_big,
    exact: view::print_win_in
  });

  loop {
    view::print_please_input();

    let guess = match view::read_guess() {
      Ok(num) => num, Err(_) => {
        view::print_please_give_number();
        continue;
      }
    };


    game.check_guess(guess);

    if game.game_has_ended() {
      break;
    }
  }
}
