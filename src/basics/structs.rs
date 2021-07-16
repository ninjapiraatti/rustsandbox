// Structs

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Function inside a struct
impl Color {
    fn new(nred: u8, ngreen: u8, nblue: u8) -> Color {
        Color {
            red: nred,
            green: ngreen,
            blue: nblue,
        }
    }
    fn fullcolor(&self) -> String {
        format!("R{}, G{}, B{}", self.red, self.green, self.blue)
    }
    fn minusred(&mut self, red: u8) {
        self.red -= red;
    }
}

// Tuple struct
struct Tuplecolor(u8, u8, u8);

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 255,
    };
    c.green = 90;
    println!("{} {} {}", c.red, c.green, c.blue);

    let t = Tuplecolor(255, 56, 0);
    println!("{} {} {}", t.0, t.1, t.2);

    c.minusred(3);
    let newcolor = Color::new(24, 48, 72);
    println!("{} {} {}", newcolor.red, newcolor.green, newcolor.blue);
    println!("{}", c.fullcolor());
}
