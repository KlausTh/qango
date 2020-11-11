
use super::{OFFSET};

#[test]
fn permutations6x6() {
	let perm : Vec<u64> = super::permutations6x6();

	assert_eq!(37, perm.len());
	
	let mut sum : u64 = 0;
	for i in 0..36 {
		sum += perm[i];
		assert_eq!(OFFSET[i+1], sum);
	}
}

//#[test]
//pub fn permutations7x7() {
//	let perm : Vec<u128> = super::permutations7x7();
//	
//	assert_eq!(50, perm.len());
//}