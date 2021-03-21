// Hands-On Data Structures and Algorithms in Rust
// Video 8: Use Memory Effectively with Pointers and Lifetimes

#[derive(Debug, Clone)] // String doesn't implement copy so you can't use it here. Clone instead
pub struct Aninmal {
	name: String,
	isfloof: bool,
	tousywoesies: i32
}

impl Aninmal {
	pub fn new(name:String, isfloof:bool, tousywoesies:i32) -> Self {
		Aninmal{name, isfloof, tousywoesies}
	}

	pub fn greet(&self) -> String { // If &self would be self instead, it would be borrowed but not returned back.
		format!("Henlo! I am {}", self.name)
	}

	pub fn makefloof(&mut self){
		self.isfloof = true;
	}

	pub fn dropme(self) {}
}

pub fn run () {
	let mut dangernoodle = Aninmal::new("Dangernoodle".to_string(), false, 0);
	println!("{:?}", dangernoodle.greet());
	println!("{:?}", dangernoodle);
	let a = count_toesy_woesies(&dangernoodle);
	//dangernoodle.makefloof(); // You can't borrow dangernoodle as mutable because it's already borrowed in the line above
	println!("{:?}", a);
	dangernoodle.makefloof(); // But here it works, because it's no longer borrowed
	println!("{:?}", dangernoodle);
	dangernoodle.dropme();
	//println!("{:?}", dangernoodle); // Dropme deletes dangernoodle
}

pub fn count_toesy_woesies(aninmal: &Aninmal) -> &i32 {
	&aninmal.tousywoesies
}