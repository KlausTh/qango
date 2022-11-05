
pub mod side;
pub mod encoding;

use board::side::{Side,Side::{NONE,WHITE,BLACK}};
use std::fmt::{Display,Formatter};

// board index
// 00 01 02 03 04 05
// 06 07 08 09 10 11
// 12 13 14 15 16 17
// 18 19 20 21 22 23
// 24 25 26 27 28 29
// 30 31 32 33 34 35

#[derive(Copy, Clone, Debug)]
pub struct Board {
	fields : [Side; 36],
}

pub const START : Board = Board { fields: [NONE; 36] };

pub const WIN3 : [[usize; 3]; 12] = [
	[00,01,06],[04,05,11],[24,30,31],[29,34,35],  // outer fields
	[02,07,12],[03,10,17],[18,25,32],[23,28,33],  // middle fields
	[08,13,14],[09,15,16],[19,20,26],[21,22,27]]; // inner fields

pub const WIN4 : [[usize; 4]; 17] = [
	[01,02,07,08],[02,03,08,09],[03,04,09,10],[06,07,12,13],[08,09,14,15],[10,11,16,17],
	[12,13,18,19],[13,14,19,20],[14,15,20,21],[15,16,21,22],[16,17,22,23],
	[18,19,24,25],[20,21,26,27],[22,23,28,29],[25,26,31,32],[26,27,32,33],[27,28,33,34]];

pub const WIN5 : [[usize; 5]; 32] = [
	[00,01,02,03,04],[01,02,03,04,05],[06,07,08,09,10],[07,08,09,10,11],
	[12,13,14,15,16],[13,14,15,16,17],[18,19,20,21,22],[19,20,21,22,23],
	[24,25,26,27,28],[25,26,27,28,29],[30,31,32,33,34],[31,32,33,34,35],
	[04,09,14,19,24],[05,10,15,20,25],[10,15,20,25,30],[11,16,21,26,31],
	[01,08,15,22,29],[00,07,14,21,28],[07,14,21,28,35],[06,13,20,27,34],
	[00,06,12,18,24],[06,12,18,24,30],[01,07,13,19,25],[07,13,19,25,31],
	[02,08,14,20,26],[08,14,20,26,32],[03,09,15,21,27],[09,15,21,27,33],
	[04,10,16,22,28],[10,16,22,28,34],[05,11,17,23,29],[11,17,23,29,35]];

