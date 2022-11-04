// https://www.codewars.com/kata/5a24a35a837545ab04001614/train/rust
// 5 kyu

// In this kata, your task is to implement what I call Interlaced Spiral Cipher (ISC).

// Encoding a string using ISC is achieved with the following steps:

// 1.Form a square large enough to fit all the string characters
// 2.Starting with the top-left corner, place string characters in the corner cells 
// moving in a clockwise direction
// 3.After the first cycle is complete, continue placing characters in the cells following t
// he last one in its respective row/column
// 4.When the outer cells are filled, repeat steps 2 through 4 for the remaining inner 
// squares (refer to the example below for further clarification)
// 5.Fill up any unused cells with a space character and return the rows joined together.

#[cfg(test)]
mod example_tests {
	use super::*;
	
	fn run_test(s1:&str, s2:&str){
		assert_eq!(&isc::encode(s1),s2);
		//assert_eq!(&isc::decode(s2),s1);
	}
	
	#[test]
	fn example_test_1() {
		let example_1a = "Kanasta_onpa_paska_peli!!";
		let example_1b = "Kso_askeata!!ln_pi_paapan";
		run_test(example_1a,example_1b);
	}
	
	#[test]
	fn example_test_2() {
		let example_2a = "LoremIpsu";
		let example_2b = "LmosuIepr";
		run_test(example_2a,example_2b);
	}

	#[test]
	fn example_test_3() {
		let example_3a = "Lorem>ipsum>dolor>sitamet>consectetur>adipiscing!";
		let example_3b = "Lmsdrtoetntr>>idicpsuoug!ieo>csni>>poatecqemslmir";
		run_test(example_3a,example_3b);
	}

	#[test]
	fn example_test_4() {
		let example_4a = "RoMani_Ite>Doxum";
		let example_4b = "RntoDoxiImuea>_M";
		run_test(example_4a,example_4b);
	}
}

mod isc {
    use std::thread::current;

	fn build_square(size: usize) -> Vec<char> {
		let mut square = Vec::with_capacity(size*size);
		square
		//if square.len() == size {
	}

	fn get_current_limit_and_multiplier(side: i32, i: i32) -> (i32, i32, i32) {
		let number_of_levels = ((side as f64 / 2.0) as f64).ceil() as usize;
		let mut limit = (side * 3) + (side - 4);
		let mut plimit = 0;
		let mut multiplier: i32 = 0;
		for lvl in 1..number_of_levels {
			if i >= limit {
				plimit = limit;
				let offset = ((side - lvl as i32 * 2) * 3) + ((side - lvl as i32 * 2) - 4);
				//println!("{:?}{:?}", limit, offset);
				limit += offset;
				multiplier += 1;
			}
		}
		// if i >= limit {
		// 	let offset = ((side - lvl as i32 * 2) * 3) + ((side - lvl as i32 * 2) - 4);
		// 	println!("{:?}{:?}", limit, offset);
		// 	limit += offset;
		// 	multiplier += lvl as i32;
		// 	return (limit, multiplier);
		// }
		return (limit, plimit, multiplier);
	}

	fn get_iterator(climit: i32, limit: i32, i: i32, current_set: i32, plimit: i32) -> i32 {
		let n = climit - 8;
		if i >= limit {
			// return  (i - climit) / 4;
			return (i - plimit) / 4;
		}
		return i / 4;
	}

	fn calculate_index(current_set: i32, current_char: i32, side: i32, i: i32) -> i32 {
		let limit = (side * 3) + (side - 4); 
		let (current_limit, plimit, multiplier)= get_current_limit_and_multiplier(side, i);
		//let multiplier = get_multiplier(i, current_limit);
		println!("\tcurrent multiplier {:?}, limit {:?}, plimit {:?}, climit {:?}, side: {:?}", multiplier, limit, plimit, current_limit, side);
		let foo = 4 * multiplier; 
		let bar = current_set - foo;
		let iterator = get_iterator(current_limit, limit, i, current_set, plimit);
		//println!("\tfoo:{:?}, bar: {:?}", foo, current_set);
		println!("\tI: {:?}, current multiplier {:?}, iterator: {:?}", i, multiplier, iterator);
		//println!("Current set: {:?} Current char: {:?} Current multiplier: {:?} I: {:?}", current_set, current_char, multiplier, i);
		match current_char {
			//0 => return iterator + ((side * multiplier) + multiplier),
			0 => return ((side + 1) * multiplier) + iterator,
			//1 => return (((iterator * side) + side) - 1) + (multiplier * 4),
			1 => return ((iterator + 1) * side - 1) + (multiplier * (side - 1)),
			2 => return ((side * side - 1) - iterator) - ((side + 1) * multiplier),
			//2 => return (side * side - 1 - current_set) - (multiplier * (side + side / 2)),
			3 => return (side * side - ((iterator + 1) * side)) - (multiplier * (side - 1)),
			//3 => return (side * side - ((current_set + 1) * side)) - (multiplier * (side - 1)),
			_ => return 0,
		};
	}

	pub fn encode(s: &str) -> String {
		let str = String::from(s);
		let mut res = vec!['X'; str.len()];
		let mut indices = vec![0; str.len()];
		let side = (res.len() as f64).sqrt().ceil() as usize;
		for i in 0..str.len() {
			let current_set = i / 4;
			let current_char = i % 4;
			let j = calculate_index(current_set as i32, current_char as i32, side as i32, i as i32);
			//println!("{:?}, {:?}", j, str.chars().nth(i).unwrap());
			res[j as usize] = str.chars().nth(i).unwrap();
			indices[i] = j;
			//res.push(str.chars().nth(i).unwrap());
		}
		println!("{:?}", indices);
		println!("{:?}", res);
		//str
		// let square_side = (res.len() as f64).sqrt().ceil() as i32;
		// let mut square = build_square(res.len());
		// println!("{:?}", square_side);
		let s: String = res.into_iter().collect();
		s
	}
	
	pub fn decode(s: &str) -> String {
		let res = String::from(s);
		//println!("{:?}", res);
		res
	}
}

pub fn run () {
	isc::encode("Lorem>ipsum>dolor>sitamet>consectetur>adipiscing!");
	//isc::encode("Romani_Ite>Domum");
}

/* CODEWARS GOOD SOLUTIONS



*/