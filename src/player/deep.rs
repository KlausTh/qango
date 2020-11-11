
use board::{Board, WIN3, WIN4, WIN5};
use board::side::Side;
use player::Player;

// simple Weights:
// 4, 6, 6, 6, 6, 4,
// 6, 9, 9, 9, 9, 6,
// 6, 9,11,11, 9, 6,
// 6, 9,11,11, 9, 6,
// 6, 9, 9, 9, 9, 6,
// 4, 6, 6, 6, 6, 4

#[derive(Clone)]
pub struct Deep {
	level : u8
}

impl Deep {
	pub fn new(level : u8) -> Deep {
		Deep {
			level : level,
		}
	}
}

impl Player for Deep {
	fn turn(&self, board : &Board) -> usize {
		let weights = weights(board);
		let max = weights.iter().max().unwrap();
		let result = weights.iter().position(|w| w == max).unwrap();

		return result;
	}
}

pub fn weights(board : &Board) -> Box<[i32]> {
	let elements = board.get_elements();
	let mut result : Vec<i32> = Vec::new();

	for i in 0..36 {
		if elements[i] != Side::NONE {
			result.push(-1)
		} else {
			result.push(weight(i))
		}
	}

	result.into_boxed_slice()
}

pub fn weight(index : usize) -> i32 {
	let mut result : i32 = 0;

	result += WIN3.iter().filter(|w| w.contains(&index)).count() as i32;
	result += WIN4.iter().filter(|w| w.contains(&index)).count() as i32;
	result += WIN5.iter().filter(|w| w.contains(&index)).count() as i32;

	return result
}

#[cfg(test)]
mod test {
	use super::*;
	use board::START;

	#[test]
	fn start_turn() {
		let player = Deep::new(0);
		let turn = player.turn(&START);

		assert_eq!(turn, 14);
	}
}
