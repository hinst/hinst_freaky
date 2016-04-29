use game::character::*;

pub struct Game {
	character: Character,
}

impl Game {
	
	pub fn default() -> Game {
		Game { character: Character::default() }
	}
	
}