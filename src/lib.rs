
extern crate rand;

pub mod board;
pub mod player;

use board::side::Side;
use board::START;
use board::Board;
use player::Player;
use std::collections::LinkedList;
use std::rc::Rc;

#[derive(Clone)]
pub struct Game {
	boards : LinkedList<Board>,
	white : Rc<dyn Player>,
	black : Rc<dyn Player>,
}

impl Game {
	pub fn new(white : Box<dyn Player>, black : Box<dyn Player>) -> Self {
		let mut boards = LinkedList::new();

		boards.push_back(START);
		Game {
			boards : boards,
			white : Rc::from(white),
			black : Rc::from(black),
		}
	}

	pub fn get_board(&self) -> &Board {
		self.boards.back().unwrap()
	}

	fn get_player(&self, side : Side) -> Rc<dyn Player> {
		match side {
			Side::WHITE => self.white.clone(),
			Side::BLACK => self.black.clone(),
			Side::NONE  => panic!("no more move is possible"),
		}
	}

	pub fn step(&mut self) -> Side {
		let clone = self.clone();
		let board : Board = *clone.boards.back().unwrap();
		let side = board.get_next();

		if side != Side::NONE {
			let player = clone.get_player(side);
			let mov : usize = player.turn(&board);
			let next_board : Board = board.turn(mov);

			self.boards.push_back(next_board);
			next_board.won()
		} else {
			board.won()
		}
	}
}
