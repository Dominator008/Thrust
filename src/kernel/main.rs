#[link(name = "rustboot",
       vers = "0.0.1",
       author = "Arcterus",
       license = "MPL v2.0")];

#[allow(ctypes)];
#[no_std];

use drivers::io::console;

pub mod reset;

#[path = "../runtime/mod.rs"]
pub mod runtime;

#[path = "../drivers"]
mod drivers {
	mod io {
		pub mod console;
	}
}

#[no_mangle]
pub fn main() {
   console::clear_screen();
	console::print("iiiiiiiiiiiiiiiiiiiiiiiiiii\niiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\x08\x08\x08\x08\x08test");
}
