#[link(name = "rustboot",
       vers = "0.0.1",
       author = "Arcterus",
       license = "MPL v2.0")];

#[allow(ctypes)];
#[no_std];
#[no_core];

//use runtime::*;
use drivers::io::console;

#[path = "runtime/runtime.rs"]
pub mod runtime;

mod drivers {
	mod io {
		#[path = "console/console.rs"]
		pub mod console;
	}
}

#[no_mangle]
pub unsafe fn main() {
   console::x86::clear_screen(console::DarkGray);
}
