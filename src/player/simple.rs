
use board::Board;
use player::Player;

// simple Weights:
static WEIGHTS : [u32;36] = [
	4, 6, 6, 6, 6, 4,
	6, 9, 9, 9, 9, 6,
	6, 9,11,11, 9, 6,
	6, 9,11,11, 9, 6,
	6, 9, 9, 9, 9, 6,
	4, 6, 6, 6, 6, 4
];

#[derive(Clone)]
pub struct Simple {
	name : Box<str>
}

impl Simple {
	pub fn new(name : Box<str>) -> Simple {
		Simple {
			name : name
		}
	}
}

impl Player for Simple {
	fn turn(&self, board : &Board) -> usize {
		can_win(board).or_else(|| do_not_loose(board)).unwrap()
	}
}

fn can_win(board : &Board) -> Option<usize> {
	board.turns().iter().find(|p| board.turn(**p).won() == board.get_next()).map(|t| *t)
}

fn do_not_loose(board : &Board) -> Option<usize> {
	let myturn = best_turn(board);
	let nextboard = board.turn(myturn);

	return can_win(&nextboard).or(Some(myturn));
}

fn best_turn(board : &Board) -> usize {
	best_weight(board.turns()).unwrap()
}

fn best_weight(turns : Box<[usize]>) -> Option<usize> {
	turns.iter().max_by_key(|t| WEIGHTS[**t]).map(|t| *t)
}

#[cfg(test)]
mod test {
	use super::*;
	use board::START;

	#[test]
	fn start_turn() {
		let player = Simple::new(Box::from("test Player"));
		let turn = player.turn(&START);

		assert!([14,15,20,21].binary_search(&turn).is_ok());
	}
}
