
use std::ops::{Div, Mul};
use std::cmp::PartialEq;
use std::convert::Into;

#[cfg_attr(all(target_arch="x86_64",target_feature="sse3"), withsse)]

#[cfg(withsse)]
use std::arch::x86_64::*;

#[cfg_attr(all(target_arch="aarch64",target_feature="neon"), withneon)]

#[cfg(withneon)]
use std::arch::aarch64::*;

const SIZE : usize = 16;
const PRIM : [u32;SIZE] = [2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53];

#[derive(Copy, Clone)]
pub struct Polynom {
	exp : [u8;SIZE] // 8 bits * SIZE = 128 bits for SSE2!
}

pub const FACULTY : [Polynom;50] = [
	Polynom {exp:[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 0!
	Polynom {exp:[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 1! =  1
	Polynom {exp:[ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 2! =  1! 2
	Polynom {exp:[ 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 3! =  2! 3
	Polynom {exp:[ 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 4! =  3! 2*2
	Polynom {exp:[ 3, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 5! =  4! 5
	Polynom {exp:[ 4, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 6! =  5! 2*3
	Polynom {exp:[ 4, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 7! =  6! 7
	Polynom {exp:[ 7, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 8! =  7! 2*2*2
	Polynom {exp:[ 7, 4, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   // 9! =  8! 3*3
	Polynom {exp:[ 8, 4, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //10! =  9! 2*5
	Polynom {exp:[ 8, 4, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //11! = 10! 11
	Polynom {exp:[10, 5, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //12! = 11! 2*2*3
	Polynom {exp:[10, 5, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //13! = 12! 13
	Polynom {exp:[11, 5, 2, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //14! = 13! 2*7
	Polynom {exp:[11, 6, 3, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //15! = 14! 3*5
	Polynom {exp:[15, 6, 3, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //16! = 15! 2*2*2*2
	Polynom {exp:[15, 6, 3, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //17! = 16! 17
	Polynom {exp:[16, 8, 3, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]},   //18! = 17! 2*3*3
	Polynom {exp:[16, 8, 3, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]},   //19! = 18! 19
	Polynom {exp:[18, 8, 4, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]},   //20! = 19! 2*2*5
	Polynom {exp:[18, 9, 4, 3, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]},   //21! = 20! 3*7
	Polynom {exp:[19, 9, 4, 3, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]},   //22! = 21! 2*11
	Polynom {exp:[19, 9, 4, 3, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]},   //23! = 22! 23
	Polynom {exp:[22,10, 4, 3, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]},   //24! = 23! 2*2*2*3
	Polynom {exp:[22,10, 6, 3, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]},   //25! = 24! 5*5
	Polynom {exp:[23,10, 6, 3, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]},   //26! = 25! 2*13
	Polynom {exp:[23,13, 6, 3, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]},   //27! = 26! 3*3*3
	Polynom {exp:[25,13, 6, 4, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]},   //28! = 27! 2*2*7
	Polynom {exp:[25,13, 6, 4, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]},   //29! = 28! 29
	Polynom {exp:[26,14, 7, 4, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]},   //30! = 29! 2*3*5
	Polynom {exp:[26,14, 7, 4, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]},   //31! = 30! 31
	Polynom {exp:[31,14, 7, 4, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]},   //32! = 31! 2*2*2*2*2
	Polynom {exp:[31,15, 7, 4, 3, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0]},   //33! = 32! 3*11
	Polynom {exp:[32,15, 7, 4, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0]},   //34! = 33! 2*17
	Polynom {exp:[32,15, 8, 5, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0]},   //35! = 34! 5*7
	Polynom {exp:[34,17, 8, 5, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0]},   //36! = 35! 2*2*3*3
	Polynom {exp:[34,17, 8, 5, 3, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0, 0]},   //37! = 36! 37
	Polynom {exp:[35,17, 8, 5, 3, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0]},   //38! = 37! 2*19
	Polynom {exp:[35,18, 8, 5, 3, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0]},   //39! = 38! 3*13
	Polynom {exp:[38,18, 9, 5, 3, 3, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0]},   //40! = 39! 2*2*2*5
	Polynom {exp:[38,18, 9, 5, 3, 3, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0]},   //41! = 40! 41
	Polynom {exp:[39,19, 9, 6, 3, 3, 2, 2, 1, 1, 1, 1, 1, 0, 0, 0]},   //42! = 41! 2*3*7
	Polynom {exp:[39,19, 9, 6, 3, 3, 2, 2, 1, 1, 1, 1, 1, 1, 0, 0]},   //43! = 42! 43
	Polynom {exp:[41,19, 9, 6, 4, 3, 2, 2, 1, 1, 1, 1, 1, 1, 0, 0]},   //44! = 43! 2*2*11
	Polynom {exp:[41,21,10, 6, 4, 3, 2, 2, 1, 1, 1, 1, 1, 1, 0, 0]},   //45! = 44! 3*3*5
	Polynom {exp:[42,21,10, 6, 4, 3, 2, 2, 2, 1, 1, 1, 1, 1, 0, 0]},   //46! = 45! 2*23
	Polynom {exp:[42,21,10, 6, 4, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 0]},   //47! = 46! 47
	Polynom {exp:[46,22,10, 6, 4, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 0]},   //48! = 47! 2*2*2*2*3
	Polynom {exp:[46,22,10, 8, 4, 3, 2, 2, 2, 1, 1, 1, 1, 1, 1, 0]}    //49! = 48! 7*7
	//50! = 49! 2*5*5
	//51! = 50! 3*17
	//52! = 51! 2*2*13
	//53! = 52! 53
];

impl Into<u64> for Polynom {
	fn into(self) -> u64 {
		self.exp.iter().enumerate()
			.map(|(i, e)| (PRIM[i] as u64).pow(*e as u32))
			.fold(1, |x,y| x*y)
	}
}

impl From<u64> for Polynom {
	fn from(c : u64) -> Polynom {
		let mut z = c;
		let mut poly = Polynom {exp:[0;SIZE]};

		for i in 0..SIZE {
			let mut e = 0;

			while z % (PRIM[i] as u64) == 0 {
				e += 1;
				z /= PRIM[i] as u64;
			}

			poly.exp[i] = e;
		}
		if z != 1 {
			panic!("Rest after factorization is {}",z);
		}
		poly
	}
}

impl Into<u128> for Polynom {
	fn into(self) -> u128 {
		self.exp.iter().enumerate()
			.map(|(i, e)| (PRIM[i] as u128).pow(*e as u32))
			.fold(1, |x,y| x*y)
	}
}

impl From<u128> for Polynom {
	fn from(c : u128) -> Polynom {
		let mut z = c;
		let mut poly = Polynom {exp:[0;SIZE]};

		for i in 0..SIZE {
			let mut e = 0;

			while z % (PRIM[i] as u128) == 0 {
				e += 1;
				z /= PRIM[i] as u128;
			}

			poly.exp[i] = e;
		}
		if z != 1 {
			panic!("Rest after factorization is {}",z);
		}
		poly
	}
}

impl Div for Polynom {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		let mut result = Polynom {exp:[0;SIZE]};

		#[cfg(withsse)]
		unsafe {
			sse_div(&self.exp, &rhs.exp, &mut result.exp);
		}

		#[cfg(withneon)]
		unsafe {
			neon_div(&self.exp, &rhs.exp, &mut result.exp);
		}

		#[cfg(not(any(withsse,withneon)))]
		for i in 0..SIZE {
			result.exp[i] = self.exp[i] - rhs.exp[i];
		}

		result
	}
}

#[cfg(withsse)]
#[inline]
unsafe fn sse_div(exp1 : &[u8;SIZE], exp2 : &[u8;SIZE], result : &mut [u8;SIZE]) {
	let r1 = _mm_loadu_si128(exp1.as_ptr() as *const __m128i);
	let r2 = _mm_loadu_si128(exp2.as_ptr() as *const __m128i);
	let r3 = _mm_sub_epi8(r1, r2);

	_mm_storeu_si128(result.as_ptr() as *mut __m128i, r3);
}

#[cfg(withneon)]
#[inline]
unsafe fn neon_div(exp1 : &[u8;SIZE], exp2 : &[u8;SIZE], result : &mut [u8;SIZE]) {
	let r1 : uint8x16_t = vld1q_u8(exp1.as_ptr() as *const u8);
	let r2 : uint8x16_t = vld1q_u8(exp2.as_ptr() as *const u8);
	//let r3 : uint8x16_t = vsubq_u8(r1, r2);
	let r3 : uint8x16_t = vsubq_u8(r2, r1);

	vst1q_u8(result.as_ptr() as *mut _, r3);
}

impl Mul for Polynom {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		let mut result = Polynom {exp:[0;SIZE]};

		#[cfg(withsse)]
		unsafe {
			sse_mul(&self.exp, &rhs.exp, &mut result.exp);
		}

		#[cfg(withneon)]
		unsafe {
			neon_mul(&self.exp, &rhs.exp, &mut result.exp);
		}

		#[cfg(not(any(withsse,withneon)))]
		for i in 0..SIZE {
			result.exp[i] = self.exp[i] + rhs.exp[i];
		}

		result
	}
}

#[cfg(withsse)]
#[inline]
unsafe fn sse_mul(exp1 : &[u8;SIZE], exp2 : &[u8;SIZE], result : &mut[u8;SIZE]) {
	let r1 = _mm_loadu_si128(exp1.as_ptr() as *const _);
	let r2 = _mm_loadu_si128(exp2.as_ptr() as *const _);
	let r3 = _mm_add_epi8(r1, r2);

	_mm_storeu_si128(result.as_ptr() as *mut _, r3);
}

#[cfg(withneon)]
#[inline]
unsafe fn neon_mul(exp1 : &[u8;SIZE], exp2 : &[u8;SIZE], result : &mut[u8;SIZE]) {
	let r1 : uint8x16_t = vld1q_u8(exp1.as_ptr() as *const u8);
	let r2 : uint8x16_t = vld1q_u8(exp2.as_ptr() as *const u8);
	let r3 : uint8x16_t = vaddq_u8(r1, r2);

	vst1q_u8(result.as_ptr() as *mut _, r3);
}

impl PartialEq for Polynom {
	fn eq(&self, other: &Self) -> bool {
		self.exp[ 0] == other.exp[ 0] && self.exp[ 1] == other.exp[ 1] && self.exp[ 2] == other.exp[ 2] &&
		self.exp[ 3] == other.exp[ 3] && self.exp[ 4] == other.exp[ 4] && self.exp[ 5] == other.exp[ 5] &&
		self.exp[ 6] == other.exp[ 6] && self.exp[ 7] == other.exp[ 7] && self.exp[ 8] == other.exp[ 8] &&
		self.exp[ 9] == other.exp[ 9] && self.exp[10] == other.exp[10] && self.exp[11] == other.exp[11] &&
		self.exp[12] == other.exp[12] && self.exp[13] == other.exp[13] && self.exp[14] == other.exp[14] &&
		self.exp[15] == other.exp[15]
	}
}

impl Eq for Polynom {}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn factorization() {
		let p = Polynom::from(12u64);

		assert_eq!(p.exp, [ 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
	}

	#[test]
	fn info() {
		let p : Polynom = FACULTY[3];
		assert_eq!(Into::<u64>::into(p), 6);
	}

	#[test]
	fn mul() {
		let d = FACULTY[3] * FACULTY[3];
		assert_eq!(Into::<u64>::into(d), 36);
	}

	#[test]
	fn div() {
		let d = FACULTY[4] / FACULTY[3];
		assert_eq!(Into::<u64>::into(d), 4);
	}

	#[test]
	fn eq() {
		assert!(FACULTY[5] == FACULTY[5]);
	}
}
