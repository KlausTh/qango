
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::Not;

#[derive(Copy, Clone, PartialEq)]
pub enum Side { WHITE=-1, NONE=0, BLACK=1 }

impl Side {
	pub fn is_empty(&self) -> bool {
		*self == Side::NONE
	} 
}

impl Display for Side {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			Side::WHITE => write!(f, "WHITE"),
			Side::NONE => write!(f, "NONE"),
			Side::BLACK => write!(f, "BLACK")
		}
	}
}

impl Not for Side {
	type Output = Self;

	fn not(self) -> Self {
		match self {
			Side::WHITE => Side::BLACK,
			Side::BLACK => Side::WHITE,
			_ => Side::NONE
		}
	}
}

impl BitAnd for Side {
	type Output = Self;
	
	fn bitand(self, rhs: Self) -> Self {
		match self {
			Side::WHITE => match rhs {
				Side::WHITE => self,
				Side::NONE => Side::NONE,
				Side::BLACK => Side::NONE
			},
			Side::NONE => self,
			Side::BLACK => match rhs {
				Side::WHITE => Side::NONE,
				Side::NONE => Side::NONE,
				Side::BLACK => self
			},
		}
	}
}

impl BitOr for Side {
	type Output = Self;
	
	fn bitor(self, rhs: Self) -> Self {
		match self {
			Side::NONE => rhs,
			_ => self
		}
	}
}

impl BitOrAssign for Side {
	fn bitor_assign(&mut self, rhs: Self) {
		match self {
			Side::NONE => *self = rhs,
			_ => {},
		}
	}
}
