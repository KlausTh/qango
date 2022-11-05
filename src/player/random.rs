
use board::Board;
use player::Player;
use ::rand::prelude::thread_rng;
use ::rand::Rng;

pub struct Random {}

impl Random {
	pub fn new() -> Random {
		Random {}
	}
}

impl Player for Random {
	fn turn(&self, board : &Board) -> usize {
		let turns : &[usize] = Box::leak(board.turns());
		let mut rng = thread_rng();

		turns[rng.gen_range(0..turns.len())]
	}
}
