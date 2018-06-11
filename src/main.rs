use game::Views;
use game::CheckGuess;
use game::GameEnded;
use game::Game;
use view::View;
use view::Output;

mod game;
mod view;

fn main() {
  let view = View::new();
  view.print_welcome_message();

  let mut game = Game::new(Views {
    lower: Output::print_to_small,
    higher: Output::print_to_big,
    exact: Output::print_win_in
  });

  loop {
    use view::Output;
    use view::Input;

    view.print_please_input();

    let guess = match view.read_guess() {
      Ok(num) => num, Err(_) => {
        view.print_please_give_number();
        continue;
      }
    };

    game.check_guess(guess, &view);

    if game.game_has_ended() {
      break;
    }
  }
}
