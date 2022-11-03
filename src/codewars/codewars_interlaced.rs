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
		let example_2a = "Sic transit gloria mundi";
		let example_2b = "Stsgiriuar i ninmd l otac";
		run_test(example_2a,example_2b);
	}
}

mod isc {
    use std::thread::current;

	fn build_square(size: usize) -> Vec<char> {
		let mut square = Vec::with_capacity(size*size);
		square
		//if square.len() == size {
	}

	fn get_level(side: usize, i: usize) -> usize {
		let number_of_levels = (side / 2) + 1;
		let mut level = 0;
		let mut limit = (side - 1) * (side - 1) - 1;
		for l in 1..number_of_levels {
			println!("L {:?} I: {:?} limit: {:?}", l, i, limit);
			if i <= limit {
				return level;
			}
			level += 1;
			limit += (side - 1 - l) * (side - 1 - l);
		}
		return level;
	}

	fn calculate_index(current_set: usize, current_char: usize, side: usize, i: usize) -> usize {
		let current_set_multiplier = get_level(side, i);
		println!("\ncurrent multiplier {:?}", current_set_multiplier);
		let foo = 4 * current_set_multiplier; 
		println!("foo:{:?}, bar: {:?}", foo, current_set);
		let bar = current_set - foo;
		let iterator = bar % 4;
		println!("I: {:?}, current multiplier {:?}, iterator: {:?}, foo:{:?}", i, current_set_multiplier, iterator, foo);
		//println!("Current set: {:?} Current char: {:?} Current multiplier: {:?} I: {:?}", current_set, current_char, current_set_multiplier, i);
		match current_char {
			0 => return ((current_set_multiplier * side) + iterator) + (current_set_multiplier),
			//1 => return (((current_set + 1) * side) - 1) - (current_set_multiplier * (side - 1)),
			//1 => return (((iterator * side) + side) - 1) + ((current_set_multiplier * side) - 1),
			//2 => return (side * side - 1 - current_set) - (current_set_multiplier * (side + side / 2)),
			//2 => return (side * side - 1 - current_set) - (current_set_multiplier * (side + side / 2)),
			//3 => return (side * side - ((current_set + 1) * side)) - (current_set_multiplier * (side - 1)),
			//3 => return (side * side - ((current_set + 1) * side)) - (current_set_multiplier * (side - 1)),
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
			let j = calculate_index(current_set, current_char, side, i);
			//println!("{:?}, {:?}", j, str.chars().nth(i).unwrap());
			res[j] = str.chars().nth(i).unwrap();
			indices[i] = j;
			//res.push(str.chars().nth(i).unwrap());
		}
		println!("{:?}", indices);
		//println!("{:?}", res);
		str
		// let square_side = (res.len() as f64).sqrt().ceil() as i32;
		// let mut square = build_square(res.len());
		// println!("{:?}", square_side);
		// res
	}
	
	pub fn decode(s: &str) -> String {
		let res = String::from(s);
		println!("{:?}", res);
		res
	}
}

pub fn run () {
	isc::encode("Kanasta_onpa_paska_peli!!");
	//isc::encode("Romani_Ite>Domum");
}

/* CODEWARS GOOD SOLUTIONS



*/