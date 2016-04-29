extern crate time;

pub struct Character {
	health: i32,
	vitality: i32,
	strength: i32,
}

impl Character {
	
	fn age() -> time::Duration {
		time::Duration::zero()
	}
	
	pub fn default() -> Character {
		Character{ 
			health: 0, 
			vitality: 0,
			strength: 0
		}
	}
	
}