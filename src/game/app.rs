use game::game::*; 

pub struct App {
	game: Game,
}

impl App{

  fn run(&self) {
  }
  
  pub fn default() -> App {
    App { game: Game::default() }
  }
  
}