
pub mod simple;
pub mod deep;

use board::Board;

pub trait Player {
	fn turn(&self, board : &Board) -> usize;
}
