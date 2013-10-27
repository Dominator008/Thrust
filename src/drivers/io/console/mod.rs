#[author = "Arcterus"];
#[license = "MPL v2.0"];

pub use self::target::*;
use runtime::iter;
use runtime::str::*;

#[path = "arch/target/console.rs"]
pub mod target;

pub enum Color {
	Black      = 0,
	Blue       = 1,
	Green      = 2,
	Cyan       = 3,
	Red        = 4,
	Magenta    = 5,
	Brown      = 6,
	LightGray  = 7,
	DarkGray   = 8,
	LightBlue  = 9,
	LightGreen = 10,
	LightCyan  = 11,
	LightRed   = 12,
	Pink       = 13,
	Yellow     = 14,
	White      = 15
}

pub static BACKGROUND_COLOR: Color = DarkGray;
pub static FOREGROUND_COLOR: Color = LightCyan;

#[packed]
struct ScreenChar {
	char: u8,
	attr: u8
}

pub type Screen = [ScreenChar, ..SCREEN_SIZE];

static mut row: uint = 0;
static mut col: uint = 0;

pub fn print(msg: &str) {
	color_print(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

pub fn clear_screen() {
	color_clear_screen(BACKGROUND_COLOR);
}

pub fn color_print(msg: &str, foreground: Color, background: Color) {
	unsafe {
		do msg.each_byte() |byte| {
			let pos = row * MAX_COLUMN + col;
			(*SCREEN)[pos].char = byte as u8;
			(*SCREEN)[pos].attr = ((background as u8) << 4) + (foreground as u8);
			col += 1;
			if col == MAX_COLUMN {
				col = 0;
				row += 1;
				if row == MAX_ROW {
					row -= 1;
					clear_line(row, background);
				}
			}
			true
		};
	}
}

pub fn color_clear_screen(background: Color) {
	unsafe {
		do iter::range(0, MAX_ROW) |i| {
			clear_line(*i, background);
		};
		row = 0;
		col = 0;
	}
}

fn clear_line(row: uint, background: Color) {
	let pos = row * MAX_COLUMN;
	unsafe {
		do iter::range(0, MAX_COLUMN) |i| {
			(*SCREEN)[pos + *i].attr = (background as u8) << 4;
		}
	}
}

