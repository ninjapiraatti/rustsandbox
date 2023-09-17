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
		assert_eq!(&isc::decode(s2),s1);
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
		let example_3b = "Lmsdrtoetntr>>idicpsuoug!ieo>csni>>poatecaemslmir";
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
	fn get_current_limit_and_multiplier(side: i32, i: i32) -> (i32, i32) {
		let number_of_levels = ((side as f64 / 2.0) as f64).ceil() as usize;
		let mut limit = (side * 3) + (side - 4);
		let mut plimit = 0;
		let mut multiplier: i32 = 0;
		for lvl in 1..number_of_levels {
			if i >= limit {
				plimit = limit;
				let offset = ((side - lvl as i32 * 2) * 3) + ((side - lvl as i32 * 2) - 4);
				limit += offset;
				multiplier += 1;
			}
		}
		return (plimit, multiplier);
	}

	fn get_iterator(limit: i32, i: i32, plimit: i32) -> i32 {
		if i >= limit {
			return (i - plimit) / 4;
		}
		return i / 4;
	}

	fn calculate_index(current_char: i32, side: i32, i: i32) -> i32 {
		let limit = (side * 3) + (side - 4); 
		let (plimit, multiplier)= get_current_limit_and_multiplier(side, i);
		let iterator = get_iterator(limit, i, plimit);
		match current_char {
			0 => return ((side + 1) * multiplier) + iterator,
			1 => return ((iterator + 1) * side - 1) + (multiplier * (side - 1)),
			2 => return ((side * side - 1) - iterator) - ((side + 1) * multiplier),
			3 => return (side * side - ((iterator + 1) * side)) - (multiplier * (side - 1)),
			_ => return 0,
		};
	}

	pub fn encode(s: &str) -> String {
		let str = String::from(s);
		let mut indices = vec![0; str.len()];
		let side = (str.len() as f64).sqrt().ceil() as usize;
		let mut res = vec![' '; side * side];
		for i in 0..str.len() {
			let current_char = i % 4;
			let j = calculate_index(current_char as i32, side as i32, i as i32);
			res[j as usize] = str.chars().nth(i).unwrap();
			indices[i] = j;
		}
		let st: String = res.into_iter().collect();
		println!("{:?}", st);
		st
	}
	
	pub fn decode(s: &str) -> String {
		let str = String::from(s);
		let side = (str.len() as f64).sqrt().ceil() as usize;
		let mut res = vec![' '; side * side];
		for i in 0..str.len() {
			let current_char = i % 4;
			let j = calculate_index(current_char as i32, side as i32, i as i32);
			res[i as usize] = str.chars().nth(j as usize).unwrap();
		}

		let st: String = res.into_iter().collect();
		let tr: &str = &st[..];
		let res = tr.trim_end();
		return res.to_string();
	}
}

pub fn run () {
	isc::encode("Lorem>ipsum>dolor>sitamet>consectetur>adipiscing!");
	isc::decode("Lmsdrtoetntr>>idicpsuoug!ieo>csni>>poatecaemslmir");
}

/* CODEWARS GOOD SOLUTIONS



*/