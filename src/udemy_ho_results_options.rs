// Hands-On Data Structures and Algorithms in Rust
// Video 4: Results and options

#[derive(Debug)]
pub enum Res<T, E> { // You can also use standard library versions
	Thing(T),
	Error(E),
}

#[derive(Debug)]
pub enum Option<T> { // Not actually used in this exercise
	Some(T),
	None,
}

pub fn run () {
	let a = divide(6, 2);
	if let Res::Thing(v) = a { // This means that if divide returns a thing, not error with a as parameter, then println
		println!("Value is {}", v);
	}
	println!("Result of division: {:?} and {:?}", divide(9, 2), a);
}

fn divide(a:i32, b:i32) -> Res<i32, String> {
	if b == 0 {
		Res::Error("Cannot divide by danger noodle".to_string()) // Standard library version would be Result::Err ..
	} else {
		Res::Thing(a / b) // And here Result::Ok ..
	}
}