#[author = "Arcterus"];
#[license = "MPL v2.0"];

use runtime::uint;
use drivers::io::console;

pub unsafe fn clear_screen(background: console::Color) {
	do uint::iterate_step(0, 2*80*25, 2) |i| {
		*((0xb8000 + i) as *mut u16) = (background as u16) << 12;
		true
	};
}
