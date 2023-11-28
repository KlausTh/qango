
pub mod pattern;
pub mod polynom;

#[cfg(test)]
mod test;

use board::side::Side::{NONE,WHITE,BLACK};
use super::Board;
use super::side::Side;
use self::pattern::Pattern;
use std::iter::Iterator;
use std::option::Option::{Some,None};

// Permutationen
//
// 0: 36! / 36!              -> 1
// 1: 36! / 35! /  1!        -> *36 /1
// 2: 36! / 34! /  1! /  1!  -> *35 /1 
// 3: 36! / 33! /  2! /  1!  -> *34 /2 
// 4: 36! / 32! /  2! /  2!  -> *33 /2
// 5: 36! / 31! /  3! /  2!  -> *32 /3
// 6: 36! / 30! /  3! /  3!  -> *31 /3
// 7: 36! / 29! /  4! /  3!  -> *30 /4
// ...
//36: 36! /     / 18! / 18!

pub const OFFSET : [u64;37] = [0,1,37,1297,22717,376147,4146067,43101907,335270707,2453494507,14315547787,78370635499,355942682251,
	1512492877051,5477807830651,18506699821051,54336152794651,148388466850351,357393609196351,798626687482351,1592846228397151,
	2943019447952311,4906907767305271,7584937293695671,10709305074484471,14094036837005671,17218404617794471,19862100432308071,
	21750454585532071,22964396541176071,23611832250852871,23913968915368711,24027270164562151,24062676804935101,24071007779140501,
	24072477951059101,24072641303494501];

const ROUNDPATTERN : [Pattern;37] = [
	Pattern {none:36,white: 0,black: 0},Pattern {none:35,white: 1,black: 0},Pattern {none:34,white: 1,black: 1},
	Pattern {none:33,white: 2,black: 1},Pattern {none:32,white: 2,black: 2},Pattern {none:31,white: 3,black: 2},
	Pattern {none:30,white: 3,black: 3},Pattern {none:29,white: 4,black: 3},Pattern {none:28,white: 4,black: 4},
	Pattern {none:27,white: 5,black: 4},Pattern {none:26,white: 5,black: 5},Pattern {none:25,white: 6,black: 5},
	Pattern {none:24,white: 6,black: 6},Pattern {none:23,white: 7,black: 6},Pattern {none:22,white: 7,black: 7},
	Pattern {none:21,white: 8,black: 7},Pattern {none:20,white: 8,black: 8},Pattern {none:19,white: 9,black: 8},
	Pattern {none:18,white: 9,black: 9},Pattern {none:17,white:10,black: 9},Pattern {none:16,white:10,black:10},
	Pattern {none:15,white:11,black:10},Pattern {none:14,white:11,black:11},Pattern {none:13,white:12,black:11},
	Pattern {none:12,white:12,black:12},Pattern {none:11,white:13,black:12},Pattern {none:10,white:13,black:13},
	Pattern {none: 9,white:14,black:13},Pattern {none: 8,white:14,black:14},Pattern {none: 7,white:15,black:14},
	Pattern {none: 6,white:15,black:15},Pattern {none: 5,white:16,black:15},Pattern {none: 4,white:16,black:16},
	Pattern {none: 3,white:17,black:16},Pattern {none: 2,white:17,black:17},Pattern {none: 1,white:18,black:17},
	Pattern {none: 0,white:18,black:18}];

const ROUNDPATTERN7 : [Pattern;50] = [
	Pattern {none:49,white: 0,black: 0},Pattern {none:48,white: 1,black: 0},Pattern {none:47,white: 1,black: 1},
	Pattern {none:46,white: 2,black: 1},Pattern {none:45,white: 2,black: 2},Pattern {none:44,white: 3,black: 2},
	Pattern {none:43,white: 3,black: 3},Pattern {none:42,white: 4,black: 3},Pattern {none:41,white: 4,black: 4},
	Pattern {none:40,white: 5,black: 4},Pattern {none:39,white: 5,black: 5},Pattern {none:38,white: 6,black: 5},
	Pattern {none:37,white: 6,black: 6},Pattern {none:36,white: 7,black: 6},Pattern {none:35,white: 7,black: 7},
	Pattern {none:34,white: 8,black: 7},Pattern {none:33,white: 8,black: 8},Pattern {none:32,white: 9,black: 8},
	Pattern {none:31,white: 9,black: 9},Pattern {none:30,white:10,black: 9},Pattern {none:29,white:10,black:10},
	Pattern {none:28,white:11,black:10},Pattern {none:27,white:11,black:11},Pattern {none:26,white:12,black:11},
	Pattern {none:25,white:12,black:12},Pattern {none:24,white:13,black:12},Pattern {none:23,white:13,black:13},
	Pattern {none:22,white:14,black:13},Pattern {none:21,white:14,black:14},Pattern {none:20,white:15,black:14},
	Pattern {none:19,white:15,black:15},Pattern {none:18,white:16,black:15},Pattern {none:17,white:16,black:16},
	Pattern {none:16,white:17,black:16},Pattern {none:15,white:17,black:17},Pattern {none:14,white:18,black:17},
	Pattern {none:13,white:18,black:18},Pattern {none:12,white:19,black:18},Pattern {none:11,white:19,black:19},
	Pattern {none:10,white:20,black:19},Pattern {none: 9,white:20,black:20},Pattern {none: 8,white:21,black:20},
	Pattern {none: 7,white:21,black:21},Pattern {none: 6,white:22,black:21},Pattern {none: 5,white:22,black:22},
	Pattern {none: 4,white:23,black:22},Pattern {none: 3,white:23,black:23},Pattern {none: 2,white:24,black:23},
	Pattern {none: 1,white:24,black:24},Pattern {none: 0,white:25,black:24}	];


