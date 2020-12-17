
pub mod simple;
pub mod deep;
pub mod evaluate;
pub mod random;

use board::Board;

pub trait Player {
	fn turn(&self, board : &Board) -> usize;
}
