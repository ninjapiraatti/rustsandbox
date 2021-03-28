extern crate termion;
use termion::color;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};

mod graphics {
	pub const HORIZONTAL_WALL: &'static str = "═"; // Public constant. The &'static (I think) tells the program that this will live until the end.
}

use self::graphics::*;

/// The UI state.
struct UI<R, W> {
    width: usize,
    height: usize,
    /// Standard input.
    stdin: R,
    /// Standard output.
    stdout: W,
	random: usize,
}

impl <R: Read, W: Write> UI<R, W> {
	fn draw_horizontal_line(&mut self, chr: &str, width: u16) {
		for _ in 0..width { self.stdout.write(chr.as_bytes()).unwrap(); }
	}

	fn reset() {

	}

	fn update(&mut self) -> bool{
		let mut key_bytes = [0];
		let width: u16 = self.width as u16;
        let height: u16 = self.height as u16;
		let random: u16 = self.random as u16;
        self.stdin.read(&mut key_bytes).unwrap();

        //self.rand.write_u8(key_bytes[0]);

        match key_bytes[0] {
            b'q' => return false,
            b'k' | b'w' => self.draw_horizontal_line(HORIZONTAL_WALL, width),
            b'j' | b's' => self.draw_horizontal_line(HORIZONTAL_WALL, height),
            b'h' | b'a' => self.draw_horizontal_line(HORIZONTAL_WALL, random),
            b'l' | b'd' => self.draw_horizontal_line(HORIZONTAL_WALL, width),
            _ => {},
        }
        true
	}
}

fn init_ui(width: usize, height: usize, random: usize) {
	let stdout = stdout();
	let mut stdout = stdout.lock().into_raw_mode().unwrap();
	let stdin = stdin();
	let stdin = stdin.lock();
	write!(stdout,
		"{}{}{}yo, 'q' will exit.{}{}",
		termion::clear::All,
		termion::cursor::Goto(5, 5),
		termion::style::Bold,
		termion::style::Reset,
		termion::cursor::Goto(20, 10))
		.unwrap();
	stdout.flush().unwrap();
	let mut ui = UI {
		width: width,
		height: height,
		stdin: stdin,
		stdout: stdout,
		random: random
	};
	ui.update();
}

pub fn main(nbr: usize) {
    // Initialize termion stuff.
	init_ui(80, 40, nbr);
}