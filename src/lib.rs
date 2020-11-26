
extern crate rand;

pub mod board;
pub mod player;

use board::side::Side;
use board::START;
use board::Board;
use player::Player;
use std::collections::LinkedList;

pub struct Game {
	boards : LinkedList<Board>,
	white : Box<dyn Player>,
	black : Box<dyn Player>,
}

impl Game {
	pub fn new(white : Box<dyn Player>, black : Box<dyn Player>) -> Self {
		let mut boards = LinkedList::new();

		boards.push_back(START);
		Game {
			boards : boards,
			white : white,
			black : black,
		}
	}

	pub fn get_board(&self) -> &Board {
		self.boards.back().unwrap()
	}

	fn get_player(&self, side : Side) -> & dyn Player {
		match side {
			Side::WHITE => self.white.as_ref(),
			Side::BLACK => self.black.as_ref(),
			Side::NONE  => panic!("where is no player at Side:NONE"),
		}
	}

	pub fn step(&mut self) -> Side {
		let board : &Board = self.boards.back().unwrap();
		let side = board.get_next();

		if side != Side::NONE {
			let player = self.get_player(side);
			let mov = player.turn(&board.clone());
			let next_board : Board = board.turn(mov);

			self.boards.push_back(next_board);
			next_board.won()
		} else {
			board.won()
		}
	}
}
