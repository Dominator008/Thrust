#[link(name = "kRnel",
       vers = "0.0.1",
       author = "Arcterus",
       license = "MPL v2.0")];

#[allow(ctypes)];
#[no_std];

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
	mod io {
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
