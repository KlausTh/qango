
use super::super::side::Side;
use super::polynom::FACULTY;

#[derive(Copy, Clone)]
pub struct Pattern {
	pub none : usize,
	pub white : usize,
	pub black : usize
}

impl Pattern {
	pub fn is_empty(&self) -> bool {
		self.none == 0 && self.white == 0 && self.black == 0
	}

	pub fn get_sum(&self) -> usize {
		self.none + self.white + self.black
	}

	pub fn next(&self, side : Side) -> Option<Self> {
		match side {
			Side::NONE => if self.none > 0 {
					Some(Pattern {none : self.none-1, white : self.white, black : self.black})
				} else {
					None
				},
			Side::WHITE => if self.white > 0 {
					Some(Pattern {none : self.none, white : self.white-1, black : self.black})
				} else {
					None
				},
			Side::BLACK => if self.black > 0 {
					Some(Pattern {none : self.none, white : self.white, black : self.black-1})
				} else {
					None
				}
		}
	}

	pub fn sub_permutations(&self, side : Side) -> u64 {
		self.next(side).map_or(0, |p| p.permutations())
	}

	pub fn permutations(&self) -> u64 {
		let fac = FACULTY[self.get_sum()] / FACULTY[self.none] / FACULTY[self.white] / FACULTY[self.black];

		fac.into()
	}
}
