
use board::Board;
use board::side::Side;
use player::Player;
use std::collections::HashMap;

static WEIGHTS : [u32;36] = [
	4, 6, 6, 6, 6, 4,
	6, 9, 9, 9, 9, 6,
	6, 9,11,11, 9, 6,
	6, 9,11,11, 9, 6,
	6, 9, 9, 9, 9, 6,
	4, 6, 6, 6, 6, 4
];

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

	fn next_turn(&self, visited : &mut HashMap<u64, Side>, board : &Board, deep : u8) -> Side {
		// is it in the cache?
		let num : u64 = Board::into(*board);
		if visited.contains_key(&num) {
			return *visited.get(&num).unwrap();
		}

		// did I already won?
		if board.won() != Side::NONE {
			let won = board.won();

			visited.insert(num, won);

			return won;
		}

		// no more calls please
		if deep == self.level {
			visited.insert(num, Side::NONE);

			return Side::NONE
		}

		// okay your turn
		let turns = board.turns();
		let you : Side = board.get_next();
		let mut i_win : Side = !you;

		for t in turns.iter() {
			let next_board = board.turn(*t);
			let win_turn = self.next_turn(visited, &next_board, deep+1);

			// if you win one turn -> you win
			if win_turn == you {
				visited.insert(num, you);

				return you;
			}

			// if i win every turn -> i win
			i_win = i_win & win_turn;
		}
		visited.insert(num, i_win);

		return i_win;
	}
}

impl Player for Deep {
	fn turn(&self, board : &Board) -> usize {
		let mut visited : HashMap<u64, Side> = HashMap::with_capacity(10000);
		let turns = board.turns();
		let me = board.get_next();
		let mut turn : usize = turns[0];
		let mut weight : u32 = 0;

		for t in turns.iter() {
			let next_board = board.turn(*t);
			let win = self.next_turn(&mut visited, &next_board, 0);

			if win == me {
				return *t;
			}

			if win == Side::NONE {
				if weight < WEIGHTS[*t] {
					turn = *t;
					weight = WEIGHTS[*t];
				}
			}
		}

		return turn;
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use board::START;

	#[test]
	fn start_turn() {
		let player = Deep::new(Box::from("Deep0"),0);
		let turn = player.turn(&START);

		assert_eq!(turn, 14);
	}
}
