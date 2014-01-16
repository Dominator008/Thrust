#[crate_id = "kRnel#0.0.1"];

#[allow(ctypes)];
#[no_std];
#[feature(globs)];
#[feature(asm)];


pub use drivers::io::console;
pub use target::reset::*;

#[path = "arch/target"]
mod target {
	pub mod reset;
}

#[path = "../runtime/mod.rs"]
pub mod runtime;

#[path = "../drivers"]
mod drivers {
	pub mod io {
		pub mod console;
	}
}

pub mod error;

#[no_mangle]
#[start]
pub fn main() {
	console::clear_screen();
	console::print("iiiiiiiiiiiiiiiiiiiiiiiiiii\niiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\x08\x08\x08\x08\x08test");
	console::println("");
	error::panic("End of kernel");
}
