
use board::Board;
use board::side::Side;
use player::Player;
use super::evaluate::eval;

#[derive(Clone)]
pub struct Deep {
	name : Box<str>,
	level : u8
}

impl Deep {
	pub fn new(name : Box<str>, level : u8) -> Deep {
		Deep {
			name : name,
			level : level
		}
	}

	fn next_turn(&self, board : &Board, deep : u8) -> i32 {
		// did I already won?
		match board.won() {
			Side::WHITE => return eval(board),
			Side::BLACK => return eval(board),
			_ => (),
		}

		// no more calls please
		if deep == self.level {
			return eval(board);
		}

		// okay your turn
		let turns = board.turns();
		let scores = turns.iter().map(|index| self.next_turn(&board.turn(*index),deep+1));

		match board.get_next() {
			Side::WHITE => scores.min().unwrap(),
			Side::BLACK => scores.max().unwrap(),
			_ => panic!("should not happend")
		}
	}
}

impl Player for Deep {
	fn turn(&self, board : &Board) -> usize {
		let turns = board.turns();
		let iter = turns.iter();

		match board.get_next() {
			Side::WHITE => *iter.min_by_key(|index| self.next_turn(&board.turn(**index),0)).unwrap(),
			Side::BLACK => *iter.max_by_key(|index| self.next_turn(&board.turn(**index),0)).unwrap(),
			_ => board.turns()[0]
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use board::START;

	#[test]
	fn turn_test_0() {
		let player = Deep::new(Box::from("Deep0"),0);
		let turn = player.turn(&START);

		assert_eq!(turn, 14, "turn = {}", turn);
	}

	#[test]
	fn turn_test_6093551267() {
		let player = Deep::new(Box::from("Deep2"),2);
		let board = Board::from(6093551267_u64);

		let turn = player.turn(&board);

		assert_eq!(turn, 23, "turn = {}", turn);
	}

	#[test]
	fn turn_test_23877844226924() {
		let player = Deep::new(Box::from("Deep1"),1);
		let board = Board::from(23877844226924_u64);

		let turn = player.turn(&board);

		assert!(turn==5 || turn==30 || turn==28, "turn = {}", turn);
	}

	
}