impl Board {
	pub fn decode(code : u64) -> Result<Board, &'static str> {
		encoding::decode(code)
	}

	pub fn get_next(&self) -> Side {
		if self.won() != NONE {
			NONE
		} else {
			let nones = self.fields.iter().filter(|s| **s == Side::NONE).count();

			if (nones % 2) == 0 {
				WHITE
			} else {
				BLACK
			}
		}
	}

	pub fn get_element(&self, index : usize) -> Side {
		self.fields[index]
	}

	pub fn get_elements(&self) -> [Side; 36] {
		self.fields.clone()
	}

	pub fn turn(&self, position : usize) -> Board {
		if self.turns().contains(&position) {
			let mut elements = self.fields.clone();

			elements[position] = self.get_next();

			Board {
				fields: elements
			}
		} else {
			*self
		}
	}

	pub fn turns(&self) -> Box<[usize]> {
		if self.won() != NONE {
			Box::new([])
		} else {
			let result : Vec<usize> = self.fields.iter()
				.enumerate()
				.filter(|(_,e)| e.is_empty())
				.map(|(i,_)| i)
				.collect();

			result.into_boxed_slice()
		}
	}
	
	pub fn get_round(&self) -> usize {
		self.fields.iter().filter(|e| !e.is_empty()).count()
	}

	pub fn won(&self) -> Side {
		self.get_winning_fields().into_iter().map(|i| self.fields[i]).fold(NONE, |r,s| r|s)
	}

	pub fn check(&self) -> bool {
		let w1 = WIN3.iter()
				.map(|i| self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]])
				.filter(|s| *s != Side::NONE).count();
		let w2 = WIN4.iter()
				.map(|i| self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]] & self.fields[i[3]])
				.filter(|s| *s != Side::NONE).count();
		let w3 = WIN5.iter()
				.map(|i| self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]] & self.fields[i[3]] & self.fields[i[4]])
				.filter(|s| *s != Side::NONE).count();

		(w1+w2+w3) < 2  // Where can be only one!
	}

	/*
	fn won3(&self) -> Side {
		WIN3.iter()
			.map(|i| self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]])
			.fold(NONE, |r,s| r|s)
	}

	fn won4(&self) -> Side {
		WIN4.iter()
			.map(|i| self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]] & self.fields[i[3]])
			.fold(NONE, |r,s| r|s)
	}

	fn won5(&self) -> Side {
		WIN5.iter()
			.map(|i| self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]] & self.fields[i[3]] & self.fields[i[4]])
			.fold(NONE, |r,s| r|s)
	}
	*/

	pub fn get_winning_fields(&self) -> Vec<usize> {
		let mut result = Vec::<usize>::new();
	
		match self.wins3() {
			Some(w3) => result.extend_from_slice(w3),
			None     => match self.wins4() {
				Some(w4) => result.extend_from_slice(w4),
				None     => match self.wins5() {
					Some(w5) => result.extend_from_slice(w5),
					None     => {},
				}
			}
		}

		result
	}

	fn wins3(&self) -> Option<&[usize; 3]> {
		WIN3.iter()
			.find(|i| (self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]]) != Side::NONE)
	}

	fn wins4(&self) -> Option<&[usize; 4]> {
		WIN4.iter()
			.find(|i| (self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]] & self.fields[i[3]]) != Side::NONE)
	}

	fn wins5(&self) -> Option<&[usize; 5]> {
		WIN5.iter()
			.find(|i| (self.fields[i[0]] & self.fields[i[1]] & self.fields[i[2]] & self.fields[i[3]] & self.fields[i[4]]) != Side::NONE)
	}
}

impl Into<u64> for Board {
	fn into(self) -> u64 {
		encoding::encode(&self)
	}
}

impl From<u64> for Board {
	fn from(c : u64) -> Self {
		encoding::decode(c).unwrap()
	}
}

impl Display for Board {
	fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
		write!(f, "{} {} {} {} {} {}\n{} {} {} {} {} {}\n{} {} {} {} {} {}\n{} {} {} {} {} {}\n{} {} {} {} {} {}\n{} {} {} {} {} {}\nNext: {}\nWon: {}\nMoves: {:?}\n",
				chr(&self.fields[00]),chr(&self.fields[01]),chr(&self.fields[02]),chr(&self.fields[03]),chr(&self.fields[04]),chr(&self.fields[05]),
				chr(&self.fields[06]),chr(&self.fields[07]),chr(&self.fields[08]),chr(&self.fields[09]),chr(&self.fields[10]),chr(&self.fields[11]),
				chr(&self.fields[12]),chr(&self.fields[13]),chr(&self.fields[14]),chr(&self.fields[15]),chr(&self.fields[16]),chr(&self.fields[17]),
				chr(&self.fields[18]),chr(&self.fields[19]),chr(&self.fields[20]),chr(&self.fields[21]),chr(&self.fields[22]),chr(&self.fields[23]),
				chr(&self.fields[24]),chr(&self.fields[25]),chr(&self.fields[26]),chr(&self.fields[27]),chr(&self.fields[28]),chr(&self.fields[29]),
				chr(&self.fields[30]),chr(&self.fields[31]),chr(&self.fields[32]),chr(&self.fields[33]),chr(&self.fields[34]),chr(&self.fields[35]),
				self.get_next(), self.won(), self.turns())
	}
}

fn chr(e : &Side) -> char {
	match e {
		WHITE => 'o',
		NONE => '.',
		BLACK => 'x'
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_check() {
		let mut board = START;

		board = board.turn(9); // white
		board = board.turn(0);
		board = board.turn(10); // white
		board = board.turn(5);
		board = board.turn(15); // white
		board = board.turn(30);
		board = board.turn(16); // white

		assert!(board.check());
	}
}