use board::{Board, WIN3, WIN4, WIN5};
use board::side::Side;

/// evaluate the board
/// the lower the score, the better for white
/// the higher the score, the better for black
pub fn eval(board : &Board) -> i32 {
    let mut sum : i32 = 0;

    for i in 0..12 {
        let counter = WIN3[i].iter().map(|i| board.get_element(*i)).fold((0,0), |acc,side| count_sides(&acc, &side));

        sum += eval_counter(&counter, 2);
    }

    for i in 0..17 {
        let counter = WIN4[i].iter().map(|i| board.get_element(*i)).fold((0,0), |acc,side| count_sides(&acc, &side));

        sum += eval_counter(&counter, 1);
    }

    for i in 0..32 {
        let counter = WIN5[i].iter().map(|i| board.get_element(*i)).fold((0,0), |acc,side| count_sides(&acc, &side));

        sum += eval_counter(&counter, 0);
    }

    return sum;
}

fn count_sides(counter : &(u32,u32), side : & Side) -> (u32,u32) {
    match side {
        Side::NONE  => *counter,
        Side::WHITE => (counter.0+1, counter.1),
        Side::BLACK => (counter.0, counter.1+1)
    }
}

fn eval_counter(count : &(u32,u32), bonus : u32) -> i32 {
    if count.0 == 0 && count.1 != 0 {
        return 10_i32.pow(count.1 + bonus - 1);
    }

    if count.0 != 0 && count.1 == 0 {
        return -10_i32.pow(count.0 + bonus - 1);
    }

    return 0;
}

#[cfg(test)]
mod test {
	use super::*;
	use board::*;

	#[test]
	fn test_eval() {
		let mut board = START;

		board = board.turn(9); // white
		board = board.turn(0);
		board = board.turn(10); // white
		board = board.turn(5);
		board = board.turn(15); // white
		board = board.turn(30);

        println!("{}", board);
        let score = eval(&board);
        
        println!("score = {}", score);
        assert!(score == -1079);
	}
}