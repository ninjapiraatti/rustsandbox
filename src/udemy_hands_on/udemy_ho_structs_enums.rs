// Hands-On Data Structures and Algorithms in Rust
// Video 3: Structs and enums

#[derive(Debug)]
pub struct Aninmal {
	name: String,
	toes: i32,
	puppers: i32,
	color: Color
}

#[derive(Debug)]
pub enum Color {
	Red(String),
	Green,
}

impl Aninmal {
	pub fn print(self) -> String {
		format!("name = {}, toes = {}, puppers = {}", self.name, self.toes, self.puppers)
	}
}

pub fn run () {
    let p = Aninmal {
		name: "Kylpynalle".to_string(),
		toes: 10,
		puppers: 17,
		color: Color::Green
	};
	let c = Color::Red("Lol".to_string());
	match c {
		Color::Red(s) => println!("color: {}", s),
		Color::Green => println!("Green"),
	}
	println!("aninmal is: {}", p.print())
}