fn create_pattern(round : usize) -> Pattern {
	ROUNDPATTERN[round]
}

pub fn permutations6x6() -> Vec<u64> {
	let mut result: Vec<u64> = Vec::new();

	for r in 0..ROUNDPATTERN.len() {
		let p = ROUNDPATTERN[r];
		result.push(p.permutations());
	}

	result
}

pub fn permutations7x7() -> Vec<u128> {
	let mut result: Vec<u128> = Vec::new();

	for i in 0..ROUNDPATTERN7.len() {
		let p = ROUNDPATTERN7[i];
		result.push(p.permutations128() as u128);
	}

	result
}

pub fn encode(board : &Board) -> u64 {
	let r = board.get_round();
	let pattern = create_pattern(r);

	encode_rec(&mut board.get_elements().iter(), &pattern) + OFFSET[r]
}

//          Nx
//         /
//     >Nxx-Wx
//     /   \  />N
//    /    >Bx--W
// xxx--Wxx   --B
//    \
//     \
//      Bxx
fn encode_rec(iter : &mut dyn Iterator<Item=&Side>, pattern : &Pattern) -> u64 {
	match iter.next() {
		Some(side) => match side {
			NONE  => encode_rec_none(iter, pattern),
			WHITE => encode_rec_white(iter, pattern),
			BLACK => encode_rec_black(iter, pattern)
		}
		None       => 0
	}
}

fn encode_rec_none(iter : &mut dyn Iterator<Item=&Side>, pattern : &Pattern) -> u64 {
	pattern.next(NONE).map_or(0, |n| encode_rec(iter, &n))
}

fn encode_rec_white(iter : &mut dyn Iterator<Item=&Side>, pattern : &Pattern) -> u64 {
	pattern.sub_permutations(NONE) +
	pattern.next(WHITE).map_or(0, |n| encode_rec(iter, &n))
}

fn encode_rec_black(iter : &mut dyn Iterator<Item=&Side>, pattern : &Pattern) -> u64 {
	pattern.sub_permutations(NONE) +
	pattern.sub_permutations(WHITE) +
	pattern.next(BLACK).map_or(0, |n| encode_rec(iter, &n))
}

pub fn decode(code : u64) -> Result<Board, &'static str> {
	match get_round(code) {
		Ok(round) => {
			let mut field : [Side;36] = [NONE;36];
			let pattern = create_pattern(round);

			decode_rec(code - OFFSET[round], &pattern, &mut field);
			Ok(Board{ round : round, fields : field })
		},
		_     => Err("unkown position")
	}
}

fn get_round(offset : u64) -> Result<usize, &'static str> {
	let round = OFFSET.iter().skip(1).position(|i| offset < *i);

	match round {
		Some(r) => Ok(r),
		None    => Err("offset to high")
	}
}

fn decode_rec(pos : u64, pattern : &Pattern, result : &mut [Side;36]) {
	let (side,npos) = get_side_and_subpos(pos, pattern);

	// save result
	let sum = pattern.get_sum();
	if sum > 0 {
		result[36-sum] = side;
	}

	// tailrec next field
	match pattern.next(side) {
		Some(p) => decode_rec(npos, &p, result),
		None    => ()
	}
}

fn get_side_and_subpos(pos : u64, pattern : &Pattern) -> (Side,u64) {
	let permu1 = pattern.sub_permutations(NONE);

	if pos >= permu1 {
		let permu2 = pattern.sub_permutations(WHITE);
		if pos >= permu1 + permu2 {
			(BLACK, pos - permu1 - permu2)
		} else {
			(WHITE, pos - permu1)
		}
	} else {
		(NONE, pos)
	}
}
