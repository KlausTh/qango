
use board::Board;
use player::Player;
use ::rand::prelude::thread_rng;
use ::rand::Rng;

pub struct Simple {
	name : Box<str>
}

impl Simple {
	pub fn new(name : Box<str>) -> Simple {
		Simple {
			name : name,
		}
	}

	pub fn get_name(&self) -> &str {
		self.name.as_ref()
	}
}

impl Player for Simple {
	fn turn(&self, board : &Board) -> usize {
		let turns : &[usize] = Box::leak(board.turns());
		let mut rng = thread_rng();

		turns[rng.gen_range(0, turns.len())]
	}
}
